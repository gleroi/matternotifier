use anyhow::{anyhow, Result};
use mm;
use mm::Gitlab;
use std::env;
use crate::core;

pub struct Plugin {
    client: mm::Client,
    token: String,
    to_core: core::Sender,
//    users: HashMap<String, mm::User>,
    channels: Vec<mm::Channel>,
    teams: Vec<mm::Team>
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
        self.channels = get_all_channels(&self.client, &self.teams)?;
        let mut ws = self.client.ws()?;
        ws.login(&self.token)?;
        loop {
            let result = ws.wait_for_event();
            match result {
                Err(err) => { dbg!(err); },
                Ok(evt) => { 
                    if evt.event == "posted" {
                        let posted_evt = serde_json::from_value::<mm::PostedEvent>(evt.data)?;
                        let post = serde_json::from_str::<mm::Post>(&posted_evt.post)?;
                        self.to_core.send(core::Event::Message(format!("{} - {} > {} : {}",
                            post.update_at, posted_evt.channel_display_name, posted_evt.sender_name, post.message)))?;
                    } else {
                        self.to_core.send(core::Event::Info(format!("{} : {}", evt.event, evt.data)))?; 
                    }
                },
            }
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
            println!("  - {}: {}", chan.display_name, chan.header);
        }
        channels.append(&mut chans);
    }
    channels.sort_by_key(|c| c.display_name.clone());
    Ok(channels)
}

