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
pub struct InlineObject51 {
    /// The ID of the scheme.
    #[serde(rename = "scheme_id")]
    pub scheme_id: String,
}

impl InlineObject51 {
    pub fn new(scheme_id: String) -> InlineObject51 {
        InlineObject51 {
            scheme_id,
        }
    }
}


