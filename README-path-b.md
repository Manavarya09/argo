# Argo — Path B (WezTerm fork)

This branch (`path-b-wezterm`) is the **WezTerm-based** exploration of Argo.

The Phase 1 GPUI build on `main` proved the foundation works but is 6+ months
from a shippable terminal. Path B redirects: instead of building a terminal,
we **fork WezTerm** (mature, pure Rust, MIT) and add Argo's unique value —
multi-agent orchestration — as overlay crates.

## Why

See `docs/superpowers/specs/2026-05-01-argo-path-b-wezterm-pivot.md`.

## Layout

- `vendor/wezterm/` — WezTerm submodule, pinned to a stable tag.
- `crates/argo-agent-registry/` — provider session manager.
- `crates/argo-auth-detector/` — scans local creds (Claude, Ollama, Gemini, …).
- `crates/argo-relay-v2/` — rate-limit handoff.
- `crates/argo-yolo-router/` — prompt-to-provider routing.
- `crates/argo*` (existing) — Phase 1 GPUI work, kept for reference.

## Build

```bash
git submodule update --init --recursive
cargo test -p argo-auth-detector -p argo-yolo-router
# WezTerm build:
cd vendor/wezterm && cargo build --release -p wezterm-gui
```

## Status

Scaffolding. See `docs/integration-plan.md` for how the overlay crates plug
into WezTerm.
