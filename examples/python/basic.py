"""
Minimal Python control envelope creation.
"""
from dataclasses import dataclass, asdict
import json
import time
import os


@dataclass
class ControlHeader:
    seq: int
    nonce: bytes
    timestamp_ms: int


header = ControlHeader(seq=1, nonce=b"\x01\x02\x03", timestamp_ms=int(time.time() * 1000))
payload = {"type": "GetStatus"}

envelope = {"header": asdict(header), "payload": payload, "signature": ""}

print(json.dumps(envelope, indent=2))
