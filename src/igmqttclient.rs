use std::{error::Error, net::SocketAddr, sync::Arc};

use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};
use tokio_rustls::{
    rustls::{self},
    TlsConnector,
};

use self::packets::ConnectPacket;

pub mod packets;

pub struct IGMQTTClient {
    config: TlsConnector,
}

impl IGMQTTClient {
    pub fn new() -> Self {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));
        let config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        IGMQTTClient {
            config: TlsConnector::from(Arc::new(config)),
        }
    }

    pub async fn connect(&self, session_id: &str) -> Result<(), Box<dyn Error + 'static>> {
        // TODO: Handle MQTToT
        // edge-mqtt.facebook.com:443
        let mut stream = TcpStream::connect("test.mosquitto.org:1883").await?;
        // TODO: TLS for MQTToT
        // let stream = self.config.connect("edge-mqtt.facebook.com".try_into().expect("Valid DNS name"), stream).await?;

        let connect_packet = ConnectPacket::new().as_bytes();

        println!("Connect packet: {:x}", connect_packet);

        stream.writable().await?;

        let n = stream.write(&connect_packet).await?;

        println!("Wrote {} bytes", n);

        stream.readable().await?;

        let res = stream.read_u8().await?;
        
        // Should be 20 (CONNACK)
        println!("Received byte: {:x}", res);

        Ok(())
    }
}

async fn read_variable_length_encoding(stream: &mut TcpStream) -> Result<u32, std::io::Error> {
    let mut multipler: u32 = 1;
    let mut value: u32 = 0;

    while {
        let encoded_byte: u8 = stream.read_u8().await?;
        value += Into::<u32>::into(encoded_byte & 127) * multipler;
        multipler *= 128;

        if multipler > 128*128*128 {
            // throw error
            panic!("Malformed length");
        }


        (encoded_byte & 128) != 0
    } {}

    Ok(value)
}
