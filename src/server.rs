use packets;
use handlers;
use mio;

//This module specifies the server trait.
//Actual servers are found elsewhere: either mio_server.rs or test_server.rs.
pub trait Server {
    //Send a packet.
    fn send(&mut self, packet: &packets::Packet, ip: &mio::IpAddr);
    //Upgrade an ip address to a connection.
    fn make_connection(&mut self, ip: &mio::IpAddr);
}
