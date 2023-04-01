use bytes::{BufMut, Bytes, BytesMut};

pub mod connack_packet;
pub mod connect_packet;
pub mod pingreq_packet;
pub mod pingres_packet;

/// MQTT 3.1.1 Spec Control Packet interface
pub trait ControlPacket {
    /// The control packet's type in the fixed header
    fn packet_type(&self) -> u8;

    /// The control packet's flags in the fixed header
    fn flags(&self) -> u8;

    /// The control packet's payload which may consist of: a variable header and/or custom payload
    fn payload(&self) -> Bytes;

    /// The control packet's MQTT 3.1.1 Spec Byte encoding
    fn as_bytes(&self) -> Bytes {
        let mut writer = BytesMut::new();
        let payload = self.payload();

        // Write Fixed Header portion
        writer.put_u8((self.packet_type() << 4) | self.flags());
        
        // Write remaining length which is the size of the payload
        write_variable_length_encoding(payload.len(), &mut writer);
        writer.put(payload);

        writer.freeze()
    }
}

/// MQTT 3.1.1 Spec string byte encoding
fn write_str(value: &str, writer: &mut BytesMut) {
    let length: u16 = value.len() as u16;
    writer.put_u16(length);
    writer.put_slice(value.as_bytes());
}

/// MQTT 3.1.1 Spec Variable Length byte encoding
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
