use reqwest;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use tungstenite;
use tungstenite::Message;

use super::Error;

type WebSocket = tungstenite::WebSocket<tungstenite::client::AutoStream>;

pub struct EventStream {
    action_seq: u32,
    ws: WebSocket,
}

#[derive(Default, Debug, Deserialize)]
pub struct Event {
    pub event: String,
    pub data: serde_json::Value,
    pub broadcast: Broadcast,
    pub seq: u64,
}

#[derive(Default, Debug, Deserialize)]
pub struct Broadcast {
    pub omit_users: Option<HashMap<String, bool>>,
    pub user_id: Option<String>,
    pub channel_id: Option<String>,
    pub team_id: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct PostedEvent {
    pub channel_display_name: String,
    pub channel_name: String,
    pub channel_type: String,
    pub sender_name: String,
    pub team_id: String,
    pub post: String,
}

impl EventStream {
    pub fn connect(url: &reqwest::Url) -> Result<EventStream, Error> {
        let (ws, resp) = tungstenite::connect(url)?;
        if resp.status().is_client_error() || resp.status().is_server_error() {
            Err(Error::Other(format!(
                "websocket: {}",
                resp.status().canonical_reason().unwrap_or("unknown error")
            )))
        } else {
            Ok(EventStream {
                action_seq: 0,
                ws: ws,
            })
        }
    }

    pub fn login(&mut self, token: &str) -> Result<(), Error> {
        self.action_seq += 1;
        let _ = self.ws.write_message(Message::Text(format!(
            "{{
              \"seq\": {},
              \"action\": \"authentication_challenge\",
              \"data\": {{
                  \"token\": \"{}\"
              }}
            }}",
            self.action_seq, &token
        )))?;
        Ok(())
    }

    pub fn wait_for_event(&mut self) -> Result<Event, Error> {
        loop {
            let msg = self.ws.read_message()?;
            println!("websocket evt: {:#?}", msg);
            match msg {
                Message::Text(str) => {
                    let val: serde_json::Value = serde_json::from_str(&str)?;
                    if let serde_json::Value::Object(map) = val.clone() {
                        if !map.get("event").is_none() {
                            let evt = serde_json::from_value(val)?;
                            return Ok(evt);
                        }
                    }
                    //TODO: handle status event
                }
                Message::Ping(_) => continue,
                _ => return super::error(&format!("unknown message:\n {:#?}", &msg)),
            }
        }
    }
}
