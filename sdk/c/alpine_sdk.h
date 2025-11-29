#ifndef ALPINE_SDK_H
#define ALPINE_SDK_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Stateful UDP client maintained by the SDK.
typedef struct {
  int socket_fd;
  uint16_t remote_port;
  char remote_host[256];
  char config_id[65];
} alpine_sdk_client_t;

/// Simple profile descriptor used for deterministic config id generation.
typedef struct {
  const char* intent;
  uint8_t latency_weight;
  uint8_t resilience_weight;
} alpine_sdk_profile_t;

/// Creates a new UDP client that will send packets to `host:port`.
int alpine_sdk_client_new(
    alpine_sdk_client_t* client,
    const char* host,
    uint16_t port,
    uint16_t local_port);

/// Sends an encoded frame payload to the configured backend.
int alpine_sdk_client_send_frame(
    alpine_sdk_client_t* client,
    const uint8_t* payload,
    size_t payload_len);

/// Closes the socket owned by the SDK client.
void alpine_sdk_client_close(alpine_sdk_client_t* client);

/// Computes the SHA256-based config id for a stream profile.
int alpine_sdk_compile_profile(
    const alpine_sdk_profile_t* profile,
    char* out_config_id,
    size_t out_config_id_len);

#ifdef __cplusplus
}
#endif

#endif // ALPINE_SDK_H
