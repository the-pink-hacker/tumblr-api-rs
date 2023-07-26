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

impl TumblrClient {
    pub async fn request(
        &mut self,
        method: HttpMethod,
        url: Url,
        level: AuthenticationLevel,
        json: Option<String>,
    ) -> DynResult<String> {
        // Refresh token if expired
        self.refresh_if_expired().await?;

        let mut builder = match method {
            HttpMethod::Get => self.request_client.get(url),
            HttpMethod::Post => self.request_client.post(url),
            HttpMethod::Delete => self.request_client.delete(url),
        };

        if let Some(json) = json {
            builder = builder
                .form(&HashMap::<String, String>::new())
                .header(CONTENT_TYPE, "application/json")
                .body(json);
        }

        Ok(match level {
            AuthenticationLevel::None => builder.send().await?.text().await?,
            AuthenticationLevel::Key => self.request_with_key(builder).await?,
            AuthenticationLevel::OAuth => self.request_with_oauth(builder).await?,
        })
    }

    async fn request_with_key(&self, builder: RequestBuilder) -> reqwest::Result<String> {
        builder
            .query(&[("api_key", self.get_api_key())])
            .send()
            .await?
            .text()
            .await
    }

    async fn request_with_oauth(&self, builder: RequestBuilder) -> reqwest::Result<String> {
        builder
            .bearer_auth(self.get_access_token().secret())
            .send()
            .await?
            .text()
            .await
    }
}
