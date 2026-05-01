# Argo v0.1 — Phase 1: Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A native macOS app that opens a dark window and runs a single working PTY pane (you can `ls`, `vim`, `htop` inside) with the locked Argo aesthetic. Validates GPUI + PTY + VT100 stack end-to-end.

**Architecture:** Cargo workspace with focused crates: `argo-theme` (design tokens), `argo-pty` (process I/O), `argo-terminal` (VT100 state via alacritty_terminal), `argo-ui` (GPUI components), and `argo` (main binary). Pure logic crates are TDD'd; GPUI rendering is smoke-tested.

**Tech Stack:** Rust 1.78+, GPUI (Zed's UI framework), `portable-pty` (PTY abstraction), `alacritty_terminal` (VT parser), `tokio` (async runtime), `anyhow` (errors), `tracing` (logging).

**Out of scope for Phase 1:** Multi-pane, AI providers, auth detection, relay, YOLO mode, command palette functionality, workspace presets.

---

## File Structure

```
argo/
├── Cargo.toml                      # workspace root
├── rust-toolchain.toml             # pin Rust version
├── crates/
│   ├── argo/                       # main binary
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   ├── argo-theme/                 # design tokens
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── colors.rs           # ColorPalette
│   │       ├── typography.rs       # font config
│   │       └── spacing.rs          # spacing scale
│   ├── argo-pty/                   # PTY wrapper
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── session.rs          # PtySession
│   ├── argo-terminal/              # VT state
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── state.rs            # TerminalState wrapping alacritty Term
│   └── argo-ui/                    # GPUI components
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── app.rs              # ArgoApp root view
│           ├── pane.rs             # Pane view (renders TerminalState)
│           └── titlebar.rs         # pane title bar
└── docs/
```

---

### Task 1: Workspace scaffolding

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `crates/argo/Cargo.toml`
- Create: `crates/argo/src/main.rs`
- Create: `crates/argo-theme/Cargo.toml`
- Create: `crates/argo-theme/src/lib.rs`
- Create: `crates/argo-pty/Cargo.toml`
- Create: `crates/argo-pty/src/lib.rs`
- Create: `crates/argo-terminal/Cargo.toml`
- Create: `crates/argo-terminal/src/lib.rs`
- Create: `crates/argo-ui/Cargo.toml`
- Create: `crates/argo-ui/src/lib.rs`
- Create: `.gitignore`

- [ ] **Step 1: Write workspace `Cargo.toml`**

```toml
[workspace]
resolver = "2"
members = [
    "crates/argo",
    "crates/argo-theme",
    "crates/argo-pty",
    "crates/argo-terminal",
    "crates/argo-ui",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.78"
license = "MIT"
authors = ["Manav Arya Singh"]

[workspace.dependencies]
anyhow = "1.0"
thiserror = "1.0"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
portable-pty = "0.8"
alacritty_terminal = "0.24"
gpui = { git = "https://github.com/zed-industries/zed", branch = "main" }
```

- [ ] **Step 2: Pin Rust toolchain**

`rust-toolchain.toml`:
```toml
[toolchain]
channel = "1.78.0"
components = ["rustfmt", "clippy"]
```

- [ ] **Step 3: Create `.gitignore`**

```
/target
**/*.rs.bk
.DS_Store
.idea/
.vscode/
```

- [ ] **Step 4: Scaffold `crates/argo/Cargo.toml`**

```toml
[package]
name = "argo"
version.workspace = true
edition.workspace = true

[dependencies]
argo-theme = { path = "../argo-theme" }
argo-ui = { path = "../argo-ui" }
anyhow.workspace = true
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true

[[bin]]
name = "argo"
path = "src/main.rs"
```

- [ ] **Step 5: Scaffold `crates/argo/src/main.rs`**

```rust
fn main() {
    println!("argo");
}
```

- [ ] **Step 6: Scaffold each library crate's `Cargo.toml`**

For `argo-theme`:
```toml
[package]
name = "argo-theme"
version.workspace = true
edition.workspace = true

[dependencies]
```

For `argo-pty`:
```toml
[package]
name = "argo-pty"
version.workspace = true
edition.workspace = true

[dependencies]
anyhow.workspace = true
portable-pty.workspace = true
tokio.workspace = true
tracing.workspace = true

[dev-dependencies]
tokio = { version = "1", features = ["full", "test-util"] }
```

For `argo-terminal`:
```toml
[package]
name = "argo-terminal"
version.workspace = true
edition.workspace = true

[dependencies]
alacritty_terminal.workspace = true
tracing.workspace = true
```

For `argo-ui`:
```toml
[package]
name = "argo-ui"
version.workspace = true
edition.workspace = true

[dependencies]
argo-theme = { path = "../argo-theme" }
argo-terminal = { path = "../argo-terminal" }
argo-pty = { path = "../argo-pty" }
gpui.workspace = true
anyhow.workspace = true
tokio.workspace = true
tracing.workspace = true
```

- [ ] **Step 7: Add `pub fn placeholder() {}` to each `src/lib.rs`**

Each library crate gets a one-line `lib.rs`:
```rust
pub fn placeholder() {}
```

- [ ] **Step 8: Verify workspace builds**

Run: `cargo build`
Expected: All 5 crates compile. Warnings about unused `placeholder()` are OK.

- [ ] **Step 9: Commit**

```bash
git add .
git commit -m "feat: cargo workspace scaffolding for argo v0.1 phase 1"
```

---

### Task 2: Color palette (TDD)

**Files:**
- Create: `crates/argo-theme/src/colors.rs`
- Modify: `crates/argo-theme/src/lib.rs`

- [ ] **Step 1: Write the failing test**

`crates/argo-theme/src/colors.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_palette_uses_locked_aesthetic() {
        let p = ColorPalette::dark();
        // background is deep neutral black, never pure black
        assert_eq!(p.bg.as_hex(), "#0a0a0c");
        assert_eq!(p.fg.as_hex(), "#e8e6e3");
        assert_eq!(p.accent.as_hex(), "#f6f5f3");
        assert_eq!(p.divider.as_hex(), "#1a1a1d");
        assert_eq!(p.status_ok.as_hex(), "#7fb069");
        assert_eq!(p.status_warn.as_hex(), "#d4a96a");
        assert_eq!(p.status_err.as_hex(), "#d97766");
    }

    #[test]
    fn color_from_hex_roundtrips() {
        let c = Color::from_hex("#0a0a0c").unwrap();
        assert_eq!(c.r, 0x0a);
        assert_eq!(c.g, 0x0a);
        assert_eq!(c.b, 0x0c);
        assert_eq!(c.as_hex(), "#0a0a0c");
    }

    #[test]
    fn color_from_hex_rejects_invalid() {
        assert!(Color::from_hex("not-a-color").is_err());
        assert!(Color::from_hex("#xyz").is_err());
        assert!(Color::from_hex("#abc").is_err()); // 3-char form not supported in v0.1
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p argo-theme`
Expected: FAIL — `Color`/`ColorPalette` not defined.

- [ ] **Step 3: Implement `Color` and `ColorPalette`**

Replace the test-only `colors.rs` with:
```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
        let hex = hex.strip_prefix('#').ok_or(ColorError::MissingHash)?;
        if hex.len() != 6 {
            return Err(ColorError::InvalidLength);
        }
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ColorError::InvalidDigit)?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ColorError::InvalidDigit)?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ColorError::InvalidDigit)?;
        Ok(Self { r, g, b })
    }

    pub fn as_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ColorError {
    #[error("hex string must start with #")]
    MissingHash,
    #[error("hex string must be 6 hex digits")]
    InvalidLength,
    #[error("hex string contains invalid digit")]
    InvalidDigit,
}

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub divider: Color,
    pub status_ok: Color,
    pub status_warn: Color,
    pub status_err: Color,
}

impl ColorPalette {
    pub fn dark() -> Self {
        Self {
            bg: Color::new(0x0a, 0x0a, 0x0c),
            fg: Color::new(0xe8, 0xe6, 0xe3),
            accent: Color::new(0xf6, 0xf5, 0xf3),
            divider: Color::new(0x1a, 0x1a, 0x1d),
            status_ok: Color::new(0x7f, 0xb0, 0x69),
            status_warn: Color::new(0xd4, 0xa9, 0x6a),
            status_err: Color::new(0xd9, 0x77, 0x66),
        }
    }
}
```

Add `thiserror` to `crates/argo-theme/Cargo.toml`:
```toml
[dependencies]
thiserror.workspace = true
```

- [ ] **Step 4: Re-export from `lib.rs`**

`crates/argo-theme/src/lib.rs`:
```rust
mod colors;
pub use colors::{Color, ColorError, ColorPalette};
```

- [ ] **Step 5: Run tests to verify pass**

Run: `cargo test -p argo-theme`
Expected: 3 tests pass.

- [ ] **Step 6: Commit**

```bash
git add crates/argo-theme
git commit -m "feat(theme): locked dark color palette"
```

---

### Task 3: Typography + spacing tokens (TDD)

**Files:**
- Create: `crates/argo-theme/src/typography.rs`
- Create: `crates/argo-theme/src/spacing.rs`
- Modify: `crates/argo-theme/src/lib.rs`

- [ ] **Step 1: Write failing tests**

`crates/argo-theme/src/typography.rs`:
```rust
#[derive(Debug, Clone)]
pub struct Typography {
    pub mono_family: &'static str,
    pub mono_fallbacks: &'static [&'static str],
    pub size_sm: f32,
    pub size_md: f32,
    pub size_lg: f32,
    pub line_height: f32,
}

impl Typography {
    pub fn default_mono() -> Self {
        Self {
            mono_family: "JetBrains Mono",
            mono_fallbacks: &["Berkeley Mono", "SF Mono", "Menlo", "monospace"],
            size_sm: 12.0,
            size_md: 14.0,
            size_lg: 16.0,
            line_height: 1.4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mono_typography_uses_jetbrains_with_fallbacks() {
        let t = Typography::default_mono();
        assert_eq!(t.mono_family, "JetBrains Mono");
        assert!(t.mono_fallbacks.contains(&"Berkeley Mono"));
        assert!(t.mono_fallbacks.contains(&"monospace"));
        assert_eq!(t.size_md, 14.0);
        assert_eq!(t.line_height, 1.4);
    }
}
```

`crates/argo-theme/src/spacing.rs`:
```rust
#[derive(Debug, Clone, Copy)]
pub struct Spacing;

impl Spacing {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 24.0;
    pub const TITLEBAR_HEIGHT: f32 = 22.0;
    pub const DIVIDER_WIDTH: f32 = 1.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_scale_is_consistent() {
        assert_eq!(Spacing::XS, 4.0);
        assert_eq!(Spacing::SM, 8.0);
        assert_eq!(Spacing::MD, 12.0);
        assert_eq!(Spacing::LG, 16.0);
        assert_eq!(Spacing::XL, 24.0);
        assert_eq!(Spacing::TITLEBAR_HEIGHT, 22.0);
        assert_eq!(Spacing::DIVIDER_WIDTH, 1.0);
    }
}
```

- [ ] **Step 2: Update `lib.rs` to expose them**

`crates/argo-theme/src/lib.rs`:
```rust
mod colors;
mod typography;
mod spacing;

pub use colors::{Color, ColorError, ColorPalette};
pub use typography::Typography;
pub use spacing::Spacing;
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p argo-theme`
Expected: 5 tests pass.

- [ ] **Step 4: Commit**

```bash
git add crates/argo-theme
git commit -m "feat(theme): typography and spacing tokens"
```

---

### Task 4: PTY session wrapper (TDD)

**Files:**
- Create: `crates/argo-pty/src/session.rs`
- Modify: `crates/argo-pty/src/lib.rs`

- [ ] **Step 1: Write the failing test**

`crates/argo-pty/src/session.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn session_runs_a_command_and_captures_stdout() {
        let mut session = PtySession::spawn("/bin/sh", &["-c", "echo hello-from-argo"], 80, 24)
            .expect("spawn");

        let mut buf = Vec::new();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
        while tokio::time::Instant::now() < deadline {
            if let Some(chunk) = session.try_read() {
                buf.extend_from_slice(&chunk);
                if String::from_utf8_lossy(&buf).contains("hello-from-argo") {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        assert!(
            String::from_utf8_lossy(&buf).contains("hello-from-argo"),
            "stdout did not contain expected text: {:?}",
            String::from_utf8_lossy(&buf)
        );
    }

    #[tokio::test]
    async fn session_accepts_input_writes() {
        let mut session = PtySession::spawn("/bin/cat", &[], 80, 24).expect("spawn");
        session.write(b"echo-back\n").expect("write");

        let mut buf = Vec::new();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
        while tokio::time::Instant::now() < deadline {
            if let Some(chunk) = session.try_read() {
                buf.extend_from_slice(&chunk);
                if String::from_utf8_lossy(&buf).contains("echo-back") {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        assert!(String::from_utf8_lossy(&buf).contains("echo-back"));
        session.kill();
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p argo-pty`
Expected: FAIL — `PtySession` not defined.

- [ ] **Step 3: Implement `PtySession`**

Replace `crates/argo-pty/src/session.rs` (above tests stay):
```rust
use anyhow::{Context, Result};
use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use std::io::{Read, Write};
use std::sync::mpsc::{Receiver, channel};
use std::thread;

pub struct PtySession {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    reader_rx: Receiver<Vec<u8>>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
}

impl PtySession {
    pub fn spawn(program: &str, args: &[&str], cols: u16, rows: u16) -> Result<Self> {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })
            .context("openpty failed")?;

        let mut cmd = CommandBuilder::new(program);
        for a in args {
            cmd.arg(a);
        }
        let child = pair.slave.spawn_command(cmd).context("spawn failed")?;
        drop(pair.slave);

        let writer = pair.master.take_writer().context("take_writer failed")?;
        let mut reader = pair.master.try_clone_reader().context("clone_reader failed")?;

        let (tx, rx) = channel::<Vec<u8>>();
        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if tx.send(buf[..n].to_vec()).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self { master: pair.master, writer, reader_rx: rx, child })
    }

    pub fn try_read(&self) -> Option<Vec<u8>> {
        self.reader_rx.try_recv().ok()
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.writer.write_all(bytes).context("pty write")?;
        self.writer.flush().context("pty flush")?;
        Ok(())
    }

    pub fn resize(&mut self, cols: u16, rows: u16) -> Result<()> {
        self.master
            .resize(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })
            .context("pty resize")
    }

    pub fn kill(&mut self) {
        let _ = self.child.kill();
    }
}

impl Drop for PtySession {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
```

- [ ] **Step 4: Re-export from `lib.rs`**

`crates/argo-pty/src/lib.rs`:
```rust
mod session;
pub use session::PtySession;
```

- [ ] **Step 5: Run tests to verify pass**

Run: `cargo test -p argo-pty -- --test-threads=1`
Expected: 2 tests pass. (Single-threaded because each spawns a real process.)

- [ ] **Step 6: Commit**

```bash
git add crates/argo-pty
git commit -m "feat(pty): PtySession with spawn, read, write, resize"
```

---

### Task 5: Terminal state via alacritty_terminal (TDD)

**Files:**
- Create: `crates/argo-terminal/src/state.rs`
- Modify: `crates/argo-terminal/src/lib.rs`

- [ ] **Step 1: Write failing tests**

`crates/argo-terminal/src/state.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state_has_empty_grid() {
        let state = TerminalState::new(80, 24);
        let cells = state.visible_cells();
        // 24 rows, 80 cols
        assert_eq!(cells.len(), 24);
        assert_eq!(cells[0].len(), 80);
        // every cell is whitespace
        for row in cells.iter() {
            for cell in row.iter() {
                assert_eq!(cell.ch, ' ');
            }
        }
    }

    #[test]
    fn feeding_text_updates_cells() {
        let mut state = TerminalState::new(80, 24);
        state.feed(b"hello");
        let cells = state.visible_cells();
        assert_eq!(cells[0][0].ch, 'h');
        assert_eq!(cells[0][1].ch, 'e');
        assert_eq!(cells[0][2].ch, 'l');
        assert_eq!(cells[0][3].ch, 'l');
        assert_eq!(cells[0][4].ch, 'o');
    }

    #[test]
    fn newline_advances_cursor() {
        let mut state = TerminalState::new(80, 24);
        state.feed(b"a\r\nb");
        let cells = state.visible_cells();
        assert_eq!(cells[0][0].ch, 'a');
        assert_eq!(cells[1][0].ch, 'b');
    }

    #[test]
    fn resize_changes_grid_dims() {
        let mut state = TerminalState::new(80, 24);
        state.resize(120, 40);
        let cells = state.visible_cells();
        assert_eq!(cells.len(), 40);
        assert_eq!(cells[0].len(), 120);
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p argo-terminal`
Expected: FAIL — `TerminalState` not defined.

- [ ] **Step 3: Implement `TerminalState`**

Replace `crates/argo-terminal/src/state.rs` (tests stay at bottom):
```rust
use alacritty_terminal::Term;
use alacritty_terminal::event::{Event, EventListener};
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::{Column, Line, Point};
use alacritty_terminal::term::Config;
use alacritty_terminal::term::test::TermSize;
use alacritty_terminal::vte::ansi::Processor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ', fg: (0xe8, 0xe6, 0xe3), bg: (0x0a, 0x0a, 0x0c) }
    }
}

#[derive(Clone)]
struct NoopListener;

impl EventListener for NoopListener {
    fn send_event(&self, _event: Event) {}
}

pub struct TerminalState {
    term: Term<NoopListener>,
    parser: Processor,
    cols: u16,
    rows: u16,
}

impl TerminalState {
    pub fn new(cols: u16, rows: u16) -> Self {
        let size = TermSize::new(cols as usize, rows as usize);
        let term = Term::new(Config::default(), &size, NoopListener);
        Self { term, parser: Processor::new(), cols, rows }
    }

    pub fn feed(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.parser.advance(&mut self.term, *byte);
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        let size = TermSize::new(cols as usize, rows as usize);
        self.term.resize(size);
        self.cols = cols;
        self.rows = rows;
    }

    pub fn visible_cells(&self) -> Vec<Vec<Cell>> {
        let grid = self.term.grid();
        let mut out = Vec::with_capacity(self.rows as usize);
        for line in 0..self.rows as i32 {
            let mut row = Vec::with_capacity(self.cols as usize);
            for col in 0..self.cols as usize {
                let p = Point::new(Line(line), Column(col));
                let cell = &grid[p];
                let ch = cell.c;
                row.push(Cell { ch, ..Cell::default() });
            }
            out.push(row);
        }
        out
    }

    pub fn cursor_position(&self) -> (u16, u16) {
        let p = self.term.grid().cursor.point;
        (p.column.0 as u16, p.line.0 as u16)
    }

    pub fn dimensions(&self) -> (u16, u16) {
        (self.cols, self.rows)
    }
}
```

- [ ] **Step 4: Update `lib.rs`**

`crates/argo-terminal/src/lib.rs`:
```rust
mod state;
pub use state::{Cell, TerminalState};
```

- [ ] **Step 5: Run tests**

Run: `cargo test -p argo-terminal`
Expected: 4 tests pass.

If alacritty_terminal API differs from above: read the actual `alacritty_terminal` 0.24 docs at `https://docs.rs/alacritty_terminal/0.24` and adapt the imports/calls. The conceptual model (Term + Processor.advance + grid()[Point]) is stable; specific path names may shift.

- [ ] **Step 6: Commit**

```bash
git add crates/argo-terminal
git commit -m "feat(terminal): TerminalState wrapping alacritty_terminal for VT100 parsing"
```

---

### Task 6: Pipe PTY output into TerminalState (integration test)

**Files:**
- Create: `crates/argo-terminal/tests/integration.rs`

- [ ] **Step 1: Write failing integration test**

`crates/argo-terminal/tests/integration.rs`:
```rust
use argo_pty::PtySession;
use argo_terminal::TerminalState;
use std::time::Duration;

#[tokio::test]
async fn pty_output_renders_in_terminal_state() {
    let session = PtySession::spawn("/bin/sh", &["-c", "printf 'argo-rocks'"], 80, 24)
        .expect("spawn");
    let mut state = TerminalState::new(80, 24);

    let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
    while tokio::time::Instant::now() < deadline {
        if let Some(chunk) = session.try_read() {
            state.feed(&chunk);
            let cells = state.visible_cells();
            let first_row: String = cells[0].iter().take(10).map(|c| c.ch).collect();
            if first_row.starts_with("argo-rocks") {
                return;
            }
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    panic!("PTY output never rendered into terminal state");
}
```

Add `argo-pty` and `argo-terminal` as `dev-dependencies` of `argo-terminal`:
```toml
[dev-dependencies]
argo-pty = { path = "../argo-pty" }
tokio = { version = "1", features = ["full", "test-util"] }
```

- [ ] **Step 2: Run integration test**

Run: `cargo test -p argo-terminal --test integration -- --test-threads=1`
Expected: PASS — confirms PTY → VT parser → cell grid pipeline works end-to-end.

- [ ] **Step 3: Commit**

```bash
git add crates/argo-terminal
git commit -m "test(terminal): integration test for PTY → TerminalState pipeline"
```

---

### Task 7: GPUI app shell (smoke test)

**Files:**
- Create: `crates/argo-ui/src/app.rs`
- Modify: `crates/argo-ui/src/lib.rs`
- Modify: `crates/argo/src/main.rs`

- [ ] **Step 1: Implement minimal `ArgoApp` view**

`crates/argo-ui/src/app.rs`:
```rust
use argo_theme::ColorPalette;
use gpui::{
    App, AppContext, Bounds, Context, IntoElement, ParentElement, Pixels,
    Render, Styled, Window, WindowBounds, WindowOptions, div, point, px, rgb, size,
};

pub struct ArgoApp {
    palette: ColorPalette,
}

impl ArgoApp {
    pub fn new() -> Self {
        Self { palette: ColorPalette::dark() }
    }

    pub fn launch() {
        App::new().run(|cx: &mut AppContext| {
            let bounds = Bounds::new(point(px(200.), px(100.)), size(px(1280.), px(800.)));
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_window, cx| cx.new(|_| ArgoApp::new()),
            )
            .unwrap();
            cx.activate(true);
        });
    }

    fn bg_rgb(&self) -> u32 {
        let c = self.palette.bg;
        ((c.r as u32) << 16) | ((c.g as u32) << 8) | (c.b as u32)
    }
}

impl Render for ArgoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .bg(rgb(self.bg_rgb()))
    }
}
```

GPUI API caveat: signatures evolve. If `App::new()`, `cx.open_window`, or `Render::render` differ in the version we pull from `main`, consult the GPUI examples at `https://github.com/zed-industries/zed/tree/main/crates/gpui/examples` and adapt — the shape is: `App::new().run(closure)` → `open_window(opts, view_factory)` → `impl Render for View { fn render(&mut self, window, cx) -> impl IntoElement }`.

- [ ] **Step 2: Re-export and call from main**

`crates/argo-ui/src/lib.rs`:
```rust
mod app;
pub use app::ArgoApp;
```

`crates/argo/src/main.rs`:
```rust
use argo_ui::ArgoApp;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "argo=debug,info".into()),
        )
        .init();
    ArgoApp::launch();
}
```

- [ ] **Step 3: Smoke test**

Run: `cargo run -p argo`
Expected: A 1280×800 macOS window opens with a deep-black (`#0a0a0c`) background. No content yet, just confirming GPUI + window + theme color all wire up.

- [ ] **Step 4: Commit**

```bash
git add .
git commit -m "feat(ui): GPUI app shell renders dark window with locked palette"
```

---

### Task 8: Pane view that renders TerminalState

**Files:**
- Create: `crates/argo-ui/src/pane.rs`
- Modify: `crates/argo-ui/src/lib.rs`
- Modify: `crates/argo-ui/src/app.rs`

- [ ] **Step 1: Implement `Pane` view**

`crates/argo-ui/src/pane.rs`:
```rust
use argo_pty::PtySession;
use argo_terminal::TerminalState;
use argo_theme::{ColorPalette, Spacing, Typography};
use gpui::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    div, px, rgb, FontStyle, FontWeight,
};
use std::sync::{Arc, Mutex};

pub struct Pane {
    session: Arc<Mutex<PtySession>>,
    state: Arc<Mutex<TerminalState>>,
    palette: ColorPalette,
    typography: Typography,
}

impl Pane {
    pub fn spawn_shell(cols: u16, rows: u16) -> anyhow::Result<Self> {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".into());
        let session = PtySession::spawn(&shell, &["-l"], cols, rows)?;
        Ok(Self {
            session: Arc::new(Mutex::new(session)),
            state: Arc::new(Mutex::new(TerminalState::new(cols, rows))),
            palette: ColorPalette::dark(),
            typography: Typography::default_mono(),
        })
    }

    pub fn drain_pty_into_state(&self) {
        let session = self.session.lock().unwrap();
        let mut state = self.state.lock().unwrap();
        while let Some(chunk) = session.try_read() {
            state.feed(&chunk);
        }
    }

    fn bg_rgb(&self) -> u32 {
        let c = self.palette.bg;
        ((c.r as u32) << 16) | ((c.g as u32) << 8) | (c.b as u32)
    }

    fn fg_rgb(&self) -> u32 {
        let c = self.palette.fg;
        ((c.r as u32) << 16) | ((c.g as u32) << 8) | (c.b as u32)
    }
}

impl Render for Pane {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.drain_pty_into_state();
        let state = self.state.lock().unwrap();
        let cells = state.visible_cells();

        let mut rows = Vec::with_capacity(cells.len());
        for row in cells.iter() {
            let line: String = row.iter().map(|c| c.ch).collect();
            rows.push(
                div()
                    .text_color(rgb(self.fg_rgb()))
                    .font_family(self.typography.mono_family)
                    .text_size(px(self.typography.size_md))
                    .child(line),
            );
        }

        div()
            .w_full()
            .h_full()
            .bg(rgb(self.bg_rgb()))
            .p(px(Spacing::MD))
            .flex()
            .flex_col()
            .children(rows)
    }
}
```

- [ ] **Step 2: Wire `Pane` into `ArgoApp`**

`crates/argo-ui/src/app.rs`: replace the `ArgoApp` struct + render to host a single Pane:
```rust
use crate::pane::Pane;
use argo_theme::ColorPalette;
use gpui::{
    App, AppContext, Bounds, Context, Entity, IntoElement, ParentElement, Render,
    Styled, Window, WindowBounds, WindowOptions, div, point, px, rgb, size,
};

pub struct ArgoApp {
    palette: ColorPalette,
    pane: Entity<Pane>,
}

impl ArgoApp {
    pub fn launch() {
        App::new().run(|cx: &mut AppContext| {
            let bounds = Bounds::new(point(px(200.), px(100.)), size(px(1280.), px(800.)));
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_window, cx| {
                    let pane = cx.new(|_| Pane::spawn_shell(120, 36).expect("spawn shell"));
                    cx.new(|_| ArgoApp { palette: ColorPalette::dark(), pane })
                },
            )
            .unwrap();
            cx.activate(true);
        });
    }

    fn bg_rgb(&self) -> u32 {
        let c = self.palette.bg;
        ((c.r as u32) << 16) | ((c.g as u32) << 8) | (c.b as u32)
    }
}

impl Render for ArgoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().w_full().h_full().bg(rgb(self.bg_rgb())).child(self.pane.clone())
    }
}
```

- [ ] **Step 3: Update `lib.rs`**

`crates/argo-ui/src/lib.rs`:
```rust
mod app;
mod pane;

pub use app::ArgoApp;
pub use pane::Pane;
```

- [ ] **Step 4: Smoke test**

Run: `cargo run -p argo`
Expected: window opens, you see your shell prompt rendered (e.g., `~ %` or `$`). No keyboard input yet — that's Task 9.

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat(ui): Pane view renders PTY output via TerminalState"
```

---

### Task 9: Wire keyboard input from GPUI to PTY

**Files:**
- Modify: `crates/argo-ui/src/pane.rs`

- [ ] **Step 1: Add focus + key handler to `Pane`**

In `crates/argo-ui/src/pane.rs`, extend `Pane` to register a key listener:
```rust
use gpui::{FocusHandle, Focusable, KeyDownEvent, on_key_down};

pub struct Pane {
    session: Arc<Mutex<PtySession>>,
    state: Arc<Mutex<TerminalState>>,
    palette: ColorPalette,
    typography: Typography,
    focus_handle: FocusHandle,
}

impl Pane {
    pub fn spawn_shell(cx: &mut Context<Self>, cols: u16, rows: u16) -> anyhow::Result<Self> {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".into());
        let session = PtySession::spawn(&shell, &["-l"], cols, rows)?;
        Ok(Self {
            session: Arc::new(Mutex::new(session)),
            state: Arc::new(Mutex::new(TerminalState::new(cols, rows))),
            palette: ColorPalette::dark(),
            typography: Typography::default_mono(),
            focus_handle: cx.focus_handle(),
        })
    }

    fn handle_key(&self, event: &KeyDownEvent) {
        let bytes = key_event_to_bytes(event);
        if !bytes.is_empty() {
            let mut session = self.session.lock().unwrap();
            let _ = session.write(&bytes);
        }
    }
}

impl Focusable for Pane {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn key_event_to_bytes(event: &KeyDownEvent) -> Vec<u8> {
    let key = &event.keystroke.key;
    let mods = &event.keystroke.modifiers;

    if mods.control && key.len() == 1 {
        let c = key.chars().next().unwrap().to_ascii_uppercase();
        if ('A'..='Z').contains(&c) {
            return vec![(c as u8) - b'A' + 1];
        }
    }

    match key.as_str() {
        "enter" => return b"\r".to_vec(),
        "tab" => return b"\t".to_vec(),
        "backspace" => return b"\x7f".to_vec(),
        "escape" => return b"\x1b".to_vec(),
        "up" => return b"\x1b[A".to_vec(),
        "down" => return b"\x1b[B".to_vec(),
        "right" => return b"\x1b[C".to_vec(),
        "left" => return b"\x1b[D".to_vec(),
        "home" => return b"\x1b[H".to_vec(),
        "end" => return b"\x1b[F".to_vec(),
        _ => {}
    }

    if let Some(text) = &event.keystroke.ime_key {
        return text.as_bytes().to_vec();
    }
    if key.chars().count() == 1 {
        return key.as_bytes().to_vec();
    }
    Vec::new()
}
```

- [ ] **Step 2: Update render to attach `on_key_down` and request focus**

In `Pane::render`, wrap the outer div:
```rust
impl Render for Pane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.drain_pty_into_state();
        let state = self.state.lock().unwrap();
        let cells = state.visible_cells();

        let mut rows = Vec::with_capacity(cells.len());
        for row in cells.iter() {
            let line: String = row.iter().map(|c| c.ch).collect();
            rows.push(
                div()
                    .text_color(rgb(self.fg_rgb()))
                    .font_family(self.typography.mono_family)
                    .text_size(px(self.typography.size_md))
                    .child(line),
            );
        }

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, _cx| {
                this.handle_key(event);
            }))
            .w_full()
            .h_full()
            .bg(rgb(self.bg_rgb()))
            .p(px(Spacing::MD))
            .flex()
            .flex_col()
            .children(rows)
    }
}
```

- [ ] **Step 3: Update spawn callsite in `app.rs`**

The `ArgoApp` view factory must pass `cx` into `Pane::spawn_shell`:
```rust
let pane = cx.new(|cx| Pane::spawn_shell(cx, 120, 36).expect("spawn shell"));
```

Also focus the pane when the window opens — add to the launch closure after pane creation:
```rust
window.focus(&pane.read(cx).focus_handle);
```

If the GPUI version doesn't expose `track_focus` / `on_key_down` exactly as written, refer to the GPUI input examples at `crates/gpui/examples/input.rs` in the Zed repo and adapt.

- [ ] **Step 4: Smoke test**

Run: `cargo run -p argo`
Expected: window opens, your shell prompt appears, you can type `ls` then Enter and see output. Try `vim`, `htop`, `top` — all should work because we're just piping bytes through a real PTY.

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat(ui): keyboard input flows from GPUI through Pane to PTY"
```

