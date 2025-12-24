//! Template compilation - converting segments to Rust code.

mod segment_dispatch;
mod stmt_segments;
mod ts_injection;

pub use stmt_segments::compile_stmt_segments;
pub use ts_injection::compile_ts_injection;
