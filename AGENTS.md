# AGENTS.md (ALPINE / ALNP)

This file gives **coding agents** (Codex, Copilot, etc.) the operational context needed to work safely in this repo. Keep it practical and up to date. 

## Project overview

ALPINE (formerly ALNP) is a secure, deterministic lighting-control protocol stack with:
- Discovery + handshake (mutual auth + key agreement)
- Control plane (signed envelopes, ACKs)
- Streaming (low-latency frames)
- Stream profiles (Auto / Realtime / Install)
- Adaptive streaming (Phase 3+): **degrade visual quality, never temporal correctness**

**Guardrail:** Do not change protocol semantics “just to make code pass.” If behavior changes, docs + tests must change too.

---

## Repo layout (high level)

- `protocol/rust/alpine-protocol-rs` — primary Rust crate (protocol core, tests, benchmarks)
- `sdk/` — high-level SDK helpers (recommended entry point for app dev)

If you touch bindings, keep them consistent with the Rust canonical types and wire format.

---

## Setup & test commands (must-run)

**Rust (required before marking work done):**
- Format: `cargo fmt --manifest-path src/alnp/Cargo.toml`
- Tests: `cargo test --manifest-path src/alnp/Cargo.toml`

**Optional (run when relevant / when CI expects it):**
- Lints: `cargo clippy --manifest-path src/alnp/Cargo.toml --all-targets`

For TS/Python/C++ bindings, use the build/test commands documented in each language directory. Don’t invent commands.

---

## Change discipline (commit style)

### Commit after meaningful, traceable changes
- Prefer **small commits** that each represent one coherent change.
- Every commit should be reviewable by reading the diff + message only.
- Avoid “mega commits” that mix refactors + feature + formatting.

### Message style
Use conventional-ish, readable messages. Examples:
- `feat(sdk): add DiscoveryClient`
- `fix(embedded): restore linker flags`
- `docs: clarify sdk vs bindings`
- `test: add regression for profile lock`

---

## Branch strategy (phases & subphases)

We use branches to keep `main` stable and reduce scope bleed.

### Major work (phases) MUST be on a phase branch
Examples:
- `phase-3.3`
- `phase-4.0`

### Subphase work branches from the phase branch
Examples:
- `p3.3.1-adaptive-core` (branches from `phase-3.3`)
- `p3.3.2-stream-integration`
- `p3.3.3-observability`

### Integration rules
- Merge subphase → phase branch via PR-style review
- Merge phase branch → `main` once phase acceptance criteria are met
- Delete phase branches after merge to keep the repo tidy

**Do not rebase/rewrite history on shared branches** unless explicitly instructed.

---

## Versioning & tags (how releases work)

We follow SemVer: `vMAJOR.MINOR.PATCH`.

### What bumps what
- **PATCH**: docs/SDK/bindings ergonomics, bug fixes, CI fixes; *no wire-format break*
- **MINOR**: additive protocol capabilities or new non-breaking functionality
- **MAJOR**: breaking changes to wire format, compatibility contracts, or public APIs

### Important packaging rule
You **cannot publish over an existing package version** (npm/PyPI/GH packages).  
If a publish fails because the version exists, bump the version and retag.

### Tagging rules
- Tags must be immutable and point to the exact release commit.
- If you cut a release from a temporary branch (e.g., SDK-only), merge that work back into `main` afterward (via cherry-pick or minimal merge) so `main` contains released changes.

### Version sync
When releasing, ensure versions are aligned across relevant packages (as applicable):
- Rust crate version (Cargo.toml)
- TS package version (package.json)
- Python version (pyproject.toml / setup.cfg)
  …and update changelog accordingly.

---

## Changelog & roadmap rules

### CHANGELOG
- All user-visible changes must be recorded under an **Unreleased** section.
- On release, move/copy the Unreleased notes into a `vX.Y.Z` section.
- Keep entries factual: what changed, who it affects, how to migrate (if needed).

### Roadmap
- Update `roadmap.md` when a phase status changes:
    - Planned → In progress → ✅ Complete
- Roadmap should reflect *reality*, not aspiration.

**Avoid weird encoding artifacts** when editing roadmap/docs (keep files UTF-8, avoid copy-paste that introduces “ƒ?” characters).

---

## SDK philosophy (especially important)

The SDK exists to make ALPINE **easy to use correctly** and **easy to debug**.

### SDK should
- Provide correct-by-construction workflows
- Surface explicit errors with useful context
- Keep protocol state contained (session/keys/locks)

### SDK must NOT
- Auto-connect during discovery
- Hide retries/heuristics implicitly
- Bypass protocol steps (discovery → connect → start_stream → send_frame)

### DiscoveryClient rule
Discovery is **pre-session** and returns **facts**, not decisions:
- addr, identity, capabilities, signed flag
- no handshake, no session, no “connect for me”

---

## When in doubt

1. Prefer the smallest change that satisfies the request.
2. Run the required tests.
3. Update docs/changelog/roadmap if behavior or phase state changes.
4. If a task risks breaking protocol guarantees, stop and surface the risk instead of guessing.
