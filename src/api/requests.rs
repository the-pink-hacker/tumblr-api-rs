use std::collections::HashMap;

use reqwest::{RequestBuilder, Url};

use super::TumblrClient;

/// The API uses three different levels of authentication, depending on the method.
///
/// `None`: No authentication. Anybody can query the method.
///
/// `API key`: Requires an API key. Use your OAuth Consumer Key as your api_key.
///
/// `OAuth`: Requires a signed request that meets the OAuth 1.0a Protocol.
pub enum AuthenticationLevel {
    None,
    Key,
    OAuth,
}

impl TumblrClient {
    pub async fn get_request(
        &mut self,
        level: AuthenticationLevel,
        url: Url,
        form: Option<HashMap<String, String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Refresh token if expired
        self.refresh_if_expired().await?;

        let form = form.unwrap_or_default();
        let builder = self.request_client.get(url).form(&form);

        Ok(match level {
            AuthenticationLevel::None => builder.send().await?.text().await?,
            AuthenticationLevel::Key => self.request_with_key(builder).await?,
            AuthenticationLevel::OAuth => self.request_with_oauth(builder).await?,
        })
    }

    pub async fn post_request(
        &mut self,
        level: AuthenticationLevel,
        url: Url,
        form: Option<HashMap<String, String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Refresh token if expired
        self.refresh_if_expired().await?;

        let form = form.unwrap_or_default();
        let builder = self.request_client.post(url).form(&form);

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
