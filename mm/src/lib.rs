use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;

use reqwest::blocking;
use reqwest::header;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod gitlab;
pub use gitlab::Gitlab;

mod posts;
pub use posts::*;

mod websocket;
pub use websocket::*;

#[derive(Default)]
pub struct Client {
    client: blocking::Client,
    base_url: String,
}

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Http(reqwest::Error),
    InvalidUrl(url::ParseError),
    WebSocket(tungstenite::Error),
    Other(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Http(err)
    }
}

impl std::convert::From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Self {
        Self::WebSocket(err)
    }
}

impl std::convert::From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::InvalidUrl(err)
    }
}

impl std::convert::From<&str> for Error {
    fn from(err: &str) -> Self {
        Self::Other(err.to_owned())
    }
}

impl std::convert::From<String> for Error {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct ApiError {
    status_code: i64,
    message: String,
    id: String,
}

pub fn error<T>(str: &str) -> Result<T, Error> {
    Err(Error::Other(str.to_owned()))
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Api(err) => write!(f, "api: {:?}", err),
            Error::Http(err) => write!(f, "http: {}", err),
            Error::InvalidUrl(err) => write!(f, "url: {}", err),
            Error::WebSocket(err) => write!(f, "error: {}", err),
            Error::Other(err) => write!(f, "error: {}", err),
        }
    }
}

impl StdError for Error {}

