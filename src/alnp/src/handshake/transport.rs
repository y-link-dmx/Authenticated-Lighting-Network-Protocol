use std::collections::HashSet;
use std::net::SocketAddr;
use std::time::Duration;

use async_trait::async_trait;
use rand::{rngs::OsRng, RngCore};
use tokio::net::UdpSocket;
use tokio::time;

use super::{HandshakeError, HandshakeMessage, HandshakeTransport};
use crate::messages::{Acknowledge, ControlEnvelope};

/// JSON-over-UDP transport for control-plane exchange; lightweight and easy to test.
pub struct JsonUdpTransport {
    socket: UdpSocket,
    peer: SocketAddr,
    max_size: usize,
}

impl JsonUdpTransport {
    pub async fn bind(local: SocketAddr, peer: SocketAddr, max_size: usize) -> Result<Self, HandshakeError> {
        let socket = UdpSocket::bind(local)
            .await
            .map_err(|e| HandshakeError::Transport(e.to_string()))?;
        socket
            .connect(peer)
            .await
            .map_err(|e| HandshakeError::Transport(e.to_string()))?;
        Ok(Self {
            socket,
            peer,
            max_size,
        })
    }
}

#[async_trait]
impl HandshakeTransport for JsonUdpTransport {
    async fn send(&mut self, msg: HandshakeMessage) -> Result<(), HandshakeError> {
        let bytes = serde_json::to_vec(&msg)
            .map_err(|e| HandshakeError::Transport(format!("encode: {}", e)))?;
        self.socket
            .send_to(&bytes, self.peer)
            .await
            .map_err(|e| HandshakeError::Transport(e.to_string()))?;
        Ok(())
    }

    async fn recv(&mut self) -> Result<HandshakeMessage, HandshakeError> {
        let mut buf = vec![0u8; self.max_size];
        let (len, _) = self
            .socket
            .recv_from(&mut buf)
            .await
            .map_err(|e| HandshakeError::Transport(e.to_string()))?;
        serde_json::from_slice(&buf[..len])
            .map_err(|e| HandshakeError::Transport(format!("decode: {}", e)))
    }
}

/// Wrapper that enforces per-message timeouts on recv.
pub struct TimeoutTransport<T> {
    inner: T,
    recv_timeout: Duration,
}

impl<T> TimeoutTransport<T> {
    pub fn new(inner: T, recv_timeout: Duration) -> Self {
        Self { inner, recv_timeout }
    }
}

#[async_trait]
impl<T> HandshakeTransport for TimeoutTransport<T>
where
    T: HandshakeTransport + Send,
{
    async fn send(&mut self, msg: HandshakeMessage) -> Result<(), HandshakeError> {
        self.inner.send(msg).await
    }

    async fn recv(&mut self) -> Result<HandshakeMessage, HandshakeError> {
        match time::timeout(self.recv_timeout, self.inner.recv()).await {
            Ok(res) => res,
            Err(_) => Err(HandshakeError::Transport("recv timeout".into())),
        }
    }
}

/// Minimal reliability layer for control envelopes with retransmissions and replay protection.
pub struct ReliableControlChannel<T> {
    transport: T,
    seq: u64,
    seen_nonces: HashSet<Vec<u8>>,
    max_attempts: u8,
    base_timeout: Duration,
    drop_threshold: u8,
}

impl<T> ReliableControlChannel<T> {
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            seq: 0,
            seen_nonces: HashSet::new(),
            max_attempts: 5,
            base_timeout: Duration::from_millis(200),
            drop_threshold: 5,
        }
    }

    fn next_nonce(&mut self) -> Vec<u8> {
        let mut nonce = vec![0u8; 16];
        OsRng.fill_bytes(&mut nonce);
        nonce
    }
}

impl<T> ReliableControlChannel<T>
where
    T: HandshakeTransport + Send,
{
    pub async fn send_reliable(
        &mut self,
        mut envelope: ControlEnvelope,
    ) -> Result<Acknowledge, HandshakeError> {
        self.seq = self.seq.wrapping_add(1);
        envelope.header.seq = self.seq;
        envelope.header.nonce = self.next_nonce();
        envelope.header.timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let mut attempt: u8 = 0;
        loop {
            attempt += 1;
            self.transport
                .send(HandshakeMessage::Control(envelope.clone()))
                .await?;

            let timeout = self
                .base_timeout
                .checked_mul(2u32.saturating_pow((attempt - 1) as u32))
                .unwrap_or(self.base_timeout * 4);

            match time::timeout(timeout, self.transport.recv()).await {
                Ok(Ok(HandshakeMessage::Ack(ack))) => {
                    if ack.header.seq == envelope.header.seq && ack.ok {
                        self.seen_nonces.insert(ack.header.nonce.clone());
                        return Ok(ack);
                    } else if self.seen_nonces.contains(&ack.header.nonce) {
                        return Err(HandshakeError::Protocol("replay detected".into()));
                    }
                }
                Ok(Ok(HandshakeMessage::Keepalive(_))) => {
                    // keepalive resets attempt counter
                    attempt = 0;
                }
                _ => {
                    if attempt >= self.max_attempts || attempt >= self.drop_threshold {
                        return Err(HandshakeError::Transport(
                            "control channel retransmit limit exceeded".into(),
                        ));
                    }
                }
            }
        }
    }

    pub fn next_seq(&mut self) -> u64 {
        self.seq = self.seq.wrapping_add(1);
        self.seq
    }

    pub async fn send_signed(
        &mut self,
        envelope: ControlEnvelope,
    ) -> Result<Acknowledge, HandshakeError> {
        self.send_reliable(envelope).await
    }
}
