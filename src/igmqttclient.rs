use std::{error::Error, sync::Arc, io::Write};

use bytes::{BytesMut, BufMut, Bytes};
use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}, sync::{Mutex, MutexGuard}};
use tokio_rustls::{
    rustls::{self},
    TlsConnector,
    client::TlsStream,
};

use self::packets::{connect_packet::ConnectPacket, ControlPacket};

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
        let stream = TcpStream::connect("broker.hivemq.com:8883").await?;
        // TODO: TLS for MQTToT
        let mut stream = self.config.connect("broker.hivemq.com".try_into().expect("Valid DNS name"), stream).await?;

        let logged_in_client = IGLoggedInMQTTClient {
            stream: Arc::new(Mutex::new(stream)),
        };

        let connect_packet = ConnectPacket::new();
        println!("Connect packet: {:x}", connect_packet.as_bytes());
        logged_in_client.send_packet(&connect_packet).await?;

        logged_in_client.read_packet().await?;

        Ok(())
    }
}

pub struct IGLoggedInMQTTClient {
    stream: Arc<Mutex<TlsStream<TcpStream>>>,
}

impl IGLoggedInMQTTClient {
    async fn send_packet(&self, packet: &dyn ControlPacket) -> Result<(), std::io::Error> {
        self.stream.lock().await.write_all(&packet.as_bytes()).await?;

        Ok(())
    }

    async fn read_packet(&self) -> Result<Bytes, std::io::Error>{
        let mut stream = self.stream.lock().await;
        let packet_fixed_header = stream.read_u8().await?;
        let remaining_length = read_variable_length_encoding(&mut *stream).await?;
        let mut bytes = BytesMut::with_capacity(remaining_length);
        stream.read_buf(&mut bytes).await?;

        println!("Received {:x}{:x}", packet_fixed_header, bytes);

        Ok(bytes.freeze())
    }
}

async fn read_variable_length_encoding(stream: &mut TlsStream<TcpStream>) -> Result<usize, std::io::Error> {
    let mut multipler: usize = 1;
    let mut value: usize = 0;

    while {
        let encoded_byte: u8 = stream.read_u8().await?;
        value += Into::<usize>::into(encoded_byte & 127) * multipler;
        multipler *= 128;

        if multipler > 128*128*128 {
            // throw error
            panic!("Malformed length");
        }


        (encoded_byte & 128) != 0
    } {}

    Ok(value)
}
