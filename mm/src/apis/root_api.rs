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

pub struct RootApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> RootApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> RootApiClient<C> {
        RootApiClient {
            configuration,
        }
    }
}

pub trait RootApi {
    fn notifications_ack_post(&self, ) -> Box<dyn Future<Item = crate::models::PushNotification, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>RootApi for RootApiClient<C> {
    fn notifications_ack_post(&self, ) -> Box<dyn Future<Item = crate::models::PushNotification, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/notifications/ack".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

}
