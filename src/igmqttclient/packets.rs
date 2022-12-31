use bytes::{BufMut, Bytes, BytesMut};

pub struct ConnectPacket {

}

impl ConnectPacket {
    const CONTROL_PACKET_TYPE: u8 = 1;
    const FLAGS: u8 = 0;
    const PROTOCOL_NAME: &str = "MQTT";
    const PROTOCOL_LEVEL: u8 = 4;
    const CONNECT_FLAGS: u8 = 2;
    const KEEP_ALIVE: u16 = 20;

    pub fn new() -> Self {
        ConnectPacket {  }
    }

    pub fn as_bytes(&self) -> Bytes {
        let mut writer = BytesMut::new().writer();
        let writer_mut = writer.get_mut();
        
        // TODO: Do not hardcode
        // Fixed Header
        writer_mut.put_u8((ConnectPacket::CONTROL_PACKET_TYPE << 4) | ConnectPacket::FLAGS);
        write_variable_length_encoding(19, writer_mut);

        // Variable header (19 bytes in length)
        write_str(ConnectPacket::PROTOCOL_NAME, writer_mut);
        writer_mut.put_u8(ConnectPacket::PROTOCOL_LEVEL);
        writer_mut.put_u8(ConnectPacket::CONNECT_FLAGS);
        writer_mut.put_u16(ConnectPacket::KEEP_ALIVE);

        // Client ID
        write_str("fjkemjj", writer_mut);

        writer.into_inner().freeze()
    }
}

fn write_str(value: &str, writer: &mut BytesMut) {
    let length: u16 = value.len() as u16;
    writer.put_u16(length);
    writer.put_slice(value.as_bytes());
}

fn write_variable_length_encoding(value: u32, writer: &mut BytesMut) {
    assert!(value <= 268_435_455, "value exceeds maximum allowed: 268435455");

    let mut x = value;
    while {
        let mut encoded_byte: u8 = (x % 128) as u8;
        x /= 128;

        if x > 0 {
            encoded_byte |= 128;
        }

        writer.put_u8(encoded_byte);

        x > 0
    } {}
}
