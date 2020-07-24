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

fn error<T>(str: &str) -> Result<T, Box<dyn Error>> {
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

fn get_header_location(resp: &blocking::Response) -> Result<&str, Box<dyn Error>> {
    let value = resp
        .headers()
        .get(reqwest::header::LOCATION)
        .ok_or("NO LOCATION");
    match value {
        Err(err) => error(err),
        Ok(header_value) => Ok(header_value.to_str()?),
    }
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        let c = blocking::Client::builder()
            .cookie_store(true)
            .danger_accept_invalid_certs(true)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        Client {
            client: c,
            base_url: base_url.to_owned(),
        }
    }

    /// login_with_gitlab returns a mattermost token or else an error
    pub fn login_with_gitlab(
        &self,
        username: &str,
        password: &str,
    ) -> Result<String, Box<dyn Error>> {
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
        let redirect_str = get_header_location(&resp)?;
        self.call_gitlab_authorize(redirect_str, username, password)
    }

    fn call_gitlab_authorize(
        &self,
        url: &str,
        username: &str,
        password: &str,
    ) -> Result<String, Box<dyn Error>> {
        let authorize_url = reqwest::Url::parse(url)?;
        if authorize_url.path() != "/oauth/authorize" {
            return error(&format!(
                "expected /oauth/authorize, got {}",
                authorize_url.path()
            ));
        }

        let resp = self.client.get(url).send()?;
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
            let resp = self.submit_gitlab_form(
                login_or_complete_redirection.as_str(),
                &content,
                username,
                password,
            )?;
            // now follow to gitlab/authorize and then to complete
            return self.call_gitlab_authorize(get_header_location(&resp)?, username, password);
        }
        if login_or_complete_redirection.path() == "/signup/gitlab/complete" {
            // complete authentication to get token
            let resp = self
                .client
                .get(login_or_complete_redirection.as_str())
                .send()?;
            let header_token = resp.headers().get("token").ok_or("NO TOKEN!!!")?;
            return Ok(header_token.to_str()?.to_owned());
        }
        error(&format!(
            "unknown url while authenticating to gitlab:\n {}",
            login_or_complete_redirection.path()
        ))
    }

    fn submit_gitlab_form(
        &self,
        url: &str,
        page: &str,
        username: &str,
        password: &str,
    ) -> Result<blocking::Response, Box<dyn Error>> {
        let doc = Html::parse_document(page);
        let form_selector = Selector::parse("#new_user").unwrap();
        let form_html = doc
            .select(&form_selector)
            .last()
            .ok_or("#new_user not found in form")?;
        let utf8 = get_input_value(&form_html, "utf8")?;
        let authenticity_token = get_input_value(&form_html, "authenticity_token")?;

        let form = blocking::multipart::Form::new()
            .text("utf8", utf8)
            .text("authenticity_token", authenticity_token)
            .text("user[login]", username.to_owned())
            .text("user[password]", password.to_owned())
            .text("user[remember_me]", "1");
        Ok(self.client.post(url).multipart(form).send()?)
    }
}
