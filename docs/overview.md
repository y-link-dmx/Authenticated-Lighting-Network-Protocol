# ALNP Overview

Authenticated Lighting Network Protocol (ALNP) provides a secure, low-latency control plane and streaming wrapper for lighting data. It layers an authenticated handshake and reliable control messaging on top of UDP while preserving sACN-like transport performance.

## Goals
- Authenticate controllers and nodes before streaming.
- Keep raw streaming performance (multicast/unicast friendly).
- Minimal round trips: single challenge/response cycle.
- Cross-language bindings (Rust, TS/JS, C, Python) with consistent wire format.

## Stack
- Transport: UDP for control + streaming.
- Security: X25519 key exchange, Ed25519 signatures, nonce + sequence replay protection.
- Control Plane: JSON envelopes over UDP with reliability (retransmit/backoff, keepalive).
- Streaming: ALNP-Stream guards access; jitter handling (hold-last, drop, lerp).

## Discovery & Onboarding
1) Device advertises over BLE: IP + public key fingerprint.  
2) Controller connects via IP and initiates ALNP handshake.  
3) Challenge/response authenticates the controller; session established.  
4) Control messages configure universes/mode; streaming allowed once Ready/Streaming state reached.
