extern crate reqwest;
use reqwest::blocking;
use std::error::Error;

#[derive(Default)]
pub struct Client {
    client: blocking::Client,
    base_url: String,
}

impl Client {
    pub fn new() -> Self {
        let c = blocking::Client::builder()
            .cookie_store(true)
            .danger_accept_invalid_certs(true)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        Client {
            client: c,
            base_url: "https://192.168.122.76:8065".to_owned(),
        }
    }

    pub fn login_with_gitlab(&self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        // go to mattermost page, get redirected to gitlab to get the cookies
        let url = format!("{}/{}", self.base_url, "oauth/gitlab/login");
        let resp = self.client.get(url.as_str()).send()?;
        let redirect_url = resp
            .headers()
            .get(reqwest::header::LOCATION)
            .ok_or("NO LOCATION!!")?;
        println!(
            "{} : status {}\n {}",
            url,
            resp.status(),
            redirect_url.to_str()?
        );
        // Strat 1:
        // call authorize url
        // if redirected to sign in, ask user to navigate to authorize url to login and give us the code
        // else if redirected to complete, call complete to get token and cookies

        // State 2:
        // parse sign_in form to get authentication_token and POST form (use crate scraper)
        Ok(())
    }
}
