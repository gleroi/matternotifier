use std::collections::HashMap;
use std::env;
use std::error::Error;

use mm::Gitlab;

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
    let channel_name = "Town Square";
    let chan1_res = channels.binary_search_by_key(&channel_name, |c| &c.display_name);
    if chan1_res.is_err() {
        return mm::error(&format!("no channel named {}", channel_name));
    }
    let chan1_idx = chan1_res.unwrap();
    let chan1 = &channels[chan1_idx];

    let mut users: HashMap<String, mm::User> = HashMap::new();
    // go through pages of posts
    let mut last_post_id = "".to_owned();
    loop {
        let posts = c
            .get_channel_posts(&chan1.id)
            .page_before(&last_post_id, None, None)?;
        println!(
            "received {} posts, next {}, prev {}",
            posts.posts.len(),
            posts.next_post_id,
            posts.prev_post_id
        );
        display_postlist(&c, &posts, &mut users)?;
        if posts.prev_post_id == "" {
            break;
        }
        last_post_id = posts.prev_post_id;
    }

    // list post of a PostList
    // let posts = c.get_channel_posts(&chan1.id).get()?;
    // let mut users: HashMap<String, mm::User> = HashMap::new();

    // send a lot a post
    // for i in 21..500 {
    //     let r = c.create_post(&chan1.id, &format!("{}", i));
    //     println!("{} : {:?}", i, r)
    // }

    /* TODO
       - /api/v4/users/{user_id}/teams/members
         get user's team members
       - websocket api
       - rework error returned type in mm crate
         maybe like (?) => enum mm::Error {
             Api(status_code, id, message, request_id)
             Http(reqwest::Error)
             Other(String)
         }
    */
    Ok(())
}

fn display_postlist(
    c: &mm::Client,
    posts: &mm::PostList,
    users: &mut HashMap<String, mm::User>,
) -> Result<(), Box<dyn Error>> {
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
