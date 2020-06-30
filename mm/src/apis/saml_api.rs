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

pub struct SAMLApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> SAMLApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SAMLApiClient<C> {
        SAMLApiClient {
            configuration,
        }
    }
}

pub trait SAMLApi {
    fn saml_certificate_idp_delete(&self, ) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>>;
    fn saml_certificate_idp_post(&self, certificate: std::path::PathBuf) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>>;
    fn saml_certificate_private_delete(&self, ) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>>;
    fn saml_certificate_private_post(&self, certificate: std::path::PathBuf) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>>;
    fn saml_certificate_public_delete(&self, ) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>>;
    fn saml_certificate_public_post(&self, certificate: std::path::PathBuf) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>>;
    fn saml_certificate_status_get(&self, ) -> Box<dyn Future<Item = crate::models::SamlCertificateStatus, Error = Error<serde_json::Value>>>;
    fn saml_metadata_get(&self, ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>>;
    fn saml_metadatafromidp_post(&self, ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>SAMLApi for SAMLApiClient<C> {
    fn saml_certificate_idp_delete(&self, ) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/saml/certificate/idp".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

    fn saml_certificate_idp_post(&self, certificate: std::path::PathBuf) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/saml/certificate/idp".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_form_param("certificate".to_string(), unimplemented!());

        req.execute(self.configuration.borrow())
    }

    fn saml_certificate_private_delete(&self, ) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/saml/certificate/private".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

    fn saml_certificate_private_post(&self, certificate: std::path::PathBuf) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/saml/certificate/private".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_form_param("certificate".to_string(), unimplemented!());

        req.execute(self.configuration.borrow())
    }

    fn saml_certificate_public_delete(&self, ) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/saml/certificate/public".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

    fn saml_certificate_public_post(&self, certificate: std::path::PathBuf) -> Box<dyn Future<Item = crate::models::StatusOk, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/saml/certificate/public".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_form_param("certificate".to_string(), unimplemented!());

        req.execute(self.configuration.borrow())
    }

    fn saml_certificate_status_get(&self, ) -> Box<dyn Future<Item = crate::models::SamlCertificateStatus, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/saml/certificate/status".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

    fn saml_metadata_get(&self, ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/saml/metadata".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

    fn saml_metadatafromidp_post(&self, ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/saml/metadatafromidp".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

}
