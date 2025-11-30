# ALPINE SDK

This directory now contains the SDK implementations that depend only on the published protocol layer (`alpine-protocol-rs`, `@alpine-core/protocol`, `alnp`). Each SDK is intentionally higher-level than the protocol primitives: they orchestrate discovery, handshake, streaming, keepalive, and enforce stream profiles while keeping the protocol helpers untouched.

Current SDKs:

- `sdk/rust` — `alpine-protocol-sdk` exposes `DiscoveryClient` and `AlpineClient` with real discovery → connect → stream workflows.
- `sdk/ts` — `@alpine-core/sdk` mirrors the Rust lifecycle in TypeScript with UDP transports and lifecycle helpers.
- `sdk/python` — `alpine-sdk` mirrors the same behavior and exposes `DiscoveryClient` + `AlpineClient`.
- `sdk/c` & `sdk/cpp` — thin wrappers to orchestrate sockets and profile IDs on top of the C protocol helpers (`protocol/c/alnp.h`).

> Control-plane convenience helpers (ping, status, health, identity, metadata) live only in the Rust SDK for now while the JS/TS and Python SDKs focus on discovery and streaming until the control channel helpers reach those runtimes.
