/**
 * Base error class emitted by the Alpine TypeScript SDK.
 */
export class AlpineSdkError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "AlpineSdkError";
  }
}

/**
 * Signals that a discovery request did not receive a reply in time.
 */
export class DiscoveryTimeoutError extends AlpineSdkError {
  constructor() {
    super("discovery response timed out");
    this.name = "DiscoveryTimeoutError";
  }
}
