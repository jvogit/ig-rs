use bytes::{BufMut, Bytes, BytesMut};

pub mod connect_packet;

pub trait ControlPacket {
    fn packet_type(&self) -> u8;

    fn flags(&self) -> u8;

    fn payload(&self) -> Bytes;

    fn as_bytes(&self) -> Bytes {
        let mut writer = BytesMut::new().writer();
        let writer_mut = writer.get_mut();
        let payload = self.payload();

        // Fixed Header
        writer_mut.put_u8((self.packet_type() << 4) | self.flags());
        // Remaining length which is the size of the payload
        write_variable_length_encoding(payload.len(), writer_mut);
        writer_mut.put(payload);

        writer.into_inner().freeze()
    }
}

fn write_str(value: &str, writer: &mut BytesMut) {
    let length: u16 = value.len() as u16;
    writer.put_u16(length);
    writer.put_slice(value.as_bytes());
}

fn write_variable_length_encoding(value: usize, writer: &mut BytesMut) {
    assert!(
        value <= 268_435_455,
        "value exceeds maximum allowed: 268435455"
    );

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
