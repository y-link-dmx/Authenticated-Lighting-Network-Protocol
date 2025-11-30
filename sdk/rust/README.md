# ALPINE Rust SDK

`alpine-protocol-sdk` is a high-level wrapper around the published `alpine-protocol-rs`
protocol artifacts. It keeps discovery, handshake, and streaming lifecycles explicit so
application code can reason about each step without diving into the lower-level
protocol helpers.

## When to use the SDK vs the protocol layer

- **Protocol layer** (`alpine-protocol-rs`) gives you access to every message, frame, and
  handshake primitive and is useful if you already have a transport layer or
  want to implement a custom state machine.
- **SDK** (`alpine-protocol-sdk`) builds on top of the protocol layer and manages sockets,
  keep-alive, and profile lifetimes so you can focus on discovery → connect →
  streaming flows.

## Quick lifecycle

1. Use `DiscoveryClient` to broadcast a request and inspect the returned
   `DiscoveryOutcome` for identity, capability, and server nonce information.
2. Call `AlpineClient::connect` with the discovered identity, capability set,
   and a credential pair; the SDK spins up the transport plus the keep-alive task.
3. Call `AlpineClient::start_stream`, pass a `StreamProfile`, and track the
   returned `config_id`.
4. Use `send_frame` to push encoded `FrameEnvelope`s or `send_control` for
   control envelopes.
5. Call `AlpineClient::ping`, `status`, `health`, `identity`, or `metadata` to
   send the corresponding control command and receive typed replies when the
   device returns structured CBOR payloads.

## Example

```ignore
use alpine_protocol_sdk::{AlpineClient, DiscoveryClient, DiscoveryClientOptions};
use alpine_protocol_rs::{
    crypto::identity::NodeCredentials,
    messages::{CapabilitySet, DeviceIdentity},
    profile::StreamProfile,
};
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
async fn main() -> Result<(), alpine_protocol_sdk::AlpineSdkError> {
    let discovery = DiscoveryClient::new(DiscoveryClientOptions::new(
        SocketAddr::new(IpAddr::V4([0, 0, 0, 0].into()), 0),
        SocketAddr::new(IpAddr::V4([192, 168, 1, 42].into()), 5555),
        std::time::Duration::from_secs(3),
    ))?;
    let outcome = discovery.discover(&["alpine-control".to_string()])?;
    let remote = SocketAddr::new(outcome.peer.ip(), outcome.peer.port());
    let identity = DeviceIdentity {
        device_id: outcome.reply.device_id.clone(),
        manufacturer_id: outcome.reply.manufacturer_id.clone(),
        model_id: outcome.reply.model_id.clone(),
        hardware_rev: outcome.reply.hardware_rev.clone(),
        firmware_rev: outcome.reply.firmware_rev.clone(),
    };
    let credentials = NodeCredentials::load("path/to/credentials")?;
    let capabilities = outcome.reply.capabilities.clone();

    let mut client = AlpineClient::connect(
        SocketAddr::new(IpAddr::V4([0, 0, 0, 0].into()), 0),
        remote,
        identity,
        capabilities,
        credentials,
    )
    .await?;
    let config_id = client.start_stream(StreamProfile::auto())?;
    println!("Streaming with config id {}", config_id);
    Ok(())
}
```

Every exported module in this crate has `///` documentation so the generated
docs on docs.rs describe the same lifecycle described here.
