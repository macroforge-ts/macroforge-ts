//! Template flushing - converting buffered content to output.

mod r#static;
mod stmt_run;

pub use r#static::flush_static;
pub use stmt_run::flush_stmt_run;
