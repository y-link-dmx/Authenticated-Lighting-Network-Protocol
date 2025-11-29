#include "alpine_sdk.h"

#include <arpa/inet.h>
#include <netinet/in.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

static int alpine_sdk_build_remote(const char* host, uint16_t port, struct sockaddr_in* out) {
  memset(out, 0, sizeof(*out));
  out->sin_family = AF_INET;
  out->sin_port = htons(port);
  return inet_pton(AF_INET, host, &out->sin_addr) == 1 ? 0 : -1;
}

/// Initializes a session client bound to the provided UDP destination.
int alpine_sdk_client_new(
    alpine_sdk_client_t* client,
    const char* host,
    uint16_t port,
    uint16_t local_port) {
  if (!client || !host) {
    return -1;
  }

  struct sockaddr_in remote;
  if (alpine_sdk_build_remote(host, port, &remote) != 0) {
    return -1;
  }

  int fd = socket(AF_INET, SOCK_DGRAM, 0);
  if (fd < 0) {
    return -1;
  }

  struct sockaddr_in local = {0};
  local.sin_family = AF_INET;
  local.sin_port = htons(local_port);
  local.sin_addr.s_addr = htonl(INADDR_ANY);
  if (local_port != 0 && bind(fd, (struct sockaddr*)&local, sizeof(local)) != 0) {
    close(fd);
    return -1;
  }

  client->socket_fd = fd;
  client->remote_port = port;
  strncpy(client->remote_host, host, sizeof(client->remote_host) - 1);
  client->remote_host[sizeof(client->remote_host) - 1] = '\0';
  client->config_id[0] = '\0';
  return 0;
}

/// Sends a pre-encoded frame over the SDK socket.
int alpine_sdk_client_send_frame(
    alpine_sdk_client_t* client,
    const uint8_t* payload,
    size_t payload_len) {
  if (!client || payload_len == 0) {
    return -1;
  }
  struct sockaddr_in remote = {0};
  if (alpine_sdk_build_remote(client->remote_host, client->remote_port, &remote) != 0) {
    return -1;
  }
  ssize_t sent = sendto(client->socket_fd, payload, payload_len, 0, (struct sockaddr*)&remote, sizeof(remote));
  return sent == (ssize_t)payload_len ? 0 : -1;
}

/// Closes the UDP socket owned by the SDK client.
void alpine_sdk_client_close(alpine_sdk_client_t* client) {
  if (!client) {
    return;
  }
  close(client->socket_fd);
  client->socket_fd = -1;
}
