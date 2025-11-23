# ALNP Python Package

Lightweight Python helpers for ALNP control envelopes and headers. This package is a thin layer for constructing/serializing control-plane messages; networking is left to the application.

## Usage
```python
from alnp import ControlHeader, ControlEnvelope, SetMode

hdr = ControlHeader(seq=1, nonce=b"\x01\x02\x03", timestamp_ms=1700000000000)
env = ControlEnvelope(header=hdr, payload=SetMode(mode="Normal"), signature=b"")
print(env.to_json())
```

## Build
```sh
python -m pip install -U build
python -m build
```
