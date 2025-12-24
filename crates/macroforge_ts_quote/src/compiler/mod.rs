//! Compiler infrastructure for the template language.
//!
//! This module provides a proper compiler architecture with:
//! - Lexer: Tokenizes input into a token stream
//! - Parser: Builds a Rowan-based CST (Concrete Syntax Tree)
//! - Semantic Analysis: Classifies placeholders (expr, type, ident, etc.)
//! - IR: Intermediate representation for code generation
//! - Codegen: Generates Rust TokenStream output

mod syntax;
mod lexer;
mod parser;
mod semantic;
mod ir;
mod codegen;
mod integration;

pub use integration::compile_segments_to_swc_ast;
