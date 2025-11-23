use std::sync::Arc;
use std::time::Duration;

use rand::{rngs::OsRng, Rng};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct MockUdpConfig {
    pub loss_pct: f32,
    pub jitter_ms: u64,
    pub reorder_prob: f32,
    pub corrupt: bool,
}

impl Default for MockUdpConfig {
    fn default() -> Self {
        Self {
            loss_pct: 0.0,
            jitter_ms: 0,
            reorder_prob: 0.0,
            corrupt: false,
        }
    }
}

#[async_trait::async_trait]
pub trait UdpLike: Send + Sync {
    async fn send(&self, data: Vec<u8>);
    async fn recv(&self) -> Option<Vec<u8>>;
}

#[derive(Clone)]
pub struct MockUdp {
    tx: Sender<Vec<u8>>,
    rx: Arc<Mutex<Receiver<Vec<u8>>>>,
    config: MockUdpConfig,
    reorder_buffer: Arc<Mutex<Option<Vec<u8>>>>,
}

impl MockUdp {
    pub fn pair_lossless() -> (Self, Self) {
        Self::pair_with(0.0, 0)
    }

    pub fn pair_with(loss_pct: f32, jitter_ms: u64) -> (Self, Self) {
        Self::pair_ext(MockUdpConfig {
            loss_pct,
            jitter_ms,
            ..Default::default()
        })
    }

    pub fn pair_ext(config: MockUdpConfig) -> (Self, Self) {
        let (a_tx, b_rx) = mpsc::channel(1024);
        let (b_tx, a_rx) = mpsc::channel(1024);

        let a = MockUdp {
            tx: a_tx,
            rx: Arc::new(Mutex::new(a_rx)),
            config: config.clone(),
            reorder_buffer: Arc::new(Mutex::new(None)),
        };
        let b = MockUdp {
            tx: b_tx,
            rx: Arc::new(Mutex::new(b_rx)),
            config,
            reorder_buffer: Arc::new(Mutex::new(None)),
        };
        (a, b)
    }

    async fn maybe_delay(&self) {
        if self.config.jitter_ms > 0 {
            let jitter = OsRng.gen_range(0..=self.config.jitter_ms);
            sleep(Duration::from_millis(jitter)).await;
        }
    }

    fn maybe_corrupt(&self, mut data: Vec<u8>) -> Vec<u8> {
        if self.config.corrupt && !data.is_empty() {
            let idx = OsRng.gen_range(0..data.len());
            data[idx] = data[idx].wrapping_add(1);
        }
        data
    }

    fn should_drop(&self) -> bool {
        let r: f32 = OsRng.gen_range(0.0..1.0);
        r < self.config.loss_pct
    }

    fn should_reorder(&self) -> bool {
        let r: f32 = OsRng.gen_range(0.0..1.0);
        r < self.config.reorder_prob
    }
}

#[async_trait::async_trait]
impl UdpLike for MockUdp {
    async fn send(&self, data: Vec<u8>) {
        if self.should_drop() {
            return;
        }
        let data = self.maybe_corrupt(data);
        self.maybe_delay().await;

        if self.should_reorder() {
            let mut buf = self.reorder_buffer.lock().await;
            if buf.is_some() {
                let delayed = buf.take().unwrap();
                drop(buf);
                let _ = self.tx.send(delayed).await;
            }
            let mut buf = self.reorder_buffer.lock().await;
            *buf = Some(data);
            return;
        }

        if let Some(delayed) = {
            let mut g = self.reorder_buffer.lock().await;
            g.take()
        } {
            let _ = self.tx.send(delayed).await;
        }
        let _ = self.tx.send(data.clone()).await;
    }

    async fn recv(&self) -> Option<Vec<u8>> {
        self.maybe_delay().await;
        let mut guard = self.rx.lock().await;
        guard.recv().await
    }
}
