use std::{error::Error, fs, io};

use http::{header::AUTHORIZATION, HeaderMap};
use serde::Deserialize;

const CREDENTIALS_PATH: &str = "credentials.json";
const URI: &str = "https://www.tumblr.com/oauth/request_token";

#[derive(Deserialize)]
struct Credentials {
    pub consumer_key: String,
    pub consumer_secret: String,
}

fn get_credentials() -> io::Result<Credentials> {
    let contents = fs::read_to_string(CREDENTIALS_PATH)?;
    Ok(serde_json::from_slice::<Credentials>(contents.as_bytes())?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credentials = get_credentials()?;

    let client = oauth::Credentials::new(credentials.consumer_key, credentials.consumer_secret);

    let authorization_header =
        oauth::Builder::<_, _>::new(client, oauth::HmacSha1::new()).post(URI, &());

    println!("{}", authorization_header);

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, authorization_header.parse().unwrap());
    let body = client
        .post(URI)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    println!("{}", body);

    Ok(())
}
