# Handshake

Modeled after E1.33 ClientConnect/Reply but simplified (no broker).

## Flow
1. **ControllerHello** → controller identity, requested version, capabilities, key exchange proposal.
2. **NodeHello** ← node identity, supported version, capabilities, key exchange proposal.
3. **ChallengeRequest** ← nonce + expected controller CID + signature scheme.
4. **ChallengeResponse** → signed nonce + key confirmation.
5. **SessionEstablished** ← session id, agreed version, optional stream key.

If any step fails, session moves to Failed and streaming is blocked.

## Session State Machine
- Init → Handshake → Authenticated → Ready → Streaming
- Any → Failed | Closed
- Timeouts or replay → Failed (fail-closed).

## Cryptography
- Key Exchange: X25519
- Signatures: Ed25519
- Nonce length: 32 bytes; session id: UUIDv4

## Keepalive
- Keepalive frames on control channel; missing keepalive triggers failure.
