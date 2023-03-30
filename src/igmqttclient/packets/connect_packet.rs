use bytes::{BufMut, Bytes, BytesMut};

use super::{write_str, ControlPacket};

pub struct ConnectPacket<'a> {
    protocol_name: &'a str,
    protocol_level: u8,
    connect_flags: u8,
    keep_alive: u16,
    connect_payload: Bytes,
}

impl ConnectPacket<'_> {
    pub const PACKET_TYPE: u8 = 1u8;

    pub fn new() -> Self {
        let mut writer = BytesMut::new();

        write_str("jdhhkhjke", &mut writer);

        ConnectPacket {
            protocol_name: "MQTToT",
            protocol_level: 3,
            // CONNECT FLAGS: 11000010
            connect_flags: 194,
            keep_alive: 20,
            connect_payload: writer.freeze(),
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

        // Write "connect payload"
        // For standard MQTT connect packet this is just the client_id
        // For MQTToT connect packet, it is zipped thrift connect_payload
        writer.put(self.connect_payload.clone());

        writer.freeze()
    }
}