---

### Task 10: Title bar component

**Files:**
- Create: `crates/argo-ui/src/titlebar.rs`
- Modify: `crates/argo-ui/src/lib.rs`
- Modify: `crates/argo-ui/src/pane.rs`

- [ ] **Step 1: Implement `PaneTitleBar`**

`crates/argo-ui/src/titlebar.rs`:
```rust
use argo_theme::{ColorPalette, Spacing, Typography};
use gpui::{
    IntoElement, ParentElement, RenderOnce, Styled, div, px, rgb,
};

#[derive(Clone)]
pub struct PaneTitleBar {
    pub provider: String,
    pub model: String,
    pub cwd: String,
    pub status: PaneStatus,
}

#[derive(Clone, Copy)]
pub enum PaneStatus {
    Ready,
    Running,
    RateLimited,
    Errored,
}

impl PaneStatus {
    fn dot_color(&self, p: &ColorPalette) -> u32 {
        let c = match self {
            Self::Ready => p.status_ok,
            Self::Running => p.accent,
            Self::RateLimited => p.status_warn,
            Self::Errored => p.status_err,
        };
        ((c.r as u32) << 16) | ((c.g as u32) << 8) | (c.b as u32)
    }
}

impl RenderOnce for PaneTitleBar {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        let palette = ColorPalette::dark();
        let typography = Typography::default_mono();
        let bg = ((palette.bg.r as u32) << 16) | ((palette.bg.g as u32) << 8) | palette.bg.b as u32;
        let fg = ((palette.fg.r as u32) << 16) | ((palette.fg.g as u32) << 8) | palette.fg.b as u32;
        let divider = ((palette.divider.r as u32) << 16)
            | ((palette.divider.g as u32) << 8)
            | palette.divider.b as u32;
        let status = self.status.dot_color(&palette);

        let label = format!("{} · {} · {}", self.provider, self.model, self.cwd);

        div()
            .w_full()
            .h(px(Spacing::TITLEBAR_HEIGHT))
            .bg(rgb(bg))
            .border_b_1()
            .border_color(rgb(divider))
            .px(px(Spacing::SM))
            .flex()
            .items_center()
            .gap(px(Spacing::SM))
            .child(
                div()
                    .w(px(6.))
                    .h(px(6.))
                    .rounded_full()
                    .bg(rgb(status)),
            )
            .child(
                div()
                    .text_color(rgb(fg))
                    .font_family(typography.mono_family)
                    .text_size(px(typography.size_sm))
                    .child(label),
            )
    }
}
```

