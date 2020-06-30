use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    bleve_api: Box<dyn crate::apis::BleveApi>,
    bots_api: Box<dyn crate::apis::BotsApi>,
    brand_api: Box<dyn crate::apis::BrandApi>,
    channels_api: Box<dyn crate::apis::ChannelsApi>,
    cluster_api: Box<dyn crate::apis::ClusterApi>,
    commands_api: Box<dyn crate::apis::CommandsApi>,
    compliance_api: Box<dyn crate::apis::ComplianceApi>,
    dataretention_api: Box<dyn crate::apis::DataretentionApi>,
    elasticsearch_api: Box<dyn crate::apis::ElasticsearchApi>,
    emoji_api: Box<dyn crate::apis::EmojiApi>,
    files_api: Box<dyn crate::apis::FilesApi>,
    groups_api: Box<dyn crate::apis::GroupsApi>,
    integration_actions_api: Box<dyn crate::apis::IntegrationActionsApi>,
    jobs_api: Box<dyn crate::apis::JobsApi>,
    ldap_api: Box<dyn crate::apis::LDAPApi>,
    ldap_api: Box<dyn crate::apis::LdapApi>,
    o_auth_api: Box<dyn crate::apis::OAuthApi>,
    open_graph_api: Box<dyn crate::apis::OpenGraphApi>,
    plugins_api: Box<dyn crate::apis::PluginsApi>,
    posts_api: Box<dyn crate::apis::PostsApi>,
    preferences_api: Box<dyn crate::apis::PreferencesApi>,
    reactions_api: Box<dyn crate::apis::ReactionsApi>,
    roles_api: Box<dyn crate::apis::RolesApi>,
    root_api: Box<dyn crate::apis::RootApi>,
    saml_api: Box<dyn crate::apis::SAMLApi>,
    schemes_api: Box<dyn crate::apis::SchemesApi>,
    status_api: Box<dyn crate::apis::StatusApi>,
    system_api: Box<dyn crate::apis::SystemApi>,
    teams_api: Box<dyn crate::apis::TeamsApi>,
    terms_of_service_api: Box<dyn crate::apis::TermsOfServiceApi>,
    users_api: Box<dyn crate::apis::UsersApi>,
    webhooks_api: Box<dyn crate::apis::WebhooksApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            bleve_api: Box::new(crate::apis::BleveApiClient::new(rc.clone())),
            bots_api: Box::new(crate::apis::BotsApiClient::new(rc.clone())),
            brand_api: Box::new(crate::apis::BrandApiClient::new(rc.clone())),
            channels_api: Box::new(crate::apis::ChannelsApiClient::new(rc.clone())),
            cluster_api: Box::new(crate::apis::ClusterApiClient::new(rc.clone())),
            commands_api: Box::new(crate::apis::CommandsApiClient::new(rc.clone())),
            compliance_api: Box::new(crate::apis::ComplianceApiClient::new(rc.clone())),
            dataretention_api: Box::new(crate::apis::DataretentionApiClient::new(rc.clone())),
            elasticsearch_api: Box::new(crate::apis::ElasticsearchApiClient::new(rc.clone())),
            emoji_api: Box::new(crate::apis::EmojiApiClient::new(rc.clone())),
            files_api: Box::new(crate::apis::FilesApiClient::new(rc.clone())),
            groups_api: Box::new(crate::apis::GroupsApiClient::new(rc.clone())),
            integration_actions_api: Box::new(crate::apis::IntegrationActionsApiClient::new(rc.clone())),
            jobs_api: Box::new(crate::apis::JobsApiClient::new(rc.clone())),
            ldap_api: Box::new(crate::apis::LDAPApiClient::new(rc.clone())),
            ldap_api: Box::new(crate::apis::LdapApiClient::new(rc.clone())),
            o_auth_api: Box::new(crate::apis::OAuthApiClient::new(rc.clone())),
            open_graph_api: Box::new(crate::apis::OpenGraphApiClient::new(rc.clone())),
            plugins_api: Box::new(crate::apis::PluginsApiClient::new(rc.clone())),
            posts_api: Box::new(crate::apis::PostsApiClient::new(rc.clone())),
            preferences_api: Box::new(crate::apis::PreferencesApiClient::new(rc.clone())),
            reactions_api: Box::new(crate::apis::ReactionsApiClient::new(rc.clone())),
            roles_api: Box::new(crate::apis::RolesApiClient::new(rc.clone())),
            root_api: Box::new(crate::apis::RootApiClient::new(rc.clone())),
            saml_api: Box::new(crate::apis::SAMLApiClient::new(rc.clone())),
            schemes_api: Box::new(crate::apis::SchemesApiClient::new(rc.clone())),
            status_api: Box::new(crate::apis::StatusApiClient::new(rc.clone())),
            system_api: Box::new(crate::apis::SystemApiClient::new(rc.clone())),
            teams_api: Box::new(crate::apis::TeamsApiClient::new(rc.clone())),
            terms_of_service_api: Box::new(crate::apis::TermsOfServiceApiClient::new(rc.clone())),
            users_api: Box::new(crate::apis::UsersApiClient::new(rc.clone())),
            webhooks_api: Box::new(crate::apis::WebhooksApiClient::new(rc.clone())),
        }
    }

    pub fn bleve_api(&self) -> &dyn crate::apis::BleveApi{
        self.bleve_api.as_ref()
    }

    pub fn bots_api(&self) -> &dyn crate::apis::BotsApi{
        self.bots_api.as_ref()
    }

    pub fn brand_api(&self) -> &dyn crate::apis::BrandApi{
        self.brand_api.as_ref()
    }

    pub fn channels_api(&self) -> &dyn crate::apis::ChannelsApi{
        self.channels_api.as_ref()
    }

    pub fn cluster_api(&self) -> &dyn crate::apis::ClusterApi{
        self.cluster_api.as_ref()
    }

    pub fn commands_api(&self) -> &dyn crate::apis::CommandsApi{
        self.commands_api.as_ref()
    }

    pub fn compliance_api(&self) -> &dyn crate::apis::ComplianceApi{
        self.compliance_api.as_ref()
    }

    pub fn dataretention_api(&self) -> &dyn crate::apis::DataretentionApi{
        self.dataretention_api.as_ref()
    }

    pub fn elasticsearch_api(&self) -> &dyn crate::apis::ElasticsearchApi{
        self.elasticsearch_api.as_ref()
    }

    pub fn emoji_api(&self) -> &dyn crate::apis::EmojiApi{
        self.emoji_api.as_ref()
    }

    pub fn files_api(&self) -> &dyn crate::apis::FilesApi{
        self.files_api.as_ref()
    }

    pub fn groups_api(&self) -> &dyn crate::apis::GroupsApi{
        self.groups_api.as_ref()
    }

    pub fn integration_actions_api(&self) -> &dyn crate::apis::IntegrationActionsApi{
        self.integration_actions_api.as_ref()
    }

    pub fn jobs_api(&self) -> &dyn crate::apis::JobsApi{
        self.jobs_api.as_ref()
    }

    pub fn ldap_api(&self) -> &dyn crate::apis::LDAPApi{
        self.ldap_api.as_ref()
    }

    pub fn ldap_api(&self) -> &dyn crate::apis::LdapApi{
        self.ldap_api.as_ref()
    }

    pub fn o_auth_api(&self) -> &dyn crate::apis::OAuthApi{
        self.o_auth_api.as_ref()
    }

    pub fn open_graph_api(&self) -> &dyn crate::apis::OpenGraphApi{
        self.open_graph_api.as_ref()
    }

    pub fn plugins_api(&self) -> &dyn crate::apis::PluginsApi{
        self.plugins_api.as_ref()
    }

    pub fn posts_api(&self) -> &dyn crate::apis::PostsApi{
        self.posts_api.as_ref()
    }

    pub fn preferences_api(&self) -> &dyn crate::apis::PreferencesApi{
        self.preferences_api.as_ref()
    }

    pub fn reactions_api(&self) -> &dyn crate::apis::ReactionsApi{
        self.reactions_api.as_ref()
    }

    pub fn roles_api(&self) -> &dyn crate::apis::RolesApi{
        self.roles_api.as_ref()
    }

    pub fn root_api(&self) -> &dyn crate::apis::RootApi{
        self.root_api.as_ref()
    }

    pub fn saml_api(&self) -> &dyn crate::apis::SAMLApi{
        self.saml_api.as_ref()
    }

    pub fn schemes_api(&self) -> &dyn crate::apis::SchemesApi{
        self.schemes_api.as_ref()
    }

    pub fn status_api(&self) -> &dyn crate::apis::StatusApi{
        self.status_api.as_ref()
    }

    pub fn system_api(&self) -> &dyn crate::apis::SystemApi{
        self.system_api.as_ref()
    }

    pub fn teams_api(&self) -> &dyn crate::apis::TeamsApi{
        self.teams_api.as_ref()
    }

    pub fn terms_of_service_api(&self) -> &dyn crate::apis::TermsOfServiceApi{
        self.terms_of_service_api.as_ref()
    }

    pub fn users_api(&self) -> &dyn crate::apis::UsersApi{
        self.users_api.as_ref()
    }

    pub fn webhooks_api(&self) -> &dyn crate::apis::WebhooksApi{
        self.webhooks_api.as_ref()
    }

}
