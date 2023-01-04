use bytes::{BufMut, Bytes, BytesMut};

use super::{write_str, ControlPacket};

pub struct ConnectPacket<'a> {
    protocol_name: &'a str,
    protocol_level: u8,
    connect_flags: u8,
    keep_alive: u16,
    client_id: &'a str,
}

impl ConnectPacket<'_> {
    pub const PACKET_TYPE: u8 = 1u8;

    pub fn new() -> Self {
        ConnectPacket {
            protocol_name: "MQTT",
            protocol_level: 4,
            connect_flags: 2,
            keep_alive: 20,
            client_id: "hkdjhjkej",
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

        // Variable header
        write_str(self.protocol_name, &mut writer);
        writer.put_u8(self.protocol_level);
        writer.put_u8(self.connect_flags);
        writer.put_u16(self.keep_alive);

        // Client ID
        write_str(self.client_id, &mut writer);

        writer.freeze()
    }
}