- [ ] **Step 2: Use it in `Pane`**

Modify `Pane::render` to wrap the cells in a column with title bar on top:
```rust
use crate::titlebar::{PaneStatus, PaneTitleBar};

// inside render(), before the outer div:
let title = PaneTitleBar {
    provider: "shell".into(),
    model: std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".into()),
    cwd: std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_default(),
    status: PaneStatus::Ready,
};

div()
    .track_focus(&self.focus_handle)
    .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, _cx| {
        this.handle_key(event);
    }))
    .w_full()
    .h_full()
    .bg(rgb(self.bg_rgb()))
    .flex()
    .flex_col()
    .child(title)
    .child(
        div()
            .flex_1()
            .p(px(Spacing::MD))
            .flex()
            .flex_col()
            .children(rows),
    )
```

- [ ] **Step 3: Update `lib.rs`**

`crates/argo-ui/src/lib.rs`:
```rust
mod app;
mod pane;
mod titlebar;

pub use app::ArgoApp;
pub use pane::Pane;
pub use titlebar::{PaneStatus, PaneTitleBar};
```

- [ ] **Step 4: Smoke test**

Run: `cargo run -p argo`
Expected: window now shows a 22px title bar at top with green dot + `shell · /bin/zsh · /Users/.../argo` and the terminal below it.

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat(ui): pane title bar with status dot, provider, model, cwd"
```

---

### Task 11: Continuous PTY drain via timer

**Problem:** Currently `Pane` only drains the PTY when GPUI re-renders (e.g., on a keypress). Long-running output (`htop`, streaming logs) won't render until the user types. Fix: schedule a periodic redraw.

**Files:**
- Modify: `crates/argo-ui/src/pane.rs`

- [ ] **Step 1: Schedule redraws on construction**

In `Pane::spawn_shell`, after building `Self`, schedule a 16ms redraw loop:
```rust
pub fn spawn_shell(cx: &mut Context<Self>, cols: u16, rows: u16) -> anyhow::Result<Self> {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".into());
    let session = PtySession::spawn(&shell, &["-l"], cols, rows)?;
    let pane = Self {
        session: Arc::new(Mutex::new(session)),
        state: Arc::new(Mutex::new(TerminalState::new(cols, rows))),
        palette: ColorPalette::dark(),
        typography: Typography::default_mono(),
        focus_handle: cx.focus_handle(),
    };

    cx.spawn(|this, mut cx| async move {
        loop {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(16))
                .await;
            if this.update(&mut cx, |_, cx| cx.notify()).is_err() {
                break;
            }
        }
    })
    .detach();

    Ok(pane)
}
```

If GPUI's `cx.spawn` / `background_executor` shape differs in the version we're on, the conceptual op is "every 16ms call `cx.notify()` so render runs and drains the PTY." Adapt to whatever timer/scheduler GPUI exposes — see Zed's terminal view for reference.

- [ ] **Step 2: Smoke test**

Run: `cargo run -p argo`, then in the pane: `for i in 1 2 3 4 5; do echo "tick $i"; sleep 1; done`
Expected: lines appear once per second without you needing to type. Then run `htop` — the live UI updates smoothly.

- [ ] **Step 3: Commit**

```bash
git add crates/argo-ui/src/pane.rs
git commit -m "feat(ui): 60fps PTY drain via background timer"
```

---

### Task 12: Resize handling

**Files:**
- Modify: `crates/argo-ui/src/pane.rs`

- [ ] **Step 1: Compute cols/rows from rendered area and resize PTY+state**

In `Pane`, add tracked dimensions and a resize method:
```rust
pub struct Pane {
    // ...existing fields...
    last_cols: Mutex<u16>,
    last_rows: Mutex<u16>,
}

