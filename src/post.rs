mod content;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    paths,
    requests::{HttpMethod, TumblrRequest, TumblrRequestBuilder},
    TumblrClient,
};

pub use self::content::*;

use super::blog::TumblrBlogId;

/// https://www.tumblr.com/docs/en/api/v2#note-about-post-states
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PostState {
    #[default]
    Published,
    Queue,
    Draft,
    Private,
    Unapproved,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PostInteractability {
    #[default]
    Everyone,
    // "noone" is not a word, tumblr!
    #[serde(rename = "noone")]
    NoOne,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PostLayout(());

#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct ReblogInfo {
    pub parent_tumblelog_uuid: String,
    pub parent_post_id: u64,
    pub reblog_key: String,
    pub hide_trail: Option<bool>,
    pub exclude_trail_items: Option<Vec<u16>>,
}

/// A Neue Tumblr post.
///
/// https://www.tumblr.com/docs/npf
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct PostCreate {
    pub content: Vec<PostContent>,
    pub layout: Option<Vec<PostLayout>>,
    pub state: Option<PostState>,
    pub publish_on: Option<String>,
    pub date: Option<String>,
    pub tags: Option<String>,
    pub source_url: Option<Url>,
    pub send_to_twitter: Option<bool>,
    pub is_private: Option<bool>,
    pub slug: Option<String>,
    pub interactability_reblog: Option<PostInteractability>,
    #[serde(flatten)]
    pub reblog_info: Option<ReblogInfo>,
}

#[derive(Debug, Deserialize)]
pub struct PostGet {
    pub object_type: String,
    #[serde(rename = "type")]
    pub post_type: String,
    pub id: u64,
    pub tumblelog_uuid: String,
    pub parent_post_id: Option<String>,
    pub parent_tumblelog_uuid: Option<String>,
    pub reblog_key: String,
    pub original_type: String,
    pub is_blocks_post_format: bool,
    pub blog_name: String,
    pub id_string: String,
    pub is_blazed: bool,
    pub is_blaze_pending: bool,
    pub can_ignite: bool,
    pub can_blaze: bool,
    pub post_url: Url,
    pub slug: String,
    pub date: String,
    pub timestamp: u32,
    pub state: PostState,
    pub tags: Vec<String>,
    pub short_url: Url,
    pub summary: String,
    pub should_open_in_legacy: bool,
    // recommended_source
    // recommended_color
    pub followed: bool,
    pub liked: bool,
    pub note_count: u32,
    pub content: Vec<PostContent>,
    pub layout: Vec<PostLayout>,
    pub trail: Vec<PostTrail>,
    //pub queued_state: Option<()>,
    //pub scheduled_publish_time: Option<()>,
    //pub publish_on: Option<()>,
    pub can_like: bool,
    pub interactability_reblog: PostInteractability,
    pub interactability_blaze: PostInteractability,
    pub can_reblog: bool,
    pub can_send_in_message: bool,
    pub muted: bool,
    pub mute_end_timestamp: u32,
    pub can_mute: bool,
}

#[derive(Debug, Deserialize)]
pub struct PostTrail {
    // blog
    pub content: Vec<PostContent>,
    pub layout: Vec<PostLayout>,
    pub post: PostTrailId,
}

#[derive(Debug, Deserialize)]
pub struct PostTrailId {
    pub id: String,
}

#[derive(Debug)]
pub struct PostCreateRequest {
    pub blog_id: TumblrBlogId,
    pub parameters: PostCreate,
}

impl TumblrRequest for PostCreateRequest {
    type Response = PostCreateResponse;

    fn build_request(
        &self,
        client: &TumblrClient,
    ) -> Result<reqwest::Request, Box<dyn std::error::Error>> {
        Ok(TumblrRequestBuilder::new(
            &client.request_client,
            HttpMethod::Post,
            paths::blog_post_create(self.blog_id.clone().to_string())?,
        )?
        .auth_by_oauth(client.get_access_token())
        .json(serde_json::to_string(&self.parameters)?)
        .build()?)
    }
}

#[derive(Debug, Deserialize)]
pub struct PostCreateResponse {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct PostGetRequest {
    pub blog_id: TumblrBlogId,
    pub post_id: String,
}

impl TumblrRequest for PostGetRequest {
    type Response = PostGetResponse;

    fn build_request(
        &self,
        client: &TumblrClient,
    ) -> Result<reqwest::Request, Box<dyn std::error::Error>> {
        Ok(TumblrRequestBuilder::new(
            &client.request_client,
            HttpMethod::Get,
            paths::blog_post(self.blog_id.clone().to_string(), self.post_id.clone())?,
        )?
        .auth_by_oauth(client.get_access_token())
        .build()?)
    }
}

#[derive(Debug, Deserialize)]
pub struct PostGetResponse {
    #[serde(flatten)]
    pub parameters: PostGet,
}
