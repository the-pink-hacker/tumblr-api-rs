mod api {
    pub mod auth {
        use std::{error::Error, fs, io};

        use http::{header::AUTHORIZATION, HeaderMap};
        use oauth::Credentials;
        use reqwest::Client;
        use serde::Deserialize;

        const CREDENTIALS_PATH: &str = "credentials.json";
        const REQUEST_TOKEN_URI: &str = "https://www.tumblr.com/oauth/request_token";

        #[derive(Deserialize)]
        struct ConsumerCredentials {
            pub consumer_key: String,
            pub consumer_secret: String,
        }

        impl From<ConsumerCredentials> for Credentials<Box<str>> {
            fn from(value: ConsumerCredentials) -> Self {
                Self {
                    identifier: value.consumer_key.into(),
                    secret: value.consumer_secret.into(),
                }
            }
        }

        #[derive(Deserialize)]
        struct TokenCredentials {
            pub oauth_token: String,
            pub oauth_token_secret: String,
        }

        impl From<TokenCredentials> for Credentials<Box<str>> {
            fn from(value: TokenCredentials) -> Self {
                Self {
                    identifier: value.oauth_token.into(),
                    secret: value.oauth_token_secret.into(),
                }
            }
        }

        pub fn get_credentials() -> io::Result<Credentials> {
            let contents = fs::read_to_string(CREDENTIALS_PATH)?;
            let file = serde_json::from_slice::<ConsumerCredentials>(contents.as_bytes())?;
            Ok(oauth::Credentials::new(
                file.consumer_key,
                file.consumer_secret,
            ))
        }

        pub async fn temporary_credentials(
            credentials: Credentials,
            client: &Client,
        ) -> Result<Credentials<Box<str>>, Box<dyn Error>> {
            let authorization_header =
                oauth::Builder::<_, _>::new(credentials, oauth::HmacSha1::new())
                    .post(REQUEST_TOKEN_URI, &());

            let mut headers = HeaderMap::new();
            headers.insert(AUTHORIZATION, authorization_header.parse().unwrap());
            let body = client
                .post(REQUEST_TOKEN_URI)
                .headers(headers)
                .send()
                .await?
                .text()
                .await?;

            Ok(serde_urlencoded::from_str::<TokenCredentials>(&body)?.into())
        }
    }
}

use std::error::Error;

use oauth::Token;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credentials = api::auth::get_credentials()?;
    let client = reqwest::Client::new();

    let token = api::auth::temporary_credentials(credentials.clone(), &client).await?;
    let token = Token::new(credentials, token);

    println!("{:?}", token);

    Ok(())
}
