use crate::igmqttclient::packets::pingres_packet::PingResPacket;

use super::PacketHandler;

pub struct PingResPacketHandler {
    pub handle: Box<dyn Fn(&PingResPacket) -> () + Send + Sync>,
    pub can_handle: Box<dyn Fn(&PingResPacket) -> bool + Send + Sync>,
}

impl PacketHandler for PingResPacketHandler {
    fn handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>) {
        assert!(
            self.can_handle(packet),
            "Expected can_handle for PingResPacketHandler to not fail"
        );

        if let Some(packet) = packet.downcast_ref::<PingResPacket>() {
            (self.handle)(packet)
        }
    }

    fn can_handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>) -> bool {
        if let Some(packet) = packet.downcast_ref::<PingResPacket>() {
            return (self.can_handle)(packet);
        }

        false
    }
}
