#ifndef ALNP_CPP_HPP
#define ALNP_CPP_HPP

#include <array>
#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

#include "alnp.h"

namespace alnp {

/// Simple span-like view into a byte buffer.
struct ByteSpan {
  const uint8_t* data = nullptr;
  std::size_t size = 0;
};

/// EncodedBuffer wraps a reusable heap buffer that C helpers can fill.
class EncodedBuffer {
 public:
  explicit EncodedBuffer(std::size_t capacity = 2048)
      : buffer_(capacity), written_(0) {}

  uint8_t* data() { return buffer_.data(); }
  std::size_t capacity() const { return buffer_.size(); }
  uint32_t size() const { return written_; }

  /// Provides an `alnp_bytes_t` descriptor with the current capacity.
  alnp_bytes_t descriptor() const {
    return alnp_bytes_t{buffer_.data(),
                        static_cast<uint32_t>(buffer_.size())};
  }

  /// Record the actual byte count written by the C helper.
  void setSize(uint32_t len) {
    if (len > buffer_.size()) {
      buffer_.resize(len);
    }
    written_ = len;
  }

 private:
  std::vector<uint8_t> buffer_;
  uint32_t written_;
};

/// DiscoveryRequest mirrors the C helper struct but keeps ownership in
/// std::array/std::vector.
struct DiscoveryRequest {
  std::array<uint8_t, 32> client_nonce{};
  std::vector<std::string> requested{};
};

/// SignedReply carries the CBOR message + signature produced by the library.
struct SignedReply {
  std::vector<uint8_t> payload;
  std::vector<uint8_t> signature;
};

inline ByteSpan make_span(const std::vector<uint8_t>& bytes) {
  return ByteSpan{bytes.empty() ? nullptr : bytes.data(),
                  bytes.size()};
}

inline int buildDiscoveryRequest(const DiscoveryRequest& request,
                                 EncodedBuffer& out) {
  alnp_discovery_request_t c_req;
  c_req.client_nonce.data = request.client_nonce.data();
  c_req.client_nonce.len =
      static_cast<uint32_t>(request.client_nonce.size());

  std::vector<const char*> requested_ptrs;
  requested_ptrs.reserve(request.requested.size());
  for (const auto& entry : request.requested) {
    requested_ptrs.push_back(entry.c_str());
  }
  c_req.requested = requested_ptrs.empty() ? nullptr
                                           : requested_ptrs.data();
  c_req.requested_len =
      static_cast<uint32_t>(requested_ptrs.size());

  auto buf = out.descriptor();
  const int rc = alnp_build_discovery_request(&c_req, &buf);
  if (rc == 0) {
    out.setSize(buf.len);
  }
  return rc;
}

inline int verifyDiscoveryReply(const SignedReply& reply,
                                const ByteSpan& expected_nonce,
                                const ByteSpan& verifying_key) {
  alnp_signed_reply_t c_reply;
  c_reply.payload.data = reply.payload.data();
  c_reply.payload.len =
      static_cast<uint32_t>(reply.payload.size());
  c_reply.signature.data = reply.signature.data();
  c_reply.signature.len =
      static_cast<uint32_t>(reply.signature.size());

  return alnp_verify_discovery_reply(
      &c_reply, expected_nonce.data,
      static_cast<uint32_t>(expected_nonce.size), verifying_key.data,
      static_cast<uint32_t>(verifying_key.size));
}

inline int encodeControl(const ByteSpan& session_id,
                         const ByteSpan& payload, uint64_t seq,
                         EncodedBuffer& out) {
  auto buf = out.descriptor();
  const int rc = alnp_encode_control(session_id.data, payload.data,
                                      static_cast<uint32_t>(payload.size),
                                      seq, &buf);
  if (rc == 0) {
    out.setSize(buf.len);
  }
  return rc;
}

inline int encodeStreamFrame(const ByteSpan& session_id,
                             const alnp_frame_t& frame,
                             EncodedBuffer& out) {
  auto buf = out.descriptor();
  const int rc =
      alnp_encode_stream_frame(session_id.data, &frame, &buf);
  if (rc == 0) {
    out.setSize(buf.len);
  }
  return rc;
}

}  // namespace alnp

#endif  // ALNP_CPP_HPP
