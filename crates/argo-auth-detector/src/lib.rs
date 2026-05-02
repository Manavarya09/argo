//! Argo auth detector — scans local credentials for known AI providers.
//!
//! Detectors run independently per provider and return a uniform
//! [`AuthStatus`]. They never read or transmit the credential contents
//! themselves; they only check existence and non-emptiness of known
//! credential files. The actual SDKs each provider ships are responsible
//! for using their own credentials.

mod claude;
mod ollama;
mod status;

pub use claude::ClaudeDetector;
pub use ollama::OllamaDetector;
pub use status::AuthStatus;
