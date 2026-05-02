use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::theme;

#[derive(Debug, Clone)]
pub struct StatusBar {
    pub agents: usize,
    pub estimated_spend_usd: f32,
}

impl StatusBar {
    pub fn render(&self, frame: &mut Frame<'_>, area: Rect) {
        let line = Line::from(vec![
            Span::styled(
                format!(" {} agents ", self.agents),
                Style::default().fg(theme::ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::styled("·", Style::default().fg(theme::DIVIDER)),
            Span::styled(
                format!(" est ${:.2} ", self.estimated_spend_usd),
                Style::default().fg(theme::FG),
            ),
            Span::styled("·", Style::default().fg(theme::DIVIDER)),
            Span::styled(
                " ⌃N new pane ",
                Style::default().fg(theme::FG),
            ),
            Span::styled("·", Style::default().fg(theme::DIVIDER)),
            Span::styled(" Tab focus ", Style::default().fg(theme::FG)),
            Span::styled("·", Style::default().fg(theme::DIVIDER)),
            Span::styled(" q quit ", Style::default().fg(theme::FG)),
        ]);
        let p = Paragraph::new(line).style(Style::default().bg(theme::BG));
        frame.render_widget(p, area);
    }
}
