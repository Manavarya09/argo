use crate::titlebar::{PaneStatus, PaneTitleBar};
use argo_pty::PtySession;
use argo_terminal::TerminalState;
use argo_theme::{ColorPalette, Spacing, Typography};
use gpui::{
    App, Context, FocusHandle, Focusable, IntoElement, KeyDownEvent, Window, div, prelude::*, px,
    rgb,
};
use std::sync::{Arc, Mutex};

const CELL_W_PX: f32 = 8.4;
const CELL_H_PX: f32 = 19.6;

pub struct Pane {
    session: Arc<Mutex<PtySession>>,
    state: Arc<Mutex<TerminalState>>,
    palette: ColorPalette,
    typography: Typography,
    focus_handle: FocusHandle,
    last_cols: Mutex<u16>,
    last_rows: Mutex<u16>,
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
            last_cols: Mutex::new(cols),
            last_rows: Mutex::new(rows),
        };

        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(33))
                    .await;
                let drained = match this.update(cx, |this, _| this.drain_pty_into_state()) {
                    Ok(v) => v,
                    Err(_) => break,
                };
                if drained {
                    if this.update(cx, |_, cx| cx.notify()).is_err() {
                        break;
                    }
                }
            }
        })
        .detach();

        Ok(pane)
    }

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

    pub fn drain_pty_into_state(&self) -> bool {
        let session = self.session.lock().unwrap();
        let mut state = self.state.lock().unwrap();
        let mut got_any = false;
        while let Some(chunk) = session.try_read() {
            state.feed(&chunk);
            got_any = true;
        }
        got_any
    }

    fn handle_key(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        tracing::debug!(
            key = %event.keystroke.key,
            ctrl = event.keystroke.modifiers.control,
            shift = event.keystroke.modifiers.shift,
            alt = event.keystroke.modifiers.alt,
            cmd = event.keystroke.modifiers.platform,
            "pane key event"
        );
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport = window.viewport_size();
        let avail_width = f32::from(viewport.width) - 2.0 * Spacing::MD;
        let avail_height =
            f32::from(viewport.height) - Spacing::TITLEBAR_HEIGHT - 2.0 * Spacing::MD;
        self.maybe_resize(avail_width, avail_height);

        let cells;
        let cursor_pos;
        {
            let state = self.state.lock().unwrap();
            cells = state.visible_cells();
            cursor_pos = state.cursor_position();
        }
        let (cursor_col, cursor_row) = cursor_pos;

        let fg = self.fg_rgb();
        let bg = self.bg_rgb();
        let mono_family = self.typography.mono_family;
        let size_md = self.typography.size_md;

        let mut rows = Vec::with_capacity(cells.len());
        for (row_idx, row) in cells.iter().enumerate() {
            let row_has_cursor =
                row_idx as u16 == cursor_row && (cursor_col as usize) < row.len();

            if row_has_cursor {
                let cur = cursor_col as usize;
                let before: String = row[..cur].iter().map(|c| c.ch).collect();
                let cursor_ch = row[cur].ch;
                let cursor_str = if cursor_ch == ' ' || cursor_ch == '\0' {
                    " ".to_string()
                } else {
                    cursor_ch.to_string()
                };
                let after: String = row[cur + 1..].iter().map(|c| c.ch).collect();

                let line = div()
                    .flex()
                    .flex_row()
                    .child(
                        div()
                            .text_color(rgb(fg))
                            .font_family(mono_family)
                            .text_size(px(size_md))
                            .child(before),
                    )
                    .child(
                        div()
                            .bg(rgb(fg))
                            .text_color(rgb(bg))
                            .font_family(mono_family)
                            .text_size(px(size_md))
                            .child(cursor_str),
                    )
                    .child(
                        div()
                            .text_color(rgb(fg))
                            .font_family(mono_family)
                            .text_size(px(size_md))
                            .child(after),
                    );
                rows.push(line);
            } else {
                let line_str: String = row.iter().map(|c| c.ch).collect();
                let line = div().flex().flex_row().child(
                    div()
                        .text_color(rgb(fg))
                        .font_family(mono_family)
                        .text_size(px(size_md))
                        .child(line_str),
                );
                rows.push(line);
            }
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
    keystroke_to_bytes(
        &event.keystroke.key,
        event.keystroke.modifiers.control,
        event.keystroke.key_char.as_deref(),
    )
}

/// Pure mapping from keystroke fields to PTY bytes. Testable without
/// constructing a full GPUI `KeyDownEvent`.
fn keystroke_to_bytes(key: &str, ctrl: bool, key_char: Option<&str>) -> Vec<u8> {
    if ctrl && key.len() == 1 {
        let c = key.chars().next().unwrap().to_ascii_uppercase();
        if ('A'..='Z').contains(&c) {
            return vec![(c as u8) - b'A' + 1];
        }
    }

    match key {
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

    if let Some(text) = key_char {
        return text.as_bytes().to_vec();
    }
    if key.chars().count() == 1 {
        return key.as_bytes().to_vec();
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::keystroke_to_bytes;

    #[test]
    fn backspace_sends_del() {
        assert_eq!(keystroke_to_bytes("backspace", false, None), b"\x7f");
    }

    #[test]
    fn enter_sends_cr() {
        assert_eq!(keystroke_to_bytes("enter", false, None), b"\r");
    }

    #[test]
    fn ctrl_c_sends_etx() {
        assert_eq!(keystroke_to_bytes("c", true, Some("c")), b"\x03");
    }

    #[test]
    fn ctrl_d_sends_eot() {
        assert_eq!(keystroke_to_bytes("d", true, Some("d")), b"\x04");
    }

    #[test]
    fn ctrl_l_sends_ff() {
        assert_eq!(keystroke_to_bytes("l", true, Some("l")), b"\x0c");
    }

    #[test]
    fn arrow_up_sends_csi_a() {
        assert_eq!(keystroke_to_bytes("up", false, None), b"\x1b[A");
    }

    #[test]
    fn tab_sends_ht() {
        assert_eq!(keystroke_to_bytes("tab", false, None), b"\t");
    }

    #[test]
    fn plain_ascii_a_sends_a() {
        assert_eq!(keystroke_to_bytes("a", false, Some("a")), b"a");
    }
}
