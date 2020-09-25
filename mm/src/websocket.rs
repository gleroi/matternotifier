use tungstenite;
use tungstenite::Message;
use reqwest;

use super::Error;

type WebSocket = tungstenite::WebSocket<tungstenite::client::AutoStream>;

pub struct EventStream {
    action_seq: u32,
    ws: WebSocket
}

impl EventStream {
    pub fn connect(url: &reqwest::Url) -> Result<EventStream, Error> {
        let (ws, resp) = tungstenite::connect(url)?;
        if resp.status().is_client_error() || resp.status().is_server_error() {
            Err(Error::Other(format!("websocket: {}", resp.status().canonical_reason().unwrap_or("unknown error"))))
        } else {
            Ok(EventStream {
                action_seq: 0,
                ws: ws
            })
        }
    }

    pub fn login(&mut self, token: &str) -> Result<(), Error> {
        self.action_seq += 1;
        let _ = self.ws.write_message(Message::Text(format!("{{
          \"seq\": {},
          \"action\": \"authentication_challenge\",
          \"data\": {{
              \"token\": \"{}\"
          }}
        }}", self.action_seq, &token)))?;
        Ok(())
    }
    
    pub fn wait_for_event(&mut self) -> Result<Message, Error> {
        let m = self.ws.read_message()?;
        Ok(m)
    }
}

