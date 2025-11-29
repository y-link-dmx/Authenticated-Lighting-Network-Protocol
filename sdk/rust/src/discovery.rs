use std::{
    fmt,
    io,
    net::{SocketAddr, UdpSocket},
    time::Duration,
};

use alpine_protocol_rs::messages::{DiscoveryReply, DiscoveryRequest};
use rand::{rngs::OsRng, RngCore};
use serde_cbor;

/// Options used to configure the blocking discovery helper.
pub struct DiscoveryClientOptions {
    pub remote_addr: SocketAddr,
    pub local_addr: SocketAddr,
    pub timeout: Duration,
}

impl DiscoveryClientOptions {
    /// Creates options with the provided remote socket and a default timeout.
    pub fn new(remote_addr: SocketAddr, local_addr: SocketAddr, timeout: Duration) -> Self {
        Self {
            remote_addr,
            local_addr,
            timeout,
        }
    }
}

/// Errors that can happen while sending or receiving discovery payloads.
#[derive(Debug)]
pub enum DiscoveryError {
    Io(io::Error),
    Decode(serde_cbor::Error),
    Timeout,
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiscoveryError::Io(err) => write!(f, "io error: {}", err),
            DiscoveryError::Decode(err) => write!(f, "cbors serialization error: {}", err),
            DiscoveryError::Timeout => write!(f, "discovery timed out"),
        }
    }
}

impl std::error::Error for DiscoveryError {}

impl From<io::Error> for DiscoveryError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock => DiscoveryError::Timeout,
            _ => DiscoveryError::Io(err),
        }
    }
}

impl From<serde_cbor::Error> for DiscoveryError {
    fn from(err: serde_cbor::Error) -> Self {
        DiscoveryError::Decode(err)
    }
}

/// The outcome of a discovery request.
pub struct DiscoveryOutcome {
    pub reply: DiscoveryReply,
    pub peer: SocketAddr,
}

/// Stateless discovery helper that wraps the protocol request/response models.
pub struct DiscoveryClient {
    socket: UdpSocket,
    remote_addr: SocketAddr,
}

impl DiscoveryClient {
    /// Creates a client that will send discovery packets to `remote_addr`.
    pub fn new(options: DiscoveryClientOptions) -> Result<Self, DiscoveryError> {
        let socket = UdpSocket::bind(options.local_addr)?;
        socket.set_read_timeout(Some(options.timeout))?;
        Ok(Self {
            socket,
            remote_addr: options.remote_addr,
        })
    }

    /// Sends a discovery payload with the requested capability names and waits for a reply.
    pub fn discover(&self, requested: &[String]) -> Result<DiscoveryOutcome, DiscoveryError> {
        let mut nonce = vec![0u8; 32];
        OsRng.fill_bytes(&mut nonce);
        let request = DiscoveryRequest::new(requested.to_vec(), nonce.clone());
        let payload = serde_cbor::to_vec(&request)?;
        self.socket.send_to(&payload, self.remote_addr)?;

        let mut buf = vec![0u8; 2048];
        let (len, peer) = self.socket.recv_from(&mut buf)?;
        let reply: DiscoveryReply = serde_cbor::from_slice(&buf[..len])?;
        Ok(DiscoveryOutcome { reply, peer })
    }
}
