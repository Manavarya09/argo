# ADR: Path B — fork WezTerm instead of building terminal from scratch

- **Date:** 2026-05-01
- **Status:** Proposed
- **Branch:** `path-b-wezterm`

## Context

Phase 1 of Argo's pure-GPUI build (16 commits, see `main`) proved the
foundation works: PTY plumbing, alacritty terminal state, GPUI render loop,
window resize/reflow. But that path's time-to-product is **6+ months** before
we can ship anything an end user would call "a terminal" — text selection,
copy/paste robustness, ligatures, IME, scrollback, search, link detection,
SSH integration, image protocols, GPU acceleration tuning, theming, config
language, accessibility, packaging across macOS/Linux/Windows. None of that
is Argo's actual product. None of that is our differentiator.

Meanwhile, **WezTerm** already exists:

- 10k+ GitHub stars, used in production by thousands.
- Pure Rust, MIT-licensed, actively maintained.
- Has every feature listed above and more.
- Has clean abstractions for plugins: a Lua config layer, "domains" for
  alternative pane backends, and a plugin event bus.

Argo's actual unique value is **agent orchestration**:
auto-detecting local AI agent auth, running them simultaneously, relay
handoff under rate limits, prompt-to-provider routing. That value can be
delivered as a *layer on top of* an existing terminal.

## Decision

Fork WezTerm. Vendor it as a git submodule under `vendor/wezterm`. Build
Argo-specific functionality as separate crates that plug into WezTerm's
existing extensibility points. Ship Argo as a custom WezTerm build with our
crates linked in plus a default `argo.lua` config.

## Trade-offs

**We inherit:**
- WezTerm's UI quality and battle-tested terminal correctness (great).
- WezTerm's Lua config language — different from a Rust-native config but
  flexible and well-documented (acceptable).
- A large unfamiliar codebase to learn (cost).
- WezTerm's release cadence and any upstream breakage risk (manageable via
  pinned submodule tag).

**We gain:**
- ~6 months of runway redirected to agent features.
- Immediate cross-platform support.
- A real user community we can ship to.

## Architecture sketch

Four new crates under `crates/`, each independent of WezTerm's core:

| Crate | Responsibility |
|---|---|
| `argo-agent-registry` | Track active provider sessions (Claude, Codex, Gemini, Ollama, Copilot). Manage lifecycle, health, restart. |
| `argo-auth-detector` | Scan local filesystem and known endpoints for authenticated providers (~/.claude, ~/.gemini, localhost:11434, gh CLI). |
| `argo-relay-v2` | Watch each agent's output for rate-limit signals; transparently switch the pane to a fallback provider mid-conversation. |
| `argo-yolo-router` | Classify user prompts (planning / coding / review / debug / docs) and route to the best-fit available provider. |

Integration points into WezTerm:

- Register a `LocalAgentDomain` so panes of type `claude`, `gemini`, etc.
  spawn the appropriate CLI under PTY (initially) and later upgrade to a
  block-based view.
- Run `argo-auth-detector` at startup via a Lua hook; render results in a
  status-bar widget.
- `argo-relay-v2` subscribes to PTY output events; on rate-limit detection it
  tells the domain to swap the underlying agent.

This ADR locks the direction. Subsequent commits scaffold the crates and
vendor WezTerm.
