# Argo — Design Spec (v0.1)

**Date:** 2026-05-01
**Status:** Approved, ready for implementation planning

---

## What it is

A native Rust terminal that auto-detects every AI coding agent authenticated on the user's machine (Claude, Codex, Copilot, Gemini, Ollama) and lets them run simultaneously in a multi-pane grid. Built-in relay handoff when one model rate-limits. Zero API keys to enter — Argo uses each tool's existing local auth.

**Tagline:** *Every AI agent on one ship.*

## Why now

Nothing combines: auto-detected local auth + unified multi-model terminal + relay handoff + multi-agent orchestration. Aider has multi-model but no auth detection or IDE. Continue.dev is plugin-only. Cursor is single-model. The "zero API key" UX is the viral hook.

## Stack

- **Language:** Rust
- **UI:** GPUI (Zed's framework) — GPU-accelerated, single-language, Warp-quality polish
- **Terminal:** `portable-pty` + `alacritty_terminal` for VT/ANSI parsing
- **Async runtime:** Tokio
- **Per-pane isolation:** async tasks (not separate processes), serializable session state

## Aesthetic (locked)

- Background `#0a0a0c` (deep neutral black), no pure `#000`
- Single accent color used sparingly
- Mono font: JetBrains Mono / Berkeley Mono
- Block-based command output (Warp-style)
- Thin 1px low-opacity dividers between panes
- Status indicators are tiny dots (●/○/⚠/✗), not badges
- Motion only when meaningful — no idle animations
- No emoji in chrome. Ever.
- Reference vibe: Warp + Linear + Zed in pure black

## Architecture (7 layers)

```
UI (GPUI)
  ↓
Session Orchestrator     ← spawns/manages N panes, focus, layout, persistence
  ↓
Terminal Engine          ← portable-pty + vt100, one per pane
  ↓
Agent Runtime            ← Provider trait, async + streaming first-class
  ↓
Provider Adapters        ← Claude / Codex / Copilot / Gemini / Ollama
  ↓
Auth Detector            ← scans local FS, probes ports, auto-runs auth flows
  ↓
Relay Engine             ← ported from existing Relay project
```

## Pane model

Each pane is a **real native PTY** running the actual provider CLI (`claude`, `codex`, `gemini`, `ollama run`, or `zsh`). Argo is the multiplexer + auth manager + relay layer. The CLIs stay native — Argo never modifies their output.

**Layouts:** `⌘1` single, `⌘2` two-col, `⌘4` 2×2, `⌘9` 3×3, free split via `⌘D`/`⌘⇧D`.

**Pane chrome:** 22px top bar — `provider · model · cwd · status dot`.

## Auth detection (the killer feature)

On first launch Argo auto-runs auth flows for any installed-but-unsigned providers:

| Provider | Detection |
|----------|-----------|
| Claude Code | `~/.claude/credentials.json` + API ping |
| Codex / OpenAI | `~/.codex/auth.json`, `OPENAI_API_KEY`, OpenAI CLI session |
| GitHub Copilot | `~/.config/github-copilot/hosts.json` or VS Code token store |
| Gemini CLI | `~/.config/google-cloud-sdk/` ADC + `~/.gemini/` |
| Ollama | HTTP `localhost:11434/api/tags` |

- Auto-runs each provider's native auth flow in an embedded sub-PTY
- Filesystem watchers trigger live re-detection
- Re-auth on token expiry happens silently in-pane
- Argo never copies, transmits, or stores credentials. Privacy promise is on the homepage.

## Execution modes

**1. Manual** — standard CLI behavior in each pane.

**2. Relay (always-on)** — watchdog detects rate-limit/429/session-death, serializes context (chat history + open files + tool state), spawns fallback provider, replays compressed context. Fallback chain configured in `~/.argo/relay.toml`.

**3. YOLO mode (`?` prefix) — in v0.1**
- A tiny classifier (Haiku/Flash) tags the prompt: `planning | coding | review | debug | docs | research | creative | chat`
- Argo routes to the user-configured provider for that task type
- Configured in `~/.argo/routing.toml`:
  ```toml
  [yolo]
  classifier = "claude-haiku-4.5"
  [routes]
  planning = "gemini-2.5-pro"
  coding   = "claude-sonnet-4.6"
  review   = "codex-gpt-5"
  ```
- Sensible defaults shipped; users tweak per project
- Override: `?gemini ...` forces a specific provider

**4. Super YOLO (`??` prefix) — v0.3**
- Planner (Opus/Pro) decomposes prompt + project context into a task DAG
- Each task spawns in its own git worktree → no conflicts
- Right-sized providers per task (Haiku for boilerplate, Opus for reasoning)
- Shared blackboard JSONL for inter-agent coordination
- Merge orchestrator resolves conflicts at end
- Live DAG dock shows tasks in flight; `[cancel]` kills all
- Hard caps: parallelism (default 6), spend (default $1/run)

## Cross-pane features

- **Broadcast** (`⌘⇧B`) — type once, send to N selected panes
- **Compare** — pin two panes, locked input, side-by-side
- **Pipe** — `pane1 | pane2`, output of one feeds next
- **Workspace presets** — JSON file saves grid + provider config
- **Universal search** (`⌘F`) — across all scrollback at once

## v0.1 ship target (6 weeks)

- Native Rust + GPUI shell with locked aesthetic
- Multi-pane grid (1, 2×2, 3×3, free split)
- Real PTYs per pane
- Auth detection + auto-run flows for Claude, Gemini, Ollama
- Workspace presets, command palette
- Manual mode + Relay mode
- **YOLO mode (`?` routing)** — promoted from v0.2 per user direction

## v0.2

- Codex + Copilot adapters
- Broadcast / Compare / Pipe modes
- MCP server support

## v0.3 (the viral release)

- Super YOLO (`??` task decomposition + worktrees + merge orchestration)
- Cost meter + spend caps
- DAG dock

## v1.0

- Cloud sync (paid)
- Team workspaces
- Hosted relay queue

## Open core / business model

- **Free + open source:** entire local app, all auth detection, all multi-agent orchestration, relay engine
- **Paid cloud:** sync agent history across machines, team session sharing, shared knowledge graph, hosted relay queue for offline handoff

## Privacy guarantee

Argo never sends credentials anywhere. They stay on the user's machine, used only by the official SDKs they belong to. This is a stated promise on the homepage and in onboarding.

## Open questions for implementation planning

1. GPUI version pinning — use latest published or vendor a specific commit?
2. Per-provider auth flow specifics (especially Copilot's token retrieval, which is undocumented)
3. Relay engine — fork existing Relay repo or extract as workspace crate?
4. Classifier model choice for YOLO — Haiku 4.5 (fast, $) vs Gemini Flash (faster, free tier)
5. Distribution — Homebrew + direct DMG + cargo install? All three?
