mod api;

use std::error::Error;

use api::auth::{read_credentials, TumblrClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credentials = read_credentials()?;
    let tumblr_client = TumblrClient::new(credentials);
    tumblr_client.authorize();
    Ok(())
}
