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
pub struct InlineObject31 {
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl InlineObject31 {
    pub fn new() -> InlineObject31 {
        InlineObject31 {
            team_id: None,
            user_id: None,
        }
    }
}


