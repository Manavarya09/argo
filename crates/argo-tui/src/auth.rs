use std::path::Path;
use std::time::Duration;

use crate::sidebar::{AuthStatus, ProviderRow};

const OLLAMA_URL: &str = "http://127.0.0.1:11434/api/tags";

pub fn detect_all() -> Vec<ProviderRow> {
    let home = dirs::home_dir().unwrap_or_else(|| Path::new(".").to_path_buf());
    vec![
        ProviderRow { name: "Claude Code", status: detect_claude(&home) },
        ProviderRow { name: "Gemini CLI", status: detect_gemini(&home) },
        ProviderRow { name: "Codex", status: detect_codex(&home) },
        ProviderRow { name: "Ollama", status: detect_ollama_blocking() },
        ProviderRow { name: "Copilot", status: detect_copilot(&home) },
    ]
}

fn detect_claude(home: &Path) -> AuthStatus {
    let dir = home.join(".claude");
    if !dir.is_dir() {
        return AuthStatus::NotInstalled;
    }
    if has_nonempty_file(&dir.join("credentials.json"))
        || has_nonempty_file(&dir.join("auth.json"))
    {
        AuthStatus::Authenticated
    } else {
        AuthStatus::NotAuthenticated
    }
}

fn detect_gemini(home: &Path) -> AuthStatus {
    let gemini_dir = home.join(".gemini");
    let gcloud_dir = home.join(".config").join("google-cloud-sdk");
    if !gemini_dir.is_dir() && !gcloud_dir.is_dir() {
        return AuthStatus::NotInstalled;
    }
    let gemini_creds = gemini_dir.join("credentials.json");
    let gcloud_adc = home
        .join(".config")
        .join("gcloud")
        .join("application_default_credentials.json");
    if has_nonempty_file(&gemini_creds) || has_nonempty_file(&gcloud_adc) {
        AuthStatus::Authenticated
    } else {
        AuthStatus::NotAuthenticated
    }
}

fn detect_codex(home: &Path) -> AuthStatus {
    if std::env::var("OPENAI_API_KEY").is_ok() {
        return AuthStatus::Authenticated;
    }
    let codex_dir = home.join(".codex");
    if !codex_dir.is_dir() {
        return AuthStatus::NotInstalled;
    }
    if has_nonempty_file(&codex_dir.join("auth.json")) {
        AuthStatus::Authenticated
    } else {
        AuthStatus::NotAuthenticated
    }
}

fn detect_copilot(home: &Path) -> AuthStatus {
    let dir = home.join(".config").join("github-copilot");
    if !dir.is_dir() {
        return AuthStatus::NotInstalled;
    }
    if has_nonempty_file(&dir.join("hosts.json"))
        || has_nonempty_file(&dir.join("apps.json"))
    {
        AuthStatus::Authenticated
    } else {
        AuthStatus::NotAuthenticated
    }
}

fn detect_ollama_blocking() -> AuthStatus {
    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(800))
        .build()
    {
        Ok(c) => c,
        Err(_) => return AuthStatus::NotInstalled,
    };
    match client.get(OLLAMA_URL).send() {
        Ok(resp) if resp.status().is_success() => AuthStatus::Running,
        Ok(_) => AuthStatus::NotAuthenticated,
        Err(_) => AuthStatus::NotInstalled,
    }
}

fn has_nonempty_file(p: &Path) -> bool {
    matches!(std::fs::metadata(p), Ok(m) if m.is_file() && m.len() > 0)
}
