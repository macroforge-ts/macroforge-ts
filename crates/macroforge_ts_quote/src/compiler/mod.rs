//! Compiler infrastructure for the template language.
//!
//! This module provides a proper compiler architecture with:
//! - Lexer: Tokenizes input into a token stream
//! - Parser: Builds a Rowan-based CST (Concrete Syntax Tree)
//! - Semantic Analysis: Classifies placeholders (expr, type, ident, etc.)
//! - IR: Intermediate representation for code generation
//! - Codegen: Generates Rust TokenStream output
//!
//! ## Usage
//!
//! The primary entry point is [`compile_template`]:
//!
//! ```ignore
//! let code = compile_template("const x = @{expr};", "__stmts")?;
//! ```

mod codegen;
mod integration;
mod ir;
mod lexer;
mod parser;
mod semantic;
mod syntax;
#[cfg(test)]
mod tests;

// Primary API
pub use integration::compile_template;
