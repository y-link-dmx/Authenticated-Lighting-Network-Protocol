#include "alpine_sdk.h"

#include <openssl/sha.h>
#include <stdio.h>
#include <string.h>

/// Generates the hashed `config_id` for a profile.
int alpine_sdk_compile_profile(
    const alpine_sdk_profile_t* profile,
    char* out_config_id,
    size_t out_config_id_len) {
  if (!profile || !out_config_id || out_config_id_len < 65) {
    return -1;
  }
  char buffer[128];
  int len = snprintf(buffer, sizeof(buffer), "%s:%u:%u", profile->intent, profile->latency_weight, profile->resilience_weight);
  unsigned char digest[SHA256_DIGEST_LENGTH];
  SHA256((const unsigned char*)buffer, len, digest);
  for (size_t i = 0; i < SHA256_DIGEST_LENGTH; ++i) {
    snprintf(out_config_id + (i * 2), 3, "%02x", digest[i]);
  }
  out_config_id[64] = '\0';
  return 0;
}
