use super::ControlPacket;

pub struct PingResPacket {}

impl PingResPacket {
    pub const PACKET_TYPE: u8 = 13u8;

    pub fn new() -> Self {
        Self {}
    }
}

impl ControlPacket for PingResPacket {
    fn packet_type(&self) -> u8 {
        PingResPacket::PACKET_TYPE
    }

    fn flags(&self) -> u8 {
        0u8
    }

    fn payload(&self) -> bytes::Bytes {
        bytes::Bytes::new()
    }
}
