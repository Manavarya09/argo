use argo_pty::PtySession;
use argo_terminal::TerminalState;
use argo_theme::{ColorPalette, Spacing, Typography};
use gpui::{Context, IntoElement, Window, div, prelude::*, px, rgb};
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

        div()
            .w_full()
            .h_full()
            .bg(rgb(bg))
            .p(px(Spacing::MD))
            .flex()
            .flex_col()
            .children(rows)
    }
}
