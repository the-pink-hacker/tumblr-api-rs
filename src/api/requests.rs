use std::collections::HashMap;

use reqwest::{header::CONTENT_TYPE, RequestBuilder, Url};

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

#[derive(Debug)]
pub struct TumblrRequest {
    pub method: HttpMethod,
    pub url: Url,
    pub level: AuthenticationLevel,
    pub json: Option<String>,
}

impl TumblrClient {
    pub async fn request(&mut self, request: TumblrRequest) -> DynResult<String> {
        // Refresh token if expired
        self.refresh_if_expired().await?;

        let mut builder = match request.method {
            HttpMethod::Get => self.request_client.get(request.url),
            HttpMethod::Post => self.request_client.post(request.url),
            HttpMethod::Delete => self.request_client.delete(request.url),
        };

        if let Some(json) = request.json {
            builder = self.request_with_json(builder, json)
        }

        builder = match request.level {
            AuthenticationLevel::None => builder,
            AuthenticationLevel::Key => self.request_with_key(builder),
            AuthenticationLevel::OAuth => self.request_with_oauth(builder),
        };

        Ok(builder.send().await?.text().await?)
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
