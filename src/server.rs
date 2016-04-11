use packets;
use std::net;

//This module specifies the server trait.
//Actual servers are found elsewhere: either mio_server.rs or test_server.rs.

pub trait Server {
    //Send a packet.
    fn send(&mut self, packet: packets::Packet, ip: net::IpAddr);
    //Upgrade an ip address to a connection.
    fn make_connection(&mut self, ip: net::IpAddr)->Result<u32, String>;
}


/**Represents responders which are associated with connections.

The single method here should return true if the packet was handled, otherwise false.
The server tries all responders associated with a connection, then tries all responders not associated with any connection.
If a packet isn't handled by anything, it is dropped.*/
pub trait ConnectedPacketResponder {
    fn handle_incoming_packet<T: Server>(&mut self, packet: &packets::Packet, server: &mut T)->bool {
        false
    }
}


/**Represents responders which are not associated with connections.

The single method here should return true if the packet was handled, otherwise false.
The server tries all responders associated with a connection, then tries all responders not associated with any connection.
If a packet isn't handled by anything, it is dropped.*/
pub trait ConnectionlessPacketResponder {
    fn handle_incoming_packet_connectionless<T: Server>(&mut self, packet: &packets::Packet, ip: net::IpAddr, server: &mut T)->bool {
        false
    }
}
