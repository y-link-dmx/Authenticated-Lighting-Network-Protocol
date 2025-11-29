#ifndef ALPINE_SDK_DISCOVERY_HPP
#define ALPINE_SDK_DISCOVERY_HPP

#include <cstdint>
#include <string>

#include "alpine_sdk.h"

namespace alpine::sdk {

/// Helpers for deterministic profile configuration and validation.
namespace discovery {

/// Returns the 64-character config id for the provided weights.
std::string compile_profile_config_id(const std::string& intent, uint8_t latency, uint8_t resilience);

}  // namespace discovery

}  // namespace alpine::sdk

#endif  // ALPINE_SDK_DISCOVERY_HPP
