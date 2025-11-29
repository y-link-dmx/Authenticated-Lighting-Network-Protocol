#include "alpine/sdk/client.hpp"

#include <stdexcept>

namespace alpine::sdk {

AlpineClient::AlpineClient(const std::string& remote_host, uint16_t remote_port, uint16_t local_port)
    : client_{-1, remote_port, {0}, {0}} {
  if (alpine_sdk_client_new(&client_, remote_host.c_str(), remote_port, local_port) != 0) {
    throw std::runtime_error("failed to create Alpine SDK client");
  }
}

AlpineClient::~AlpineClient() {
  if (client_.socket_fd >= 0) {
    alpine_sdk_client_close(&client_);
  }
}

std::string AlpineClient::start_stream(const std::string& intent, uint8_t latency, uint8_t resilience) {
  alpine_sdk_profile_t profile{intent.c_str(), latency, resilience};
  if (alpine_sdk_compile_profile(&profile, client_.config_id, sizeof(client_.config_id)) != 0) {
    throw std::runtime_error("failed to compile stream profile");
  }
  return std::string(client_.config_id);
}

void AlpineClient::send_frame(const std::vector<uint8_t>& payload) {
  if (alpine_sdk_client_send_frame(&client_, payload.data(), payload.size()) != 0) {
    throw std::runtime_error("failed to send frame");
  }
}

const char* AlpineClient::config_id() const noexcept {
  return client_.config_id;
}

void AlpineClient::close() {
  if (client_.socket_fd >= 0) {
    alpine_sdk_client_close(&client_);
    client_.socket_fd = -1;
  }
}

}  // namespace alpine::sdk
