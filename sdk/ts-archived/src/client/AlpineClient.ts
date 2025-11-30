import * as cbor from "cbor";

import { ControlEnvelope, FrameEnvelope, StreamProfile } from "@alpine-core/protocol";

import { AlpineSdkError } from "../errors";
import { UdpTransport, UdpTransportOptions } from "../transport/udp";

export interface AlpineClientOptions {
  remoteHost: string;
  remotePort: number;
  localPort?: number;
}

/**
 * High-level streaming client that mirrors the Rust SDK lifecycle.
 */
export class AlpineClient {
  private readonly transport: UdpTransport;
  private configId?: string;
  private streaming = false;

  private constructor(options: AlpineClientOptions) {
    const transportOptions: UdpTransportOptions = {
      localPort: options.localPort,
      remoteHost: options.remoteHost,
      remotePort: options.remotePort,
    };
    this.transport = new UdpTransport(transportOptions);
  }

  /**
   * Returns an SDK client connected to the provided UDP address.
   */
  public static async connect(options: AlpineClientOptions): Promise<AlpineClient> {
    return new AlpineClient(options);
  }

  /**
   * Starts a stream with the supplied profile and returns its config identifier.
   */
  public startStream(profile: StreamProfile = StreamProfile.auto()): string {
    if (this.streaming) {
      throw new AlpineSdkError("stream already started");
    }
    const configId = profile.configId();
    this.configId = configId;
    this.streaming = true;
    return configId;
  }

  /**
   * Sends an encoded frame envelope over the streaming socket.
   */
  public async sendFrame(frame: FrameEnvelope): Promise<void> {
    this.ensureStreaming();
    const payload = cbor.encode(frame);
    await this.transport.send(payload);
  }

  /**
   * Sends a control-plane envelope to the remote peer.
   */
  public async sendControl(envelope: ControlEnvelope): Promise<void> {
    const payload = cbor.encode(envelope);
    await this.transport.send(payload);
  }

  /**
   * Releases socket resources without waiting for pending frames.
   */
  public close(): void {
    this.transport.close();
  }

  private ensureStreaming(): void {
    if (!this.streaming) {
      throw new AlpineSdkError("stream must be started before sending payloads");
    }
  }
}