// Constants for character cell metrics. JetBrains Mono at 14px → ~8.4×19.6.
// We approximate; precise metrics can come from a measure pass later.
const CELL_W_PX: f32 = 8.4;
const CELL_H_PX: f32 = 19.6;

impl Pane {
    fn maybe_resize(&self, width_px: f32, height_px: f32) {
        let cols = ((width_px / CELL_W_PX).floor() as u16).max(20);
        let rows = ((height_px / CELL_H_PX).floor() as u16).max(5);
        let mut last_c = self.last_cols.lock().unwrap();
        let mut last_r = self.last_rows.lock().unwrap();
        if *last_c != cols || *last_r != rows {
            let mut session = self.session.lock().unwrap();
            let mut state = self.state.lock().unwrap();
            let _ = session.resize(cols, rows);
            state.resize(cols, rows);
            *last_c = cols;
            *last_r = rows;
        }
    }
}
```

Initialize `last_cols`/`last_rows` to the spawn dims in `spawn_shell`.

- [ ] **Step 2: Call `maybe_resize` from render**

GPUI exposes window size via `window.viewport_size()` (or similar — check current API). In `render`:
```rust
let viewport = window.viewport_size();
let avail_height = viewport.height.0 - Spacing::TITLEBAR_HEIGHT - 2.0 * Spacing::MD;
let avail_width = viewport.width.0 - 2.0 * Spacing::MD;
self.maybe_resize(avail_width, avail_height);
```

(If `viewport_size` is named differently — `bounds`, `window_bounds`, etc. — check the GPUI window API in current Zed source.)

- [ ] **Step 3: Smoke test**

Run: `cargo run -p argo`. Resize the window; run `tput cols && tput lines` in the pane.
Expected: values change as you resize.

- [ ] **Step 4: Commit**

```bash
git add crates/argo-ui/src/pane.rs
git commit -m "feat(ui): pane reflows PTY + state on window resize"
```

---

### Task 13: Cursor rendering

**Files:**
- Modify: `crates/argo-ui/src/pane.rs`

- [ ] **Step 1: Render cursor as inverted cell**

Modify the row-building loop in `Pane::render` to highlight the cursor cell. Get cursor position from state:
```rust
let (cursor_col, cursor_row) = state.cursor_position();

