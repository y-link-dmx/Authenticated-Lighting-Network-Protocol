# ALPINE C SDK

This folder contains a tiny stateful wrapper around the C protocol helpers (`protocol/c/alnp.h`).
It exists so C applications can keep the discovery/control/stream lifecycle simple while the
protocol artifacts continue to provide raw CBOR helpers.

## Contents

- `alpine_sdk.h` — exported structs + helpers for creating a socket, sending frames, and
  computing config ids.
- `alpine_client.c` — socket management, configuration, and frame emission.
- `alpine_discovery.c` — deterministic config id generation that mirrors the other SDKs.

## Usage

```c
alpine_sdk_client_t client;
alpine_sdk_profile_t profile = {"live", 50, 50};
char config_id[65];

alpine_sdk_compile_profile(&profile, config_id, sizeof(config_id));
alpine_sdk_client_new(&client, "192.168.1.42", 5555, 0);
alpine_sdk_client_send_frame(&client, payload, payload_len);
alpine_sdk_client_close(&client);
```

## Embedded considerations

The SDK sticks to C99, POSIX sockets, and OpenSSL's `SHA256`. When building the protocol layer
with `ALPINE_EMBEDDED`, you still get no RTTI, exceptions, or heap allocations; the SDK
layers simply reuse those artifacts.
