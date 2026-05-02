use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::theme;

#[derive(Debug, Clone, Copy)]
pub enum AuthStatus {
    Authenticated,
    Running,
    NotInstalled,
    NotAuthenticated,
}

impl AuthStatus {
    fn dot(&self) -> Span<'static> {
        let (ch, color) = match self {
            Self::Authenticated => ("●", theme::STATUS_OK),
            Self::Running => ("●", theme::STATUS_OK),
            Self::NotInstalled => ("○", theme::DIVIDER),
            Self::NotAuthenticated => ("○", theme::STATUS_WARN),
        };
        Span::styled(ch, Style::default().fg(color))
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Authenticated => "ready",
            Self::Running => "running",
            Self::NotInstalled => "—",
            Self::NotAuthenticated => "sign in",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProviderRow {
    pub name: &'static str,
    pub status: AuthStatus,
}

pub fn default_rows() -> Vec<ProviderRow> {
    vec![
        ProviderRow { name: "Claude Code", status: AuthStatus::Authenticated },
        ProviderRow { name: "Gemini CLI", status: AuthStatus::Authenticated },
        ProviderRow { name: "Codex", status: AuthStatus::NotInstalled },
        ProviderRow { name: "Ollama", status: AuthStatus::Running },
        ProviderRow { name: "Copilot", status: AuthStatus::NotInstalled },
    ]
}

pub fn render_sidebar(frame: &mut Frame<'_>, area: Rect, rows: &[ProviderRow]) {
    let block = Block::default()
        .title(" providers ")
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(theme::DIVIDER))
        .title_style(
            Style::default()
                .fg(theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        );
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines: Vec<Line> = rows
        .iter()
        .map(|row| {
            Line::from(vec![
                Span::raw(" "),
                row.status.dot(),
                Span::raw("  "),
                Span::styled(row.name, Style::default().fg(theme::FG)),
                Span::raw("  "),
                Span::styled(
                    row.status.label(),
                    Style::default().fg(theme::DIVIDER),
                ),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .style(Style::default().bg(theme::BG).fg(theme::FG));
    frame.render_widget(paragraph, inner);
}
