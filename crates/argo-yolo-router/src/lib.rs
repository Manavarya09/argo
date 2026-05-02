//! Argo YOLO router — prompt classifier and provider routing table.
//!
//! YOLO mode (`?` prefix) routes a single prompt to the best provider
//! based on what kind of task it looks like. The default classifier
//! is a fast keyword heuristic; in production a small model
//! (Haiku / Gemini Flash) replaces it.
//!
//! Super-YOLO (`??`) layers task decomposition on top — that lives in
//! a separate module added in a later commit.

mod classifier;
mod routing;
mod task;

pub use classifier::{Classifier, HeuristicClassifier};
pub use routing::RoutingTable;
pub use task::TaskType;
