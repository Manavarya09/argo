use std::io::{self, Stdout};
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;

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
    loop {
        terminal.draw(|frame| {
            let block = Block::default().title(" Argo ").borders(Borders::ALL);
            frame.render_widget(block, frame.area());
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if should_quit(key) {
                    return Ok(());
                }
            }
        }
    }
}
