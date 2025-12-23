//! Diagnostics module - multi-tool diagnostic aggregation

pub mod aggregator;
pub mod biome;
pub mod clippy;
pub mod runner;
pub mod svelte;
pub mod tsc;

pub use aggregator::{DiagnosticLevel, DiagnosticTool, UnifiedDiagnostic};
pub use runner::DiagnosticsRunner;
