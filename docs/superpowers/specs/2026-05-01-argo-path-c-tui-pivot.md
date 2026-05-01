# ADR: Path C — TUI prototype with ratatui

Date: 2026-05-01
Status: Accepted (Path C only)
Branch: `path-c-tui`

## Context

Argo's Phase 1 GPUI work took 16 commits to reach "zsh in a window" — a single
PTY-backed pane rendered with native GPU. The product story Argo is selling,
however, is *multi-agent*: open the app, see which AI agents you're already
authenticated with (Claude Code, Codex, Copilot, Gemini, Ollama), spawn several
of them at once into split panes, type a prompt, watch them stream in parallel,
hand work between them.

None of that has been validated yet. Building the multi-agent UX directly on
GPUI means continuing to wrestle with native GPU rendering, font shaping,
layout reflow, and cross-platform packaging *before* we know if the product
itself feels right.

## Decision

Open a parallel branch — `path-c-tui` — that pivots Argo to a `ratatui`-based
TUI. We rebuild the full Argo product surface in the terminal:

- multi-pane layout (sidebar + 2x2 agent grid + input + status bar)
- auth detection for the five providers (real filesystem / HTTP probes)
- mocked streaming agent output that *looks* real
- a spawn flow for adding new panes from authenticated providers

The TUI is the product prototype. If it sells the multi-agent story, we
incrementally replace the mocked agent layer with real adapters (Claude Code
SDK, Ollama HTTP, Codex API) without touching the UI. If/when we want native
polish back, we rebuild the UI on top of the same state types.

## Trade-offs

What we lose vs. GPUI:

- No GPU-accelerated rendering — coarse character grid, no sub-pixel typography.
- Animation is limited to redraw ticks; no smooth easing.
- No image rendering, no rich diff blocks, no inline charts.
- Mouse interaction is awkward in many terminals.

What we gain:

- Single-binary CLI ships immediately, runs over SSH, runs in tmux.
- Iteration is dramatically faster — no font atlas, no GPU pipeline.
- We can prove the multi-agent UX before committing to a native rebuild.
- Ratatui is mature and stable; crossterm handles the platform quirks.

## Migration path

State and domain types (auth status, agent registry, pane model, message
buffers) are written *as if* the UI were native. The TUI is one renderer over
those types. When we rebuild native, we keep the state crate and write a new
`argo-ui` against it — the mocked streaming layer is replaced by real
adapters in the same shape.

## Non-goals for this branch

- Real chat with any provider (mocks only).
- Persistence of conversation history.
- Multi-window / multi-workspace support.
- Plugin / MCP integration.
