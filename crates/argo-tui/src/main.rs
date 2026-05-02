use std::io::{self, Stdout};
use std::sync::atomic::AtomicUsize;
use std::time::{Duration, Instant};

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

mod auth;
mod input;
mod mock;
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
    let rows = auth::detect_all();
    let mut grid = panes::default_grid();
    grid[0].focused = true;
    let mut input = input::InputField::default();
    let mut status = statusbar::StatusBar { agents: grid.len(), estimated_spend_usd: 0.0 };

    // One counter per pane to drive its mock script.
    let counters: Vec<AtomicUsize> = grid.iter().map(|_| AtomicUsize::new(0)).collect();
    // Stagger start times so panes feel alive but not in lockstep.
    let stagger_ms = [200u64, 350, 500, 650];
    let start = Instant::now();
    let mut last_ticks: Vec<Instant> = grid.iter().map(|_| start).collect();

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let bg = Block::default().style(Style::default().bg(theme::BG).fg(theme::FG));
            frame.render_widget(bg, area);

            let outer = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(22), Constraint::Min(0)])
                .split(area);

            sidebar::render_sidebar(frame, outer[0], &rows);

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

        // Drive mock streaming: each pane produces a new line every ~stagger ms.
        let now = Instant::now();
        for (i, pane) in grid.iter_mut().enumerate() {
            let interval = Duration::from_millis(stagger_ms[i % stagger_ms.len()]);
            if now.duration_since(last_ticks[i]) >= interval {
                if let Some(line) = mock::next_line(pane.provider, &counters[i]) {
                    pane.append(line);
                    last_ticks[i] = now;
                }
            }
        }

        if event::poll(Duration::from_millis(60))? {
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
