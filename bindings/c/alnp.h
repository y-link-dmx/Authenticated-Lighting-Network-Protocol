#ifndef ALNP_H
#define ALNP_H

#include <stdint.h>
#include <stdbool.h>

typedef enum {
  ALNP_STATE_INIT = 0,
  ALNP_STATE_HANDSHAKE,
  ALNP_STATE_AUTHENTICATED,
  ALNP_STATE_READY,
  ALNP_STATE_STREAMING,
  ALNP_STATE_FAILED,
  ALNP_STATE_CLOSED
} alnp_state_t;

typedef void (*alnp_control_callback)(const uint8_t* data, uint32_t len, void* ctx);

// Initialize ALNP and prepare control-plane sockets.
int alnp_init(void);

// Send a control-plane message (JSON/UDP encoded envelope).
int alnp_send_control(const uint8_t* data, uint32_t len);

// Register callback for inbound control messages.
void alnp_set_control_callback(alnp_control_callback cb, void* ctx);

// Start/stop streaming after authentication.
int alnp_start_streaming(void);
int alnp_stop_streaming(void);

// Current session state.
alnp_state_t alnp_get_state(void);

#endif // ALNP_H
