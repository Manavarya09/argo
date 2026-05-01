# Argo

Every AI agent on one ship.

A native Rust terminal that auto-detects every AI coding agent authenticated on
your machine — Claude Code, OpenAI Codex, GitHub Copilot, Gemini, Ollama — and
runs them simultaneously in a multi-pane grid. Built-in relay handoff when one
model rate-limits. Zero API keys to enter.

## Status

Phase 1 (Foundation). Single dark-themed pane with a real native PTY. Multi-pane
grid, provider auto-detection, relay, and YOLO mode arrive in subsequent phases
of v0.1.

## Requirements

- macOS (Linux/Windows targets later)
- Rust stable (1.88+, edition2024)
- macOS Metal Toolchain for GPU shader compilation:
  ```bash
  xcodebuild -downloadComponent MetalToolchain
  ```

## Run (dev)

```bash
cargo run -p argo
```

A 1280x800 dark window opens with your `$SHELL` (fallback `/bin/zsh`) running in
a single pane. Type any command. Run `vim`, `htop`, `top` — full PTY emulation.

## Build (release)

```bash
cargo build --release -p argo
```

The binary lands at `target/release/argo`.

## Tests

```bash
cargo test --workspace -- --test-threads=1
```

12 tests across `argo-theme`, `argo-pty`, and `argo-terminal`.

## Repository layout

```
crates/
  argo/            main binary
  argo-theme/      design tokens (palette, typography, spacing)
  argo-pty/        portable-pty wrapper
  argo-terminal/   alacritty_terminal-backed VT state
  argo-ui/         GPUI views (ArgoApp, Pane, PaneTitleBar)
docs/
  superpowers/specs/  design specs
  superpowers/plans/  implementation plans
assets/            app icon (added before release)
```

## License

MIT.
