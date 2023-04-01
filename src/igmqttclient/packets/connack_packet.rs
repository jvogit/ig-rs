use bytes::Bytes;

use super::ControlPacket;

pub struct ConnackPacket {
    pub session_present: bool,
    pub return_code: u8,
}

impl ConnackPacket {
    pub const PACKET_TYPE: u8 = 2u8;

    pub fn from_payload(payload: Bytes) -> Self {
        assert!(payload.len() == 2, "Payload needs length of 2 for ConnackPacket");
        Self {
            session_present: (payload[0] & 1u8) == 1u8,
            return_code: payload[1],
        }
    }
}

impl ControlPacket for ConnackPacket {
    fn packet_type(&self) -> u8 {
        ConnackPacket::PACKET_TYPE
    }

    fn flags(&self) -> u8 {
        0u8
    }

    fn payload(&self) -> Bytes {
        vec![self.session_present as u8, self.return_code].into()
    }
}
