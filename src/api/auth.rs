use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use chrono::{prelude::*, OutOfRangeError};
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    reqwest::async_http_client,
    AccessToken, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken,
    EmptyExtraTokenFields, PkceCodeChallenge, RedirectUrl, RefreshToken, RequestTokenError,
    RevocationErrorResponseType, Scope, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse, TokenResponse, TokenUrl,
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

type TumblrTokenResponse = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

type OauthClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    TumblrTokenResponse,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
>;

type Result<T> = core::result::Result<T, Error>;

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
    #[snafu(display("Expire duration is too big: {}", source))]
    ExpireDurationTooBig { source: OutOfRangeError },
    #[snafu(display("Failed to load client cache file: {}", source))]
    LoadClientCacheFile { source: io::Error },
    #[snafu(display("Failed to save client cache file: {}", source))]
    SaveClientCacheFile { source: io::Error },
    #[snafu(display("Failed to deserialize client cache file: {}", source))]
    DeserializeClientCacheFile { source: serde_json::Error },
    #[snafu(display("Failed to serialize client cache file: {}", source))]
    SerializeClientCacheFile { source: serde_json::Error },
}

#[derive(Deserialize)]
pub struct ConsumerCredentials {
    pub consumer_key: String,
    pub consumer_secret: String,
}

pub fn read_credentials() -> io::Result<ConsumerCredentials> {
    let file_contents = fs::read_to_string(CREDENTIALS_PATH)?;
    Ok(serde_json::from_str(&file_contents)?)
}

async fn listen_for_code() -> Result<(AuthorizationCode, CsrfToken)> {
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
    refresh_at: DateTime<Utc>,
    refresh_token: Option<RefreshToken>,
}

impl TumblrClientTokens {
    async fn authorize(client: &OauthClient) -> Result<Self> {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes([
                Scope::new("offline_access".to_string()),
                Scope::new("basic".to_string()),
                Scope::new("write".to_string()),
            ])
            .set_pkce_challenge(pkce_challenge)
            .url();

        println!("Browse to: {}", auth_url);

        // Redirect server
        let (code, state) = listen_for_code().await?;

        // The state received from Tumblr, should match what we sent
        ensure!(
            csrf_token.secret() == state.secret(),
            StateDoNotMatchSnafu {
                expected: csrf_token.secret(),
                result: state.secret()
            }
        );

        // Exchange the code with a token.
        client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
            .context(AccessTokenSnafu)?
            .try_into()
    }

    fn from_file(path: PathBuf) -> Result<Self> {
        let contents = fs::read_to_string(path).context(LoadClientCacheFileSnafu)?;
        serde_json::from_str(&contents).context(DeserializeClientCacheFileSnafu)
    }

    async fn try_from_file_or_authorize(path: PathBuf, client: &OauthClient) -> Result<Self> {
        if let Ok(cached) = Self::from_file(path) {
            println!("Client cached. Loading from file.");
            Ok(cached)
        } else {
            Self::authorize(client).await
        }
    }

    fn save_to_file(&self, path: PathBuf) -> Result<()> {
        let mut file = File::create(path).context(SaveClientCacheFileSnafu)?;
        let contents = serde_json::to_string_pretty(self).context(SerializeClientCacheFileSnafu)?;
        file.write_all(contents.as_bytes())
            .context(SaveClientCacheFileSnafu)
    }

    async fn refresh_token(&mut self, client: &OauthClient) -> Result<()> {
        let refresh_token = self
            .refresh_token
            .as_ref()
            .expect("Refresh token no present.");
        let response = client
            .exchange_refresh_token(&refresh_token)
            .request_async(async_http_client)
            .await
            .context(AccessTokenSnafu)?;
        self.load_response(response)
    }

    /// Takes `expires_in` and adds it to the current time.
    fn calculate_refresh_at(expires_in: Option<std::time::Duration>) -> Result<DateTime<Utc>> {
        let expires_in = chrono::Duration::from_std(expires_in.context(NoRefreshTokenSnafu)?)
            .context(ExpireDurationTooBigSnafu)?;
        Ok(Utc::now() + expires_in)
    }

    fn needs_refresh(&self) -> bool {
        self.refresh_at <= Utc::now()
    }

    async fn refresh_if_expired(&mut self, client: &OauthClient) -> Result<()> {
        if self.needs_refresh() {
            println!("Refreshing token.");
            self.refresh_token(client).await
        } else {
            Ok(())
        }
    }

    fn load_response(&mut self, response: TumblrTokenResponse) -> Result<()> {
        self.access_token = response.access_token().clone();
        self.refresh_at = Self::calculate_refresh_at(response.expires_in())?;
        self.refresh_token = response.refresh_token().cloned();
        Ok(())
    }
}

impl TryFrom<TumblrTokenResponse> for TumblrClientTokens {
    type Error = Error;

    fn try_from(value: TumblrTokenResponse) -> core::result::Result<Self, Self::Error> {
        Ok(Self {
            access_token: value.access_token().clone(),
            refresh_at: Self::calculate_refresh_at(value.expires_in())?,
            refresh_token: value.refresh_token().cloned(),
        })
    }
}

#[derive(Debug)]
pub struct TumblrClient {
    token: TumblrClientTokens,
    pub client: OauthClient,
    pub request_client: reqwest::Client,
}

impl TumblrClient {
    pub async fn authorize(
        credentials: ConsumerCredentials,
        request_client: reqwest::Client,
    ) -> Result<Self> {
        let client = Self::create_oauth_client(credentials);
        Ok(Self {
            token: TumblrClientTokens::authorize(&client).await?,
            client,
            request_client,
        })
    }

    pub fn from_file(
        path: PathBuf,
        credentials: ConsumerCredentials,
        request_client: reqwest::Client,
    ) -> Result<Self> {
        Ok(Self {
            token: TumblrClientTokens::from_file(path)?,
            client: Self::create_oauth_client(credentials),
            request_client,
        })
    }

    pub async fn try_from_file_or_authorize(
        path: PathBuf,
        credentials: ConsumerCredentials,
        request_client: reqwest::Client,
    ) -> Result<Self> {
        let client = Self::create_oauth_client(credentials);
        Ok(Self {
            token: TumblrClientTokens::try_from_file_or_authorize(path, &client).await?,
            client,
            request_client,
        })
    }

    pub fn save_to_file(&self, path: PathBuf) -> Result<()> {
        self.token.save_to_file(path)
    }

    pub async fn refresh_if_expired(&mut self) -> Result<()> {
        self.token.refresh_if_expired(&self.client).await
    }

    pub fn get_api_key(&self) -> &ClientId {
        self.client.client_id()
    }

    pub fn get_access_token(&self) -> &AccessToken {
        &self.token.access_token
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
}
