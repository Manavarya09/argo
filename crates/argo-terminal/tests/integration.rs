use argo_pty::PtySession;
use argo_terminal::TerminalState;
use std::time::Duration;

#[tokio::test]
async fn pty_output_renders_in_terminal_state() {
    let session = PtySession::spawn("/bin/sh", &["-c", "printf 'argo-rocks'"], 80, 24)
        .expect("spawn");
    let mut state = TerminalState::new(80, 24);

    let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
    while tokio::time::Instant::now() < deadline {
        if let Some(chunk) = session.try_read() {
            state.feed(&chunk);
            let cells = state.visible_cells();
            let first_row: String = cells[0].iter().take(10).map(|c| c.ch).collect();
            if first_row.starts_with("argo-rocks") {
                return;
            }
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    panic!("PTY output never rendered into terminal state");
}
