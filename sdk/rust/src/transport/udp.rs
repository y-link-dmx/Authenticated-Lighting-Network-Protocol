use std::net::{SocketAddr, UdpSocket as StdUdpSocket};

use alpine_protocol_rs::stream::FrameTransport;

/// UDP-based transport used by the SDK streaming client.
#[derive(Debug)]
pub struct UdpFrameTransport {
    socket: StdUdpSocket,
    _peer: SocketAddr,
}

impl UdpFrameTransport {
    pub fn new(local: SocketAddr, peer: SocketAddr) -> Result<Self, std::io::Error> {
        let socket = StdUdpSocket::bind(local)?;
        socket.connect(peer)?;
        Ok(Self { socket, _peer: peer })
    }
}

impl FrameTransport for UdpFrameTransport {
    fn send_frame(&self, bytes: &[u8]) -> Result<(), String> {
        self.socket
            .send(bytes)
            .map_err(|e| format!("udp stream send: {}", e))?;
        Ok(())
    }
}
