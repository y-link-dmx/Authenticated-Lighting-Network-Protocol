# ALPINE Roadmap
*Authenticated Lighting Protocol*

This roadmap outlines the planned evolution of **ALPINE (ALP)** as a stable, secure, and future-proof real-time lighting protocol.  
The focus is long-term reliability, predictable behavior, and an ecosystem that scales from small setups to professional installations.

---

## Phase 1 — Core Protocol Foundation (v1.0)
**Status:** Near-term / Baseline

**Goal:** A rock-solid protocol that behaves correctly on real-world networks.

- Finalize v1 wire format for:
    - discovery
    - handshake
    - control
    - streaming
- Explicit real-time semantics:
    - late-frame dropping
    - out-of-order handling
    - deterministic failure behavior
- Define **keyframes and delta frames**:
    - delta always anchored to keyframes
    - full state recovery guaranteed
- Transport-agnostic UDP support (Ethernet & WiFi).
- Clear documentation of:
    - packet loss behavior
    - jitter handling
    - recovery guarantees
- SDKs expose a **single safe default mode** (`Auto`).

**Outcome:**  
ALPINE v1 works reliably without requiring network expertise or special configuration.

---

## Phase 2 — Stream Profiles & Selectable Behavior
**Goal:** Let users choose behavior without compromising safety.

- Introduce **Stream Profiles** as a first-class concept.
- Provide selectable built-in profiles (e.g. Auto, Realtime, Install).
- Define strict **profile validation rules** to prevent unsafe combinations.
- Compile validated profiles into a compact `config_id` used at runtime.
- Bind stream behavior to session identity for correctness and security.
- Profiles become immutable once streaming starts.

**Outcome:**  
Behavior is selectable, predictable, and safe by design.

---

## Phase 3 — Adaptive Streaming & Network Resilience
**Goal:** Make ALPINE feel stable even on poor networks.

- Runtime detection of:
    - packet loss
    - sequence gaps
    - jitter
    - late frames
- Automatic adaptation:
    - dynamic keyframe frequency
    - delta encoding enable/disable
    - adaptive deadlines
    - optional frame rate scaling
- Forced keyframes to recover from detected divergence.
- Optional device-side smoothing and prediction strategies.

**Outcome:**  
ALPINE gracefully degrades visual fidelity while preserving correct timing.

---

## Phase 4 — Custom Profiles & User Preferences
**Goal:** Enable flexibility for advanced users without breaking guarantees.

- Allow users to define **custom stream profiles**.
- Profiles expressed as high-level preferences (not low-level flags), such as:
    - latency sensitivity
    - smoothness vs responsiveness
    - resilience vs bandwidth usage
- All custom profiles are:
    - validated
    - normalized
    - compiled before use
- Invalid or unsafe profiles are rejected with clear errors.
- Profiles can be named, reused, and shared.

**Outcome:**  
Power users gain control while ALPINE remains deterministic and stable.

---

## Phase 5 — Security & Trust Hardening
**Goal:** Strong security without added complexity.

- Harden authenticated device identities.
- Certificate-based identity and session binding.
- Replay protection across restarts.
- Optional encrypted payloads for sensitive deployments.
- Clear, conservative security documentation.

**Outcome:**  
Security is built-in, not optional configuration.

---

## Phase 6 — SDKs, Tooling & Developer Experience
**Goal:** Make ALPINE easy to adopt and easy to debug.

- First-class SDKs (Rust, TypeScript, C):
    - opinionated defaults
    - minimal setup
- Developer CLI:
    - profile inspection
    - session tracing
    - health and diagnostics
- Observability surfaces that explain:
    - why frames are dropped
    - how the system is adapting
- Clear examples and reference implementations.

**Outcome:**  
Developers can integrate ALPINE confidently and quickly.

---

## Phase 7 — Ecosystem Growth & Future Compatibility
**Goal:** Enable long-term growth without fragmentation.

- Capability negotiation for future extensions.
- Vendor-defined extension ranges.
- Strict backward compatibility guarantees.
- No breaking changes required to evolve the protocol.
- Clean upgrade paths for future hardware and software.

**Outcome:**  
ALPINE becomes a stable foundation for a growing lighting ecosystem.

---

## Design Commitment

> **Under packet loss, jitter, or delay, ALPINE degrades visual quality — never temporal correctness.**

This principle guides every phase of development.