let mut rows = Vec::with_capacity(cells.len());
for (row_idx, row) in cells.iter().enumerate() {
    let mut row_children = Vec::with_capacity(row.len());
    for (col_idx, cell) in row.iter().enumerate() {
        let is_cursor = row_idx as u16 == cursor_row && col_idx as u16 == cursor_col;
        let glyph = if cell.ch == ' ' { ' ' } else { cell.ch };
        let mut span = div()
            .text_color(rgb(self.fg_rgb()))
            .font_family(self.typography.mono_family)
            .text_size(px(self.typography.size_md))
            .child(glyph.to_string());
        if is_cursor {
            span = span.bg(rgb(self.fg_rgb())).text_color(rgb(self.bg_rgb()));
        }
        row_children.push(span);
    }
    rows.push(div().flex().children(row_children));
}
```

This is per-cell rendering — heavier but correct. (We can optimize to runs of same-style cells in a later pass.)

- [ ] **Step 2: Smoke test**

Run: `cargo run -p argo`
Expected: a solid block cursor renders at the prompt position. Type — cursor advances. Run `vim`, see cursor move with arrow keys.

- [ ] **Step 3: Commit**

```bash
git add crates/argo-ui/src/pane.rs
git commit -m "feat(ui): render cursor as inverted cell"
```

---

### Task 14: macOS app bundle config

**Files:**
- Create: `crates/argo/build.rs` (optional — keep simple for v0.1)
- Create: `Info.plist` template
- Modify: `Cargo.toml` (add `cargo-bundle` config)

For v0.1 we ship via `cargo run` for development. Real .app/DMG bundling lands in a later phase.

- [ ] **Step 1: Add bundle metadata to root `Cargo.toml`**

```toml
[workspace.metadata.bundle]
name = "Argo"
identifier = "dev.argo.app"
icon = ["assets/icon.icns"]
version = "0.1.0"
copyright = "Argo contributors"
category = "DeveloperTool"
short_description = "Every AI agent on one ship."
long_description = """
Argo is a native multi-agent terminal that auto-detects every AI coding agent
authenticated on your machine.
"""
```

- [ ] **Step 2: Add a placeholder icon**

Create `assets/icon.icns` — for v0.1, copy any 512×512 PNG and convert with macOS `iconutil`. Or stub with an empty file; bundle will fall back to default.

- [ ] **Step 3: Document run instructions in repo `README.md`**

```markdown
# Argo

