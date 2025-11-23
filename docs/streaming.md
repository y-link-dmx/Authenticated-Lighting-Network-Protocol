# Streaming

ALNP-Stream wraps streaming payloads and enforces authentication gates.

## Rules
- Streaming allowed only in Ready/Streaming states.
- Fail-closed: any handshake/control failure stops streaming and disables universes.
- Universe enable/disable via control plane.
- Sequence rollover resets cached frames.

## Jitter Strategies
- **HoldLast**: repeat last frame when jitter/empty frame detected.
- **Drop**: drop missing frame.
- **Lerp**: blend previous and current frame for smoothing.

## Adapter
- `SacnStreamAdapter` trait bridges to transport; `CSacnAdapter` provides FFI hooks to existing sACN libs if desired.
- Streaming methods: `send(universe, payload)` and `subscribe(universe)` respect session gates.
