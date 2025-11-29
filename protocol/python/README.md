# ALNP Python Package

Lightweight Python helpers for ALNP control envelopes and headers. This package is a thin layer for constructing/serializing control-plane messages; networking is left to the application.

## Usage
```python
from alnp import (
    build_discovery_request,
    build_control_envelope,
    build_frame_envelope,
    CapabilitySet,
    DeviceIdentity,
)

discovery = build_discovery_request(["identity", "capabilities"], b"\x00" * 32)
control = build_control_envelope(
    session_id="00000000-0000-0000-0000-000000000000",
    seq=1,
    op="set_mode",
    payload={"mode": "Normal"},
    mac=b"\x00" * 16,
)
frame = build_frame_envelope(
    session_id="00000000-0000-0000-0000-000000000000",
    timestamp_us=0,
    priority=5,
    channel_format="u8",
    channels=[0, 1, 2, 3],
)
print(discovery, control, frame)
```

## Build
```sh
python -m pip install -U build
python -m build
```
