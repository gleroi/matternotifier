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

pub struct TeamsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> TeamsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TeamsApiClient<C> {
        TeamsApiClient {
            configuration,
        }
    }
}

pub trait TeamsApi {
    fn teams_get(&self, page: Option<i32>, per_page: Option<i32>, include_total_count: Option<bool>) -> Box<dyn Future<Output = Result<Vec<crate::models::Team>, Error<serde_json::Value>>>>;
    fn teams_invite_invite_id_get(&self, invite_id: &str) -> Box<dyn Future<Output = Result<crate::models::InlineResponse2006, Error<serde_json::Value>>>>;
    fn teams_invites_email_delete(&self, ) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_members_invite_post(&self, token: &str) -> Box<dyn Future<Output = Result<crate::models::TeamMember, Error<serde_json::Value>>>>;
    fn teams_name_name_exists_get(&self, name: &str) -> Box<dyn Future<Output = Result<crate::models::TeamExists, Error<serde_json::Value>>>>;
    fn teams_name_name_get(&self, name: &str) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>;
    fn teams_post(&self, inline_object26: crate::models::InlineObject26) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>;
    fn teams_search_post(&self, inline_object30: crate::models::InlineObject30) -> Box<dyn Future<Output = Result<crate::models::InlineResponse2004, Error<serde_json::Value>>>>;
    fn teams_team_id_delete(&self, team_id: &str, permanent: Option<bool>) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_team_id_get(&self, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>;
    fn teams_team_id_image_delete(&self, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_team_id_image_get(&self, team_id: &str) -> Box<dyn Future<Output = Result<(), Error<serde_json::Value>>>>;
    fn teams_team_id_image_post(&self, team_id: &str, image: std::path::PathBuf) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_team_id_import_post(&self, team_id: &str, file: std::path::PathBuf, filesize: i32, import_from: &str) -> Box<dyn Future<Output = Result<crate::models::InlineResponse2005, Error<serde_json::Value>>>>;
    fn teams_team_id_invite_email_post(&self, team_id: &str, request_body: Vec<String>) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_team_id_invite_guests_email_post(&self, team_id: &str, inline_object35: crate::models::InlineObject35) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_team_id_members_batch_post(&self, team_id: &str, team_member: Vec<crate::models::TeamMember>, graceful: Option<bool>) -> Box<dyn Future<Output = Result<Vec<crate::models::TeamMember>, Error<serde_json::Value>>>>;
    fn teams_team_id_members_get(&self, team_id: &str, page: Option<i32>, per_page: Option<i32>) -> Box<dyn Future<Output = Result<Vec<crate::models::TeamMember>, Error<serde_json::Value>>>>;
    fn teams_team_id_members_ids_post(&self, team_id: &str, request_body: Vec<String>) -> Box<dyn Future<Output = Result<Vec<crate::models::TeamMember>, Error<serde_json::Value>>>>;
    fn teams_team_id_members_minus_group_members_get(&self, team_id: &str, group_ids: &str, page: Option<i32>, per_page: Option<i32>) -> Box<dyn Future<Output = Result<(), Error<serde_json::Value>>>>;
    fn teams_team_id_members_post(&self, team_id: &str, inline_object31: crate::models::InlineObject31) -> Box<dyn Future<Output = Result<crate::models::TeamMember, Error<serde_json::Value>>>>;
    fn teams_team_id_members_user_id_delete(&self, team_id: &str, user_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_team_id_members_user_id_get(&self, team_id: &str, user_id: &str) -> Box<dyn Future<Output = Result<crate::models::TeamMember, Error<serde_json::Value>>>>;
    fn teams_team_id_members_user_id_roles_put(&self, team_id: &str, user_id: &str, inline_object33: crate::models::InlineObject33) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_team_id_members_user_id_scheme_roles_put(&self, team_id: &str, user_id: &str, inline_object34: crate::models::InlineObject34) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_team_id_patch_put(&self, team_id: &str, inline_object28: crate::models::InlineObject28) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>;
    fn teams_team_id_privacy_put(&self, team_id: &str, inline_object29: crate::models::InlineObject29) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>;
    fn teams_team_id_put(&self, team_id: &str, inline_object27: crate::models::InlineObject27) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>;
    fn teams_team_id_regenerate_invite_id_post(&self, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>;
    fn teams_team_id_restore_post(&self, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>;
    fn teams_team_id_scheme_put(&self, team_id: &str, inline_object37: crate::models::InlineObject37) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>;
    fn teams_team_id_stats_get(&self, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::TeamStats, Error<serde_json::Value>>>>;
    fn users_user_id_teams_get(&self, user_id: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::Team>, Error<serde_json::Value>>>>;
    fn users_user_id_teams_members_get(&self, user_id: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::TeamMember>, Error<serde_json::Value>>>>;
    fn users_user_id_teams_team_id_unread_get(&self, user_id: &str, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::TeamUnread, Error<serde_json::Value>>>>;
    fn users_user_id_teams_unread_get(&self, user_id: &str, exclude_team: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::TeamUnread>, Error<serde_json::Value>>>>;
}

impl<C: hyper::client::Connect>TeamsApi for TeamsApiClient<C> {
    fn teams_get(&self, page: Option<i32>, per_page: Option<i32>, include_total_count: Option<bool>) -> Box<dyn Future<Output = Result<Vec<crate::models::Team>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/teams".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        if let Some(ref s) = page {
            req = req.with_query_param("page".to_string(), s.to_string());
        }
        if let Some(ref s) = per_page {
            req = req.with_query_param("per_page".to_string(), s.to_string());
        }
        if let Some(ref s) = include_total_count {
            req = req.with_query_param("include_total_count".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn teams_invite_invite_id_get(&self, invite_id: &str) -> Box<dyn Future<Output = Result<crate::models::InlineResponse2006, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/teams/invite/{invite_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("invite_id".to_string(), invite_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_invites_email_delete(&self, ) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/teams/invites/email".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;

        req.execute(self.configuration.borrow())
    }

    fn teams_members_invite_post(&self, token: &str) -> Box<dyn Future<Output = Result<crate::models::TeamMember, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/members/invite".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_query_param("token".to_string(), token.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_name_name_exists_get(&self, name: &str) -> Box<dyn Future<Output = Result<crate::models::TeamExists, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/teams/name/{name}/exists".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_name_name_get(&self, name: &str) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/teams/name/{name}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_post(&self, inline_object26: crate::models::InlineObject26) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_body_param(inline_object26);

        req.execute(self.configuration.borrow())
    }

    fn teams_search_post(&self, inline_object30: crate::models::InlineObject30) -> Box<dyn Future<Output = Result<crate::models::InlineResponse2004, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/search".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_body_param(inline_object30);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_delete(&self, team_id: &str, permanent: Option<bool>) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/teams/{team_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        if let Some(ref s) = permanent {
            req = req.with_query_param("permanent".to_string(), s.to_string());
        }
        req = req.with_path_param("team_id".to_string(), team_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_get(&self, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/teams/{team_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_image_delete(&self, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/teams/{team_id}/image".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_image_get(&self, team_id: &str) -> Box<dyn Future<Output = Result<(), Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/teams/{team_id}/image".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_image_post(&self, team_id: &str, image: std::path::PathBuf) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/{team_id}/image".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_form_param("image".to_string(), unimplemented!());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_import_post(&self, team_id: &str, file: std::path::PathBuf, filesize: i32, import_from: &str) -> Box<dyn Future<Output = Result<crate::models::InlineResponse2005, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/{team_id}/import".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_form_param("file".to_string(), unimplemented!());
        req = req.with_form_param("filesize".to_string(), filesize.to_string());
        req = req.with_form_param("importFrom".to_string(), import_from.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_invite_email_post(&self, team_id: &str, request_body: Vec<String>) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/{team_id}/invite/email".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_body_param(request_body);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_invite_guests_email_post(&self, team_id: &str, inline_object35: crate::models::InlineObject35) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/{team_id}/invite-guests/email".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_body_param(inline_object35);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_members_batch_post(&self, team_id: &str, team_member: Vec<crate::models::TeamMember>, graceful: Option<bool>) -> Box<dyn Future<Output = Result<Vec<crate::models::TeamMember>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/{team_id}/members/batch".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        if let Some(ref s) = graceful {
            req = req.with_query_param("graceful".to_string(), s.to_string());
        }
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_body_param(team_member);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_members_get(&self, team_id: &str, page: Option<i32>, per_page: Option<i32>) -> Box<dyn Future<Output = Result<Vec<crate::models::TeamMember>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/teams/{team_id}/members".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        if let Some(ref s) = page {
            req = req.with_query_param("page".to_string(), s.to_string());
        }
        if let Some(ref s) = per_page {
            req = req.with_query_param("per_page".to_string(), s.to_string());
        }
        req = req.with_path_param("team_id".to_string(), team_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_members_ids_post(&self, team_id: &str, request_body: Vec<String>) -> Box<dyn Future<Output = Result<Vec<crate::models::TeamMember>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/{team_id}/members/ids".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_body_param(request_body);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_members_minus_group_members_get(&self, team_id: &str, group_ids: &str, page: Option<i32>, per_page: Option<i32>) -> Box<dyn Future<Output = Result<(), Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/teams/{team_id}/members_minus_group_members".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_query_param("group_ids".to_string(), group_ids.to_string());
        if let Some(ref s) = page {
            req = req.with_query_param("page".to_string(), s.to_string());
        }
        if let Some(ref s) = per_page {
            req = req.with_query_param("per_page".to_string(), s.to_string());
        }
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_members_post(&self, team_id: &str, inline_object31: crate::models::InlineObject31) -> Box<dyn Future<Output = Result<crate::models::TeamMember, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/{team_id}/members".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_body_param(inline_object31);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_members_user_id_delete(&self, team_id: &str, user_id: &str) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/teams/{team_id}/members/{user_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_path_param("user_id".to_string(), user_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_members_user_id_get(&self, team_id: &str, user_id: &str) -> Box<dyn Future<Output = Result<crate::models::TeamMember, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/teams/{team_id}/members/{user_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_path_param("user_id".to_string(), user_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_members_user_id_roles_put(&self, team_id: &str, user_id: &str, inline_object33: crate::models::InlineObject33) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/teams/{team_id}/members/{user_id}/roles".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_body_param(inline_object33);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_members_user_id_scheme_roles_put(&self, team_id: &str, user_id: &str, inline_object34: crate::models::InlineObject34) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/teams/{team_id}/members/{user_id}/schemeRoles".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_body_param(inline_object34);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_patch_put(&self, team_id: &str, inline_object28: crate::models::InlineObject28) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/teams/{team_id}/patch".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_body_param(inline_object28);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_privacy_put(&self, team_id: &str, inline_object29: crate::models::InlineObject29) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/teams/{team_id}/privacy".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_body_param(inline_object29);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_put(&self, team_id: &str, inline_object27: crate::models::InlineObject27) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/teams/{team_id}".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_body_param(inline_object27);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_regenerate_invite_id_post(&self, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/{team_id}/regenerate_invite_id".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_restore_post(&self, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::Team, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/teams/{team_id}/restore".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_scheme_put(&self, team_id: &str, inline_object37: crate::models::InlineObject37) -> Box<dyn Future<Output = Result<crate::models::StatusOk, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/teams/{team_id}/scheme".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());
        req = req.with_body_param(inline_object37);

        req.execute(self.configuration.borrow())
    }

    fn teams_team_id_stats_get(&self, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::TeamStats, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/teams/{team_id}/stats".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("team_id".to_string(), team_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_teams_get(&self, user_id: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::Team>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{user_id}/teams".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_teams_members_get(&self, user_id: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::TeamMember>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{user_id}/teams/members".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_teams_team_id_unread_get(&self, user_id: &str, team_id: &str) -> Box<dyn Future<Output = Result<crate::models::TeamUnread, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{user_id}/teams/{team_id}/unread".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_path_param("user_id".to_string(), user_id.to_string());
        req = req.with_path_param("team_id".to_string(), team_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn users_user_id_teams_unread_get(&self, user_id: &str, exclude_team: &str) -> Box<dyn Future<Output = Result<Vec<crate::models::TeamUnread>, Error<serde_json::Value>>>>{
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/users/{user_id}/teams/unread".to_string())
            .with_auth(__internal_request::Auth::Basic)
        ;
        req = req.with_query_param("exclude_team".to_string(), exclude_team.to_string());
        req = req.with_path_param("user_id".to_string(), user_id.to_string());

        req.execute(self.configuration.borrow())
    }

}
