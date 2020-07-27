extern crate mm;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var_os("MM_URL").ok_or("Please define env var MM_URL")?;
    let user = env::var_os("MM_USER").ok_or("Please define env var MM_USER")?;
    let pass = env::var_os("MM_PASS").ok_or("Please define env var MM_PASS")?;
    let mut c = mm::Client::new(url.to_str().unwrap());
    let token = c.login_with_gitlab(user.to_str().unwrap(), pass.to_str().unwrap())?;
    println!("token is {}", token);
    let me = c.get_user("me")?;
    println!("i am {:?}", me);
    let teams = c.get_user_teams("me")?;
    println!("my teams are:\n{:?}", teams);
    for team in teams {
        let chans = c.get_user_channels("me", &team.id)?;
        println!("channels are:\n{:?}", chans);
    }
    Ok(())
}
