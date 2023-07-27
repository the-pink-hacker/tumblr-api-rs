use chrono::{serde::ts_seconds, DateTime, Utc};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;

use super::requests::{AuthenticationLevel, HttpMethod, TumblrRequest};

#[derive(Debug, Serialize, Deserialize)]
pub struct TumblrUuid(String);

/// For Post creation, only the UUID field is required.
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct BlogMention {
    uuid: TumblrUuid,
    name: Option<String>,
    url: Option<Url>,
}

#[derive(Debug, Serialize)]
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

impl TryFrom<BlogInfoRequest> for TumblrRequest {
    type Error = oauth2::url::ParseError;

    fn try_from(value: BlogInfoRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            method: HttpMethod::Get,
            url: Url::parse(&format!(
                "https://api.tumblr.com/v2/blog/{}/info",
                value.blog_id.to_string()
            ))?,
            level: AuthenticationLevel::Key,
            json: None,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct TumblrBlog {
    ask: bool,
    ask_anon: bool,
    ask_page_title: String,
    asks_allow_media: bool,
    //avatar: {}
    can_chat: bool,
    can_subscribe: bool,
    description: String,
    is_nsfw: bool,
    name: String,
    posts: u32,
    share_likes: bool,
    subscribed: bool,
    //theme: {}
    title: String,
    total_posts: u32,
    #[serde(with = "ts_seconds")]
    updated: DateTime<Utc>,
    url: Url,
    uuid: TumblrUuid,
}

#[derive(Debug, Deserialize)]
pub struct BlogInfoResponse {
    blog: TumblrBlog,
}
