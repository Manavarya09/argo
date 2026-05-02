use std::time::Duration;

use crate::status::AuthStatus;

/// Detects whether Ollama is running locally by probing its HTTP API.
///
/// Ollama listens on `127.0.0.1:11434` by default and exposes
/// `/api/tags` which returns 200 OK with a JSON list when alive.
pub struct OllamaDetector;

const DEFAULT_URL: &str = "http://127.0.0.1:11434/api/tags";
const TIMEOUT: Duration = Duration::from_millis(800);

impl OllamaDetector {
    /// Probe the default URL.
    pub async fn detect() -> AuthStatus {
        Self::detect_url(DEFAULT_URL).await
    }

    /// Probe an explicit URL. Used by tests.
    pub async fn detect_url(url: &str) -> AuthStatus {
        let client = match reqwest::Client::builder().timeout(TIMEOUT).build() {
            Ok(c) => c,
            Err(_) => return AuthStatus::NotInstalled,
        };
        match client.get(url).send().await {
            Ok(resp) if resp.status().is_success() => AuthStatus::Authenticated,
            Ok(_) => AuthStatus::NotAuthenticated,
            Err(_) => AuthStatus::NotInstalled,
        }
    }

    /// Synchronous wrapper for callers that don't have an async runtime
    /// already running. Spins up a single-threaded tokio runtime.
    pub fn detect_blocking() -> AuthStatus {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(_) => return AuthStatus::NotInstalled,
        };
        rt.block_on(Self::detect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unreachable_url_means_not_installed() {
        let status = OllamaDetector::detect_url("http://127.0.0.1:1/api/tags").await;
        assert_eq!(status, AuthStatus::NotInstalled);
    }

    #[tokio::test]
    async fn invalid_url_scheme_means_not_installed() {
        let status = OllamaDetector::detect_url("not-a-url").await;
        assert_eq!(status, AuthStatus::NotInstalled);
    }
}
