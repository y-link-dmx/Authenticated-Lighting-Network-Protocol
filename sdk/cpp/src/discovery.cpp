#include "alpine/sdk/discovery.hpp"

#include <stdexcept>

namespace alpine::sdk::discovery {

std::string compile_profile_config_id(const std::string& intent, uint8_t latency, uint8_t resilience) {
  alpine_sdk_profile_t profile{intent.c_str(), latency, resilience};
  char buffer[65];
  if (alpine_sdk_compile_profile(&profile, buffer, sizeof(buffer)) != 0) {
    throw std::runtime_error("failed to compile stream profile");
  }
  return std::string(buffer);
}

}  // namespace alpine::sdk::discovery
