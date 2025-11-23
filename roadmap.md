# ALNP Roadmap

## Phase 0 — Baseline
- Keep sACN transport untouched; treat ALNP as an overlay.
- Land protocol scaffolding (handshake modules, message schema, session guard, ALNP-Stream wrapper).
- Write SPEC.md and examples to guide implementers.

## Phase 1 — Authenticated Handshake MVP
- Implement real X25519/Ed25519 primitives (replace placeholder key/nonce code).
- Add persistent identity store and certificate/key loading API.
- Integrate timeouts, retries, and keepalive pings on the control channel.
- Record session state machine telemetry for debugging.

## Phase 2 — Controller/Node Integration
- Bind ALNP to existing sACN sender/receiver APIs via an adapter layer.
- Enforce session gating for all universes; include graceful rejection paths.
- Add capability negotiation (max universes, optional payload privacy flags).
- Provide C API shims if sACN clients are not Rust-aware.

## Phase 3 — Security Hardening
- Replace StaticKeyAuthenticator with signature validation (Ed25519/ECDSA).
- Add certificate-based identities and pinning/CRL-style revocation hooks.
- Define replay protections (nonce freshness, session expiry, channel binding).
- Add configurable TLS wrapping for control channel (behind feature flags).

## Phase 4 — Streaming Enhancements
- Optional stream payload encryption/MAC with minimal overhead.
- Redundant controllers + fast failover for Y-Link deployments.
- Version negotiation tests and interop fixtures with golden vectors.
- Load/perf validation to prove sACN throughput remains unaffected.

## Phase 5 — Tooling & Ecosystem
- Developer CLI to generate keys/certs and visualize sessions.
- Wire up conformance tests aligned with E1.33 semantics.
- Produce documentation site + integration guides for fixture vendors.
