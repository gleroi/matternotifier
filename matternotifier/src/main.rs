extern crate mm;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var_os("MM_URL").ok_or("Please define env var MM_URL")?;
    let token_var = env::var_os("MM_TOKEN");
    let c = if let Some(token) = token_var {
        mm::Client::new(url.to_str().unwrap(), Some(token.to_str().unwrap()))
    } else {
        let user = env::var_os("MM_USER").ok_or("Please define env var MM_USER")?;
        let pass = env::var_os("MM_PASS").ok_or("Please define env var MM_PASS")?;
        let c = mm::Client::new(url.to_str().unwrap(), None);
        let token = c.login_with_gitlab(user.to_str().unwrap(), pass.to_str().unwrap())?;
        println!("token is {}", token);
        c
    };
    let me = c.get_user("me")?;
    println!("Hello, {}!", me.nickname);
    let teams = c.get_user_teams("me")?;
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
    let chan1_idx =
        channels.binary_search_by_key(&"test".to_owned(), |c| c.display_name.clone()).unwrap();
    let chan1 = &channels[chan1_idx];
    let posts = c.get_channel_posts(&chan1.id)?;
    for (_post_id, post) in posts.posts {
        println!("{}: {}", post.user_id, post.message);
    }
    // TODO
    // - websocket api
    Ok(())
}
