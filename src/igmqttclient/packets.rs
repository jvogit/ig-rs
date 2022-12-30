use bytes::BufMut;

pub struct ConnectPacket {

}

impl ConnectPacket {
    const CONTROL_PACKET_TYPE: u8 = 1;
    const FLAGS: u8 = 0;
}

pub fn write_variable_length_encoding(value: u32, buffer: &mut dyn BufMut) {
    assert!(value <= 268_435_455, "value exceeds maximum allowed: 268435455");

    let mut x = value;
    while {
        let mut encoded_byte: u8 = (x % 128).try_into().unwrap();
        x /= 128;

        if x > 0 {
            encoded_byte |= 128;
        }

        buffer.put_u8(encoded_byte);

        x > 0
    } {}
}
