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
        1u8
    }

    fn flags(&self) -> u8 {
        0u8
    }

    fn payload(&self) -> Bytes {
        let mut writer = BytesMut::new().writer();
        let writer_mut = writer.get_mut();

        // Variable header 
        write_str(self.protocol_name, writer_mut);
        writer_mut.put_u8(self.protocol_level);
        writer_mut.put_u8(self.connect_flags);
        writer_mut.put_u16(self.keep_alive);

        // Client ID
        write_str(self.client_id, writer_mut);

        writer.into_inner().freeze()
    }
}
