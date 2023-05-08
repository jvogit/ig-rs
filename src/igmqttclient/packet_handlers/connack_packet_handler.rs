use crate::igmqttclient::packets::connack_packet::ConnackPacket;

use super::{Context, PacketHandler};

pub struct ConnackPacketHandler {
    pub handle: Box<dyn Fn(&ConnackPacket, &Context) -> () + Send + Sync>,
}

impl ConnackPacketHandler {
    pub fn handle(handle: impl Fn(&ConnackPacket, &Context) + 'static + Send + Sync) -> Self {
        Self {
            handle: Box::new(handle),
        }
    }
}

impl PacketHandler for ConnackPacketHandler {
    fn handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>, cx: &Context) {
        assert!(
            self.can_handle(packet, cx),
            "Expected can_handle to not fail"
        );

        if let Some(connack_packet) = packet.downcast_ref::<ConnackPacket>() {
            (self.handle)(connack_packet, cx)
        }
    }

    fn can_handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>, _cx: &Context) -> bool {
        if let Some(_) = packet.downcast_ref::<ConnackPacket>() {
            return true;
        }

        false
    }
}
