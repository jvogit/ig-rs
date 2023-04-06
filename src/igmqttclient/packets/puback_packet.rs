use bytes::{BytesMut, BufMut};

use super::ControlPacket;

pub struct PubAckPacket {
    packet_identifier: u16,
}

impl PubAckPacket {
    pub const PACKET_TYPE: u8 = 4;
}

impl ControlPacket for PubAckPacket {
    fn packet_type(&self) -> u8 {
        PubAckPacket::PACKET_TYPE
    }

    fn flags(&self) -> u8 {
        0u8
    }

    fn payload(&self) -> bytes::Bytes {
        let mut writer = BytesMut::new();

        writer.put_u16(self.packet_identifier);

        writer.freeze()

    }
}
