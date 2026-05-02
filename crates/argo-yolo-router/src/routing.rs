use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use crate::task::TaskType;

/// User-configurable mapping from task type to provider+model identifier.
///
/// Loaded from TOML, e.g.:
/// ```toml
/// [routes]
/// planning = "gemini-2.5-pro"
/// coding   = "claude-sonnet-4.6"
/// review   = "codex-gpt-5"
/// ```
#[derive(Debug, Clone)]
pub struct RoutingTable {
    routes: HashMap<TaskType, String>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    pub fn defaults() -> Self {
        let mut t = Self::new();
        t.set(TaskType::Planning, "gemini-2.5-pro");
        t.set(TaskType::Coding, "claude-sonnet-4.6");
        t.set(TaskType::Review, "codex-gpt-5");
        t.set(TaskType::Debug, "claude-sonnet-4.6");
        t.set(TaskType::Docs, "gemini-2.5-flash");
        t.set(TaskType::Research, "perplexity");
        t.set(TaskType::Creative, "claude-opus-4.7");
        t.set(TaskType::Chat, "ollama:llama3");
        t
    }

    pub fn set(&mut self, task: TaskType, provider: impl Into<String>) {
        self.routes.insert(task, provider.into());
    }

    pub fn route(&self, task: TaskType) -> Option<&str> {
        self.routes.get(&task).map(|s| s.as_str())
    }

    pub fn from_toml(input: &str) -> Result<Self> {
        let raw: TomlConfig = toml::from_str(input).context("parsing routing TOML")?;
        let mut table = Self::new();
        for (k, v) in raw.routes {
            let task = task_from_key(&k)
                .ok_or_else(|| anyhow!("unknown task type in routing config: {}", k))?;
            table.set(task, v);
        }
        Ok(table)
    }
}

impl Default for RoutingTable {
    fn default() -> Self {
        Self::defaults()
    }
}

#[derive(Deserialize)]
struct TomlConfig {
    routes: HashMap<String, String>,
}

fn task_from_key(key: &str) -> Option<TaskType> {
    Some(match key {
        "planning" => TaskType::Planning,
        "coding" => TaskType::Coding,
        "review" => TaskType::Review,
        "debug" => TaskType::Debug,
        "docs" => TaskType::Docs,
        "research" => TaskType::Research,
        "creative" => TaskType::Creative,
        "chat" => TaskType::Chat,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_cover_all_task_types() {
        let t = RoutingTable::defaults();
        for task in [
            TaskType::Planning,
            TaskType::Coding,
            TaskType::Review,
            TaskType::Debug,
            TaskType::Docs,
            TaskType::Research,
            TaskType::Creative,
            TaskType::Chat,
        ] {
            assert!(t.route(task).is_some(), "missing default route for {task:?}");
        }
    }

    #[test]
    fn from_toml_parses_a_user_config() {
        let toml = r#"
            [routes]
            planning = "gemini-2.5-pro"
            coding = "claude-sonnet-4.6"
            review = "codex-gpt-5"
        "#;
        let t = RoutingTable::from_toml(toml).unwrap();
        assert_eq!(t.route(TaskType::Planning), Some("gemini-2.5-pro"));
        assert_eq!(t.route(TaskType::Coding), Some("claude-sonnet-4.6"));
        assert_eq!(t.route(TaskType::Review), Some("codex-gpt-5"));
    }

    #[test]
    fn from_toml_rejects_unknown_task_keys() {
        let toml = r#"
            [routes]
            invented_task = "anything"
        "#;
        let err = RoutingTable::from_toml(toml).unwrap_err();
        assert!(err.to_string().contains("invented_task"));
    }
}
