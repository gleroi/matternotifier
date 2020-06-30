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

pub struct FilesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> FilesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> FilesApiClient<C> {
        FilesApiClient {
            configuration,
        }
    }
}

pub trait FilesApi {
    fn files_file_id_get(&self, file_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn files_file_id_info_get(&self, file_id: &str) -> Box<dyn Future<Item = crate::models::FileInfo, Error = Error<serde_json::Value>>>;
    fn files_file_id_link_get(&self, file_id: &str) -> Box<dyn Future<Item = crate::models::InlineResponse2009, Error = Error<serde_json::Value>>>;
    fn files_file_id_preview_get(&self, file_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn files_file_id_public_get(&self, file_id: &str, h: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn files_file_id_thumbnail_get(&self, file_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn files_post(&self, channel_id: Option<&str>, filename: Option<&str>, files: Option<std::path::PathBuf>, channel_id: Option<&str>, client_ids: Option<&str>) -> Box<dyn Future<Item = crate::models::InlineResponse201, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>FilesApi for FilesApiClient<C> {
    fn files_file_id_get(&self, file_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/files/{file_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("file_id".to_string(), file_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn files_file_id_info_get(&self, file_id: &str) -> Box<dyn Future<Item = crate::models::FileInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/files/{file_id}/info".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("file_id".to_string(), file_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn files_file_id_link_get(&self, file_id: &str) -> Box<dyn Future<Item = crate::models::InlineResponse2009, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/files/{file_id}/link".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("file_id".to_string(), file_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn files_file_id_preview_get(&self, file_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/files/{file_id}/preview".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("file_id".to_string(), file_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn files_file_id_public_get(&self, file_id: &str, h: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/files/{file_id}/public".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_query_param("h".to_string(), h.to_string());
        req = req.with_path_param("file_id".to_string(), file_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn files_file_id_thumbnail_get(&self, file_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/files/{file_id}/thumbnail".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("file_id".to_string(), file_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn files_post(&self, channel_id: Option<&str>, filename: Option<&str>, files: Option<std::path::PathBuf>, channel_id: Option<&str>, client_ids: Option<&str>) -> Box<dyn Future<Item = crate::models::InlineResponse201, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/files".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        if let Some(ref s) = channel_id {
            req = req.with_query_param("channel_id".to_string(), s.to_string());
        }
        if let Some(ref s) = filename {
            req = req.with_query_param("filename".to_string(), s.to_string());
        }
        if let Some(param_value) = files {
            req = req.with_form_param("files".to_string(), unimplemented!());
        }
        if let Some(param_value) = channel_id {
            req = req.with_form_param("channel_id".to_string(), param_value.to_string());
        }
        if let Some(param_value) = client_ids {
            req = req.with_form_param("client_ids".to_string(), param_value.to_string());
        }

        req.execute(self.configuration.borrow())
    }

}
