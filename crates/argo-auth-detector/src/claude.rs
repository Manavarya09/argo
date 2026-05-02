use std::path::{Path, PathBuf};

use crate::status::AuthStatus;

/// Detects Claude Code authentication state by inspecting `~/.claude/`.
pub struct ClaudeDetector;

impl ClaudeDetector {
    /// Detect using the user's real home directory.
    pub fn detect() -> AuthStatus {
        match dirs::home_dir() {
            Some(home) => Self::detect_in(&home),
            None => AuthStatus::NotInstalled,
        }
    }

    /// Detect against an explicit home root. Used by tests.
    pub fn detect_in(home: &Path) -> AuthStatus {
        let claude_dir = home.join(".claude");
        if !claude_dir.is_dir() {
            return AuthStatus::NotInstalled;
        }

        // Claude Code stores creds in either credentials.json or auth.json
        // depending on version. Treat either as authenticated if non-empty.
        let candidates: [PathBuf; 2] = [
            claude_dir.join("credentials.json"),
            claude_dir.join("auth.json"),
        ];

        for path in &candidates {
            if let Ok(meta) = std::fs::metadata(path) {
                if meta.is_file() && meta.len() > 0 {
                    return AuthStatus::Authenticated;
                }
            }
        }

        AuthStatus::NotAuthenticated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn no_claude_dir_means_not_installed() {
        let tmp = tempdir().unwrap();
        assert_eq!(ClaudeDetector::detect_in(tmp.path()), AuthStatus::NotInstalled);
    }

    #[test]
    fn empty_claude_dir_means_not_authenticated() {
        let tmp = tempdir().unwrap();
        fs::create_dir(tmp.path().join(".claude")).unwrap();
        assert_eq!(
            ClaudeDetector::detect_in(tmp.path()),
            AuthStatus::NotAuthenticated
        );
    }

    #[test]
    fn empty_credentials_file_is_not_authenticated() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join(".claude");
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("credentials.json"), b"").unwrap();
        assert_eq!(
            ClaudeDetector::detect_in(tmp.path()),
            AuthStatus::NotAuthenticated
        );
    }

    #[test]
    fn nonempty_credentials_file_is_authenticated() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join(".claude");
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("credentials.json"), b"{\"token\":\"abc\"}").unwrap();
        assert_eq!(
            ClaudeDetector::detect_in(tmp.path()),
            AuthStatus::Authenticated
        );
    }

    #[test]
    fn nonempty_auth_json_is_authenticated() {
        let tmp = tempdir().unwrap();
        let dir = tmp.path().join(".claude");
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("auth.json"), b"{\"token\":\"xyz\"}").unwrap();
        assert_eq!(
            ClaudeDetector::detect_in(tmp.path()),
            AuthStatus::Authenticated
        );
    }
}
