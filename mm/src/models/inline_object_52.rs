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
pub struct InlineObject52 {
    /// The channel ID to post in
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// The message contents, can be formatted with Markdown
    #[serde(rename = "message")]
    pub message: String,
    /// The post ID to comment on
    #[serde(rename = "root_id", skip_serializing_if = "Option::is_none")]
    pub root_id: Option<String>,
    /// A list of file IDs to associate with the post. Note that posts are limited to 5 files maximum. Please use additional posts for more files.
    #[serde(rename = "file_ids", skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// A general JSON property bag to attach to the post
    #[serde(rename = "props", skip_serializing_if = "Option::is_none")]
    pub props: Option<serde_json::Value>,
}

impl InlineObject52 {
    pub fn new(channel_id: String, message: String) -> InlineObject52 {
        InlineObject52 {
            channel_id,
            message,
            root_id: None,
            file_ids: None,
            props: None,
        }
    }
}


