//! Compiler infrastructure for the template language.
//!
//! This module provides a clean compiler architecture:
//! - Lexer: Tokenizes input into a token stream
//! - Parser: Parses tokens directly into an AST with inline placeholder classification
//! - Codegen: Generates Rust TokenStream output from AST
//!
//! ## Architecture
//!
//! ```text
//! Template string → Lexer → Tokens → Parser → AST → Codegen → Rust TokenStream
//! ```
//!
//! ## Usage
//!
//! The primary entry point is [`compile_template`]:
//!
//! ```ignore
//! let code = compile_template("const x = @{expr};", "__stmts")?;
//! ```

mod codegen;
mod ir;
mod lexer;
mod parser;
mod syntax;
#[cfg(test)]
mod tests;

use codegen::{Codegen, CodegenConfig};
use parser::Parser;
use proc_macro2::TokenStream;
use quote::quote;

/// Compiles a template string into Rust code that builds TypeScript AST.
///
/// The generated code pushes `ModuleItem` nodes into a `Vec` named by `output_var`.
pub fn compile_template(template: &str, output_var: &str) -> syn::Result<TokenStream> {
    if template.trim().is_empty() {
        return Ok(quote! {});
    }

    // Step 1: Parse directly to IR with inline placeholder classification
    let ir = Parser::new(template).parse();

    // Step 2: Generate code
    let config = CodegenConfig {
        output_var: output_var.to_string(),
    };

    let code = Codegen::with_config(config).generate(&ir);

    Ok(code)
}
