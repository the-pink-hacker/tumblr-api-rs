use std::{
    error::Error,
    fs,
    io::{self, BufRead, BufReader, Write},
    net::TcpListener,
};

use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, PkceCodeChallenge, RedirectUrl, StandardRevocableToken, TokenResponse, TokenUrl,
};
use reqwest::Url;
use serde::Deserialize;

const CREDENTIALS_PATH: &str = "credentials.json";
const REDIRECT_URL: &str = "http://localhost:8080/";
const AUTHORIZE_URL: &str = "https://www.tumblr.com/oauth2/authorize";
const TOKEN_URL: &str = "https://api.tumblr.com/v2/oauth2/token";

#[derive(Deserialize)]
struct ConsumerCredentials {
    pub consumer_key: String,
    pub consumer_secret: String,
}

fn read_credentials() -> io::Result<ConsumerCredentials> {
    let file_contents = fs::read_to_string(CREDENTIALS_PATH)?;
    Ok(serde_json::from_str(&file_contents)?)
}

pub async fn authorize() -> Result<(), Box<dyn Error>> {
    let credentials = read_credentials()?;
    let client = BasicClient::new(
        ClientId::new(credentials.consumer_key),
        Some(ClientSecret::new(credentials.consumer_secret)),
        AuthUrl::new(AUTHORIZE_URL.to_string())?,
        Some(TokenUrl::new(TOKEN_URL.to_string())?),
    )
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URL.to_string())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&(REDIRECT_URL.to_string() + redirect_url)).unwrap();

                // Parse url for code
                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                // Parse url for state
                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            println!("Tumblr returned the following code:\n{}\n", code.secret());
            assert_eq!(
                csrf_token.secret(),
                state.secret(),
                "Tumblr's state doesn't not match. Expected: {}, Received: {}",
                csrf_token.secret(),
                state.secret(),
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
            let token_response = token_response.unwrap();
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