Every AI agent on one ship.

## Run (dev)

```bash
cargo run -p argo
```

## Build

```bash
cargo build --release -p argo
```

Phase 1 ships a single dark-themed PTY pane. Multi-pane, providers, auth detection, relay, and YOLO arrive in subsequent phases.
```

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml assets/ README.md
git commit -m "chore: bundle metadata + run instructions"
```

---

### Task 15: Phase 1 acceptance test (manual)

**Files:** none

Run through the manual acceptance checklist:

- [ ] `cargo build --release` succeeds with no warnings (clippy-clean)
- [ ] `cargo test --workspace -- --test-threads=1` — all tests pass
- [ ] `cargo run -p argo` opens a 1280×800 dark window
- [ ] Title bar shows: green dot, `shell · /bin/zsh · <cwd>`
- [ ] Type `ls -la`, hit Enter — directory listing renders correctly
- [ ] Type `vim test.txt` — vim opens, you can edit, `:q!` exits cleanly
- [ ] Type `htop` (if installed) — live UI animates smoothly
- [ ] Resize the window — terminal reflows, `tput cols` reflects new width
- [ ] Background is `#0a0a0c` (verify with screenshot color picker), text is `#e8e6e3`
- [ ] No emoji, no gradients, no idle animations visible
- [ ] Window closes via ⌘Q without hanging processes (verify with `ps`)

