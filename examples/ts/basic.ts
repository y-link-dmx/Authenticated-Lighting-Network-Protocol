import {
  ControlEnvelope,
  ControlHeader,
  ControlPayload,
  SetMode,
} from "../../bindings/ts/src";

const header: ControlHeader = {
  seq: 1,
  nonce: new Uint8Array([1, 2, 3]),
  timestamp_ms: Date.now(),
};

const payload: ControlPayload = {
  type: "SetMode",
  body: { mode: "Normal" },
};

const envelope: ControlEnvelope = {
  header,
  payload,
  signature: new Uint8Array([]), // attach Ed25519 signature here
};

console.log("Example envelope", envelope);
