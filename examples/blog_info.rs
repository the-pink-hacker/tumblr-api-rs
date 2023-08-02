use tumblr_api::{
    auth::read_credentials,
    blog::{BlogInfoRequest, TumblrBlogId},
    TumblrClient,
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

    let blog_id = TumblrBlogId::BlogName("the-pink-hacker".to_string());

    let response = tumblr_client
        .send_request(&BlogInfoRequest { blog_id })
        .await?;
    println!("Response: {:#?}", response);
    tumblr_client.save_to_file(CLIENT_CACHE_PATH.into())?;
    Ok(())
}
