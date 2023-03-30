use bytes::BytesMut;
use packets::{connack_packet::ConnackPacket, connect_packet::ConnectPacket, ControlPacket};
use std::{error::Error, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};
use tokio_rustls::{
    client::TlsStream,
    rustls::{self},
    TlsConnector,
};

mod packets;
mod payloads;

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
        // TODO: edge-mqtt.facebook.com:443
        let stream = TcpStream::connect("broker.hivemq.com:8883").await?;
        let stream = self
            .config
            .connect("broker.hivemq.com".try_into().unwrap(), stream)
            .await?;
        let logged_in_client = IGLoggedInMQTTClient {
            stream: Arc::new(Mutex::new(stream)),
        };
        let connect_packet = ConnectPacket::new();

        println!("Connect packet: {:x}", connect_packet.as_bytes());

        logged_in_client.send_packet(&connect_packet).await?;
        if let Some(connack_packet) = logged_in_client
            .read_packet()
            .await?
            .downcast_ref::<ConnackPacket>()
        {
            println!(
                "Received connack return code {}",
                connack_packet.return_code
            );
        }

        Ok(())
    }
}

pub struct IGLoggedInMQTTClient {
    stream: Arc<Mutex<TlsStream<TcpStream>>>,
}

impl IGLoggedInMQTTClient {
    async fn send_packet(&self, packet: &dyn ControlPacket) -> Result<(), std::io::Error> {
        self.stream
            .lock()
            .await
            .write_all(&packet.as_bytes())
            .await?;

        Ok(())
    }

    async fn read_packet(&self) -> Result<Box<dyn std::any::Any>, std::io::Error> {
        let mut stream = self.stream.lock().await;
        let packet_fixed_header = stream.read_u8().await?;
        let control_packet_type = packet_fixed_header >> 4;
        let remaining_length = read_variable_length_encoding(&mut *stream).await?;
        let mut bytes = BytesMut::with_capacity(remaining_length);
        stream.read_buf(&mut bytes).await?;
        let bytes = bytes.freeze();

        println!("Received {:x}{:x}", packet_fixed_header, bytes);

        return match control_packet_type {
            ConnackPacket::PACKET_TYPE => Ok(Box::new(ConnackPacket::from(bytes))),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Expected valid control packet type instead of {:x}",
                    control_packet_type
                ),
            )),
        };
    }
}

async fn read_variable_length_encoding(
    stream: &mut TlsStream<TcpStream>,
) -> Result<usize, std::io::Error> {
    let mut multipler: usize = 1;
    let mut value: usize = 0;

    while {
        let encoded_byte: u8 = stream.read_u8().await?;
        value += Into::<usize>::into(encoded_byte & 127) * multipler;
        multipler *= 128;

        if multipler > 128 * 128 * 128 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Expected nonmalformed variable length encoding!",
            ));
        }

        (encoded_byte & 128) != 0
    } {}

    Ok(value)
}
