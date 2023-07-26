mod api;

use std::error::Error;

use api::{auth::read_credentials, requests::AuthenticationLevel, TumblrClient};
use reqwest::Url;

use crate::api::{
    post::{Post, PostContent, PostCreateRequest, PostState, ReblogInteractability},
    requests::{HttpMethod, TumblrRequest},
};

const CLIENT_CACHE_PATH: &str = "client.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let post = Post {
        content: vec![PostContent::Text {
            text: "hello from serde".to_string(),
            subtype: None,
            indent_level: None,
        }],
        state: Some(PostState::Draft),
        ..Default::default()
    };
    println!("Post: {}", serde_json::to_string_pretty(&post)?);
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
            PostCreateRequest {
                blog_identifier: blog_id.to_string(),
                parameters: post,
            }
            .try_into()?,
        )
        .await?;
    println!("Response: {}", response);
    tumblr_client.save_to_file(CLIENT_CACHE_PATH.into())?;
    Ok(())
}
