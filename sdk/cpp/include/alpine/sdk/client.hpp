#ifndef ALPINE_SDK_CLIENT_HPP
#define ALPINE_SDK_CLIENT_HPP

#include <cstdint>
#include <string>
#include <vector>

#include "alpine_sdk.h"

namespace alpine::sdk {

/// RAII wrapper around the C SDK client.
class AlpineClient {
 public:
  explicit AlpineClient(const std::string& remote_host, uint16_t remote_port, uint16_t local_port = 0);
  ~AlpineClient();

  AlpineClient(const AlpineClient&) = delete;
  AlpineClient& operator=(const AlpineClient&) = delete;

  /// Starts a stream and returns the protocol config identifier.
  std::string start_stream(const std::string& intent, uint8_t latency, uint8_t resilience);

  /// Sends the provided payload as a stream frame.
  void send_frame(const std::vector<uint8_t>& payload);

  /// Returns the last computed config id (empty if `start_stream` was not called).
  const char* config_id() const noexcept;

  /// Closes the socket owned by the client.
  void close();

 private:
  alpine_sdk_client_t client_;
};

}  // namespace alpine::sdk

#endif  // ALPINE_SDK_CLIENT_HPP
