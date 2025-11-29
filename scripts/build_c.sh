#!/bin/sh
set -e

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
VERSION=$(cat "$ROOT_DIR/VERSION")
DIST="$ROOT_DIR/dist/c"

mkdir -p "$DIST"
cd "$ROOT_DIR/protocol/rust/alpine-protocol-rs"

echo "==> Building static library for C consumers (version $VERSION)"
echo "==> Validating UDP E2E tests (cargo test --tests -- --ignored)"
cargo test --tests -- --ignored
cargo build --release

cp -f target/release/libalpine.a "$DIST/libalpine-$VERSION.a"
cp -f "$ROOT_DIR/protocol/c/alnp.h" "$DIST/alnp.h"
mkdir -p "$DIST/sdk"
cp -f "$ROOT_DIR/protocol/cpp/sdk/alpine_sdk.hpp" "$DIST/sdk/"
cp -f "$ROOT_DIR/protocol/cpp/alnp.hpp" "$DIST/alnp.hpp"
echo "C artifacts written to $DIST"
