# ALNP — Authenticated Lighting Network Protocol

[![Build (Rust)](https://github.com/your-org/alnp/actions/workflows/rust-release.yml/badge.svg)](https://github.com/your-org/alnp/actions/workflows/rust-release.yml)
[![Build (TS)](https://github.com/your-org/alnp/actions/workflows/ts-release.yml/badge.svg)](https://github.com/your-org/alnp/actions/workflows/ts-release.yml)
[![Build (Python)](https://github.com/your-org/alnp/actions/workflows/python-release.yml/badge.svg)](https://github.com/your-org/alnp/actions/workflows/python-release.yml)
[![Build (C)](https://github.com/your-org/alnp/actions/workflows/c-release.yml/badge.svg)](https://github.com/your-org/alnp/actions/workflows/c-release.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

ALNP adds an authenticated control plane and reliability layer to high-performance lighting streaming. It uses a compact handshake (X25519 + Ed25519) and signed control envelopes over UDP, then gates streaming through ALNP-Stream with jitter handling.

## Highlights
- Deterministic session state machine (Init → Handshake → Authenticated → Ready → Streaming).
- Control messages: Identify, DeviceInfo, Capabilities, Wifi creds, Universe mapping, Mode, Status, Restart.
- Security: X25519 key exchange, Ed25519 signatures, nonce + sequence replay protection.
- Reliability: retransmit with exponential backoff, keepalives, fail-closed behavior.
- Streaming: authenticated-only, universe enable/disable, jitter strategies (hold-last, drop, lerp).
- Bindings: Rust crate, TypeScript package, C header + static lib, Python package stub.

## Getting Started
See `docs/overview.md` and `docs/architecture.md` for a quick tour.

### Rust
```sh
cargo test
cargo build --release
```

### TypeScript
```sh
cd bindings/ts
pnpm install
pnpm run build
```

### Python
```sh
cd bindings/python
python -m pip install -U build
python -m build
```

### C
```sh
./scripts/build_c.sh
```

## Examples
- Rust: `examples/rust/basic.rs`
- TypeScript: `examples/ts/basic.ts`
- Python: `examples/python/basic.py`
- C: `examples/c/basic.c`

## Protocol Docs
- `SPEC.md` — wire format, control messages, security model.
- `docs/handshake.md` — handshake flow.
- `docs/control_plane.md` — control envelopes and reliability.
- `docs/streaming.md` — ALNP-Stream behavior.
- `docs/bindings.md` — language-specific notes.

## Versioning & Releases
- Current version: see `VERSION`.
- Release history: `CHANGELOG.md`.
- GitHub Actions publish Rust/TS/Python/C artifacts on `v*` tags to GitHub Packages (and npm if configured).

## License
Apache-2.0

## Documentation

For instructions on building the sACN library, as well as an sACN overview and in-depth
documentation, please see the [documentation](https://etclabs.github.io/sACNDocs).

## Supported Platforms

sACN uses EtcPal for platform abstraction.  See [EtcPal's README](https://github.com/ETCLabs/EtcPal#readme) for more information on supported platforms.

## Supported Languages

C++ wrappers support C++ version 14.

C functionality supports C99 with the exception of the following features:

* variable-length arrays
* flexible array members
* designated initializers
* the "restrict" keyword

## Quality Gates

This library meets a high quality standard by employing a variety of quality gates, including code review, automated tests, and a variety of static and runtime analysis tools.

### Code Reviews

* At least 2 developers must approve all code changes made before they can be merged into the integration branch.
* API and major functionality reviews typically include application developers as well.

### Automated Testing

* This consists primarily of unit tests covering the individual API modules.
* Some integration tests have also been made.

### Automated Static Analysis

* Clang Tidy (in phases) is set up. Refer to `.clang-tidy` to see which rulesets have been added.
* Klocwork is also set up - like Clang Tidy, rulesets will continue to be added over time.
* Warnings-as-errors is enabled for MSVC, GCC, Clang, and Doxygen.

### Automated Style Checking

* Clang format is enabled – currently this follows the style guidelines established for our libraries, and it may be updated from time to time. See `.clang-format` for more details.
* Non-conformance to `.clang-format` will result in pipeline failures.  The code is not automatically re-formatted.

### Continuous Integration

* A GitLab CI pipeline is being used to run builds and tests that enforce all supported quality gates for all merge requests, and for generating new library builds from `main`. See `.gitlab-ci.yml` for details.

### Automated Runtime Analysis

* All automated tests* are run under Address Sanitizer (Windows, Mac, & Linux), Memory Sanitizer (Linux), Undefined Behavior Sanitizer (Mac & Linux), and Thread Sanitizer (Mac & Linux). *E2E tests are only run on Windows (ASAN) and Linux (all).

## Revision Control

sACN development is using Git for revision control.

## License

sACN is licensed under the Apache License 2.0. sACN also incorporates some third-party software
with different license terms, disclosed in ThirdPartySoftware.txt in the directory containing this
README file.

## Standards Version

This library implements ANSI E1.31-2018. You can download the standard document for free from the
[ESTA TSP downloads page](https://tsp.esta.org/tsp/documents/published_docs.php).

## About this ETCLabs Project

sACN is official, open-source software developed by ETC employees and is designed to interact with
ETC products. For challenges using, integrating, compiling, or modifying this software, we
encourage posting on the [issues page](https://github.com/ETCLabs/sACN/issues) of this project.
