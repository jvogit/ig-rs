use bytes::{BufMut, Bytes, BytesMut};

use crate::igmqttclient::utils::write_str;

use super::ControlPacket;

pub struct PublishPacket<'a> {
    dup: bool,
    qos: u8,
    retain: bool,
    topic_name: &'a str,
    packet_identifier: Option<u16>,
    payload: Bytes,
}

impl PublishPacket<'_> {
    pub const PACKET_TYPE: u8 = 3;
}

impl ControlPacket for PublishPacket<'_> {
    fn packet_type(&self) -> u8 {
        PublishPacket::PACKET_TYPE
    }

    fn flags(&self) -> u8 {
        assert!(self.qos < 3, "QoS must be 0, 1, or 2");
        let mut flags: u8 = 0;

        flags |= (self.dup as u8) << 3;
        flags |= self.qos << 1;
        flags |= self.retain as u8;

        flags
    }

    fn payload(&self) -> Bytes {
        let mut writer = BytesMut::new();

        // Variable header
        write_str(self.topic_name, &mut writer);

        if self.qos == 1 || self.qos == 2 {
            writer.put_u16(
                self.packet_identifier
                    .expect("Packet Identifier to be present when QoS = 1 or QoS = 2"),
            );
        }

        // Payload
        writer.put(self.payload.clone());

        writer.freeze()
    }
}
