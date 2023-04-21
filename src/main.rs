use ig_rs::igclient::{
    igrequests::direct_v2_inbox::DirectV2InboxRequest, IGClient, IGClientConfig,
};
use std::env;

#[derive(Debug)]
enum IGCLIErr {
    IGClientErr(ig_rs::igclient::IGClientErr),
    #[cfg(feature = "realtime")]
    IGMQTTClientErr(ig_rs::igmqttclient::IGMQTTClientErr),
}

impl From<ig_rs::igclient::IGClientErr> for IGCLIErr {
    fn from(value: ig_rs::igclient::IGClientErr) -> Self {
        IGCLIErr::IGClientErr(value)
    }
}

#[cfg(feature = "realtime")]
impl From<ig_rs::igmqttclient::IGMQTTClientErr> for IGCLIErr {
    fn from(value: ig_rs::igmqttclient::IGMQTTClientErr) -> Self {
        IGCLIErr::IGMQTTClientErr(value)
    }
}

#[tokio::main]
async fn main() -> Result<(), IGCLIErr> {
    let client = get_ig_client().await?;
    let ig_client_config = client.ig_client_config().await;
    let session_id = ig_client_config.get_cookie_value("sessionid").unwrap();
    let ig_client_config_str =
        serde_json::to_string(&ig_client_config).expect("IG_CLIENT_CONFIG to deserialize");

    let direct_inbox_res = client.get(&DirectV2InboxRequest::new()).await?;

    println!("direct_v2_inbox/: {:#?}", direct_inbox_res);

    println!("sessionid={session_id}");
    println!("IG_CLIENT_CONFIG={ig_client_config_str}");

    #[cfg(feature = "realtime")]
    realtime().await?;

    Ok(())
}

async fn get_ig_client() -> Result<IGClient, IGCLIErr> {
    let ig_client_config = env::var("IG_CLIENT_CONFIG");
    let username = env::var("USERNAME");
    let password = env::var("PASSWORD");

    if let Ok(ig_client_config) = ig_client_config {
        let ig_client_config = serde_json::from_str::<IGClientConfig>(&ig_client_config)
            .expect("IG_CLIENT_CONFIG should be valid JSON!");

        return Ok(IGClient::from_ig_client_config(ig_client_config).await?);
    } else if let (Ok(username), Ok(password)) = (username, password) {
        let client = IGClient::new();
        client.login(&username, &password).await?;

        return Ok(client);
    }

    panic!("Either IG_CLIENT_CONFIG or USERNAME and PASSWORD should be provided as env variables")
}

#[cfg(feature = "realtime")]
async fn realtime() -> Result<(), IGCLIErr> {
    use ig_rs::igmqttclient::{
        packet_handlers::ping_res_packet_handler::PingResPacketHandler, IGMQTTClient,
    };

    let mut mqtt_client = IGMQTTClient::new();
    let ig_client_config =
        serde_json::from_str::<IGClientConfig>(&env::var("IG_CLIENT_CONFIG").unwrap()[..]).unwrap();
    mqtt_client.register_handler(Box::new(PingResPacketHandler {
        handle: Box::new(|x| { println!("Received PingResPacket from handler!") }),
        can_handle: Box::new(|_| true),
    }));

    mqtt_client.connect(ig_client_config).await?;

    Ok(())
}
