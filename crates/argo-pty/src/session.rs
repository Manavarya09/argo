use anyhow::{Context, Result};
use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use std::io::{Read, Write};
use std::sync::mpsc::{Receiver, channel};
use std::thread;

pub struct PtySession {
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    reader_rx: Receiver<Vec<u8>>,
    child: Box<dyn portable_pty::Child + Send + Sync>,
}

impl PtySession {
    pub fn spawn(program: &str, args: &[&str], cols: u16, rows: u16) -> Result<Self> {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })
            .context("openpty failed")?;

        let mut cmd = CommandBuilder::new(program);
        for a in args {
            cmd.arg(a);
        }
        let child = pair.slave.spawn_command(cmd).context("spawn failed")?;
        drop(pair.slave);

        let writer = pair.master.take_writer().context("take_writer failed")?;
        let mut reader = pair.master.try_clone_reader().context("clone_reader failed")?;

        let (tx, rx) = channel::<Vec<u8>>();
        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if tx.send(buf[..n].to_vec()).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self { master: pair.master, writer, reader_rx: rx, child })
    }

    pub fn try_read(&self) -> Option<Vec<u8>> {
        self.reader_rx.try_recv().ok()
    }

    pub fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.writer.write_all(bytes).context("pty write")?;
        self.writer.flush().context("pty flush")?;
        Ok(())
    }

    pub fn resize(&mut self, cols: u16, rows: u16) -> Result<()> {
        self.master
            .resize(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })
            .context("pty resize")
    }

    pub fn kill(&mut self) {
        let _ = self.child.kill();
    }
}

impl Drop for PtySession {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn session_runs_a_command_and_captures_stdout() {
        let session = PtySession::spawn("/bin/sh", &["-c", "echo hello-from-argo"], 80, 24)
            .expect("spawn");

        let mut buf = Vec::new();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
        while tokio::time::Instant::now() < deadline {
            if let Some(chunk) = session.try_read() {
                buf.extend_from_slice(&chunk);
                if String::from_utf8_lossy(&buf).contains("hello-from-argo") {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        assert!(
            String::from_utf8_lossy(&buf).contains("hello-from-argo"),
            "stdout did not contain expected text: {:?}",
            String::from_utf8_lossy(&buf)
        );
    }

    #[tokio::test]
    async fn session_accepts_input_writes() {
        let mut session = PtySession::spawn("/bin/cat", &[], 80, 24).expect("spawn");
        session.write(b"echo-back\n").expect("write");

        let mut buf = Vec::new();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
        while tokio::time::Instant::now() < deadline {
            if let Some(chunk) = session.try_read() {
                buf.extend_from_slice(&chunk);
                if String::from_utf8_lossy(&buf).contains("echo-back") {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        assert!(String::from_utf8_lossy(&buf).contains("echo-back"));
        session.kill();
    }
}
