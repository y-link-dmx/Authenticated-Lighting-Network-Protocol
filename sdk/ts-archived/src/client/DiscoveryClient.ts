import * as cbor from "cbor";
import * as crypto from "crypto";
import * as dgram from "dgram";

import { buildDiscoveryRequest, DiscoveryReply } from "@alpine-core/protocol";

import { AlpineSdkError, DiscoveryTimeoutError } from "../errors";

export interface DiscoveryClientOptions {
  remoteHost: string;
  remotePort: number;
  localPort?: number;
  timeoutMs?: number;
}

const DEFAULT_TIMEOUT = 3000;

/**
 * Stateless helper responsible for exchanging discovery payloads with devices.
 */
export class DiscoveryClient {
  private readonly socket: dgram.Socket;
  private readonly remoteHost: string;
  private readonly remotePort: number;
  private readonly timeoutMs: number;

  constructor(options: DiscoveryClientOptions) {
    this.remoteHost = options.remoteHost;
    this.remotePort = options.remotePort;
    this.timeoutMs = options.timeoutMs ?? DEFAULT_TIMEOUT;
    this.socket = dgram.createSocket("udp4");
    this.socket.bind(options.localPort ?? 0);
  }

  /**
   * Sends a discovery request and returns the decoded reply.
   */
  public async discover(requested: string[], nonce?: Buffer): Promise<DiscoveryReply> {
    const requestNonce = nonce ?? crypto.randomBytes(32);
    const request = buildDiscoveryRequest(requested, requestNonce);
    const payload = cbor.encode(request);
    await this.send(payload);
    const response = await this.receive();
    return cbor.decodeFirstSync(response) as DiscoveryReply;
  }

  /**
   * Releases the discovery socket.
   */
  public close(): void {
    this.socket.close();
  }

  private send(payload: Buffer): Promise<void> {
    return new Promise((resolve, reject) => {
      this.socket.send(
        payload,
        this.remotePort,
        this.remoteHost,
        (err: Error | null) => {
          if (err) {
            reject(new AlpineSdkError(err.message));
          } else {
            resolve();
          }
        },
      );
    });
  }

  private receive(): Promise<Buffer> {
    return new Promise((resolve, reject) => {
      let timer: NodeJS.Timeout;

      const cleanup = () => {
        this.socket.off("message", onMessage);
        clearTimeout(timer);
      };

      const onMessage = (msg: Buffer) => {
        cleanup();
        resolve(msg);
      };

      const onTimeout = () => {
        cleanup();
        reject(new DiscoveryTimeoutError());
      };

      this.socket.once("message", onMessage);
      timer = setTimeout(onTimeout, this.timeoutMs);
    });
  }
}
