use std::io::{self, Stdout};
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::Terminal;

mod input;
mod panes;
mod sidebar;
mod statusbar;
mod theme;

type Tui = Terminal<CrosstermBackend<Stdout>>;

fn setup() -> Result<Tui> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, crossterm::cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn teardown(mut terminal: Tui) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;
    terminal.show_cursor().ok();
    Ok(())
}

fn should_quit(key: KeyEvent) -> bool {
    matches!(key.code, KeyCode::Char('q'))
        || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
}

fn main() -> Result<()> {
    let mut terminal = setup()?;
    let result = run(&mut terminal);
    teardown(terminal)?;
    result
}

fn run(terminal: &mut Tui) -> Result<()> {
    let rows = sidebar::default_rows();
    let mut grid = panes::default_grid();
    grid[0].focused = true;
    let mut input = input::InputField::default();
    let mut status = statusbar::StatusBar { agents: grid.len(), estimated_spend_usd: 0.0 };

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let bg = Block::default().style(Style::default().bg(theme::BG).fg(theme::FG));
            frame.render_widget(bg, area);

            // Outer split: sidebar | (panes + input + status)
            let outer = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(22), Constraint::Min(0)])
                .split(area);

            sidebar::render_sidebar(frame, outer[0], &rows);

            // Right column: panes (rest), input row (1), status row (1)
            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .split(outer[1]);

            panes::render_grid(frame, right[0], &grid);
            input.render(frame, right[1]);
            status.render(frame, right[2]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if should_quit(key) {
                    return Ok(());
                }
                match key.code {
                    KeyCode::Tab => {
                        let len = grid.len();
                        let focused_idx = grid.iter().position(|p| p.focused).unwrap_or(0);
                        grid[focused_idx].focused = false;
                        grid[(focused_idx + 1) % len].focused = true;
                    }
                    KeyCode::Enter => {
                        let prompt = input.clear();
                        if !prompt.is_empty() {
                            let idx = grid.iter().position(|p| p.focused).unwrap_or(0);
                            grid[idx].append(format!("> {}", prompt));
                            // Mock cost increment: every prompt adds a few cents.
                            status.estimated_spend_usd += 0.04;
                        }
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                        input.push(c);
                    }
                    _ => {}
                }
            }
        }
    }
}
