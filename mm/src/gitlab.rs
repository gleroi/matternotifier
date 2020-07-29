use std::error::Error;

use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

use reqwest::blocking;

use super::error;
use super::Client;

pub trait Gitlab {
    fn login_with_gitlab(&self, username: &str, password: &str) -> Result<String, Box<dyn Error>>;
}

impl Gitlab for Client {
    /// login_with_gitlab returns a mattermost token or else an error
    fn login_with_gitlab(&self, username: &str, password: &str) -> Result<String, Box<dyn Error>> {
        // go to mattermost /oauth/gitlab/login, get redirected to gitlab /oauth/authorize
        let url = self.url("oauth/gitlab/login");
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
}

impl Client {
    fn call_gitlab_authorize(
        &self,
        url: &str,
        username: &str,
        password: &str,
    ) -> Result<String, Box<dyn Error>> {
        let authorize_url = reqwest::Url::parse(url)?;
        if authorize_url.path() != "/oauth/authorize" {
            return error(&format!("expected /oauth/authorize, got {}", authorize_url));
        }

        let resp = self.client.get(url).send()?;
        if !resp.status().is_redirection() {
            return error(&format!("expected a redirection, got {}", resp.status()));
        }
        let redirect_str = get_header_location(&resp)?;
        let login_or_complete_redirection = reqwest::Url::parse(redirect_str)?;
        if login_or_complete_redirection.path() == "/users/sign_in" {
            // do user login
            let resp = self
                .client
                .get(login_or_complete_redirection.as_str())
                .send()?;
            let origin = login_or_complete_redirection
                .origin()
                .unicode_serialization();
            let content = resp.text()?;
            let resp = self
                .submit_gitlab_ldap_form(&origin, &content, username, password)
                .or_else(|_| self.submit_gitlab_form(&origin, &content, username, password))?;
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
            let token = header_token.to_str()?.to_owned();
            return Ok(token);
        }
        error(&format!(
            "unknown url while authenticating to gitlab:\n {}",
            login_or_complete_redirection.path()
        ))
    }

    fn submit_gitlab_form(
        &self,
        origin: &str,
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
        let url_path = form_html
            .value()
            .attr("action")
            .ok_or("form has not action attribute")?;
        let utf8 = get_input_value(&form_html, "utf8")?;
        let authenticity_token = get_input_value(&form_html, "authenticity_token")?;

        let form = blocking::multipart::Form::new()
            .text("utf8", utf8)
            .text("authenticity_token", authenticity_token)
            .text("user[login]", username.to_owned())
            .text("user[password]", password.to_owned())
            .text("user[remember_me]", "1");
        let url = reqwest::Url::parse(origin)?.join(&url_path)?;
        let req = self.client.post(url).multipart(form);
        let resp = req.send()?;
        if !resp.status().is_redirection() {
            return error(&format!(
                "expected a redirection, got {}\n{:?}",
                resp.status(),
                resp.text()?
            ));
        }
        Ok(resp)
    }

    fn submit_gitlab_ldap_form(
        &self,
        origin: &str,
        page: &str,
        username: &str,
        password: &str,
    ) -> Result<blocking::Response, Box<dyn Error>> {
        let doc = Html::parse_document(page);
        let form_selector = Selector::parse("#new_ldap_user").unwrap();
        let form_html = doc
            .select(&form_selector)
            .last()
            .ok_or("#new_ldap_user not found in form")?;
        let url_path = form_html
            .value()
            .attr("action")
            .ok_or("form has not action attribute")?;
        let utf8 = get_input_value(&form_html, "utf8")?;
        let authenticity_token = get_input_value(&form_html, "authenticity_token")?;

        let form = [
            ("utf8", utf8),
            ("authenticity_token", authenticity_token),
            ("username", username.to_owned()),
            ("password", password.to_owned()),
            ("remember_me", "1".to_owned()),
        ];
        let url = reqwest::Url::parse(origin)?.join(&url_path)?;
        let req = self.client.post(url).form(&form);
        let resp = req.send()?;
        if !resp.status().is_redirection() {
            return error(&format!(
                "expected a redirection, got {}\n{:?}",
                resp.status(),
                resp.text()?
            ));
        }
        Ok(resp)
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
        .ok_or(format!("NO LOCATION: \n{:?}", resp));
    match value {
        Err(err) => error(&err),
        Ok(header_value) => Ok(header_value.to_str()?),
    }
}
