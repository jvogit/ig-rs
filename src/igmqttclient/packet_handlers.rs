pub mod publish_packet_handler;

pub trait PacketHandler {
    fn handle(&self, packet: &Box<dyn std::any::Any>);
    fn can_handle(&self, packet: &Box<dyn std::any::Any>) -> bool;
}
