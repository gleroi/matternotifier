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

pub struct ReactionsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ReactionsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ReactionsApiClient<C> {
        ReactionsApiClient {
            configuration,
        }
    }
}

pub trait ReactionsApi {
    fn posts_ids_reactions_post(&self, request_body: Vec<String>) -> Box<dyn Future<Output = Result<::std::collections::HashMap<String, Vec<crate::models::Reaction>>, Error<serde_json::Value>>>>;
    fn posts_post_id_reactions_get(&self, post_id: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::Reaction>, Error<serde_json::Value>>>>;
    fn reactions_post(&self, reaction: crate::models::Reaction) -> Box<dyn Future<Output = Result<crate::models::Reaction, Error<serde_json::Value>>>>;
    fn users_user_id_posts_post_id_reactions_emoji_name_delete(&self, user_id: &str, post_id: &str, emoji_name: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
}

impl<C: hyper::client::Connect>ReactionsApi for ReactionsApiClient<C> {
    fn posts_ids_reactions_post(&self, request_body: Vec<String>) -> Box<dyn Future<Output = Result<::std::collections::HashMap<String, Vec<crate::models::Reaction>>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/posts/ids/reactions".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_body_param(request_body);

        req.execute(self.configuration.borrow())
    }

    fn posts_post_id_reactions_get(&self, post_id: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::Reaction>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/posts/{post_id}/reactions".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("post_id".to_string(), post_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn reactions_post(&self, reaction: crate::models::Reaction) -> Box<dyn Future<Output = Result<crate::models::Reaction, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/reactions".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_body_param(reaction);

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_posts_post_id_reactions_emoji_name_delete(&self, user_id: &str, post_id: &str, emoji_name: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/users/{user_id}/posts/{post_id}/reactions/{emoji_name}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_path_param("post_id".to_string(), post_id.to_string());
        req = req.with_path_param("emoji_name".to_string(), emoji_name.to_string());

        req.execute(self.configuration.borrow())
    }

}
