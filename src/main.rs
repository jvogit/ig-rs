use ig_rs::igclient::{IGClient, IGClientConfig, Result};
use std::env;

async fn get_ig_client() -> Result<IGClient> {
    let ig_client_config = env::var("IG_CLIENT_CONFIG");
    let username = env::var("USERNAME");
    let password = env::var("PASSWORD");

    if let Ok(ig_client_config) = ig_client_config {
        let ig_client_config = serde_json::from_str::<IGClientConfig>(&ig_client_config)
            .expect("IG_CLIENT_CONFIG should be valid JSON!");

        return Ok(IGClient::with_ig_client_config(ig_client_config).await?);
    } else if let (Ok(username), Ok(password)) = (username, password) {
        let client = IGClient::new();
        client.login(&username, &password).await?;

        return Ok(client);
    }

    panic!("Either IG_CLIENT_CONFIG or USERNAME and PASSWORD should be provided as env variables")
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = get_ig_client().await?;
    let ig_client_config = client.ig_client_config().await;
    let session_id = ig_client_config.get_cookie_value("sessionid").unwrap();
    let ig_client_config_str = serde_json::to_string(&ig_client_config)
        .expect("IG_CLIENT_CONFIG to deserialize");
    println!("sessionid={session_id}");
    println!("IG_CLIENT_CONFIG={ig_client_config_str}");
    Ok(())
}
