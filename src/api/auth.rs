use std::{fs, io};

use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, PkceCodeChallenge, RedirectUrl, StandardRevocableToken, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use snafu::prelude::*;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

const CREDENTIALS_PATH: &str = "credentials.json";
const REDIRECT_URL: &str = "http://localhost:8080/";
const AUTHORIZE_URL: &str = "https://www.tumblr.com/oauth2/authorize";
const TOKEN_URL: &str = "https://api.tumblr.com/v2/oauth2/token";

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to load credential json file. {}", source))]
    CredentialFileParse {
        source: io::Error,
    },
    #[snafu(display(
        "Tumblr's state doesn't not match. Expected: '{}', Received: '{}'.",
        expected,
        result
    ))]
    StateDoNotMatch {
        expected: String,
        result: String,
    },
    #[snafu(display("URL can't be parsed."))]
    UrlParse,
    #[snafu(display("URL arguments can't be parsed. {}", source))]
    UrlArgumentsParse {
        source: serde_urlencoded::de::Error,
    },
    RedirectServer {
        source: io::Error,
    },
}

#[derive(Deserialize)]
struct ConsumerCredentials {
    pub consumer_key: String,
    pub consumer_secret: String,
}

fn read_credentials() -> io::Result<ConsumerCredentials> {
    let file_contents = fs::read_to_string(CREDENTIALS_PATH)?;
    Ok(serde_json::from_str(&file_contents)?)
}

#[derive(Debug, Deserialize)]
struct RedirectUrlArguments {
    pub code: String,
    pub state: String,
}

pub async fn authorize() -> Result<(), Error> {
    let credentials = read_credentials().context(CredentialFileParseSnafu)?;
    let client = BasicClient::new(
        ClientId::new(credentials.consumer_key),
        Some(ClientSecret::new(credentials.consumer_secret)),
        AuthUrl::new(AUTHORIZE_URL.to_string()).expect("Auth URL failed to be created."),
        Some(TokenUrl::new(TOKEN_URL.to_string()).expect("Token URL failed to be created.")),
    )
    .set_redirect_uri(
        RedirectUrl::new(REDIRECT_URL.to_string()).expect("Redirect URL failed to be created."),
    );

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    // Redirect server
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .context(RedirectServerSnafu)?;
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let code;
            let state;
            let mut buffer;
            {
                buffer = BufReader::new(stream);

                let mut request_line = String::new();
                buffer
                    .read_line(&mut request_line)
                    .await
                    .context(RedirectServerSnafu)?;

                let redirect_url = serde_urlencoded::from_str::<RedirectUrlArguments>(
                    request_line
                        .split_whitespace()
                        .nth(1)
                        .context(UrlParseSnafu)?
                        .split('?')
                        .nth(1)
                        .context(UrlParseSnafu)?,
                )
                .context(UrlArgumentsParseSnafu)?;

                code = AuthorizationCode::new(redirect_url.code);
                state = CsrfToken::new(redirect_url.state);
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );

            buffer
                .write_all(response.as_bytes())
                .await
                .context(RedirectServerSnafu)?;

            println!("Tumblr returned the following code:\n{}\n", code.secret());
            ensure!(
                csrf_token.secret() == state.secret(),
                StateDoNotMatchSnafu {
                    expected: csrf_token.secret(),
                    result: state.secret()
                }
            );

            // Exchange the code with a token.
            let token_response = client
                .exchange_code(code)
                .set_pkce_verifier(pkce_verifier)
                .request(http_client);

            println!(
                "Tumblr returned the following token:\n{:?}\n",
                token_response
            );

            // Revoke the obtained token
            let token_response = token_response.expect("Failed to get token.");
            let token_to_revoke: StandardRevocableToken = match token_response.refresh_token() {
                Some(token) => token.into(),
                None => token_response.access_token().into(),
            };

            client
                .revoke_token(token_to_revoke)
                .unwrap()
                .request(http_client)
                .expect("Failed to revoke token");

            // The server will terminate itself after revoking the token.
            break;
        }
    }

    Ok(())
}
