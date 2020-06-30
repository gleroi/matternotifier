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
pub struct InlineObject41 {
    /// The channel's id, not updatable
    #[serde(rename = "id")]
    pub id: String,
    /// The unique handle for the channel, will be present in the channel URL
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The non-unique UI name for the channel
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// A short description of the purpose of the channel
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Markdown-formatted text to display in the header of the channel
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
}

impl InlineObject41 {
    pub fn new(id: String) -> InlineObject41 {
        InlineObject41 {
            id,
            name: None,
            display_name: None,
            purpose: None,
            header: None,
        }
    }
}


