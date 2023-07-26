use reqwest::Url;
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

use super::requests::{AuthenticationLevel, HttpMethod, TumblrRequest};

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PostContent {
    Text {
        text: String,
        subtype: Option<String>,
        indent_level: Option<u8>,
    },
}

/// https://www.tumblr.com/docs/en/api/v2#note-about-post-states
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostState {
    #[default]
    Published,
    Queue,
    Draft,
    Private,
    Unapproved,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReblogInteractability {
    #[default]
    Everyone,
    // "noone" is not a word, tumblr!
    #[serde(rename = "noone")]
    NoOne,
}

/// A Neue Tumblr post.
///
/// https://www.tumblr.com/docs/npf
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct Post {
    pub content: Vec<PostContent>,
    pub layout: Option<Vec<()>>,
    pub state: Option<PostState>,
    pub publish_on: Option<String>,
    pub date: Option<String>,
    pub tags: Option<String>,
    pub source_url: Option<String>,
    pub send_to_twitter: Option<bool>,
    pub is_private: Option<bool>,
    pub slug: Option<String>,
    pub interactability_reblog: Option<ReblogInteractability>,
}

#[derive(Debug)]
pub struct PostCreateRequest {
    pub blog_identifier: String,
    pub parameters: Post,
}

impl TryFrom<PostCreateRequest> for TumblrRequest {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: PostCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            method: HttpMethod::Post,
            url: Url::parse(&format!(
                "https://api.tumblr.com/v2/blog/{}/posts",
                value.blog_identifier
            ))?,
            level: AuthenticationLevel::OAuth,
            json: Some(serde_json::to_string(&value.parameters)?),
        })
    }
}
