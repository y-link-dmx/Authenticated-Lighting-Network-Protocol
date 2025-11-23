# ALNP (Authenticated Lighting Network Protocol)

ALNP layers an authenticated control plane over sACN (E1.31) without touching the streaming packet
format. Controllers and nodes complete a handshake derived from ESTA E1.33 (ClientConnect /
ClientConnectReply) before any universes are forwarded.

## Components
- `handshake/`: async controller and node drivers built around a minimal transport trait.
- `messages/`: Rust message types plus a protobuf schema (`messages/alnp_handshake.proto`).
- `crypto/`: X25519 key exchange, Ed25519 signing, identity loading from PEM, and TLS placeholder traits.
- `session/`: deterministic `AlnpSession` state machine (`Init → Handshake → Authenticated → Ready → Streaming`, fail/close paths).
- `stream.rs`: `AlnpStream` wrapper that blocks sACN send/subscribe calls until authenticated; jitter strategies (hold-last, drop, lerp), fail-closed.
- `handshake/transport.rs`: JSON-over-UDP transport + timeout + reliable control channel with retransmits/backoff and replay protection.
- `handshake/keepalive.rs`: keepalive task helper for post-handshake liveness.
- `stream/sacn_adapter.rs`: thin FFI adapter to gate existing sACN sender/receiver handles.
- `bindings/ts`: TypeScript bindings for Studio; `bindings/c/alnp.h` for firmware.

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

## Control Channel
- Use `JsonUdpTransport` for quick controller/node testing.
- Wrap any transport in `TimeoutTransport` to enforce receive timeouts.
- Call `spawn_keepalive` to periodically emit keepalive frames after session establishment.
- For control payloads, use `ControlEnvelope` with `ControlHeader` (seq + nonce) and Ed25519 signatures; `ReliableControlChannel` retransmits with exponential backoff and drops after repeated failures.
