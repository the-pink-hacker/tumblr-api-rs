use tumblr_api::{
    auth::read_credentials, blog::TumblrBlogId, requests::blog::posts::PostGetRequest, TumblrClient,
};

const CLIENT_CACHE_PATH: &str = "client.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = read_credentials()?;
    let mut tumblr_client = TumblrClient::try_from_file_or_authorize(
        CLIENT_CACHE_PATH.into(),
        credentials,
        reqwest::Client::new(),
    )
    .await?;
    tumblr_client.save_to_file(CLIENT_CACHE_PATH.into())?;

    let response = tumblr_client
        .send_request(&PostGetRequest {
            blog_id: TumblrBlogId::BlogName("the-pink-hacker".to_string()),
            post_id: "724420450593144832".to_string(),
        })
        .await?;
    println!("Response: {:#?}", response);
    tumblr_client.save_to_file(CLIENT_CACHE_PATH.into())?;
    Ok(())
}
