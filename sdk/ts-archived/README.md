# Archived TypeScript SDK Runtime

`@alpine-core/sdk` used to provide a TypeScript runtime for discovery, handshake, and streaming. This directory is now archived:

- No runtime maintenance is performed here.
- There is no control-plane implementation or MAC verification.
- Refer to the Rust SDK for the canonical behavior; TypeScript only ships bindings (see `protocol/ts`).

This directory remains for reference or experimentation, but it should not be treated as a supported SDK. Future TypeScript consumers should rely on the Rust runtime via bindings or wait for an explicitly reintroduced SDK.
