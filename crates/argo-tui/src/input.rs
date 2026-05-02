use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::theme;

/// One-line prompt input field.
#[derive(Debug, Default, Clone)]
pub struct InputField {
    buffer: String,
}

impl InputField {
    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    pub fn push(&mut self, ch: char) {
        self.buffer.push(ch);
    }

    pub fn pop(&mut self) {
        self.buffer.pop();
    }

    pub fn clear(&mut self) -> String {
        std::mem::take(&mut self.buffer)
    }

    pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
        let prompt_style = Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD);
        let body_style = Style::default().fg(theme::FG);
        let cursor_style = Style::default().bg(theme::FG).fg(theme::BG);

        let line = Line::from(vec![
            Span::styled(" > ", prompt_style),
            Span::styled(self.buffer.clone(), body_style),
            Span::styled(" ", cursor_style),
        ]);
        let p = Paragraph::new(line).style(Style::default().bg(theme::BG));
        frame.render_widget(p, area);
    }
}
