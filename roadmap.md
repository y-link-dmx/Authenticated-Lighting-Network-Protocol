# ALNP Roadmap

## Phase 1 — Baseline
- Stabilize v1 wire format and control API.
- Document onboarding/discovery flows and bindings.

## Phase 2 — Reliability & Telemetry
- Add richer logging/metrics around state transitions and retries.
- Expose counters for retransmits, replays, and keepalive gaps.

## Phase 3 — Security Hardening
- Certificate-based identities, pinning, and revocation hooks.
- Configurable TLS wrapping for control channel.
- Replay detection across restarts (persistent nonce set).

## Phase 4 — Streaming Enhancements
- Optional payload encryption/MAC.
- Redundant controllers and fast failover for large installs.
- Performance validation to confirm low jitter and low latency.

## Phase 5 — Tooling & Ecosystem
- Developer CLI for key/cert generation and session tracing.
- Interop/conformance fixtures and reference integrations.
- Documentation site and sample deployments.
