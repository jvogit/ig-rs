use crate::{
    igclient::IGClientConfig,
    igmqttclient::{
        bytes_mut_write_transport::BytesMutWriteTransport,
        payloads::connect_payload::{ClientInfo, ConnectPayload}, utils::write_str,
    },
};
use bytes::{BufMut, Bytes, BytesMut};
use miniz_oxide::deflate::compress_to_vec_zlib;
use std::time::{SystemTime, UNIX_EPOCH};
use thrift::protocol::{TCompactOutputProtocol, TSerializable};

use super::ControlPacket;

pub struct ConnectPacket<'a> {
    protocol_name: &'a str,
    protocol_level: u8,
    connect_flags: u8,
    keep_alive: u16,
    connect_payload: Bytes,
}

/// MQTT 3.1.1 Spec CONNECT packet
impl ConnectPacket<'_> {
    pub const PACKET_TYPE: u8 = 1u8;

    pub fn new(ig_client_config: &IGClientConfig) -> Self {
        // Construct thrift Connect Payload w/ ig client config details
        // Connect payload uses cookie authorization (sessionid cookie)
        let connect_payload = ConnectPayload::new(
            Some((&ig_client_config.device.device_id[..20]).into()),
            Some(Box::new(ClientInfo::new(
                Some(ig_client_config.pk),
                Some(ig_client_config.device.user_agent.clone()),
                Some(183),
                Some(0),
                Some(1),
                Some(false),
                Some(true),
                Some(ig_client_config.device.device_id.clone()),
                Some(true),
                Some(1),
                Some(0),
                Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as i64,
                ),
                Some(vec![88, 135, 149, 150, 133, 146]),
                Some("cookie_auth".into()),
                Some(567067343352427),
                Some("".into()),
                Some(3i8),
            ))),
            format!(
                "sessionid={}",
                ig_client_config
                    .get_cookie_value("sessionid")
                    .expect("sessionid cookie to be present")
            ),
            Some(
                [
                    ("app_version".into(), "148.0.0.33.121".into()),
                    ("X-IG-Capabilities".into(), ig_client_config.device.capabilities.clone()),
                    ("everclear_subscriptions".into(), "{\"inapp_notification_subscribe_comment\":\"17899377895239777\",\"inapp_notification_subscribe_comment_mention_and_reply\":\"17899377895239777\",\"video_call_participant_state_delivery\":\"17977239895057311\",\"presence_subscribe\":\"17846944882223835\"}".into()),
                    ("User-Agent".into(), ig_client_config.device.user_agent.clone()),
                    ("Accept-Language".into(), "en-US".into()),
                    ("platform".into(), "android".into()),
                    ("ig_mqtt_route".into(), "django".into()),
                    ("pubsub_msg_type_blacklist".into(), "direct, typing_type".into()),
                    ("auth_cache_enabled".into(), "0".into()),
                ].iter().cloned().collect()
            ),
        );
        let mut write_transport = BytesMutWriteTransport::new();

        // using thrift compact protocol to write the connect payload to Bytes object
        connect_payload
            .write_to_out_protocol(&mut TCompactOutputProtocol::new(&mut write_transport))
            .expect("Connect payload to successfuly write to transport");

        let connect_payload = write_transport.into_bytes();
        // zip the connect payload
        let connect_payload = compress_to_vec_zlib(&connect_payload, 9);

        ConnectPacket {
            protocol_name: "MQTToT",
            protocol_level: 3,
            // CONNECT FLAGS: 11000010
            connect_flags: 194,
            keep_alive: 20,
            // In "MQTToT" the payload is a zipped thrift payload
            connect_payload: connect_payload.into(),
        }
    }
}

impl ControlPacket for ConnectPacket<'_> {
    fn packet_type(&self) -> u8 {
        ConnectPacket::PACKET_TYPE
    }

    fn flags(&self) -> u8 {
        0u8
    }

    fn payload(&self) -> Bytes {
        let mut writer = BytesMut::new();

        // Write connect packet variable header
        write_str(self.protocol_name, &mut writer);
        writer.put_u8(self.protocol_level);
        writer.put_u8(self.connect_flags);
        writer.put_u16(self.keep_alive);

        // Write connect packet payload
        writer.put(self.connect_payload.clone());

        writer.freeze()
    }
}
