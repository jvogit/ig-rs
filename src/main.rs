use ig_rs::igclient::{IGLoggedOutClient, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let ig_client_config = env::var("IG_CLIENT_CONFIG");
    let username = env::var("USERNAME");
    let password = env::var("PASSWORD");
    if let Ok(ig_client_config) = ig_client_config {
        todo!()
    }
    let client = IGLoggedOutClient::login("", "").await?;

    Ok(())
}
