use anyhow::{anyhow, Result};
use mm;
use mm::Gitlab;
use std::collections::HashMap;
use std::env;
use crate::core;
use tungstenite::Message;

pub struct Plugin {
    client: mm::Client,
    token: String,
    to_core: core::Sender,
    users: HashMap<String, mm::User>,
    channels: HashMap<String, mm::Channel>,
}

impl Plugin {
    pub fn init(to_core: core::Sender) -> Result<Self> {
        let (client, token) = login_with_envvars()?;
        println!("token: {}", token);
        Ok(Plugin {
            client,
            token,
            to_core,
            users: HashMap::new(),
            channels: HashMap::new(),
        })
    }

    pub fn run(self) -> Result<()> {
        let teams = self.client.get_user_teams("me")?;
//        self.channels = get_all_channels(&self.client, &self.teams)?;
        let mut ws = self.client.ws()?;
        ws.login(&self.token)?;
        loop {
            let msg = ws.wait_for_event()?;
            match msg {
                Message::Text(str) => self.to_core.send(core::Event::Message(str.clone()))?,
                _ => { dbg!(msg); () },
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

fn create_500_posts(c: &mm::Client, channel: &mm::Channel) -> Result<()> {
    // send a lot a post
    for i in 21..500 {
        let r = c.create_post(&channel.id, &format!("{}", i))?;
        println!("{} : {:?}", i, r)
    }
    Ok(())
}

fn display_postlist(
    c: &mm::Client,
    posts: &mm::PostList,
    users: &mut HashMap<String, mm::User>,
) -> Result<()> {
    for post_id in posts.order.iter() {
        let post = &posts.posts[post_id];
        let user = users
            .entry(post.user_id.clone())
            .or_insert(c.get_user(&post.user_id)?);
        println!(
            "{} {}: {}\n  {}",
            post.created(),
            user.display_name(),
            post.message,
            serde_json::to_string_pretty(&post.metadata)?
        );
    }
    Ok(())
}
