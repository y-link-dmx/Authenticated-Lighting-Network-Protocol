# ALNP — Authenticated Lighting Network Protocol (Draft)

ALNP adds a lightweight, E1.33-inspired control plane in front of sACN (E1.31) streaming. The goal is to gate sACN universes behind an authenticated handshake without altering the sACN packet format or transport behavior.

## Roles
- **Controller**: initiates control-plane sessions before streaming sACN universes.
- **Node**: responder that gates sACN reception/transmission until authentication succeeds.

## Handshake Overview
Modeled after E1.33 ClientConnect/ClientConnectReply, but simplified to a direct controller ↔ node exchange (no broker):
1. `ControllerHello` — controller identity, requested protocol version, capabilities, key exchange proposal.
2. `NodeHello` — node identity, supported version, capabilities, key exchange proposal, `auth_required` flag.
3. `ChallengeRequest` — node-issued nonce + signature scheme indicating the controller must prove possession of its key.
4. `ChallengeResponse` — controller-signed nonce and key confirmation blob (derived from key exchange).
5. `SessionEstablished` — node acknowledgement; provides session id, agreed protocol version, and optional stream key.

After step 5, ALNP-Stream may start forwarding sACN packets. Any failure or timeout ends the handshake and sACN remains blocked.

## Messages (see `src/alnp/messages/alnp_handshake.proto`)
- **ControllerHello**: `{controller_cid, manufacturer, model, firmware_rev, requested_version, capabilities, key_exchange}`
- **NodeHello**: `{node_cid, manufacturer, model, firmware_rev, supported_version, capabilities, key_exchange, auth_required}`
- **ChallengeRequest**: `{nonce, controller_expected, signature_scheme}`
- **ChallengeResponse**: `{nonce, signature, key_confirmation}`
- **SessionEstablished**: `{session_id, agreed_version, stream_key, expires_at_epoch_ms}`
- **Keepalive**: `{session_id?, tick_ms}`

Control-plane envelopes (JSON over UDP with reliability/backoff):
- **ControlHeader**: `{seq, nonce, timestamp_ms}`
- **ControlEnvelope**: `{header, payload, signature}`
- **Acknowledge**: `{header, ok, detail?, signature}`

Control payloads:
- `Identify` / `IdentifyResponse`
- `GetDeviceInfo` / `DeviceInfo`
- `GetCapabilities` / `Capabilities`
- `SetWifiCreds`
- `SetUniverseMapping`
- `SetMode { normal | calibration | maintenance }`
- `GetStatus` / `StatusReport`
- `Restart`

Capabilities reflect E1.33-style feature flags: encryption support, redundancy awareness, max universes, vendor data.

## Authentication & Key Exchange
- **Algorithms**: X25519 or ECDH P-256 for key agreement; Ed25519 or ECDSA-P256 for signatures.
- **Identity**: 128-bit CID + manufacturer/model/firmware string tuples; certificate-based identities are expected but not mandatory in the draft.
- **Challenge**: node issues nonce, controller signs it (Ed25519 implemented); node validates signature and key confirmation to bind identity to the agreed key.
- **Session Keys**: derived from key exchange; `stream_key` reserved for optional payload encryption/MAC.
- **TLS**: optional wrapping point for the control channel; left as an interface to keep footprint small.

## Keepalive & Versioning
- Keepalive pings ride the control channel post-handshake; failure to respond tears down the session and blocks streaming.
- Version negotiation requires major version alignment; minor/patch may diverge if capabilities are compatible.
- Keepalive task can periodically send `Keepalive` frames on the control channel to detect dead sessions. Keepalive reception resets retransmission counters.
- Control-plane reliability: sequence numbers, nonce-based replay protection, exponential backoff on retransmit, drop connection after repeated failures.

## Streaming Integration (ALNP-Stream)
- Existing sACN multicast/unicast behavior is preserved.
- The streaming wrapper must call `session.ensure_established()` before sending/accepting any universe.
- Universe rejection: nodes drop or NACK sACN frames if the session is not authenticated or has expired.
- Payload encryption is optional and not in the baseline; ALNP-Stream keeps a hook for inserting it later.
- Jitter strategies supported: hold-last, drop, or lerp between frames. Sequence rollover resets cached frames.

## Rejection Paths
- Identity mismatch (CID not authorized, firmware too old).
- Signature failure or stale nonce.
- Unsupported protocol version or missing required capabilities.
- Expired sessions or missing keepalive heartbeats.

## Y-Link Considerations
- Minimize round trips: single challenge cycle before streaming.
- Allow lightweight implementations without a broker.
- Keep message sizes compact; avoid impacting sACN throughput or timing windows.