impl Client {
    pub fn new(base_url: &str, token: Option<&str>) -> Self {
        let mut headers = header::HeaderMap::new();
        if let Some(token_str) = token {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {}", token_str)).unwrap(),
            );
        }
        let c = blocking::Client::builder()
            .cookie_store(true)
            .danger_accept_invalid_certs(true)
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:79.0) Gecko/20100101 Firefox/79.0",
            )
            .redirect(reqwest::redirect::Policy::none())
            .default_headers(headers)
            .no_proxy()
            .build()
            .unwrap();
        Client {
            client: c,
            base_url: base_url.to_owned(),
        }
    }

    fn url(&self, path: &str) -> reqwest::Url {
        reqwest::Url::parse(&self.base_url)
            .unwrap()
            .join(path)
            .unwrap()
    }

    pub fn ws_url(&self) -> reqwest::Url {
        let mut ws_api = reqwest::Url::parse(&self.base_url)
            .unwrap()
            .join("/api/v4/websocket")
            .unwrap();
        ws_api.set_scheme("wss").unwrap();
        ws_api
    }

    pub fn ws(&self) -> Result<EventStream, Error> {
        EventStream::connect(&self.ws_url())
    }

    fn handle_response<T>(http_result: reqwest::Result<blocking::Response>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let resp = http_result?;
        if resp.status().is_success() {
            match resp.json::<T>() {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::Http(err)),
            }
        } else {
            match resp.json::<ApiError>() {
                Ok(err) => Err(Error::Api(err)),
                Err(err) => Err(Error::Http(err)),
            }
        }
    }

    fn get_builder(&self, api_url: &str) -> blocking::RequestBuilder {
        self.client
            .get(self.url(api_url))
            .header(header::CONTENT_TYPE, "application/json")
    }

    fn get<T>(&self, api_url: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let req = self.get_builder(api_url);
        Client::handle_response(req.send())
    }

    fn post<TOutput, TInput>(&self, api_url: &str, body: &TInput) -> Result<TOutput, Error>
    where
        TInput: Serialize,
        TOutput: DeserializeOwned,
    {
        let req = self
            .client
            .post(self.url(api_url))
            .json(&body)
            .header(header::CONTENT_TYPE, "application/json");
        Client::handle_response(req.send())
    }

    pub fn get_user(&self, user_id: &str) -> Result<User, Error> {
        self.get(&format!("/api/v4/users/{}", user_id))
    }

    pub fn get_user_teams(&self, user_id: &str) -> Result<Vec<Team>, Error> {
        self.get(&format!("/api/v4/users/{}/teams", user_id))
    }

    pub fn get_user_channels(&self, user_id: &str, team_id: &str) -> Result<Vec<Channel>, Error> {
        self.get(&format!(
            "/api/v4/users/{}/teams/{}/channels",
            user_id, team_id
        ))
    }

    pub fn get_channel_posts<'a>(&'a self, channel_id: &'a str) -> Pager<'a> {
        Pager::new(self, channel_id)
    }

    pub fn create_post(&self, channel_id: &str, msg: &str) -> Result<Post, Error> {
        let mut cmd: HashMap<&str, &str> = HashMap::new();
        cmd.insert("channel_id", channel_id);
        cmd.insert("message", msg);
        self.post("/api/v4/posts", &cmd)
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct Channel {
    pub id: String,
    /// The time in milliseconds a channel was created
    pub create_at: i64,
    /// The time in milliseconds a channel was last updated
    pub update_at: i64,
    /// The time in milliseconds a channel was deleted
    pub delete_at: i64,
    pub team_id: String,
    #[serde(rename = "type")]
    pub channel_type: String,
    pub display_name: String,
    pub name: String,
    pub header: String,
    pub purpose: String,
    /// The time in milliseconds of the last post of a channel
    pub last_post_at: i64,
    pub total_msg_count: i64,
    /// Deprecated in Mattermost 5.0 release
    pub extra_update_at: i64,
    pub creator_id: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Team {
    pub id: String,
    /// The time in milliseconds a team was created
    pub create_at: i64,
    /// The time in milliseconds a team was last updated
    pub update_at: i64,
    /// The time in milliseconds a team was deleted
    pub delete_at: i64,
    pub display_name: String,
    pub name: String,
    pub description: String,
    pub email: String,
    #[serde(rename = "type")]
    pub team_type: String,
    pub allowed_domains: String,
    pub invite_id: String,
    pub allow_open_invite: bool,
}

#[derive(Default, Debug, Deserialize)]
pub struct User {
    pub id: String,
    /// The time in milliseconds a user was created
    pub create_at: i64,
    /// The time in milliseconds a user was last updated
    pub update_at: i64,
    /// The time in milliseconds a user was deleted
    pub delete_at: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub email: String,
    #[serde(default)]
    pub email_verified: bool,
    pub auth_service: String,
    pub roles: String,
    pub locale: String,
    pub notify_props: Option<UserNotifyProps>,
    //props	: object
    pub last_password_update: Option<i64>,
    pub last_picture_update: Option<i64>,
    pub failed_attempts: Option<i64>,
    pub mfa_active: Option<bool>,
    pub timezone: Timezone,
    // ID of accepted terms of service, if any. This field is not present if empty.
    pub terms_of_service_id: Option<String>,
    /// The time in milliseconds the user accepted the terms of service
    pub terms_of_service_create_at: Option<i64>,
}

impl User {
    pub fn display_name(&self) -> String {
        if !self.nickname.is_empty() {
            return self.nickname.to_owned();
        }
        if !self.first_name.is_empty() && !self.last_name.is_empty() {
            return format!("{} {}", self.first_name, self.last_name);
        }
        self.username.to_owned()
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct UserNotifyProps {
    /// Set to "true" to enable email notifications, "false" to disable. Defaults to "true".
    pub email: String,
    /// Set to "all" to receive push notifications for all activity, "mention" for mentions and direct messages only, and "none" to disable. Defaults to "mention".
    pub push: String,
    /// Set to "all" to receive desktop notifications for all activity, "mention" for mentions and direct messages only, and "none" to disable. Defaults to "all".
    pub desktop: String,
    /// Set to "true" to enable sound on desktop notifications, "false" to disable. Defaults to "true".
    pub desktop_sound: String,
    /// A comma-separated list of words to count as mentions. Defaults to username and @username.
    pub mention_keys: String,
    /// Set to "true" to enable channel-wide notifications (@channel, @all, etc.), "false" to disable. Defaults to "true".
    pub channel: String,
    /// Set to "true" to enable mentions for first name. Defaults to "true" if a first name is set, "false" otherwise.
    pub first_name: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Timezone {
    /// Set to "true" to use the browser/system timezone, "false" to set manually. Defaults to "true".
    #[serde(rename = "useAutomaticTimezone")]
    pub use_automatic_timezone: String,
    /// Value when setting manually the timezone, i.e. "Europe/Berlin".
    #[serde(rename = "manualTimezone")]
    pub manual_timezone: String,
    /// This value is set automatically when the "useAutomaticTimezone" is set to "true".
    #[serde(rename = "automaticTimezone")]
    pub automatic_timezone: String,
}
