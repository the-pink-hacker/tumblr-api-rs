use reqwest::{header::CONTENT_TYPE, RequestBuilder, Url};
use serde::{de::DeserializeOwned, Deserialize};

use super::TumblrClient;

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

/// The API uses three different levels of authentication, depending on the method.
///
/// `None`: No authentication. Anybody can query the method.
///
/// `API key`: Requires an API key. Use your OAuth Consumer Key as your api_key.
///
/// `OAuth`: Requires a signed request that meets the OAuth 1.0a Protocol.
#[derive(Debug)]
pub enum AuthenticationLevel {
    None,
    Key,
    OAuth,
}

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

#[derive(Debug)]
pub struct TumblrRequest {
    pub method: HttpMethod,
    pub url: Url,
    pub level: AuthenticationLevel,
    pub json: Option<String>,
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

impl TumblrClient {
    pub async fn request<T>(&mut self, request: TumblrRequest) -> DynResult<T>
    where
        T: DeserializeOwned,
    {
        // Refresh token if expired
        self.refresh_if_expired().await?;

        let mut builder = self
            .request_client
            .request(request.method.into(), request.url);

        if let Some(json) = request.json {
            builder = self.request_with_json(builder, json)
        }

        builder = match request.level {
            AuthenticationLevel::None => builder,
            AuthenticationLevel::Key => self.request_with_key(builder),
            AuthenticationLevel::OAuth => self.request_with_oauth(builder),
        };

        let response = builder.send().await?.text().await?;
        Ok(serde_json::from_str(&response)?)
    }

    fn request_with_json(&self, builder: RequestBuilder, json: String) -> RequestBuilder {
        builder.header(CONTENT_TYPE, "application/json").body(json)
    }

    fn request_with_key(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.query(&[("api_key", self.get_api_key())])
    }

    fn request_with_oauth(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.bearer_auth(self.get_access_token().secret())
    }
}
