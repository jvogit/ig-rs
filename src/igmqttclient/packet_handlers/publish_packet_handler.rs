use crate::igmqttclient::packets::publish_packet::PublishPacket;

use super::PacketHandler;

pub struct PublishPacketHandler {
    handle: Box<dyn Fn(&PublishPacket) -> ()>,
    can_handle: Box<dyn Fn(&PublishPacket) -> bool>,
}

impl PublishPacketHandler {
    fn on(condition: impl Fn(&PublishPacket) -> bool + 'static) -> PublishPacketHandlerBuilder {
        PublishPacketHandlerBuilder {
            handle: None,
            can_handle: Box::new(condition),
        }
    }

    fn handle(handler: impl Fn(&PublishPacket) -> () + 'static) -> PublishPacketHandler {
        PublishPacketHandler {
            handle: Box::new(handler),
            can_handle: Box::new(|_| true),
        }
    }
}

impl PacketHandler for PublishPacketHandler {
    fn handle(&self, packet: &Box<dyn std::any::Any>) {
        assert!(
            self.can_handle(packet),
            "Expected can_handle for PublishPacketHandler to not fail"
        );

        if let Some(publish_packet) = packet.downcast_ref::<PublishPacket>() {
            (self.handle)(publish_packet)
        }
    }

    fn can_handle(&self, packet: &Box<dyn std::any::Any>) -> bool {
        if let Some(publish_packet) = packet.downcast_ref::<PublishPacket>() {
            return (self.can_handle)(publish_packet);
        }

        false
    }
}

struct PublishPacketHandlerBuilder {
    handle: Option<Box<dyn Fn(&PublishPacket) -> ()>>,
    can_handle: Box<dyn Fn(&PublishPacket) -> bool>,
}

impl PublishPacketHandlerBuilder {
    fn on(self, condition: impl Fn(&PublishPacket) -> bool + 'static) -> Self {
        Self {
            handle: self.handle,
            // Conjunctive normal form of "on" conditions
            can_handle: Box::new(move |p| (self.can_handle)(p) && (condition)(p)),
        }
    }

    fn handle(self, handler: impl Fn(&PublishPacket) -> () + 'static) -> PublishPacketHandler {
        PublishPacketHandler {
            handle: Box::new(handler),
            can_handle: self.can_handle,
        }
    }
}
