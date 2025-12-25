//! Compiler integration for template processing.
//!
//! This module provides the main entry point for compiling templates using
//! the Rowan-based compiler pipeline.
//!
//! ## Main Entry Point
//!
//! Use [`compile_template`] to compile a template string directly:
//!
//! ```ignore
//! let code = compile_template("const x = @{expr};", "__stmts")?;
//! ```

use super::codegen::{Codegen, CodegenConfig};
use super::ir::lower;
use super::parser::Parser;
use super::semantic::analyze;
use super::syntax::SyntaxNode;
use proc_macro2::TokenStream;
use quote::quote;

/// Compiles a template string into Rust code that builds TypeScript AST.
///
/// The generated code pushes `ModuleItem` nodes into a `Vec` named by `output_var`.
pub fn compile_template(template: &str, output_var: &str) -> syn::Result<TokenStream> {
    if template.trim().is_empty() {
        return Ok(quote! {});
    }

    // Step 1: Parse with Rowan parser
    let parser = Parser::new(template);
    let green = parser.parse();
    let root = SyntaxNode::new_root(green);

    // Step 2: Semantic analysis (classify placeholders)
    let analysis = analyze(&root);

    // Step 3: Lower to IR
    let ir = lower(&root, analysis);

    // Step 4: Generate code
    let config = CodegenConfig {
        output_var: output_var.to_string(),
    };

    let code = Codegen::with_config(config).generate(&ir);

    Ok(code)
}
