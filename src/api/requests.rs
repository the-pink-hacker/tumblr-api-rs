use std::collections::HashMap;

use reqwest::Url;

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
    pub async fn get(
        &mut self,
        level: AuthenticationLevel,
        url: Url,
        form: Option<HashMap<String, String>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Refresh token if expired

        self.refresh_if_expired().await?;

        let form = &form.unwrap_or_default();

        Ok(match level {
            AuthenticationLevel::None => self.get_none(url, form).await?,
            AuthenticationLevel::Key => self.get_key(url, form).await?,
            AuthenticationLevel::OAuth => self.get_oauth(url, form).await?,
        })
    }

    async fn get_none(&self, url: Url, form: &HashMap<String, String>) -> reqwest::Result<String> {
        self.request_client
            .get(url)
            .form(form)
            .send()
            .await?
            .text()
            .await
    }

    async fn get_key(&self, url: Url, form: &HashMap<String, String>) -> reqwest::Result<String> {
        self.request_client
            .get(url)
            .query(&[("api_key", self.get_api_key())])
            .form(form)
            .send()
            .await?
            .text()
            .await
    }

    async fn get_oauth(&self, url: Url, form: &HashMap<String, String>) -> reqwest::Result<String> {
        self.request_client
            .get(url)
            .bearer_auth(self.get_access_token().secret())
            .form(&form)
            .send()
            .await?
            .text()
            .await
    }
}
