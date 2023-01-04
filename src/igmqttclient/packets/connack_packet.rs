use bytes::Bytes;

use super::ControlPacket;

pub struct ConnackPacket {
    pub session_present: bool,
    pub return_code: u8,
}

impl ConnackPacket {
    pub const PACKET_TYPE: u8 = 2u8;
}

impl ControlPacket for ConnackPacket {
    fn packet_type(&self) -> u8 {
        ConnackPacket::PACKET_TYPE
    }

    fn flags(&self) -> u8 {
        0u8
    }

    fn payload(&self) -> bytes::Bytes {
        vec![self.session_present as u8, self.return_code].into()
    }
}

impl From<Bytes> for ConnackPacket {
    fn from(value: Bytes) -> Self {
        assert!(value.len() == 2, "Bytes need length of 2 for ConnackPacket");
        ConnackPacket {
            session_present: (value[0] & 1u8) == 1u8,
            return_code: value[1],
        }
    }
}
