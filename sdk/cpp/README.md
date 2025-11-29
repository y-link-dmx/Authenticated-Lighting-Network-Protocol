# ALPINE C++ SDK

lThe C++ SDK (`alpine::sdk`) is a thin RAII wrapper around the C helpers that
come from `protocol/c/alnp.h`. It exposes `AlpineClient`, which manages the socket
lifecycle plus frame emission, and a small `discovery` helper for compiling
profile config ids.

## Public API

- `alpine::sdk::AlpineClient` opens a UDP socket, computes a config id via
  `alpine_sdk_compile_profile`, and sends frames through `alpine_sdk_client_send_frame`.
- `alpine::sdk::discovery::compile_profile_config_id(...)` mirrors the C helper so
  C++ callers can compute the same config ids without touching the C structs.

## Example

```cpp
#include "alpine/sdk/client.hpp"

alpine::sdk::AlpineClient client("192.168.1.42", 5555);
const auto config_id = client.start_stream("live", 50, 50);
client.send_frame(buffer);
client.close();
```

## Embedded support

The SDK shares the same build decisions as `protocol/c`: there are no RTTI or
exceptions in the protocol layer when `ALPINE_EMBEDDED` is defined, and the
SDK simply links against those artifacts. When building for desktop platforms
you still get the `-fno-exceptions`/`-fno-rtti` experience because the protocol
layer forces them at build time.
