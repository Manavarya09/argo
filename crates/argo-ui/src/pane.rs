use crate::titlebar::{PaneStatus, PaneTitleBar};
use argo_pty::PtySession;
use argo_terminal::TerminalState;
use argo_theme::{ColorPalette, Spacing, Typography};
use gpui::{
    App, Context, FocusHandle, Focusable, IntoElement, KeyDownEvent, Window, div, prelude::*, px,
    rgb,
};
use std::sync::{Arc, Mutex};

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
        let pane = Self {
            session: Arc::new(Mutex::new(session)),
            state: Arc::new(Mutex::new(TerminalState::new(cols, rows))),
            palette: ColorPalette::dark(),
            typography: Typography::default_mono(),
            focus_handle: cx.focus_handle(),
        };

        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(16))
                    .await;
                if this.update(cx, |_, cx| cx.notify()).is_err() {
                    break;
                }
            }
        })
        .detach();

        Ok(pane)
    }

    pub fn drain_pty_into_state(&self) {
        let session = self.session.lock().unwrap();
        let mut state = self.state.lock().unwrap();
        while let Some(chunk) = session.try_read() {
            state.feed(&chunk);
        }
    }

    fn handle_key(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let bytes = key_event_to_bytes(event);
        if !bytes.is_empty() {
            let mut session = self.session.lock().unwrap();
            let _ = session.write(&bytes);
            cx.notify();
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

impl Focusable for Pane {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Pane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.drain_pty_into_state();
        let cells = {
            let state = self.state.lock().unwrap();
            state.visible_cells()
        };

        let fg = self.fg_rgb();
        let bg = self.bg_rgb();
        let mono_family = self.typography.mono_family;
        let size_md = self.typography.size_md;

        let mut rows = Vec::with_capacity(cells.len());
        for row in cells.iter() {
            let line: String = row.iter().map(|c| c.ch).collect();
            rows.push(
                div()
                    .text_color(rgb(fg))
                    .font_family(mono_family)
                    .text_size(px(size_md))
                    .child(line),
            );
        }

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
            .on_key_down(cx.listener(Self::handle_key))
            .w_full()
            .h_full()
            .bg(rgb(bg))
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

    if let Some(text) = &event.keystroke.key_char {
        return text.as_bytes().to_vec();
    }
    if key.chars().count() == 1 {
        return key.as_bytes().to_vec();
    }
    Vec::new()
}
