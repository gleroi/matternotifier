use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod bleve_api;
pub use self::bleve_api::{ BleveApi, BleveApiClient };
mod bots_api;
pub use self::bots_api::{ BotsApi, BotsApiClient };
mod brand_api;
pub use self::brand_api::{ BrandApi, BrandApiClient };
mod channels_api;
pub use self::channels_api::{ ChannelsApi, ChannelsApiClient };
mod cluster_api;
pub use self::cluster_api::{ ClusterApi, ClusterApiClient };
mod commands_api;
pub use self::commands_api::{ CommandsApi, CommandsApiClient };
mod compliance_api;
pub use self::compliance_api::{ ComplianceApi, ComplianceApiClient };
mod dataretention_api;
pub use self::dataretention_api::{ DataretentionApi, DataretentionApiClient };
mod elasticsearch_api;
pub use self::elasticsearch_api::{ ElasticsearchApi, ElasticsearchApiClient };
mod emoji_api;
pub use self::emoji_api::{ EmojiApi, EmojiApiClient };
mod files_api;
pub use self::files_api::{ FilesApi, FilesApiClient };
mod groups_api;
pub use self::groups_api::{ GroupsApi, GroupsApiClient };
mod integration_actions_api;
pub use self::integration_actions_api::{ IntegrationActionsApi, IntegrationActionsApiClient };
mod jobs_api;
pub use self::jobs_api::{ JobsApi, JobsApiClient };
mod ldap_api;
pub use self::ldap_api::{ LdapApi, LdapApiClient };
mod o_auth_api;
pub use self::o_auth_api::{ OAuthApi, OAuthApiClient };
mod open_graph_api;
pub use self::open_graph_api::{ OpenGraphApi, OpenGraphApiClient };
mod plugins_api;
pub use self::plugins_api::{ PluginsApi, PluginsApiClient };
mod posts_api;
pub use self::posts_api::{ PostsApi, PostsApiClient };
mod preferences_api;
pub use self::preferences_api::{ PreferencesApi, PreferencesApiClient };
mod reactions_api;
pub use self::reactions_api::{ ReactionsApi, ReactionsApiClient };
mod roles_api;
pub use self::roles_api::{ RolesApi, RolesApiClient };
mod root_api;
pub use self::root_api::{ RootApi, RootApiClient };
mod saml_api;
pub use self::saml_api::{ SAMLApi, SAMLApiClient };
mod schemes_api;
pub use self::schemes_api::{ SchemesApi, SchemesApiClient };
mod status_api;
pub use self::status_api::{ StatusApi, StatusApiClient };
mod system_api;
pub use self::system_api::{ SystemApi, SystemApiClient };
mod teams_api;
pub use self::teams_api::{ TeamsApi, TeamsApiClient };
mod terms_of_service_api;
pub use self::terms_of_service_api::{ TermsOfServiceApi, TermsOfServiceApiClient };
mod users_api;
pub use self::users_api::{ UsersApi, UsersApiClient };
mod webhooks_api;
pub use self::webhooks_api::{ WebhooksApi, WebhooksApiClient };

pub mod configuration;
pub mod client;
