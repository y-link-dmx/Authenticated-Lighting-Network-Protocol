# ALPINE — Authenticated Lighting Protocol

[![Rust](https://img.shields.io/badge/Rust-crates.io-000000?style=for-the-badge\&logo=rust\&logoColor=white)](https://crates.io/crates/alpine-protocol-rs)
[![TypeScript](https://img.shields.io/badge/TypeScript-npm-CB3837?style=for-the-badge\&logo=npm\&logoColor=white)](https://www.npmjs.com/package/@alpine-core/protocol)
[![Python](https://img.shields.io/badge/Python-PyPI-3776AB?style=for-the-badge\&logo=python\&logoColor=white)](https://pypi.org/project/alnp/)
[![License](https://img.shields.io/badge/License-Apache--2.0-blue?style=for-the-badge)](LICENSE)

---

ALPINE is a **modern, secure, vendor-agnostic lighting control protocol** designed to replace legacy systems such as **sACN / E1.31, RDMnet, and proprietary device APIs**.

It is built for **real-time correctness**, **cryptographic identity**, and **predictable behavior under network stress**.

---

## What ALPINE provides

* **Discovery** — authenticated device identification without knowing IPs
* **Handshake** — mutual authentication and session key agreement
* **Control plane** — reliable, signed control envelopes
* **Streaming layer** — low-latency real-time lighting frames
* **Capabilities** — devices explicitly declare supported features
* **Extensibility** — vendor namespaces and structured envelopes
* **No universes, no DMX limits** — modern frame-based model

---

## Core design

ALPINE is built around:

* **CBOR** for compact, structured messages
* **Ed25519** signatures for identity & integrity
* **X25519** for session key exchange
* **UDP broadcast** discovery
* **UDP or QUIC** for streaming
* **Deterministic session state machines**

**Design guarantee:**

> Under packet loss, jitter, or delay, ALPINE degrades visual quality — **never temporal correctness**.

---

## Architecture overview

ALPINE is intentionally split into **three layers**:

#### 1. Protocol layer (canonical, low-level)

**Vendor-agnostic protocol truth.**

Protocol artifacts expose:

* wire/message definitions
* cryptographic primitives
* stream profiles and compiled configs
* stateless helpers and codecs

The protocol layer **does not**:

* open sockets
* manage sessions
* expose clients
* perform retries or lifecycles

Packages:

* **Rust**: `alpine-protocol-rs`
* **TypeScript**: `@alpine-core/protocol`
* **Python**: `alnp`
* **C / C++**: static library + headers (embedded-friendly)

Use the protocol layer when you need:

* embedded targets (ESP32, MCU, C-only)
* heap-free or allocation-controlled runtimes
* full control over transport and scheduling

---

### 2. SDKs (recommended for applications)

**High-level, ergonomic clients built on top of the protocol layer.**

SDKs provide:

* discovery orchestration
* connection lifecycle
* streaming helpers
* retries, defaults, and intent-driven APIs

Official SDKs:

* **Rust**: `alpine-protocol-sdk`
* **TypeScript**: `@alpine-core/sdk`
* **Python**: `alpine-sdk`

SDKs depend **only on the public protocol API** and can evolve independently of the protocol layer.

For most applications, **start with the SDK**.

---

### 3. Tooling (CLI)

**Authoritative inspection & debugging tools.**

An official **ALPINE CLI** (planned) provides:

* device discovery & inspection
* handshake validation
* capability inspection
* protocol diagnostics
* conformance & test tooling

The CLI is built on the SDKs with optional raw protocol access.

---

## Phase 0 — Modular architecture split (target v2.0)
**Status:** In progress

**Goal:** Treat the protocol layer and SDKs as separate release artifacts while preserving the Phase 2 guarantees.

- Move `alpine-protocol-rs` to `protocol/rust/alpine-protocol-rs` and keep its API focused on wire helpers, cryptographic primitives, and profiles.
- Build the SDKs inside `sdk/` so they depend only on the published protocol public API and orchestrate explicit discovery → connect → stream workflows.
- Introduce `protocol-publish.yml` and `sdk-publish.yml` so the protocol layer releases first and the SDK runs against the freshly published artifacts.
- Version the protocol artifacts as v2.x.x while letting SDK packages (such as `alpine-protocol-sdk`) follow their own semantic versions.

Phase 0 is the milestone that lets us treat SDKs as optional helpers while keeping the protocol layer stable for embedded targets and constrained runtimes.

---

## Quick start (SDK-first)

Typical workflow:

1. Discover devices on the network
2. Inspect identity & capabilities
3. Explicitly choose a target
4. Connect with a declared stream profile
5. Start streaming frames

Discovery and connection are **always explicit** — the SDK never auto-connects or caches silently.

This makes failures explainable and behavior predictable.

---

## Stream profiles

Streaming behavior is selected explicitly using `StreamProfile`:

* `Auto` — safe default
* `Realtime` — minimum latency
* `Install` — resilience-focused

Profiles are:

* declarative
* validated
* compiled into immutable `config_id`s
* bound to the session (never swapped silently)

Invalid combinations are rejected **before** hitting the wire.

---

## Documentation

ALPINE treats documentation as part of the API contract.

Protocol and architecture:

* `SPEC.md`
* `docs/architecture.md`
* `docs/discovery.md`
* `docs/handshake.md`
* `docs/control_plane.md`
* `docs/streaming.md`
* `docs/security.md`
* `docs/errors.md`

Guarantees, edge-cases, and failure modes are documented explicitly — especially under loss, jitter, and load.

---

## CI & correctness

Protocol-level E2E tests run real UDP discovery, handshake, control, and streaming paths on Linux.

Bindings, SDKs, and embedded builds are validated independently so correctness does not depend on tooling.

---

## Versioning & stability

* **Bindings** evolve conservatively and prioritize stability.
* **SDKs** evolve faster and focus on ergonomics.
* Protocol behavior is versioned and regression-tested.

Major architectural changes (such as the SDK/protocol layer split) are treated as **v2-class releases**.

---

## License

Apache-2.0
