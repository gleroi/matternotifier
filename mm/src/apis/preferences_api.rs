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

pub struct PreferencesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PreferencesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PreferencesApiClient<C> {
        PreferencesApiClient {
            configuration,
        }
    }
}

pub trait PreferencesApi {
    fn users_user_id_preferences_category_get(&self, user_id: &str, category: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::Preference>, Error<serde_json::Value>>>>;
    fn users_user_id_preferences_category_name_preference_name_get(&self, user_id: &str, category: &str, preference_name: &str) -> Box<dyn Future<Output = Result<crate::models::Preference, Error<serde_json::Value>>>>;
    fn users_user_id_preferences_delete_post(&self, user_id: &str, preference: Vec<crate::models::Preference>) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn users_user_id_preferences_get(&self, user_id: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::Preference>, Error<serde_json::Value>>>>;
    fn users_user_id_preferences_put(&self, user_id: &str, preference: Vec<crate::models::Preference>) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
}

impl<C: hyper::client::Connect>PreferencesApi for PreferencesApiClient<C> {
    fn users_user_id_preferences_category_get(&self, user_id: &str, category: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::Preference>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{user_id}/preferences/{category}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_path_param("category".to_string(), category.to_string());

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_preferences_category_name_preference_name_get(&self, user_id: &str, category: &str, preference_name: &str) -> Box<dyn Future<Output = Result<crate::models::Preference, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{user_id}/preferences/{category}/name/{preference_name}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_path_param("category".to_string(), category.to_string());
        req = req.with_path_param("preference_name".to_string(), preference_name.to_string());

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_preferences_delete_post(&self, user_id: &str, preference: Vec<crate::models::Preference>) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/users/{user_id}/preferences/delete".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_body_param(preference);

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_preferences_get(&self, user_id: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::Preference>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{user_id}/preferences".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_preferences_put(&self, user_id: &str, preference: Vec<crate::models::Preference>) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/users/{user_id}/preferences".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_body_param(preference);

        req.execute(self.configuration.borrow())
    }

}
