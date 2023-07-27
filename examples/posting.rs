use tumblr_api::{
    auth::read_credentials,
    blog::TumblrBlogId,
    post::{Formatting, Post, PostContent, PostCreateRequest, PostState},
    TumblrClient,
};

const CLIENT_CACHE_PATH: &str = "client.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let post = Post {
        content: vec![PostContent::Text {
            text: "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588} ".to_string(),
            subtype: None,
            indent_level: None,
            formatting: Some(vec![
                Formatting::Color {
                    start: 0,
                    end: 1,
                    hex: "#ff0000".to_string(),
                },
                Formatting::Color {
                    start: 1,
                    end: 2,
                    hex: "#ff4000".to_string(),
                },
                Formatting::Color {
                    start: 2,
                    end: 3,
                    hex: "#ff8000".to_string(),
                },
                Formatting::Color {
                    start: 3,
                    end: 4,
                    hex: "#ffbf00".to_string(),
                },
                Formatting::Color {
                    start: 4,
                    end: 5,
                    hex: "#ffff00".to_string(),
                },
                Formatting::Color {
                    start: 5,
                    end: 6,
                    hex: "#bfff00".to_string(),
                },
                Formatting::Color {
                    start: 6,
                    end: 7,
                    hex: "#80ff00".to_string(),
                },
                Formatting::Color {
                    start: 7,
                    end: 8,
                    hex: "#40ff00".to_string(),
                },
                Formatting::Color {
                    start: 8,
                    end: 9,
                    hex: "#00ff00".to_string(),
                },
                Formatting::Color {
                    start: 9,
                    end: 10,
                    hex: "#00ff40".to_string(),
                },
                Formatting::Color {
                    start: 10,
                    end: 11,
                    hex: "#00ff80".to_string(),
                },
                Formatting::Color {
                    start: 11,
                    end: 12,
                    hex: "#00ffbf".to_string(),
                },
                Formatting::Color {
                    start: 12,
                    end: 13,
                    hex: "#00ffff".to_string(),
                },
                Formatting::Color {
                    start: 13,
                    end: 14,
                    hex: "#00bfff".to_string(),
                },
                Formatting::Color {
                    start: 14,
                    end: 15,
                    hex: "#0080ff".to_string(),
                },
                Formatting::Color {
                    start: 15,
                    end: 16,
                    hex: "#0040ff".to_string(),
                },
                Formatting::Color {
                    start: 16,
                    end: 17,
                    hex: "#0000ff".to_string(),
                },
                Formatting::Color {
                    start: 17,
                    end: 18,
                    hex: "#4000ff".to_string(),
                },
                Formatting::Color {
                    start: 18,
                    end: 19,
                    hex: "#8000ff".to_string(),
                },
                Formatting::Color {
                    start: 19,
                    end: 20,
                    hex: "#bf00ff".to_string(),
                },
                Formatting::Color {
                    start: 20,
                    end: 21,
                    hex: "#ff00ff".to_string(),
                },
                Formatting::Color {
                    start: 21,
                    end: 22,
                    hex: "#ff00bf".to_string(),
                },
                Formatting::Color {
                    start: 22,
                    end: 23,
                    hex: "#ff0080".to_string(),
                },
                Formatting::Color {
                    start: 23,
                    end: 24,
                    hex: "#ff0040".to_string(),
                },
                Formatting::Color {
                    start: 24,
                    end: 25,
                    hex: "#ff0000".to_string(),
                },
            ]),
        }],
        state: Some(PostState::Draft),
        tags: Some("tumblr api,api,the pink hacker,tumblr api shenanigans".to_string()),
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

    let blog_id = TumblrBlogId::BlogName("the-pink-hacker".to_string());

    let response = tumblr_client
        .request(
            PostCreateRequest {
                blog_id,
                parameters: post,
            }
            .try_into()?,
        )
        .await?;
    println!("Response: {}", response);
    tumblr_client.save_to_file(CLIENT_CACHE_PATH.into())?;
    Ok(())
}
