use super::ControlPacket;

/// MQTT 3.1.1 Spec PINGREQ Packet
pub struct PingReqPacket {}

impl PingReqPacket {
    pub const PACKET_TYPE: u8 = 12u8;

    pub fn new() -> Self {
        Self {}
    }
}

impl ControlPacket for PingReqPacket {
    fn packet_type(&self) -> u8 {
        PingReqPacket::PACKET_TYPE
    }

    fn flags(&self) -> u8 {
        0u8
    }

    fn payload(&self) -> bytes::Bytes {
        bytes::Bytes::new()
    }
}
