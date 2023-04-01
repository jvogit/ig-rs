use bytes::BytesMut;
use packets::{connack_packet::ConnackPacket, connect_packet::ConnectPacket, ControlPacket};
use std::{sync::Arc, fs::read};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::Mutex,
};
use tokio_rustls::{
    client::TlsStream,
    rustls::{self},
    TlsConnector,
};

use crate::{
    igclient::IGClientConfig,
    igmqttclient::packets::{pingreq_packet::PingReqPacket, pingres_packet::PingResPacket},
};

mod bytes_mut_write_transport;
mod packets;
mod payloads;

#[derive(Debug)]
pub enum IGMQTTClientErr {
    IOErr(std::io::Error),
    ConnackErr(u8),
    UnknownPacketType(u8),
}

impl From<std::io::Error> for IGMQTTClientErr {
    fn from(value: std::io::Error) -> Self {
        IGMQTTClientErr::IOErr(value)
    }
}

pub type Result<T> = std::result::Result<T, IGMQTTClientErr>;

/// IGMQTTClient
pub struct IGMQTTClient {
    config: TlsConnector,
}

impl IGMQTTClient {
    /// Construct a new IGMQTTClient
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

    /// Connects the client to the broker
    pub async fn connect(&self, ig_client_config: IGClientConfig) -> Result<()> {
        // TODO: edge-mqtt.facebook.com:443
        let stream = TcpStream::connect("edge-mqtt.facebook.com:443").await?;
        let (reader_stream, writer_stream) = tokio::io::split(self
            .config
            .connect("edge-mqtt.facebook.com".try_into().unwrap(), stream)
            .await?);

        let logged_in_client = IGLoggedInMQTTClient {
            reader_stream: Arc::new(Mutex::new(reader_stream)),
            writer_stream: Arc::new(Mutex::new(writer_stream)),
            ig_client_config,
        };
        let connect_packet = Box::new(ConnectPacket::new(&logged_in_client.ig_client_config));

        println!("Connect packet: {:x}", connect_packet.as_bytes());

        logged_in_client.send_packet(connect_packet).await?;

        let response_packet = logged_in_client.read_packet().await?;
        if let Some(connack_packet) = response_packet.downcast_ref::<ConnackPacket>() {
            if connack_packet.return_code != 0 {
                return Err(IGMQTTClientErr::ConnackErr(connack_packet.return_code));
            }

            return logged_in_client.connect().await;
        } else {
            panic!(
                "Expected ConnackPacket but received: {:?}",
                response_packet.type_id()
            );
        }
    }
}

pub struct IGLoggedInMQTTClient {
    reader_stream: Arc<Mutex<ReadHalf<TlsStream<TcpStream>>>>,
    writer_stream: Arc<Mutex<WriteHalf<TlsStream<TcpStream>>>>,
    ig_client_config: IGClientConfig,
}

impl IGLoggedInMQTTClient {
    async fn connect(self) -> Result<()> {
        let client = Arc::new(self);
        let ping_client = client.clone();
        let ping_task_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));

            loop {
                interval.tick().await;
                match ping_client
                    .send_packet(Box::new(PingReqPacket::new()))
                    .await
                {
                    Err(err) => println!("Error occured during ping req task: {:?}", err),
                    _ => {}
                }
            }
        });

        loop {
            match client.read_packet().await {
                Ok(response_packet) => {
                    println!("Received packet during read packet task: {:?}", response_packet)
                }
                Err(err) => println!("Error occured during read packet task: {:?}", err),
            }
        }

        ping_task_handle.abort();

        Ok(())
    }

    /// Sends a ControlPacket
    async fn send_packet(&self, packet: Box<dyn ControlPacket + Send + Sync>) -> Result<()> {
        self.writer_stream
            .lock()
            .await
            .write_all(&packet.as_bytes())
            .await?;

        Ok(())
    }

    /// Reads a ControlPacket
    async fn read_packet(&self) -> Result<Box<dyn std::any::Any + Send + Sync>> {
        let mut stream = self.reader_stream.lock().await;
        let packet_fixed_header = stream.read_u8().await?;
        let control_packet_type = packet_fixed_header >> 4;
        let remaining_length = read_variable_length_encoding(&mut stream).await?;
        let mut bytes = BytesMut::with_capacity(remaining_length);

        stream.read_buf(&mut bytes).await?;

        let bytes = bytes.freeze();

        println!("Received {:x}{:x}", packet_fixed_header, bytes);

        return match control_packet_type {
            ConnackPacket::PACKET_TYPE => Ok(Box::new(ConnackPacket::from_payload(bytes))),
            PingResPacket::PACKET_TYPE => Ok(Box::new(PingResPacket::new())),
            _ => Err(IGMQTTClientErr::UnknownPacketType(control_packet_type)),
        };
    }
}

async fn read_variable_length_encoding(stream: &mut ReadHalf<TlsStream<TcpStream>>) -> Result<usize> {
    let mut multipler: usize = 1;
    let mut value: usize = 0;

    while {
        let encoded_byte: u8 = stream.read_u8().await?;
        value += Into::<usize>::into(encoded_byte & 127) * multipler;
        multipler *= 128;

        if multipler > 128 * 128 * 128 {
            panic!("Expected nonmalformed variable length encoding!");
        }

        (encoded_byte & 128) != 0
    } {}

    Ok(value)
}
