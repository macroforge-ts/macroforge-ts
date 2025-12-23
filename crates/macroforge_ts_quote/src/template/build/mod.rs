//! Template building - constructing template strings and bindings.

mod comment_expr;
mod placeholder_source;
mod string_interp_expr;
mod template_and_bindings;
mod template_interp_expr;

pub use comment_expr::build_comment_expr;
pub use placeholder_source::{build_placeholder_source, PlaceholderSourceKind};
pub use string_interp_expr::build_string_interp_expr;
pub use template_and_bindings::build_template_and_bindings;
pub use template_interp_expr::build_template_interp_expr;
