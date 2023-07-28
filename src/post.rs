mod content;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub use self::content::*;

use super::{
    blog::TumblrBlogId,
    requests::{AuthenticationLevel, HttpMethod, TumblrRequest},
};

/// https://www.tumblr.com/docs/en/api/v2#note-about-post-states
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PostState {
    #[default]
    Published,
    Queue,
    Draft,
    Private,
    Unapproved,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
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
    pub source_url: Option<Url>,
    pub send_to_twitter: Option<bool>,
    pub is_private: Option<bool>,
    pub slug: Option<String>,
    pub interactability_reblog: Option<ReblogInteractability>,
}

#[derive(Debug)]
pub struct PostCreateRequest {
    pub blog_id: TumblrBlogId,
    pub parameters: Post,
}

impl TryFrom<PostCreateRequest> for TumblrRequest {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: PostCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            method: HttpMethod::Post,
            url: Url::parse(&format!(
                "https://api.tumblr.com/v2/blog/{}/posts",
                value.blog_id.to_string()
            ))?,
            level: AuthenticationLevel::OAuth,
            json: Some(serde_json::to_string(&value.parameters)?),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct PostCreateResponse {
    id: String,
}
