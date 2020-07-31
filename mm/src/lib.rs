use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use reqwest::blocking;
use reqwest::header;

use serde::Deserialize;

mod gitlab;
pub use gitlab::Gitlab;

#[derive(Default)]
pub struct Client {
    client: blocking::Client,
    base_url: String,
}

#[derive(Debug, Clone)]
pub struct MError {
    str: String,
}

pub struct ApiError {
    status: String,
    message: String,
    id: String,
}

pub fn error<T>(str: &str) -> Result<T, Box<dyn Error>> {
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

    fn get<T>(&self, api_url: &str) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let req = self
            .client
            .get(self.url(api_url))
            .header(header::CONTENT_TYPE, "application/json");
        let http_result = req.send();
        match http_result {
            Ok(resp) => {
                // TODO: return OK for 4xx et 5xx
                if resp.status().is_success() {
                    let parsed_result = resp.json::<T>();
                    return match parsed_result {
                        Ok(value) => Ok(value),
                        Err(err) => Err(Box::new(err)),
                    };
                } else {
                    let parsed_result = resp.json<>();
                    return match parsed_result {
                        Ok(value) => Ok(value),
                        Err(err) => Err(Box::new(err)),
                    }
                }
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn get_user(&self, user_id: &str) -> Result<User, Box<dyn Error>> {
        let url = self.url(&format!("/api/v4/users/{}", user_id));
        let req = self.client.get(url);
        let resp = req.send()?;
        let user = resp.json::<User>()?;
        Ok(user)
    }

    pub fn get_user_teams(&self, user_id: &str) -> Result<Vec<Team>, Box<dyn Error>> {
        self.get::<Vec<Team>>(&format!("/api/v4/users2/{}/teams", user_id))
    }

    pub fn get_user_channels(
        &self,
        user_id: &str,
        team_id: &str,
    ) -> Result<Vec<Channel>, Box<dyn Error>> {
        let url = self.url(&format!(
            "/api/v4/users/{}/teams/{}/channels",
            user_id, team_id
        ));
        let resp = self.client.get(url).send()?;
        let channels = resp.json::<Vec<Channel>>()?;
        Ok(channels)
    }

    pub fn get_channel_posts(&self, channel_id: &str) -> Result<PostList, Box<dyn Error>> {
        let url = self.url(&format!("/api/v4/channels/{}/posts", channel_id));
        let resp = self.client.get(url).send()?;
        let posts = resp.json::<PostList>()?;
        Ok(posts)
    }

    pub fn create_post(&self, channel_id: &str, msg: &str) -> Result<Post, Box<dyn Error>> {
        let url = self.url("/api/v4/posts");
        let mut cmd: HashMap<&str, &str> = HashMap::new();
        cmd.insert("channel_id", channel_id);
        cmd.insert("message", msg);
        let resp = self
            .client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&cmd)?)
            .send()?;
        let post = resp.json::<Post>()?;
        Ok(post)
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct PostList {
    pub order: Vec<String>,
    pub posts: HashMap<String, Post>,
    /// $ref: '#/components/schemas/Post'
    /// The ID of next post. Not omitted when empty or not relevant.
    pub next_post_id: String,
    /// The ID of previous post. Not omitted when empty or not relevant.
    pub prev_post_id: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Post {
    pub id: String,
    /// The time in milliseconds a post was created,
    pub create_at: i64,
    /// The time in milliseconds a post was last updated
    pub update_at: i64,
    /// The time in milliseconds a post was deleted
    pub delete_at: i64,
    pub edit_at: i64,
    pub user_id: String,
    pub channel_id: String,
    pub root_id: String,
    pub parent_id: String,
    pub original_id: String,
    pub message: String,
    #[serde(rename = "type")]
    pub post_type: String,
    // TODO: is there a better way to handle this ?
    pub props: serde_json::Value,
    pub hashtag: Option<String>,
    // This field will only appear on some posts created before Mattermost
    // 3.5 and has since been deprecated.
    pub filenames: Option<Vec<String>>,
    pub file_ids: Option<Vec<String>>,
    pub pending_post_id: String,
    // pub metadata:
    //     $ref: '#/components/schemas/PostMetadata'
}

impl Post {
    pub fn created(&self) -> chrono::NaiveDateTime {
        let secs = self.create_at / 1000;
        chrono::NaiveDateTime::from_timestamp(secs, 0)
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
