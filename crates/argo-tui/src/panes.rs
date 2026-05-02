use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::theme;

#[derive(Debug, Clone)]
pub struct AgentPane {
    pub provider: &'static str,
    pub model: &'static str,
    pub cwd: String,
    pub log: Vec<String>,
    pub focused: bool,
}

impl AgentPane {
    pub fn new(provider: &'static str, model: &'static str) -> Self {
        let cwd = std::env::current_dir()
            .map(|p| {
                let s = p.display().to_string();
                let home = dirs::home_dir().map(|h| h.display().to_string()).unwrap_or_default();
                if !home.is_empty() && s.starts_with(&home) {
                    format!("~{}", &s[home.len()..])
                } else {
                    s
                }
            })
            .unwrap_or_else(|_| ".".into());
        Self {
            provider,
            model,
            cwd,
            log: Vec::new(),
            focused: false,
        }
    }

    pub fn append(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
        // Cap scrollback at 200 lines per pane.
        if self.log.len() > 200 {
            let drop = self.log.len() - 200;
            self.log.drain(0..drop);
        }
    }
}

pub fn default_grid() -> Vec<AgentPane> {
    vec![
        AgentPane::new("claude", "sonnet-4.6"),
        AgentPane::new("gemini", "2.5-pro"),
        AgentPane::new("codex", "gpt-5"),
        AgentPane::new("ollama", "qwen-coder"),
    ]
}

pub fn render_grid(frame: &mut Frame<'_>, area: Rect, panes: &[AgentPane]) {
    let cols = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cols[0]);

    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cols[1]);

    let cells = [top_row[0], top_row[1], bottom_row[0], bottom_row[1]];

    for (i, pane) in panes.iter().enumerate().take(4) {
        render_pane(frame, cells[i], pane);
    }
}

fn render_pane(frame: &mut Frame<'_>, area: Rect, pane: &AgentPane) {
    let title_color = if pane.focused {
        theme::ACCENT
    } else {
        theme::FG
    };
    let border_color = if pane.focused {
        theme::ACCENT
    } else {
        theme::DIVIDER
    };

    let dot_color = theme::STATUS_OK;

    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("●", Style::default().fg(dot_color)),
        Span::raw("  "),
        Span::styled(
            pane.provider,
            Style::default()
                .fg(title_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" · "),
        Span::styled(pane.model, Style::default().fg(theme::DIVIDER)),
        Span::raw(" · "),
        Span::styled(pane.cwd.clone(), Style::default().fg(theme::DIVIDER)),
        Span::raw(" "),
    ]);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Show the most recent N lines that fit.
    let visible = inner.height as usize;
    let start = pane.log.len().saturating_sub(visible);
    let lines: Vec<Line> = pane.log[start..]
        .iter()
        .map(|s| {
            Line::from(Span::styled(s.clone(), Style::default().fg(theme::FG)))
        })
        .collect();
    let body = Paragraph::new(lines)
        .style(Style::default().bg(theme::BG))
        .wrap(Wrap { trim: false });
    frame.render_widget(body, inner);
}
