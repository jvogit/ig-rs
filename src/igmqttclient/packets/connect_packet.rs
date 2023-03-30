use bytes::{BufMut, Bytes, BytesMut};
use thrift::protocol::{TSerializable, TBinaryOutputProtocol};
use crate::igmqttclient::{bytes_mut_channel::BytesMutChannel, payloads::connect_payload::ConnectPayload};

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
        let mut bytes_mut_channel = BytesMutChannel::new();
        let mut out_protocol = TBinaryOutputProtocol::new(&mut bytes_mut_channel, false);
        let connect_payload = ConnectPayload::new(None, None, None, None);

        connect_payload.write_to_out_protocol(&mut out_protocol).expect("Connect payload to successfuly write");

        ConnectPacket {
            protocol_name: "MQTToT",
            protocol_level: 3,
            // CONNECT FLAGS: 11000010
            connect_flags: 194,
            keep_alive: 20,
            connect_payload: bytes_mut_channel.into_bytes(),
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
