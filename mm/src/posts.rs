use super::{Client, Error};
use serde::Deserialize;
use std::collections::HashMap;

pub struct Pager<'a> {
    client: &'a Client,
    channel_id: &'a str,
    params: Option<Page<'a>>,
}

enum Page<'a> {
    After {
        page: i64,
        per_page: i64,
        post: &'a str,
    },
    Before {
        page: i64,
        per_page: i64,
        post: &'a str,
    },
    Since(chrono::NaiveDateTime),
}

impl<'a> Pager<'a> {
    pub(crate) fn new(c: &'a Client, channel_id: &'a str) -> Pager<'a> {
        Pager {
            client: c,
            channel_id,
            params: None,
        }
    }

    pub fn get(&self) -> Result<PostList, Error> {
        let mut req = self
            .client
            .get_builder(&format!("/api/v4/channels/{}/posts", self.channel_id));
        if let Some(ref params) = self.params {
            match params {
                Page::Since(since) => req = req.query(&[("since", since.timestamp_millis())]),
                Page::Before {
                    page,
                    per_page,
                    post,
                } => {
                    req = req
                        .query(&[("page", page), ("per_page", per_page)])
                        .query(&[("before", post)])
                }
                Page::After {
                    page,
                    per_page,
                    post,
                } => {
                    req = req
                        .query(&[("page", page), ("per_page", per_page)])
                        .query(&[("after", post)])
                }
            }
        }
        Client::handle_response(req.send())
    }

    pub fn page_before(
        &mut self,
        post_id: &'a str,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<PostList, Error> {
        self.params = Some(Page::Before {
            page: page.unwrap_or(0),
            per_page: per_page.unwrap_or(60),
            post: post_id,
        });
        self.get()
    }

    pub fn page_after(
        &mut self,
        post_id: &'a str,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<PostList, Error> {
        self.params = Some(Page::After {
            page: page.unwrap_or(0),
            per_page: per_page.unwrap_or(60),
            post: post_id,
        });
        self.get()
    }

    pub fn since(&mut self, timestamp: chrono::NaiveDateTime) -> Result<PostList, Error> {
        self.params = Some(Page::Since(timestamp));
        self.get()
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct PostList {
    pub order: Vec<String>,
    pub posts: HashMap<String, Post>,
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
    /// This field will only appear on some posts created before Mattermost
    /// 3.5 and has since been deprecated.
    pub filenames: Option<Vec<String>>,
    pub file_ids: Option<Vec<String>>,
    pub pending_post_id: String,
    /// $ref: '#/components/schemas/PostMetadata'
    pub metadata: serde_json::Value,
}

impl Post {
    pub fn created(&self) -> chrono::NaiveDateTime {
        let secs = self.create_at / 1000;
        chrono::NaiveDateTime::from_timestamp(secs, 0)
    }
}
