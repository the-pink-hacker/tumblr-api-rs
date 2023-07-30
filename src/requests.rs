use oauth2::{AccessToken, ClientId};
use reqwest::{header::CONTENT_TYPE, Client, Request, RequestBuilder, Url};
use serde::{de::DeserializeOwned, Deserialize};

use super::TumblrClient;

const TUMBLR_API_URL: &str = "https://api.tumblr.com";
const JSON_HEADER_VALUE: &str = "application/json";
const API_KEY_HEADER_KEY: &str = "api_key";

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Delete,
}

impl From<HttpMethod> for reqwest::Method {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Get => Self::GET,
            HttpMethod::Post => Self::POST,
            HttpMethod::Delete => Self::DELETE,
        }
    }
}

pub struct TumblrRequestBuilder {
    builder: RequestBuilder,
}

impl TumblrRequestBuilder {
    pub fn new(
        request_client: &Client,
        method: HttpMethod,
        path: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            builder: request_client
                .request(method.into(), Url::parse(TUMBLR_API_URL)?.join(&path)?),
        })
    }

    pub fn json(mut self, json: String) -> Self {
        self.builder = self
            .builder
            .header(CONTENT_TYPE, JSON_HEADER_VALUE)
            .body(json);
        self
    }

    pub fn auth_by_key(mut self, key: &ClientId) -> Self {
        self.builder = self.builder.query(&[(API_KEY_HEADER_KEY, key)]);
        self
    }

    pub fn auth_by_oauth(mut self, token: &AccessToken) -> Self {
        self.builder = self.builder.bearer_auth(token.secret());
        self
    }

    pub fn build(self) -> Result<Request, reqwest::Error> {
        self.builder.build()
    }
}

#[derive(Debug, Deserialize)]
pub struct TumblrResponseMeta {
    pub status: u16,
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct TumblrResponse<T> {
    pub meta: TumblrResponseMeta,
    pub response: T,
}

pub type TumblrResponseEmpty = TumblrResponse<()>;

pub trait TumblrRequest {
    type Response: DeserializeOwned;

    fn build_request(&self, client: &TumblrClient) -> Result<Request, Box<dyn std::error::Error>>;

    fn deserialize_response(self, response_raw: &str) -> Result<Self::Response, serde_json::Error>;
}

impl TumblrClient {
    pub async fn send_request<R>(
        &mut self,
        request: R,
    ) -> Result<<R>::Response, Box<dyn std::error::Error>>
    where
        R: TumblrRequest,
    {
        // Refresh token if expired
        self.refresh_if_expired().await?;

        let response_raw = self
            .request_client
            .execute(request.build_request(self)?)
            .await?
            .text()
            .await?;

        Ok(request.deserialize_response(&response_raw)?)
    }
}
