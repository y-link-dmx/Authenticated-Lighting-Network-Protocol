# Architecture

```
Controller                    Node
----------                    ----
UDP Control Channel (JSON)
  ControllerHello  ---->
                     <----  NodeHello
  ChallengeRequest <----
  ChallengeResponse ---->
  SessionEstablished <----  (session id, stream key)
  Keepalive  <----/---->
  Control Envelopes (Identify, GetInfo, SetMode, ...)

UDP Streaming
  ALNP-Stream gating existing DMX/sACN payloads
```

## Session Lifecycle
- **Init → Handshake → Authenticated → Ready → Streaming → (Failed|Closed)**
- Any invalid message, timeout, or replay → Failed (fail-closed; streaming blocked).
- Keepalives reset retransmission timers; missing keepalive triggers failure.

## Reliability
- ControlHeader `{seq, nonce, timestamp_ms}` for every envelope.
- Retransmit with exponential backoff; drop after N failures.
- Replay protection via nonce set and signature verification.

## Jitter Handling (Streaming)
- `HoldLast`: reuse last frame if jitter detected.
- `Drop`: drop empty/late frames.
- `Lerp`: blend previous and current frame (lightweight smoothing).

## Components
- `alnp` Rust crate: handshake, control, reliability, streaming adapter.
- Bindings: TS/JS (`bindings/ts`), C header (`bindings/c/alnp.h`), Python stub (`bindings/python`).
- GitHub Actions workflows publish per-language artifacts on tags.
