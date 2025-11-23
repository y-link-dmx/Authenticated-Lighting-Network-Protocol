export type Uuid = string;

export enum OperatingMode {
  Normal = "Normal",
  Calibration = "Calibration",
  Maintenance = "Maintenance",
}

export interface DeviceIdentity {
  cid: Uuid;
  manufacturer: string;
  model: string;
  firmware_rev: string;
}

export interface ProtocolVersion {
  major: number;
  minor: number;
  patch: number;
}

export interface CapabilitySet {
  supports_encryption: boolean;
  supports_redundancy: boolean;
  max_universes?: number;
  vendor_data?: string;
}

export interface ControlHeader {
  seq: number;
  nonce: Uint8Array;
  timestamp_ms: number;
}

export interface IdentifyRequest {
  blink: boolean;
  metadata?: string;
}

export interface IdentifyResponse {
  acknowledged: boolean;
  detail?: string;
}

export interface SetWifiCreds {
  ssid: string;
  password: string;
}

export interface UniverseMapping {
  universe: number;
  output_port: number;
}

export interface SetUniverseMapping {
  mappings: UniverseMapping[];
}

export interface SetMode {
  mode: OperatingMode;
}

export interface StatusReport {
  healthy: boolean;
  detail?: string;
  uptime_secs: number;
}

export interface RestartRequest {
  reason?: string;
}

export type ControlPayload =
  | { type: "Identify"; body: IdentifyRequest }
  | { type: "IdentifyResponse"; body: IdentifyResponse }
  | { type: "GetDeviceInfo" }
  | { type: "DeviceInfo"; body: DeviceInfo }
  | { type: "GetCapabilities" }
  | { type: "Capabilities"; body: CapabilitySet }
  | { type: "SetWifiCreds"; body: SetWifiCreds }
  | { type: "SetUniverseMapping"; body: SetUniverseMapping }
  | { type: "SetMode"; body: SetMode }
  | { type: "GetStatus" }
  | { type: "StatusReport"; body: StatusReport }
  | { type: "Restart"; body: RestartRequest };

export interface DeviceInfo {
  identity: DeviceIdentity;
  version: ProtocolVersion;
  mode: OperatingMode;
}

export interface ControlEnvelope {
  header: ControlHeader;
  payload: ControlPayload;
  signature: Uint8Array;
}

export interface Acknowledge {
  header: ControlHeader;
  ok: boolean;
  detail?: string;
  signature: Uint8Array;
}

export interface SessionState {
  state: "Init" | "Handshake" | "Authenticated" | "Ready" | "Streaming" | "Failed" | "Closed";
  reason?: string;
}
