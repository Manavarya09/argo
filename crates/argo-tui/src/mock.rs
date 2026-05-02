use std::sync::atomic::{AtomicUsize, Ordering};

/// Returns the next mock output line for a given agent provider.
///
/// Each provider has its own canned script. The script index advances
/// each call, wrapping around. This lets us drive a "live" demo
/// without hitting any real API while we build the rest of the UX.
pub fn next_line(provider: &str, counter: &AtomicUsize) -> Option<String> {
    let script = script_for(provider);
    if script.is_empty() {
        return None;
    }
    let idx = counter.fetch_add(1, Ordering::Relaxed) % script.len();
    Some(script[idx].to_string())
}

fn script_for(provider: &str) -> &'static [&'static str] {
    match provider {
        "claude" => CLAUDE_SCRIPT,
        "gemini" => GEMINI_SCRIPT,
        "codex" => CODEX_SCRIPT,
        "ollama" => OLLAMA_SCRIPT,
        _ => &[],
    }
}

const CLAUDE_SCRIPT: &[&str] = &[
    "▸ reading src/auth.ts (147 lines)",
    "▸ reading src/middleware.ts (89 lines)",
    "◆ proposing changes to 3 files",
    "  + src/auth.ts (jwt support)",
    "  + src/middleware.ts (validate token)",
    "  + tests/auth.test.ts (new)",
    "[accept] [reject] [diff]",
    "",
    "tokens: 2,341 in / 1,872 out · $0.04",
];

const GEMINI_SCRIPT: &[&str] = &[
    "◆ analyzing PR diff",
    "▸ 12 files changed, +384 -127",
    "▸ ran cargo clippy on changed files",
    "  · 3 minor warnings (unused import x2, format x1)",
    "▸ ran cargo test --workspace",
    "  · 47 passing, 0 failing",
    "◆ summary: ready for merge after clippy fixes",
    "",
    "tokens: 4,102 in / 612 out · $0.02",
];

const CODEX_SCRIPT: &[&str] = &[
    "▸ scanning workspace for OAuth callsites",
    "  · src/auth/login.ts:42",
    "  · src/auth/callback.ts:18",
    "  · src/middleware/session.ts:73",
    "◆ no security issues detected in current diff",
    "▸ writing 6 unit tests for new flows",
    "[accept] [reject]",
    "",
    "tokens: 1,807 in / 2,145 out · $0.03",
];

const OLLAMA_SCRIPT: &[&str] = &[
    "[qwen-coder local]",
    "▸ explanation:",
    "  the auth flow uses a short-lived",
    "  access token + refresh token pair.",
    "  refresh tokens persist across",
    "  restarts in localStorage.",
    "",
    "(local · no network · no spend)",
];
