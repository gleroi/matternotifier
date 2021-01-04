use crate::core;
use crate::core::Event;
use anyhow::{anyhow, Result};
use mm;
use mm::Gitlab;
use std::env;

pub struct Plugin {
    client: mm::Client,
    token: String,
    to_core: core::Sender,
    //    users: HashMap<String, mm::User>,
    channels: Vec<mm::Channel>,
    teams: Vec<mm::Team>,
}

impl Plugin {
    pub fn init(to_core: core::Sender) -> Result<Self> {
        let (client, token) = login_with_envvars()?;
        println!("token: {}", token);
        Ok(Plugin {
            client,
            token,
            to_core,
            //            users: HashMap::new(),
            channels: Vec::new(),
            teams: Vec::new(),
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.teams = self.client.get_user_teams("me")?;
        for team in &self.teams {
            self.to_core.send(Event::NewChannel(core::Channel {
                name: team.name.clone(),
                id: team.id.clone(),
                parent_id: None,
            }))?;
        }
        self.channels = get_all_channels(&self.client, &self.teams)?;
        for channel in &self.channels {
            if channel.channel_type != "D" {
                self.to_core.send(Event::NewChannel(core::Channel {
                    name: channel.pretty_name().clone(),
                    id: channel.id.clone(),
                    parent_id: Some(channel.team_id.clone()),
                }))?;
            }
        }
        let mut ws = self.client.ws()?;
        ws.login(&self.token)?;
        loop {
            let result = ws.wait_for_event();
            match result {
                Err(err) => {
                    dbg!(err);
                }
                Ok(evt) => match evt.event.as_str() {
                    "posted" => {
                        self.to_core
                            .send(Event::Message(core::Message::try_from(evt)?))?;
                    }
                    _ => {
                        self.to_core
                            .send(Event::Info(format!("{} : {}", evt.event, evt.data)))?;
                    }
                },
            }
        }
    }
}

use std::convert::TryFrom;

impl TryFrom<mm::Event> for core::Message {
    type Error = anyhow::Error;
    fn try_from(evt: mm::Event) -> Result<Self, Self::Error> {
        if evt.event == "posted" {
            let posted_evt = serde_json::from_value::<mm::PostedEvent>(evt.data)?;
            let post = serde_json::from_str::<mm::Post>(&posted_evt.post)?;
            Ok(core::Message {
                timestamp: post.update_at,
                channel_name: posted_evt.channel_display_name,
                sender_name: posted_evt.sender_name,
                content: post.message,
            })
        } else {
            Err(anyhow!("expected 'posted' event, got '{}'", evt.event))
        }
    }
}

fn login_with_envvars() -> Result<(mm::Client, String)> {
    let url = env::var_os("MM_URL").ok_or(anyhow!("Please define env var MM_URL"))?;
    let token_var = env::var_os("MM_TOKEN");
    if let Some(token) = token_var {
        let token = token.into_string().unwrap();
        Ok((mm::Client::new(url.to_str().unwrap(), Some(&token)), token))
    } else {
        let user = env::var_os("MM_USER").ok_or(anyhow!("Please define env var MM_USER"))?;
        let pass = env::var_os("MM_PASS").ok_or(anyhow!("Please define env var MM_PASS"))?;
        let c = mm::Client::new(url.to_str().unwrap(), None);
        let token = c.login_with_gitlab(user.to_str().unwrap(), pass.to_str().unwrap())?;
        Ok((c, token))
    }
}

fn get_all_channels(c: &mm::Client, teams: &Vec<mm::Team>) -> Result<Vec<mm::Channel>> {
    let mut channels: Vec<mm::Channel> = Vec::new();
    for team in teams {
        println!("- Team {}\n  {}", team.display_name, team.description);
        let mut chans = c.get_user_channels("me", &team.id)?;
        for chan in &chans {
            println!(
                "  {} {}: {}",
                chan.channel_type, chan.display_name, chan.header
            );
        }
        channels.append(&mut chans);
    }
    channels.sort_by_key(|c| c.display_name.clone());
    Ok(channels)
}
