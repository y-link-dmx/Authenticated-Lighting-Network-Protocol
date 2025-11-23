# ALNP (Authenticated Lighting Network Protocol)

ALNP layers an authenticated control plane over sACN (E1.31) without touching the streaming packet
format. Controllers and nodes complete a handshake derived from ESTA E1.33 (ClientConnect /
ClientConnectReply) before any universes are forwarded.

## Components
- `handshake/`: async controller and node drivers built around a minimal transport trait.
- `messages/`: Rust message types plus a protobuf schema (`messages/alnp_handshake.proto`).
- `crypto/`: key-exchange and TLS placeholder traits (X25519 by default).
- `session/`: `AlnpSession` state machine with helpers for both roles.
- `stream.rs`: `AlnpStream` wrapper that blocks sACN send/subscribe calls until authenticated.

## Rust Scaffolding
Add the crate to a workspace or run `cargo build` directly inside `src/alnp/` to experiment.

```rust
use alnp::{
    messages::{CapabilitySet, DeviceIdentity, ProtocolVersion},
    session::{example_controller_session, example_node_session, AlnpRole},
};

// Provide your own HandshakeTransport to move ALNP messages between peers.
```

## Handshake Outline
1. `ControllerHello` → controller identity + key exchange proposal.
2. `NodeHello` → node identity + auth requirement.
3. `ChallengeRequest` → nonce targeting the controller CID.
4. `ChallengeResponse` → signed nonce + key confirmation blob.
5. `SessionEstablished` → stream key and session id; unlock ALNP-Stream.

If any step fails, sACN traffic must be rejected until a clean session is negotiated.

## ALNP-Stream
- Wrap an existing sACN sender/receiver with `AlnpStream`.
- Calls to `send()`/`subscribe()` check `session.ensure_established()`.
- Future work: inject payload encryption/MAC once a stream key is negotiated.
