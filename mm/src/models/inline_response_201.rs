/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse201 {
    /// A list of file metadata that has been stored in the database
    #[serde(rename = "file_infos", skip_serializing_if = "Option::is_none")]
    pub file_infos: Option<Vec<crate::models::FileInfo>>,
    /// A list of the client_ids that were provided in the request
    #[serde(rename = "client_ids", skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<Vec<String>>,
}

impl InlineResponse201 {
    pub fn new() -> InlineResponse201 {
        InlineResponse201 {
            file_infos: None,
            client_ids: None,
        }
    }
}


