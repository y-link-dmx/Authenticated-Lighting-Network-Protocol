# ALPINE TypeScript SDK

The TypeScript SDK (`@alpine-core/sdk`) wraps the `@alpine-core/protocol` helpers so
you can orchestrate discovery, handshake, and streaming without managing CBOR or UDP
helpers directly. This package is the **SDK** layer—not the primitive protocol helpers—and
depends on the published protocol helpers from npm.

## Installation

```bash
pnpm add @alpine-core/sdk
```

_or use `npm install @alpine-core/sdk` / `yarn add @alpine-core/sdk` depending on your workflow._

## Quick example

```ts
import { AlpineClient, DiscoveryClient, StreamProfile } from "@alpine-core/sdk";

const discovery = new DiscoveryClient({
  remoteHost: "192.168.1.42",
  remotePort: 5555,
});

const reply = await discovery.discover(["alpine-control"]);
const client = await AlpineClient.connect({
  remoteHost: "192.168.1.42",
  remotePort: 5555,
});

const configId = client.startStream(StreamProfile.auto());
await client.sendFrame({ /* FrameEnvelope */ });
```

Use the SDK when you want lifecycle helpers and UDP sockets wired up for you.
If you only need to read or encode CBOR payloads, import `@alpine-core/protocol` directly.
