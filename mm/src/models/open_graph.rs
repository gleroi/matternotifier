/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */

/// OpenGraph : OpenGraph metadata of a webpage



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenGraph {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "determiner", skip_serializing_if = "Option::is_none")]
    pub determiner: Option<String>,
    #[serde(rename = "site_name", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "locales_alternate", skip_serializing_if = "Option::is_none")]
    pub locales_alternate: Option<Vec<String>>,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::OpenGraphImages>>,
    #[serde(rename = "videos", skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<crate::models::OpenGraphVideos>>,
    #[serde(rename = "audios", skip_serializing_if = "Option::is_none")]
    pub audios: Option<Vec<crate::models::OpenGraphAudios>>,
    #[serde(rename = "article", skip_serializing_if = "Option::is_none")]
    pub article: Option<crate::models::OpenGraphArticle>,
    #[serde(rename = "book", skip_serializing_if = "Option::is_none")]
    pub book: Option<crate::models::OpenGraphBook>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<crate::models::OpenGraphArticleAuthors>,
}

impl OpenGraph {
    /// OpenGraph metadata of a webpage
    pub fn new() -> OpenGraph {
        OpenGraph {
            _type: None,
            url: None,
            title: None,
            description: None,
            determiner: None,
            site_name: None,
            locale: None,
            locales_alternate: None,
            images: None,
            videos: None,
            audios: None,
            article: None,
            book: None,
            profile: None,
        }
    }
}


