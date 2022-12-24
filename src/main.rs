use ig_rs::igclient::{IGLoggedOutClient, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let username = env::var("USERNAME").expect("Username to be present");
    let password = env::var("PASSWORD").expect("Password to be present");
    let client = IGLoggedOutClient::login(&username, &password).await?;

    Ok(())
}
