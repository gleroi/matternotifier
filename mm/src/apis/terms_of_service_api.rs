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

pub struct TermsOfServiceApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> TermsOfServiceApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TermsOfServiceApiClient<C> {
        TermsOfServiceApiClient {
            configuration,
        }
    }
}

pub trait TermsOfServiceApi {
    fn terms_of_service_get(&self, ) -> Box<dyn Future<Item = crate::models::TermsOfService, Error = Error<serde_json::Value>>>;
    fn terms_of_service_post(&self, ) -> Box<dyn Future<Item = crate::models::TermsOfService, Error = Error<serde_json::Value>>>;
    fn users_user_id_terms_of_service_get(&self, user_id: &str) -> Box<dyn Future<Item = crate::models::UserTermsOfService, Error = Error<serde_json::Value>>>;
    fn users_user_id_terms_of_service_post(&self, user_id: &str, inline_object23: crate::models::InlineObject23) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>TermsOfServiceApi for TermsOfServiceApiClient<C> {
    fn terms_of_service_get(&self, ) -> Box<dyn Future<Item = crate::models::TermsOfService, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/terms_of_service".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

    fn terms_of_service_post(&self, ) -> Box<dyn Future<Item = crate::models::TermsOfService, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/terms_of_service".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_terms_of_service_get(&self, user_id: &str) -> Box<dyn Future<Item = crate::models::UserTermsOfService, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{user_id}/terms_of_service".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_terms_of_service_post(&self, user_id: &str, inline_object23: crate::models::InlineObject23) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/users/{user_id}/terms_of_service".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_body_param(inline_object23);

        req.execute(self.configuration.borrow())
    }

}
