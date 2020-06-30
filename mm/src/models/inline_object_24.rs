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
pub struct InlineObject24 {
    /// The id of the channel to which to direct the typing event.
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// The optional id of the root post of the thread to which the user is replying. If unset, the typing event is directed at the entire channel.
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

impl InlineObject24 {
    pub fn new(channel_id: String) -> InlineObject24 {
        InlineObject24 {
            channel_id,
            parent_id: None,
        }
    }
}


