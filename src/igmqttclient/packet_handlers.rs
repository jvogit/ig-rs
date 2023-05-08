use std::sync::Arc;

use super::IGLoggedInMQTTClient;

pub mod connack_packet_handler;
pub mod pingres_packet_handler;
pub mod publish_packet_handler;

pub struct Context<'a> {
    pub client: &'a Arc<IGLoggedInMQTTClient>,
}

pub trait PacketHandler {
    fn handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>, cx: &Context);
    fn can_handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>, cx: &Context) -> bool;
}
