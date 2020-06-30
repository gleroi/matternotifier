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
pub struct InlineObject11 {
    /// The current password for the user
    #[serde(rename = "current_password", skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    /// The new password for the user
    #[serde(rename = "new_password")]
    pub new_password: String,
}

impl InlineObject11 {
    pub fn new(new_password: String) -> InlineObject11 {
        InlineObject11 {
            current_password: None,
            new_password,
        }
    }
}


