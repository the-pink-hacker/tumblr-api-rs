mod api;

use std::error::Error;

use api::{auth::read_credentials, requests::AuthenticationLevel, TumblrClient};
use reqwest::Url;

use crate::api::requests::HttpMethod;

const CLIENT_CACHE_PATH: &str = "client.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credentials = read_credentials()?;
    let mut tumblr_client = TumblrClient::try_from_file_or_authorize(
        CLIENT_CACHE_PATH.into(),
        credentials,
        reqwest::Client::new(),
    )
    .await?;

    let blog_id = "the-pink-hacker";

    let response = tumblr_client
        .request(
            HttpMethod::Post,
            Url::parse(&format!("https://api.tumblr.com/v2/blog/{}/post", blog_id))?,
            AuthenticationLevel::OAuth,
            None,
        )
        .await?;
    println!("Response: {}", response);
    tumblr_client.save_to_file(CLIENT_CACHE_PATH.into())?;
    Ok(())
}
