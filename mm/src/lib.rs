extern crate reqwest;
extern crate scraper;

use reqwest::blocking;
use std::error::Error;
use std::fmt;

use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

#[derive(Default)]
pub struct Client {
    client: blocking::Client,
    base_url: String,
}

#[derive(Debug, Clone)]
pub struct MError {
    str: String,
}

fn error(str: &str) -> Result<(), Box<dyn Error>> {
    Err(Box::new(MError {
        str: str.to_owned(),
    }))
}

impl fmt::Display for MError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl Error for MError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn get_input_value(e: &ElementRef, name: &str) -> Result<String, String> {
    let sel = Selector::parse(&format!("input[name=\"{}\"]", name)).unwrap();
    let input = e.select(&sel).next().unwrap();
    let value = input.value().attr("value");
    if value.is_none() {
        return Err(format!("input field named {} not found", name));
    }
    Ok(value.unwrap().to_owned())
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
        // go to mattermost /oauth/gitlab/login, get redirected to gitlab /oauth/authorize
        let url = format!("{}/{}", self.base_url, "oauth/gitlab/login");
        let resp = self.client.get(url.as_str()).send()?;
        if !resp.status().is_redirection() {
            return error(&format!(
                "expected a redirection, got {}\n  url: {}",
                resp.status(),
                resp.url()
            ));
        }
        let redirect_str = resp
            .headers()
            .get(reqwest::header::LOCATION)
            .ok_or("NO LOCATION")?
            .to_str()?;
        let authorize_url = reqwest::Url::parse(redirect_str)?;
        if authorize_url.path() != "/oauth/authorize" {
            return error(&format!(
                "expected /oauth/authorize, got {}\n  url {}",
                authorize_url.path(),
                resp.url()
            ));
        }

        // call gitlab/oauth/authorize
        let resp = self.client.get(authorize_url.as_str()).send()?;
        if !resp.status().is_redirection() {
            return error(&format!("expected a redirection, got {}", resp.status()));
        }
        let redirect_str = resp
            .headers()
            .get(reqwest::header::LOCATION)
            .ok_or("NO LOCATION")?
            .to_str()?;
        let login_or_complete_redirection = reqwest::Url::parse(redirect_str)?;
        if login_or_complete_redirection.path() == "/users/sign_in" {
            // do user login
            let resp = self
                .client
                .get(login_or_complete_redirection.as_str())
                .send()?;
            let content = resp.text()?;
            let doc = Html::parse_document(&content);
            let form_selector = Selector::parse("#new_user").unwrap();
            let form_html = doc.select(&form_selector).last().unwrap();
            let utf8 = get_input_value(&form_html, "utf8")?;
            let authenticity_token = get_input_value(&form_html, "authenticity_token")?;

            let form = blocking::multipart::Form::new()
                .text("utf8", utf8)
                .text("authenticity_token", authenticity_token)
                .text("user[login]", username.to_owned())
                .text("user[password]", password.to_owned())
                .text("user[remember_me]", "1");
            let resp = self
                .client
                .post(login_or_complete_redirection)
                .multipart(form)
                .send()?;
            println!("{:?}", resp);
        // now follow to gitlab/authorize and then to complete
        } else if login_or_complete_redirection.path() == "/signup/gitlab/complete" {
            unimplemented!("follow to complete and get token")
        }

        Ok(())
    }
}
