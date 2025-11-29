import * as dgram from "dgram";

import { AlpineSdkError } from "../errors";

export interface UdpTransportOptions {
  remoteHost: string;
  remotePort: number;
  localPort?: number;
}

/**
 * Simple UDP helper used by the SDK clients.
 */
export class UdpTransport {
  private readonly socket: dgram.Socket;

  constructor(private readonly options: UdpTransportOptions) {
    this.socket = dgram.createSocket("udp4");
    this.socket.bind(options.localPort ?? 0);
  }

  /**
   * Sends the provided payload to the configured remote backend.
   */
  send(payload: Buffer): Promise<void> {
    return new Promise((resolve, reject) => {
      this.socket.send(
        payload,
        this.options.remotePort,
        this.options.remoteHost,
        (err: Error | null) => {
          if (err) {
            reject(new AlpineSdkError(`udp send failure: ${err.message}`));
          } else {
            resolve();
          }
        },
      );
    });
  }

  /**
   * Releases the socket resources.
   */
  close(): void {
    this.socket.close();
  }
}
