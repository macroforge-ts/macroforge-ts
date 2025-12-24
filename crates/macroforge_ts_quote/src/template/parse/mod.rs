//! Template parsing - tokenizing and segment extraction.

mod backtick_template;
mod doc_attributes;
mod ident_block_parts;
mod if_chain;
mod if_let_chain;
mod match_arms;
mod segments;
mod string_interpolation;
mod template;
mod while_let_loop;
mod while_loop;

pub use backtick_template::parse_backtick_template;
pub use doc_attributes::parse_doc_attribute;
pub use ident_block_parts::parse_ident_block_parts;
pub use if_chain::parse_if_chain;
pub use if_let_chain::parse_if_let_chain;
pub use match_arms::parse_match_arms;
pub use segments::parse_segments;
pub use string_interpolation::parse_string_interpolation;
pub use template::parse_template;
pub use while_let_loop::parse_while_let_loop;
pub use while_loop::parse_while_loop;
