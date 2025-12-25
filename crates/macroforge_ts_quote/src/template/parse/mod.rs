//! Template parsing - converts token streams to Rust code that builds TypeScript AST.

mod template;

pub use template::parse_template;
pub use template::parse_template_body;
