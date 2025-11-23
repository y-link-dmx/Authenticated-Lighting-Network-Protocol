# Control Plane

ALNP control messages are JSON envelopes sent over UDP with reliability helpers.

## Envelope
- `ControlHeader { seq, nonce, timestamp_ms }`
- `ControlPayload` (tagged union)
- `signature` (Ed25519 over payload)

## Messages
- `Identify` / `IdentifyResponse` (blink/metadata)
- `GetDeviceInfo` / `DeviceInfo`
- `GetCapabilities` / `Capabilities`
- `SetWifiCreds`
- `SetUniverseMapping`
- `SetMode { Normal | Calibration | Maintenance }`
- `GetStatus` / `StatusReport`
- `Restart`
- `Acknowledge` wraps responses with signature

## Security
- X25519 for key agreement during handshake.
- Ed25519 signatures for control envelopes and acknowledgements.
- Nonce + sequence required; replayed nonces are rejected.

## Reliability
- Retransmit on timeout with exponential backoff.
- Keepalive resets counters; drop after repeated failures.
- Mandatory signed response for every request.

## Example (JSON)
```json
{
  "header": { "seq": 42, "nonce": "b64==", "timestamp_ms": 1700000000000 },
  "payload": { "type": "SetMode", "body": { "mode": "Normal" } },
  "signature": "b64sig=="
}
```
