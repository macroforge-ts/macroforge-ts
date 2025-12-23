//! Template flushing - converting buffered content to output.

mod class_wrapped_path;
mod helpers;
mod r#static;
mod standard_path;
mod stmt_run;
mod type_placeholder_path;

pub use r#static::flush_static;
pub use stmt_run::flush_stmt_run;
