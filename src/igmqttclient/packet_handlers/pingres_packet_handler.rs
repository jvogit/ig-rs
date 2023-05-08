use crate::igmqttclient::packets::pingres_packet::PingResPacket;

use super::{Context, PacketHandler};

pub struct PingResPacketHandler {
    pub handle: Box<dyn Fn(&PingResPacket, &Context) -> () + Send + Sync>,
    pub can_handle: Box<dyn Fn(&PingResPacket, &Context) -> bool + Send + Sync>,
}

impl PacketHandler for PingResPacketHandler {
    fn handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>, cx: &Context) {
        assert!(
            self.can_handle(packet, cx),
            "Expected can_handle for PingResPacketHandler to not fail"
        );

        if let Some(packet) = packet.downcast_ref::<PingResPacket>() {
            (self.handle)(packet, cx)
        }
    }

    fn can_handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>, cx: &Context) -> bool {
        if let Some(packet) = packet.downcast_ref::<PingResPacket>() {
            return (self.can_handle)(packet, cx);
        }

        false
    }
}
