use serde::{Deserialize, Serialize};

/// What kind of work the user's prompt is asking for.
///
/// Routing config maps each variant to a provider+model. The
/// classifier maps a free-text prompt to a variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskType {
    Planning,
    Coding,
    Review,
    Debug,
    Docs,
    Research,
    Creative,
    Chat,
}

impl TaskType {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Planning => "planning",
            Self::Coding => "coding",
            Self::Review => "review",
            Self::Debug => "debug",
            Self::Docs => "docs",
            Self::Research => "research",
            Self::Creative => "creative",
            Self::Chat => "chat",
        }
    }
}
