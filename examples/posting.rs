use tumblr_api::{
    auth::read_credentials,
    blog::TumblrBlogId,
    post::{Formatting, PostContent, PostCreate, PostCreateRequest, PostState},
    tags, TumblrClient,
};

const CLIENT_CACHE_PATH: &str = "client.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let post = PostCreate {
        content: vec![PostContent::Text {
            text: "hello there".to_string(),
            subtype: None,
            indent_level: None,
            formatting: Some(vec![Formatting::Color {
                start: 0,
                end: 11,
                hex: "#ff0000".to_string(),
            }]),
        }],
        state: Some(PostState::Draft),
        tags: Some(
            tags!(
                "tumblr api",
                "api",
                "the pink hacker",
                "tumblr api shenanigans"
            )
            .to_string(),
        ),
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
    tumblr_client.save_to_file(CLIENT_CACHE_PATH.into())?;

    let blog_id = TumblrBlogId::BlogName("the-pink-hacker".to_string());

    let response = tumblr_client
        .send_request(&PostCreateRequest {
            blog_id,
            parameters: post,
        })
        .await?;
    println!("Response: {:#?}", response);
    tumblr_client.save_to_file(CLIENT_CACHE_PATH.into())?;
    Ok(())
}
