/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct PostsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PostsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PostsApiClient<C> {
        PostsApiClient {
            configuration,
        }
    }
}

pub trait PostsApi {
    fn channels_channel_id_posts_get(&self, channel_id: &str, page: Option<i32>, per_page: Option<i32>, since: Option<i32>, before: Option<&str>, after: Option<&str>) -> Box<dyn Future<Output = Result<crate::models::PostList, Error<serde_json::Value>>>>;
    fn posts_ephemeral_post(&self, inline_object53: crate::models::InlineObject53) -> Box<dyn Future<Output = Result<crate::models::Post, Error<serde_json::Value>>>>;
    fn posts_post(&self, inline_object52: crate::models::InlineObject52, set_online: Option<bool>) -> Box<dyn Future<Output = Result<crate::models::Post, Error<serde_json::Value>>>>;
    fn posts_post_id_actions_action_id_post(&self, post_id: &str, action_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn posts_post_id_delete(&self, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn posts_post_id_files_info_get(&self, post_id: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::FileInfo>, Error<serde_json::Value>>>>;
    fn posts_post_id_get(&self, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::Post, Error<serde_json::Value>>>>;
    fn posts_post_id_patch_put(&self, post_id: &str, inline_object55: crate::models::InlineObject55) -> Box<dyn Future<Output = Result<crate::models::Post, Error<serde_json::Value>>>>;
    fn posts_post_id_pin_post(&self, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn posts_post_id_put(&self, post_id: &str, inline_object54: crate::models::InlineObject54) -> Box<dyn Future<Output = Result<crate::models::Post, Error<serde_json::Value>>>>;
    fn posts_post_id_thread_get(&self, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::PostList, Error<serde_json::Value>>>>;
    fn posts_post_id_unpin_post(&self, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_team_id_posts_search_post(&self, team_id: &str, inline_object56: crate::models::InlineObject56) -> Box<dyn Future<Output = Result<crate::models::PostListWithSearchMatches, Error<serde_json::Value>>>>;
    fn users_user_id_channels_channel_id_posts_unread_get(&self, user_id: &str, channel_id: &str, limit_before: Option<i32>, limit_after: Option<i32>) -> Box<dyn Future<Output = Result<crate::models::PostList, Error<serde_json::Value>>>>;
    fn users_user_id_posts_flagged_get(&self, user_id: &str, team_id: Option<&str>, channel_id: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Box<dyn Future<Output = Result<Vec<crate::models::PostList>, Error<serde_json::Value>>>>;
    fn users_user_id_posts_post_id_set_unread_post(&self, user_id: &str, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::ChannelUnreadAt, Error<serde_json::Value>>>>;
}

impl<C: hyper::client::Connect>PostsApi for PostsApiClient<C> {
    fn channels_channel_id_posts_get(&self, channel_id: &str, page: Option<i32>, per_page: Option<i32>, since: Option<i32>, before: Option<&str>, after: Option<&str>) -> Box<dyn Future<Output = Result<crate::models::PostList, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/channels/{channel_id}/posts".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        if let Some(ref s) = page {
            req = req.with_query_param("page".to_string(), s.to_string());
        }
        if let Some(ref s) = per_page {
            req = req.with_query_param("per_page".to_string(), s.to_string());
        }
        if let Some(ref s) = since {
            req = req.with_query_param("since".to_string(), s.to_string());
        }
        if let Some(ref s) = before {
            req = req.with_query_param("before".to_string(), s.to_string());
        }
        if let Some(ref s) = after {
            req = req.with_query_param("after".to_string(), s.to_string());
        }
        req = req.with_path_param("channel_id".to_string(), channel_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn posts_ephemeral_post(&self, inline_object53: crate::models::InlineObject53) -> Box<dyn Future<Output = Result<crate::models::Post, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/posts/ephemeral".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_body_param(inline_object53);

        req.execute(self.configuration.borrow())
    }

    fn posts_post(&self, inline_object52: crate::models::InlineObject52, set_online: Option<bool>) -> Box<dyn Future<Output = Result<crate::models::Post, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/posts".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        if let Some(ref s) = set_online {
            req = req.with_query_param("set_online".to_string(), s.to_string());
        }
        req = req.with_body_param(inline_object52);

        req.execute(self.configuration.borrow())
    }

    fn posts_post_id_actions_action_id_post(&self, post_id: &str, action_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/posts/{post_id}/actions/{action_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("post_id".to_string(), post_id.to_string());
        req = req.with_path_param("action_id".to_string(), action_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn posts_post_id_delete(&self, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/posts/{post_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("post_id".to_string(), post_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn posts_post_id_files_info_get(&self, post_id: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::FileInfo>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/posts/{post_id}/files/info".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("post_id".to_string(), post_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn posts_post_id_get(&self, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::Post, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/posts/{post_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("post_id".to_string(), post_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn posts_post_id_patch_put(&self, post_id: &str, inline_object55: crate::models::InlineObject55) -> Box<dyn Future<Output = Result<crate::models::Post, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/posts/{post_id}/patch".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("post_id".to_string(), post_id.to_string());
        req = req.with_body_param(inline_object55);

        req.execute(self.configuration.borrow())
    }

    fn posts_post_id_pin_post(&self, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/posts/{post_id}/pin".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("post_id".to_string(), post_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn posts_post_id_put(&self, post_id: &str, inline_object54: crate::models::InlineObject54) -> Box<dyn Future<Output = Result<crate::models::Post, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/posts/{post_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("post_id".to_string(), post_id.to_string());
        req = req.with_body_param(inline_object54);

        req.execute(self.configuration.borrow())
    }

    fn posts_post_id_thread_get(&self, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::PostList, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/posts/{post_id}/thread".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("post_id".to_string(), post_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn posts_post_id_unpin_post(&self, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/posts/{post_id}/unpin".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("post_id".to_string(), post_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_posts_search_post(&self, team_id: &str, inline_object56: crate::models::InlineObject56) -> Box<dyn Future<Output = Result<crate::models::PostListWithSearchMatches, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/{team_id}/posts/search".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_body_param(inline_object56);

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_channels_channel_id_posts_unread_get(&self, user_id: &str, channel_id: &str, limit_before: Option<i32>, limit_after: Option<i32>) -> Box<dyn Future<Output = Result<crate::models::PostList, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{user_id}/channels/{channel_id}/posts/unread".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        if let Some(ref s) = limit_before {
            req = req.with_query_param("limit_before".to_string(), s.to_string());
        }
        if let Some(ref s) = limit_after {
            req = req.with_query_param("limit_after".to_string(), s.to_string());
        }
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_path_param("channel_id".to_string(), channel_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_posts_flagged_get(&self, user_id: &str, team_id: Option<&str>, channel_id: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Box<dyn Future<Output = Result<Vec<crate::models::PostList>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{user_id}/posts/flagged".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        if let Some(ref s) = team_id {
            req = req.with_query_param("team_id".to_string(), s.to_string());
        }
        if let Some(ref s) = channel_id {
            req = req.with_query_param("channel_id".to_string(), s.to_string());
        }
        if let Some(ref s) = page {
            req = req.with_query_param("page".to_string(), s.to_string());
        }
        if let Some(ref s) = per_page {
            req = req.with_query_param("per_page".to_string(), s.to_string());
        }
        req = req.with_path_param("user_id".to_string(), user_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_posts_post_id_set_unread_post(&self, user_id: &str, post_id: &str) -> Box<dyn Future<Output = Result<crate::models::ChannelUnreadAt, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/users/{user_id}/posts/{post_id}/set_unread".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_path_param("post_id".to_string(), post_id.to_string());

        req.execute(self.configuration.borrow())
    }

}
