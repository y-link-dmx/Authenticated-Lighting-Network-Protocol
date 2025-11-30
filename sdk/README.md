# ALPINE SDK

This directory now contains the SDK implementations that depend only on the published protocol layer (`alpine-protocol-rs`, `@alpine-core/protocol`, `alnp`). Each SDK is intentionally higher-level than the protocol primitives: they orchestrate discovery, handshake, streaming, keepalive, and enforce stream profiles while keeping the protocol helpers untouched.

Current SDKs/runtime directories:

- `sdk/rust` — `alpine-protocol-sdk` exposes `DiscoveryClient` and `AlpineClient` with real discovery, handshake, streaming, and control helpers. This is the only fully supported runtime currently.
- `sdk/ts-archived` — archived TypeScript runtime; it is retained for reference only and is not part of the supported release cadence.
- `sdk/python-archived` — archived Python runtime; Python consumers should rely on the bindings under `protocol/python`.
- `sdk/c` & `sdk/cpp` — thin wrappers to orchestrate sockets and profile IDs on top of the C protocol helpers (`protocol/c/alnp.h`).

> Control-plane convenience helpers (`ping`, `status`, `health`, `identity`, `metadata`) are exclusive to the Rust SDK in this phase. Bindings-only packages in other languages do not implement the control lifecycle.
