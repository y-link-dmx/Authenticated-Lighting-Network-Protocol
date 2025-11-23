# Bindings

## Rust
- Crate: `alnp`
- Build: `cargo test && cargo build --release`
- Public types: `AlnpSession`, `AlnpControlClient`, `AlnpStreamingClient`, `ControlEnvelope`, `ControlPayload`, `Acknowledge`.

## TypeScript / Node
- Package: `@alnp/bindings` (`bindings/ts`)
- Build: `pnpm install && pnpm run build`
- Provides typed ControlPayload, headers, device/capability types.

## C
- Header: `bindings/c/alnp.h`
- Static lib: build via `cargo build --release` (libalnp.a) for embedding in firmware.

## Python
- Package stub in `bindings/python` (pure-Python helpers for payloads and sessions).
- Build: `python -m build` (requires `build`).

## Examples
- See `examples/` for Rust/TS/C/Python snippets demonstrating handshake setup, control envelope send, and streaming gating.
