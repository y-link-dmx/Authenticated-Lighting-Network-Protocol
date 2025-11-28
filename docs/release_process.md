# Release Process

Use these steps to keep Phase 2 releases boring, repeatable, and documented. The CI workflows mirror this checklist so that tagging the repository simply re-triggers the same validation pipeline.

1. **Run the Rust suite**  
   `cargo test --manifest-path src/alnp/Cargo.toml`  
   Verifies SDK/unit tests, the profile guarantees, and the UDP E2E suites that guard the handshake, control, and streaming helpers.

2. **Build the C artifacts**  
   `scripts/build_c.sh`  
   This script runs `cargo build --release`, packages `libalpine.a` and headers into `dist/c`, and copies the C++ SDK metadata that the desktop / embedded clients consume.

3. **Validate the embedded flags**  
   `scripts/build_embedded_cpp.sh`  
   Confirms the constrained C++ binding links against `libalpine-<version>.a` with `ALPINE_EMBEDDED`, `-fno-exceptions`, `-fno-rtti`, `-Os`, and the other ESP32-safe compiler options. This mirrors `.github/workflows/embedded.yml`.

4. **Build the TypeScript SDK**  
   `scripts/build_ts.sh`  
   Produces the `dist/ts` package that `@alpine-core/protocol` publishes. Run `npm pack` or `pnpm pack` as needed and check the bundled docs/SDK surfaces.

5. **Build the Python SDK**  
   `scripts/build_python.sh`  
   Generates wheel/sdist artifacts placed in `dist/python` so `twine upload` can publish to GitHub Packages or PyPI.

6. **Document and package**  
   Bundle `README.md`, `SPEC.md`, `docs/`, and the SDK layers into the release tarball so every artifact ships with the API contract. CI already copies these into the GHCR `/dist` assets for each release tag.

7. **Tag and push**  
   Once every build/test step is green, create the release tag (e.g., `git tag v1.2.2 && git push origin v1.2.2`). The release workflows will pick up the tag, publish the Rust/C/TS/Python packages, and keep the CI jobs happy.

If any step above fails, fix the root cause before tagging so Phase 2 remains frozen with a reproducible release process.
