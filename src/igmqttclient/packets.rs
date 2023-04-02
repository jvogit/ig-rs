use bytes::{BufMut, Bytes, BytesMut};

use super::utils::write_variable_length_encoding;

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
