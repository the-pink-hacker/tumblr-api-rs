use chrono::{serde::ts_seconds, DateTime, Utc};
use reqwest::{Request, Url};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{paths, requests::TumblrRequestBuilder, TumblrClient};

use super::requests::{HttpMethod, TumblrRequest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TumblrUuid(String);

/// For Post creation, only the UUID field is required.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogMention {
    uuid: TumblrUuid,
    name: Option<String>,
    url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TumblrBlogId {
    Uuid(TumblrUuid),
    Hostname(String),
    BlogName(String),
}

impl TumblrBlogId {
    pub fn to_string(self) -> String {
        match self {
            TumblrBlogId::Uuid(uuid) => uuid.0,
            TumblrBlogId::Hostname(hostname) => hostname,
            TumblrBlogId::BlogName(blog_name) => blog_name,
        }
    }
}

impl From<TumblrBlogId> for String {
    fn from(value: TumblrBlogId) -> Self {
        value.to_string()
    }
}

#[derive(Debug)]
pub struct BlogInfoRequest {
    pub blog_id: TumblrBlogId,
}

impl TumblrRequest for BlogInfoRequest {
    type Response = BlogInfoResponse;

    fn build_request(&self, client: &TumblrClient) -> Result<Request, Box<dyn std::error::Error>> {
        Ok(TumblrRequestBuilder::new(
            &client.request_client,
            HttpMethod::Get,
            paths::blog_info(self.blog_id.clone().to_string())?,
        )?
        .auth_by_key(client.get_api_key())
        .build()?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TumblrBlogAvatar {
    pub width: u16,
    pub height: u16,
    pub url: Url,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvatarShape {
    Square,
    Circle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TumblrBlogTheme {
    pub header_full_width: u16,
    pub header_full_height: u16,
    pub avatar_shape: AvatarShape,
    pub background_color: String,
    pub body_font: String,
    pub header_bounds: String,
    pub header_image: Url,
    pub header_image_poster: Url,
    pub header_image_scaled: Url,
    pub header_stretch: bool,
    pub link_color: String,
    pub show_avatar: bool,
    pub show_description: bool,
    pub show_header_image: bool,
    pub show_title: bool,
    pub title_color: String,
    pub title_font: String,
    pub title_font_weight: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TumblrBlog {
    pub ask: bool,
    pub ask_anon: bool,
    pub ask_page_title: String,
    pub asks_allow_media: bool,
    pub avatar: Vec<TumblrBlogAvatar>,
    pub can_chat: bool,
    pub can_subscribe: bool,
    pub description: String,
    pub is_nsfw: bool,
    pub name: String,
    pub posts: u32,
    pub share_likes: bool,
    pub subscribed: bool,
    pub theme: TumblrBlogTheme,
    pub title: String,
    pub total_posts: u32,
    #[serde(with = "ts_seconds")]
    pub updated: DateTime<Utc>,
    pub url: Url,
    pub uuid: TumblrUuid,
}

#[derive(Debug, Deserialize)]
pub struct BlogInfoResponse {
    pub blog: TumblrBlog,
}
