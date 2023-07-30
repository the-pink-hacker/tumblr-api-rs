mod content;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    requests::{HttpMethod, TumblrRequest, TumblrRequestBuilder, TumblrResponse},
    TumblrClient,
};

pub use self::content::*;

use super::blog::TumblrBlogId;

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

impl TumblrRequest for PostCreateRequest {
    type Response = TumblrResponse<PostCreateResponse>;

    fn build_request(
        &self,
        client: &TumblrClient,
    ) -> Result<reqwest::Request, Box<dyn std::error::Error>> {
        Ok(TumblrRequestBuilder::new(
            &client.request_client,
            HttpMethod::Post,
            format!("v2/blog/{}/posts", self.blog_id.clone().to_string()),
        )?
        .auth_by_oauth(client.get_access_token())
        .json(serde_json::to_string(&self.parameters)?)
        .build()?)
    }

    fn deserialize_response(self, response_raw: &str) -> Result<Self::Response, serde_json::Error> {
        serde_json::from_str(response_raw)
    }
}

#[derive(Debug, Deserialize)]
pub struct PostCreateResponse {
    id: String,
}
