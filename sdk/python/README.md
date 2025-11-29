# ALPINE Python SDK

`alpine-sdk` wraps the published `alnp` protocol helpers so you get a
discovery → connect → stream lifecycle without wiring up sockets and CBOR
serialisation manually. This package is the SDK layer, not the protocol
helpers themselves.

## Installation

```bash
pip install alpine-sdk
```

## Quick example

```python
from alpine_sdk import (
    AlpineClient,
    AlpineClientOptions,
    DiscoveryClient,
    DiscoveryClientOptions,
    StreamProfile,
)

discovery = DiscoveryClient(
    DiscoveryClientOptions(remote_host="192.168.1.42", remote_port=5555)
)
reply = discovery.discover(["alpine-control"])

client = AlpineClient.connect(
    AlpineClientOptions(remote_host="192.168.1.42", remote_port=5555)
)
config_id = client.start_stream(StreamProfile.auto())

print("Streaming with config id", config_id)
```

Use the SDK whenever you want lifecycle helpers and UDP transport wiring out of
the box. For raw message manipulation you can still depend on `alnp`.
