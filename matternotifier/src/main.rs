use std::collections::HashMap;
use std::env;
use std::error::Error;

use tungstenite;
use tungstenite::Message;

use rusqlite;
use rusqlite::{params, Connection};

use mm::Gitlab;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_gtk();
    return Ok(());

    let url = env::var_os("MM_URL").ok_or("Please define env var MM_URL")?;
    let token_var = env::var_os("MM_TOKEN");
    let (c, token) = if let Some(token) = token_var {
        let token = token.into_string().unwrap();
        (mm::Client::new(url.to_str().unwrap(), Some(&token)), token)
    } else {
        let user = env::var_os("MM_USER").ok_or("Please define env var MM_USER")?;
        let pass = env::var_os("MM_PASS").ok_or("Please define env var MM_PASS")?;
        let c = mm::Client::new(url.to_str().unwrap(), None);
        let token = c.login_with_gitlab(user.to_str().unwrap(), pass.to_str().unwrap())?;
        (c, token)
    };
    let me = c.get_user("me")?;
    println!("Hello, {}!", me.nickname);

    let teams = c.get_user_teams("me")?;
    let channel_name = "Suivi ZMASTER";
    let channels = get_all_channels(&c, &teams)?;
    let chan1_res = channels.binary_search_by_key(&channel_name, |c| &c.display_name);
    if chan1_res.is_err() {
        return mm::error(&format!("no channel named {}", channel_name))?;
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

    /* TODO
       - /api/v4/users/{user_id}/teams/members
         get user's team members
       - websocket api
    */

    let mut ws = c.ws()?;
    ws.login(&token)?;
    loop {
        let msg = ws.wait_for_event()?;
        dbg!(msg);
    }
    Ok(())
}

use gio::prelude::*;
use gio::ListStore;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Grid, Label, ListBox, Notebook};

fn test_gtk() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let notebook = Notebook::new();
        let g1 = create_grid("Coucou");
        let g2 = create_grid("Hello");
        notebook.add(&g1);
        notebook.set_tab_label_text(&g1, "grille 1");
        notebook.add(&g2);
        notebook.set_tab_label_text(&g2, "grille 2");
        window.add(&notebook);
        window.show_all();
    });

    application.run(&[]);
}

fn create_grid(title: &str) -> ListBox {
    let grid = ListBox::new();
    let store = ListStore::new();
    // see https://github.com/gtk-rs/examples/blob/master/src/bin/list_store.rs
    for i in 1..5 {
        store.append(model)
    }
    grid
}

fn test_sqlite() -> Result<(), Box<dyn Error>> {
    // test sqlite
    println!("sqlite: {}", rusqlite::version());
    let conn = Connection::open("./matter.db")?;
    let mut stmt =
        conn.prepare("INSERT INTO posts (user_id, channel_id, message) values (?, ?, ?)")?;
    stmt.execute(&["u123", "c456", "plop 1"])?;
    stmt.execute(&["u123", "c456", "plop 2"])?;
    stmt.execute(&["u123", "c456", "plop 3"])?;
    stmt.execute(&["u123", "c456", "plop 4"])?;
    return Ok(());
}

fn get_all_channels(
    c: &mm::Client,
    teams: &Vec<mm::Team>,
) -> Result<Vec<mm::Channel>, Box<dyn Error>> {
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

fn create_500_posts(c: &mm::Client, channel: &mm::Channel) -> Result<(), Box<dyn Error>> {
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
