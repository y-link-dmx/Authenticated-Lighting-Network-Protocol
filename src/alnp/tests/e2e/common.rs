use std::sync::Arc;

use alnp::handshake::{HandshakeError, HandshakeMessage, HandshakeTransport};
use alnp::handshake::keepalive::spawn_keepalive;
use alnp::session::{example_controller_session, example_node_session, LoopbackTransport};
use serde_json;
use tokio::sync::Mutex;

use crate::support::{MockUdp, UdpLike};

pub struct MockHandshakeTransport {
    udp: Arc<dyn UdpLike>,
}

impl MockHandshakeTransport {
    pub fn new(udp: Arc<dyn UdpLike>) -> Self {
        Self { udp }
    }
}

#[async_trait::async_trait]
impl HandshakeTransport for MockHandshakeTransport {
    async fn send(&mut self, msg: HandshakeMessage) -> Result<(), HandshakeError> {
        let bytes = serde_json::to_vec(&msg).map_err(|e| HandshakeError::Transport(e.to_string()))?;
        self.udp.send(bytes).await;
        Ok(())
    }

    async fn recv(&mut self) -> Result<HandshakeMessage, HandshakeError> {
        let raw = self
            .udp
            .recv()
            .await
            .ok_or_else(|| HandshakeError::Transport("channel closed".into()))?;
        serde_json::from_slice(&raw).map_err(|e| HandshakeError::Transport(e.to_string()))
    }
}

/// Build controller/node sessions over mock UDP.
pub async fn make_sessions(
    client_udp: Arc<dyn UdpLike>,
    server_udp: Arc<dyn UdpLike>,
) -> (alnp::session::AlnpSession, alnp::session::AlnpSession) {
    let client_id = alnp::messages::DeviceIdentity {
        cid: uuid::Uuid::new_v4(),
        manufacturer: "ALNP".into(),
        model: "Controller".into(),
        firmware_rev: "1.0.0".into(),
    };
    let node_id = alnp::messages::DeviceIdentity {
        cid: uuid::Uuid::new_v4(),
        manufacturer: "ALNP".into(),
        model: "Node".into(),
        firmware_rev: "1.0.0".into(),
    };

    let mut c_transport = MockHandshakeTransport::new(client_udp.clone());
    let mut n_transport = MockHandshakeTransport::new(server_udp.clone());

    let controller = tokio::spawn(async move {
        example_controller_session(client_id, &mut c_transport).await.unwrap()
    });

    let node = tokio::spawn(async move {
        example_node_session(node_id, &mut n_transport).await.unwrap()
    });

    let controller = controller.await.unwrap();
    let node = node.await.unwrap();
    (controller, node)
}

/// Helper to spawn keepalive tasks on a given transport.
pub async fn start_keepalive<T: HandshakeTransport + Send + 'static>(
    transport: T,
    session_id: Option<uuid::Uuid>,
    interval_ms: u64,
) {
    let shared = Arc::new(Mutex::new(transport));
    spawn_keepalive(shared, std::time::Duration::from_millis(interval_ms), session_id).await;
}
