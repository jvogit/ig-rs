use crate::igmqttclient::packets::publish_packet::PublishPacket;

use super::{PacketHandler, Context};

pub struct PublishPacketHandler {
    pub handle: Box<dyn Fn(&PublishPacket, &Context) -> () + Send + Sync>,
    pub can_handle: Box<dyn Fn(&PublishPacket, &Context) -> bool + Send + Sync>,
}

impl PublishPacketHandler {
    pub fn on(condition: impl Fn(&PublishPacket, &Context) -> bool + Send + Sync + 'static) -> PublishPacketHandlerBuilder {
        PublishPacketHandlerBuilder {
            handle: None,
            can_handle: Box::new(condition),
        }
    }

    pub fn handle(handler: impl Fn(&PublishPacket, &Context) -> () + Send + Sync + 'static) -> PublishPacketHandler {
        PublishPacketHandler {
            handle: Box::new(handler),
            can_handle: Box::new(|_, _| true),
        }
    }
}

impl PacketHandler for PublishPacketHandler {
    fn handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>, cx: &Context) {
        assert!(
            self.can_handle(packet, cx),
            "Expected can_handle for PublishPacketHandler to not fail"
        );

        if let Some(publish_packet) = packet.downcast_ref::<PublishPacket>() {
            (self.handle)(publish_packet, cx)
        }
    }

    fn can_handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>, cx: &Context) -> bool {
        if let Some(publish_packet) = packet.downcast_ref::<PublishPacket>() {
            return (self.can_handle)(publish_packet, cx);
        }

        false
    }
}

pub struct PublishPacketHandlerBuilder {
    handle: Option<Box<dyn Fn(&PublishPacket, &Context) -> () + Send + Sync>>,
    can_handle: Box<dyn Fn(&PublishPacket, &Context) -> bool + Send + Sync>,
}

impl PublishPacketHandlerBuilder {
    pub fn on(self, condition: impl Fn(&PublishPacket, &Context) -> bool + Send + Sync + 'static) -> Self {
        Self {
            handle: self.handle,
            // Conjunctive normal form of "on" conditions
            can_handle: Box::new(move |p, c| (self.can_handle)(p, c) && (condition)(p, c)),
        }
    }

    pub fn handle(self, handler: impl Fn(&PublishPacket, &Context) -> () + Send + Sync + 'static) -> PublishPacketHandler {
        PublishPacketHandler {
            handle: Box::new(handler),
            can_handle: self.can_handle,
        }
    }
}
