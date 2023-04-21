pub mod publish_packet_handler;
pub mod ping_res_packet_handler;

pub trait PacketHandler {
    fn handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>);
    fn can_handle(&self, packet: &Box<dyn std::any::Any + Send + Sync>) -> bool;
}