If any of these fail, file an issue against the failing task and fix before declaring Phase 1 done.

---

## Self-Review Notes

**Spec coverage check:**
- ✅ Native Rust + GPUI shell (Tasks 1, 7)
- ✅ Locked aesthetic (Tasks 2, 3 — and applied throughout)
- ✅ Real native PTYs per pane (Tasks 4, 5, 6)
- ✅ Pane chrome with title bar (Task 10)
- ⏭️ Multi-pane grid → Phase 2
- ⏭️ Auth detection → Phase 3
- ⏭️ Provider adapters (Claude/Gemini/Ollama) → Phase 4
- ⏭️ Relay → Phase 5
- ⏭️ YOLO mode → Phase 6
- ⏭️ Workspace presets, command palette → Phase 2
- ⏭️ MCP support → v0.2

**Risks acknowledged:**
- GPUI is fast-moving and pulled from `main`; APIs may shift between when this plan is written and when it's executed. Each task using GPUI has a fallback note pointing engineers at the current Zed source.
- Cell-per-div rendering in Task 13 is correct but not fast at large grid sizes. Acceptable for Phase 1; optimize in Phase 2 (run-length grouping).
- Character metrics in Task 12 are approximated. A measure-pass against the loaded font would be more correct; deferred for v0.1.

---

## Plan Complete

Plan saved to `docs/superpowers/plans/2026-05-01-argo-v01-phase1-foundation.md`.

Two execution options:

**1. Subagent-Driven (recommended)** — I dispatch a fresh subagent per task, review between tasks, fast iteration.

**2. Inline Execution** — Execute tasks in this session using executing-plans, batch execution with checkpoints.

Which approach?
