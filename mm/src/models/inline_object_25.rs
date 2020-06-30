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
pub struct InlineObject25 {
    /// User ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// User status, can be `online`, `away`, `offline` and `dnd`
    #[serde(rename = "status")]
    pub status: String,
}

impl InlineObject25 {
    pub fn new(user_id: String, status: String) -> InlineObject25 {
        InlineObject25 {
            user_id,
            status,
        }
    }
}


