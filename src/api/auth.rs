use std::{fs, io, time::Duration};

use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    reqwest::http_client,
    AccessToken, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken,
    EmptyExtraTokenFields, PkceCodeChallenge, RedirectUrl, RefreshToken, RequestTokenError,
    RevocationErrorResponseType, Scope, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

const CREDENTIALS_PATH: &str = "credentials.json";
const REDIRECT_URL: &str = "http://localhost:8080/";
const AUTHORIZE_URL: &str = "https://www.tumblr.com/oauth2/authorize";
const TOKEN_URL: &str = "https://api.tumblr.com/v2/oauth2/token";

type OauthClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to load credential json file. {}", source))]
    CredentialFileParse { source: io::Error },
    #[snafu(display("CSRF token mis. Expected: '{}', Received: '{}'.", expected, result))]
    StateDoNotMatch { expected: String, result: String },
    #[snafu(display("URL can't be parsed."))]
    UrlParse,
    #[snafu(display("URL arguments can't be parsed: {}", source))]
    UrlArgumentsParse { source: serde_urlencoded::de::Error },
    #[snafu(display("Redirect server failed: {}", source))]
    RedirectServer { source: io::Error },
    #[snafu(display("Failed to get access token: {}", source))]
    AccessToken {
        source: RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    },
    #[snafu(display("No refresh token was present."))]
    NoRefreshToken,
}

#[derive(Deserialize)]
struct ConsumerCredentials {
    pub consumer_key: String,
    pub consumer_secret: String,
}

pub fn read_credentials() -> io::Result<ConsumerCredentials> {
    let file_contents = fs::read_to_string(CREDENTIALS_PATH)?;
    Ok(serde_json::from_str(&file_contents)?)
}

async fn listen_for_code() -> Result<(AuthorizationCode, CsrfToken), Error> {
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

            return Ok((code, state));
        }
    }
}

#[derive(Debug, Deserialize)]
struct RedirectUrlArguments {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TumblrClientTokens {
    access_token: AccessToken,
    expires_in: Duration,
    last_refreshed: u64,
    refresh_token: Option<RefreshToken>,
}

impl TumblrClientTokens {
    fn refresh_token(&mut self, client: OauthClient) -> Result<(), Error> {
        let refresh_token = self.refresh_token.expect("Refresh token no present.");
        let refresh_response = client
            .exchange_refresh_token(&refresh_token)
            .request(http_client)
            .context(AccessTokenSnafu)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TumblrClient {
    token: Option<TumblrClientTokens>,
    client: OauthClient,
}

impl TumblrClient {
    pub fn new(credentials: ConsumerCredentials) -> Self {
        Self {
            token: None,
            client: Self::create_oauth_client(credentials),
        }
    }

    fn create_oauth_client(credentials: ConsumerCredentials) -> OauthClient {
        BasicClient::new(
            ClientId::new(credentials.consumer_key),
            Some(ClientSecret::new(credentials.consumer_secret)),
            AuthUrl::new(AUTHORIZE_URL.to_string()).expect("Auth URL failed to be created."),
            Some(TokenUrl::new(TOKEN_URL.to_string()).expect("Token URL failed to be created.")),
        )
        .set_redirect_uri(
            RedirectUrl::new(REDIRECT_URL.to_string()).expect("Redirect URL failed to be created."),
        )
    }

    pub async fn authorize(&self) -> Result<(), Error> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("offline_access".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        println!("Browse to: {}", auth_url);

        // Redirect server
        let (code, state) = listen_for_code().await?;

        ensure!(
            csrf_token.secret() == state.secret(),
            StateDoNotMatchSnafu {
                expected: csrf_token.secret(),
                result: state.secret()
            }
        );
        println!("Tumblr returned the following code:\n{}\n", code.secret());

        // Exchange the code with a token.
        let token_response = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request(http_client)
            .context(AccessTokenSnafu)?;

        println!(
            "Tumblr returned the following token:\n{:?}\n",
            token_response
        );

        Ok(())
    }
}
