use crate::task::TaskType;

/// Classifies a free-text prompt into a [`TaskType`].
pub trait Classifier {
    fn classify(&self, prompt: &str) -> TaskType;
}

/// Keyword-heuristic classifier. Cheap, no LLM call, deterministic.
/// Used as a fallback when the user has not configured a remote
/// classifier model, and as the always-on default for routing.
pub struct HeuristicClassifier;

impl Classifier for HeuristicClassifier {
    fn classify(&self, prompt: &str) -> TaskType {
        let p = prompt.to_lowercase();

        // Order matters: more specific intents win.
        if any(&p, &["plan ", "design ", "architect", "spec ", "roadmap", "rfc"]) {
            return TaskType::Planning;
        }
        if any(&p, &["review", "audit ", "feedback on", "critique"]) {
            return TaskType::Review;
        }
        if any(&p, &["debug", "fix ", "bug", "error", "stack trace", "panic", "regression"]) {
            return TaskType::Debug;
        }
        if any(&p, &["doc ", "docs ", "documentation", "readme", "explain "]) {
            return TaskType::Docs;
        }
        if any(&p, &["research ", "find papers", "compare ", "evaluate "]) {
            return TaskType::Research;
        }
        if any(&p, &["write a story", "poem", "creative", "brainstorm "]) {
            return TaskType::Creative;
        }
        if any(&p, &["implement", "refactor", "add ", "build ", "code ", "function", "class ", "feature"]) {
            return TaskType::Coding;
        }
        TaskType::Chat
    }
}

fn any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| haystack.contains(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classify(s: &str) -> TaskType {
        HeuristicClassifier.classify(s)
    }

    #[test]
    fn planning_words_route_to_planning() {
        assert_eq!(classify("plan the migration"), TaskType::Planning);
        assert_eq!(classify("design the auth flow"), TaskType::Planning);
        assert_eq!(classify("write the spec for OAuth"), TaskType::Planning);
    }

    #[test]
    fn review_words_route_to_review() {
        assert_eq!(classify("review this PR"), TaskType::Review);
        assert_eq!(classify("audit the code"), TaskType::Review);
    }

    #[test]
    fn debug_words_route_to_debug() {
        assert_eq!(classify("fix the null pointer bug"), TaskType::Debug);
        assert_eq!(classify("debug this stack trace"), TaskType::Debug);
    }

    #[test]
    fn docs_words_route_to_docs() {
        assert_eq!(classify("write the readme"), TaskType::Docs);
        assert_eq!(classify("explain how this works"), TaskType::Docs);
    }

    #[test]
    fn research_words_route_to_research() {
        assert_eq!(classify("compare Postgres vs SQLite"), TaskType::Research);
    }

    #[test]
    fn creative_words_route_to_creative() {
        assert_eq!(classify("write a poem about Rust"), TaskType::Creative);
    }

    #[test]
    fn coding_words_route_to_coding() {
        assert_eq!(classify("implement OAuth login"), TaskType::Coding);
        assert_eq!(classify("refactor the api module"), TaskType::Coding);
        assert_eq!(classify("add a function for json parsing"), TaskType::Coding);
    }

    #[test]
    fn unknown_falls_back_to_chat() {
        assert_eq!(classify("hello there"), TaskType::Chat);
        assert_eq!(classify(""), TaskType::Chat);
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(classify("PLAN the rollout"), TaskType::Planning);
        assert_eq!(classify("REVIEW THIS"), TaskType::Review);
    }
}
