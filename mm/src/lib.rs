use std::error::Error;
use std::fmt;

use reqwest::blocking;

use serde::Deserialize;

use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;

#[derive(Default)]
pub struct Client {
    client: blocking::Client,
    base_url: String,
    token: String,
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
        .ok_or(format!("NO LOCATION: \n{:?}", resp));
    match value {
        Err(err) => error(&err),
        Ok(header_value) => Ok(header_value.to_str()?),
    }
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        let c = blocking::Client::builder()
            .cookie_store(true)
            .danger_accept_invalid_certs(true)
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:79.0) Gecko/20100101 Firefox/79.0",
            )
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .build()
            .unwrap();
        Client {
            client: c,
            base_url: base_url.to_owned(),
            token: "".to_owned(),
        }
    }

    fn url(&self, path: &str) -> reqwest::Url {
        reqwest::Url::parse(&self.base_url)
            .unwrap()
            .join(path)
            .unwrap()
    }

    /// login_with_gitlab returns a mattermost token or else an error
    pub fn login_with_gitlab(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<String, Box<dyn Error>> {
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

    fn call_gitlab_authorize(
        &mut self,
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
            self.token = token.clone();
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

    pub fn get_user(&self, user_id: &str) -> Result<User, Box<dyn Error>> {
        let url = self.url(&format!("/api/v4/users/{}", user_id));
        let req = self.client.get(url);
        let resp = req.send()?;
        let user = resp.json::<User>()?;
        Ok(user)
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct User {
    id: String,
    /// The time in milliseconds a user was created
    create_at: i64,
    /// The time in milliseconds a user was last updated
    update_at: i64,
    /// The time in milliseconds a user was deleted
    delete_at: i64,
    username: String,
    first_name: String,
    last_name: String,
    nickname: String,
    email: String,
    email_verified: bool,
    auth_service: String,
    roles: String,
    locale: String,
    notify_props: UserNotifyProps,
    //props	: object
    last_password_update: i64,
    last_picture_update: Option<i64>,
    failed_attempts: Option<i64>,
    mfa_active: Option<bool>,
    timezone: Timezone,
    // ID of accepted terms of service, if any. This field is not present if empty.
    terms_of_service_id: Option<String>,
    /// The time in milliseconds the user accepted the terms of service
    terms_of_service_create_at: Option<i64>,
}

#[derive(Default, Debug, Deserialize)]
pub struct UserNotifyProps {
    /// Set to "true" to enable email notifications, "false" to disable. Defaults to "true".
    email: String,
    /// Set to "all" to receive push notifications for all activity, "mention" for mentions and direct messages only, and "none" to disable. Defaults to "mention".
    push: String,
    /// Set to "all" to receive desktop notifications for all activity, "mention" for mentions and direct messages only, and "none" to disable. Defaults to "all".
    desktop: String,
    /// Set to "true" to enable sound on desktop notifications, "false" to disable. Defaults to "true".
    desktop_sound: String,
    /// A comma-separated list of words to count as mentions. Defaults to username and @username.
    mention_keys: String,
    /// Set to "true" to enable channel-wide notifications (@channel, @all, etc.), "false" to disable. Defaults to "true".
    channel: String,
    /// Set to "true" to enable mentions for first name. Defaults to "true" if a first name is set, "false" otherwise.
    first_name: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Timezone {
    /// Set to "true" to use the browser/system timezone, "false" to set manually. Defaults to "true".
    #[serde(rename = "useAutomaticTimezone")]
    use_automatic_timezone: String,

    /// Value when setting manually the timezone, i.e. "Europe/Berlin".
    #[serde(rename = "manualTimezone")]
    manual_timezone: String,
    /// This value is set automatically when the "useAutomaticTimezone" is set to "true".
    #[serde(rename = "automaticTimezone")]
    automatic_timezone: String,
}
