//! Template compilation - converting segments to Rust code.

mod control_expr;
mod expr_segments;
mod ident_block;
mod segment_dispatch;
mod stmt_control;
mod stmt_segments;
mod ts_injection;

pub use control_expr::compile_control_expr;
pub use expr_segments::compile_expr_segments;
pub use ident_block::compile_ident_block;
pub use stmt_control::compile_stmt_control;
pub use stmt_segments::compile_stmt_segments;
pub use ts_injection::compile_ts_injection;
