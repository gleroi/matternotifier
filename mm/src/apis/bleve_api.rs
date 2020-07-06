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

pub struct BleveApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> BleveApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> BleveApiClient<C> {
        BleveApiClient {
            configuration,
        }
    }
}

pub trait BleveApi {
    fn bleve_purge_indexes_post(&self, ) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
}

impl<C: hyper::client::Connect>BleveApi for BleveApiClient<C> {
    fn bleve_purge_indexes_post(&self, ) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/bleve/purge_indexes".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

}
