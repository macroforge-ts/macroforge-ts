//! A `syn`-like parsing API for TypeScript using SWC.
//!
//! This module provides a [`TsStream`] type and [`ParseTs`] trait that abstract
//! over SWC's parser, making it feel more like Rust's `syn` crate. This is the
//! low-level parsing API; most macro authors will prefer using
//! [`DeriveInput`](crate::DeriveInput) from the [`derive`](crate::derive) module.
//!
//! ## Overview
//!
//! - [`TsStream`] - A parsing stream wrapping SWC's parser
//! - [`ParseTs`] - A trait for types that can be parsed from TypeScript source
//! - [`parse_ts_str`] - Parse a string into any [`ParseTs`] type
//! - [`parse_ts_expr`], [`parse_ts_stmt`], [`parse_ts_module`] - Convenience functions
//! - [`format_ts_source`] - Format TypeScript code using SWC's emitter
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use macroforge_ts_syn::{TsStream, parse_ts_str, TsSynError};
//! use swc_core::ecma::ast::{Expr, Stmt, Module};
//!
//! fn main() -> Result<(), TsSynError> {
//!     // Parse individual constructs
//!     let expr: Box<Expr> = parse_ts_str("x + y")?;
//!     let stmt: Stmt = parse_ts_str("const x = 5;")?;
//!     let module: Module = parse_ts_str("export class Foo {}")?;
//!
//!     // Or use TsStream directly
//!     let mut stream = TsStream::new("const x = 5;", "input.ts")?;
//!     let stmt = stream.parse_stmt()?;
//!     Ok(())
//! }
//! ```
//!
//! ## Macro Context
//!
//! When used within the macro system, [`TsStream`] carries context information
//! about the macro invocation:
//!
//! ```rust,no_run
//! use macroforge_ts_syn::TsStream;
//!
//! fn example(stream: TsStream) {
//!     // In a macro implementation
//!     let ctx = stream.context().expect("macro context");
//!     let decorator_span = ctx.decorator_span;
//!     let _target = &ctx.target;
//! }
//! ```
//!
//! ## Adding Imports
//!
//! [`TsStream`] provides helpers for adding imports that will be inserted
//! at the top of the file:
//!
//! ```rust,no_run
//! use macroforge_ts_syn::{TsStream, TsSynError};
//!
//! fn main() -> Result<(), TsSynError> {
//!     let source = "class Foo {}";
//!     let mut stream = TsStream::new(source, "input.ts")?;
//!
//!     // Add a runtime import
//!     stream.add_import("deserialize", "./runtime");
//!
//!     // Add a type-only import
//!     stream.add_type_import("Options", "./types");
//!
//!     // Convert to result
//!     let _result = stream.into_result();
//!     Ok(())
//! }
//! ```
//!
//! ## ParseTs Implementations
//!
//! The following SWC types implement [`ParseTs`]:
//!
//! - `Ident` - Single identifier
//! - `Stmt` - Any statement
//! - `Box<Expr>` - Any expression
//! - `Module` - Complete module
//!
//! Custom types can implement [`ParseTs`] to enable parsing with [`TsStream::parse`].

#[cfg(feature = "swc")]
use swc_core::common::{FileName, SourceMap, sync::Lrc};
#[cfg(feature = "swc")]
use swc_core::ecma::ast::*;
#[cfg(feature = "swc")]
use swc_core::ecma::codegen::{Config, Emitter, text_writer::JsWriter};
#[cfg(feature = "swc")]
use swc_core::ecma::parser::{PResult, Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

use crate::TsSynError;

/// A parsing stream that wraps SWC's parser, analogous to `syn::parse::ParseBuffer`.
///
/// `TsStream` manages the parsing context for TypeScript source code. It holds:
/// - The source code being parsed
/// - An optional macro context with span information
/// - Accumulated runtime patches (imports, etc.)
///
/// # Creating a TsStream
///
/// ```rust
/// use macroforge_ts_syn::{TsStream, TsSynError};
///
/// fn main() -> Result<(), TsSynError> {
///     // From source code
///     let _stream = TsStream::new("const x = 5;", "input.ts")?;
///
///     // From an owned string
///     let generated_code = "const y = 10;".to_string();
///     let _stream = TsStream::from_string(generated_code);
///     Ok(())
/// }
/// ```
///
/// With macro context (typically used by the host system):
///
/// ```rust,ignore
/// use macroforge_ts_syn::{TsStream, TsSynError, MacroContextIR, SpanIR, ClassIR};
///
/// // MacroContextIR requires a target type - this is typically provided by the host
/// let source = "class Foo {}";
/// let ctx = MacroContextIR::new_derive_class(/* ... */);
/// let stream = TsStream::with_context(source, "input.ts", ctx)?;
/// ```
///
/// # Parsing
///
/// ```rust,no_run
/// use macroforge_ts_syn::{TsStream, TsSynError};
///
/// fn main() -> Result<(), TsSynError> {
///     let mut stream = TsStream::new("const x = 5;", "input.ts")?;
///     // Parse specific constructs
///     let _stmt = stream.parse_stmt()?;
///     Ok(())
/// }
/// ```
///
/// # Working with Imports
///
/// ```rust,no_run
/// use macroforge_ts_syn::{TsStream, TsSynError};
///
/// fn main() -> Result<(), TsSynError> {
///     let source = "class Foo {}";
///     let mut stream = TsStream::new(source, "file.ts")?;
///
///     // These imports will be added to the file
///     stream.add_import("deserialize", "./runtime");
///     stream.add_type_import("Options", "./types");
///
///     // Get the result with accumulated patches
///     let _result = stream.into_result();
///     Ok(())
/// }
/// ```
#[cfg(feature = "swc")]
pub struct TsStream {
    source_map: Lrc<SourceMap>,
    source: String,
    file_name: String,
    /// Macro context data (decorator span, target span, etc.)
    /// This is populated when TsStream is created by the macro host
    pub ctx: Option<crate::abi::MacroContextIR>,
    /// Runtime patches to apply (e.g., imports at file level)
    pub runtime_patches: Vec<crate::abi::Patch>,
}

/// Formats TypeScript source code using SWC's emitter.
///
/// Attempts to parse and re-emit the code with consistent formatting. This is
/// useful for normalizing generated code output.
///
/// # Parsing Strategy
///
/// The function tries two parsing approaches:
/// 1. Parse as a complete module (for full file content)
/// 2. If that fails, wrap in a class and extract (for class members)
///
/// # Example
///
/// ```rust
/// use macroforge_ts_syn::format_ts_source;
///
/// let messy = "function foo(){return 42}";
/// let formatted = format_ts_source(messy);
/// assert!(formatted.contains("function foo()"));
/// assert!(formatted.contains("return 42"));
/// ```
///
/// # Fallback
///
/// If parsing fails entirely, returns the original source unchanged.
#[cfg(feature = "swc")]
pub fn format_ts_source(source: &str) -> String {
    let cm = Lrc::new(SourceMap::default());

    // 1. Try parsing as a valid module (e.g. full file content or statements)
    let fm = cm.new_source_file(FileName::Custom("fmt.ts".into()).into(), source.to_string());
    let syntax = Syntax::Typescript(TsSyntax {
        tsx: true,
        decorators: true,
        ..Default::default()
    });

    let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);

    if let Ok(module) = parser.parse_module() {
        let mut buf = vec![];
        {
            let mut emitter = Emitter {
                cfg: Config::default().with_minify(false),
                cm: cm.clone(),
                comments: None,
                wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
            };

            if emitter.emit_module(&module).is_ok() {
                return String::from_utf8(buf).unwrap_or_else(|_| source.to_string());
            }
        }
    }

    // 2. If failed, try wrapping in a class (common for macro output generating methods/fields)
    let wrapped_source = format!("class __FmtWrapper {{ {} }}", source);
    let fm_wrapped = cm.new_source_file(
        FileName::Custom("fmt_wrapped.ts".into()).into(),
        wrapped_source,
    );

    let lexer = Lexer::new(
        syntax,
        EsVersion::latest(),
        StringInput::from(&*fm_wrapped),
        None,
    );
    let mut parser = Parser::new_from(lexer);

    if let Ok(module) = parser.parse_module() {
        let mut buf = vec![];
        {
            let mut emitter = Emitter {
                cfg: Config::default().with_minify(false),
                cm: cm.clone(),
                comments: None,
                wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
            };

            if emitter.emit_module(&module).is_ok() {
                let full_output = String::from_utf8(buf).unwrap_or_default();
                // Extract content between class braces
                // Output format: class __FmtWrapper {\n    content...\n}
                if let (Some(start), Some(end)) = (full_output.find('{'), full_output.rfind('}')) {
                    let content = &full_output[start + 1..end];
                    // Simple unindent: remove first newline and 4 spaces of indentation if present
                    let lines: Vec<&str> = content.lines().collect();
                    let mut result = String::new();
                    for line in lines {
                        // Naive unindent
                        let trimmed = line.strip_prefix("    ").unwrap_or(line);
                        if !trimmed.trim().is_empty() {
                            result.push_str(trimmed);
                            result.push('\n');
                        }
                    }
                    return result.trim().to_string();
                }
            }
        }
    }
    source.to_string()
}

#[cfg(feature = "swc")]
impl TsStream {
    /// Create a new parsing stream from source code.
    pub fn new(source: &str, file_name: &str) -> Result<Self, TsSynError> {
        Ok(TsStream {
            source_map: Lrc::new(Default::default()),
            source: source.to_string(),
            file_name: file_name.to_string(),
            ctx: None,
            runtime_patches: vec![],
        })
    }

    /// Create a new parsing stream from an owned string.
    pub fn from_string(source: String) -> Self {
        TsStream {
            source_map: Lrc::new(Default::default()),
            source,
            file_name: "macro_output.ts".to_string(),
            ctx: None,
            runtime_patches: vec![],
        }
    }

    /// Get the source code of the stream.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Create a new parsing stream with macro context attached.
    /// This is used by the macro host to provide context to macros.
    pub fn with_context(
        source: &str,
        file_name: &str,
        ctx: crate::abi::MacroContextIR,
    ) -> Result<Self, TsSynError> {
        Ok(TsStream {
            source_map: Lrc::new(Default::default()),
            source: source.to_string(),
            file_name: file_name.to_string(),
            ctx: Some(ctx),
            runtime_patches: vec![],
        })
    }

    /// Get the macro context if available
    pub fn context(&self) -> Option<&crate::abi::MacroContextIR> {
        self.ctx.as_ref()
    }

    /// Convert the stream into a MacroResult
    pub fn into_result(self) -> crate::abi::MacroResult {
        crate::abi::MacroResult {
            runtime_patches: self.runtime_patches,
            type_patches: vec![],
            diagnostics: vec![],
            tokens: Some(self.source),
            debug: None,
        }
    }

    /// Add an import statement to be inserted at the top of the file.
    /// The import will be deduplicated if it already exists.
    pub fn add_import(&mut self, specifier: &str, module: &str) {
        use crate::abi::{Patch, SpanIR};
        let import_code = format!("import {{ {specifier} }} from \"{module}\";\n");
        self.runtime_patches.push(Patch::InsertRaw {
            at: SpanIR::new(1, 1), // Position 1 = start of file (1-indexed)
            code: import_code,
            context: Some("import".to_string()),
            source_macro: Some("Deserialize".to_string()),
        });
    }

    /// Add a type-only import statement to be inserted at the top of the file.
    /// Use this for TypeScript types/interfaces that don't exist at runtime.
    pub fn add_type_import(&mut self, specifier: &str, module: &str) {
        use crate::abi::{Patch, SpanIR};
        let import_code = format!("import type {{ {specifier} }} from \"{module}\";\n");
        self.runtime_patches.push(Patch::InsertRaw {
            at: SpanIR::new(1, 1), // Position 1 = start of file (1-indexed)
            code: import_code,
            context: Some("import".to_string()),
            source_macro: Some("Deserialize".to_string()),
        });
    }

    /// Create a temporary parser for a parsing operation.
    /// This is an internal helper that manages SWC's complex lifetimes.
    fn with_parser<F, T>(&self, f: F) -> Result<T, TsSynError>
    where
        F: for<'a> FnOnce(&mut Parser<Lexer<'a>>) -> PResult<T>,
    {
        let fm = self.source_map.new_source_file(
            FileName::Custom(self.file_name.clone()).into(),
            self.source.clone(),
        );

        let syntax = Syntax::Typescript(TsSyntax {
            tsx: self.file_name.ends_with(".tsx"),
            decorators: true,
            ..Default::default()
        });

        let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);

        let mut parser = Parser::new_from(lexer);
        f(&mut parser).map_err(|e| TsSynError::Parse(format!("{:?}", e)))
    }

    /// Parse a value of type T from the stream.
    /// This is analogous to `syn::ParseBuffer::parse::<T>()`.
    pub fn parse<T: ParseTs>(&mut self) -> Result<T, TsSynError> {
        T::parse(self)
    }

    /// Parse an identifier.
    pub fn parse_ident(&self) -> Result<Ident, TsSynError> {
        self.with_parser(|parser| {
            use swc_core::common::DUMMY_SP;
            use swc_core::ecma::parser::error::{Error, SyntaxError};
            // Parse as expression and extract identifier
            parser.parse_expr().and_then(|expr| match *expr {
                Expr::Ident(ident) => Ok(ident),
                _ => Err(Error::new(DUMMY_SP, SyntaxError::TS1003)),
            })
        })
    }

    /// Parse a statement.
    pub fn parse_stmt(&self) -> Result<Stmt, TsSynError> {
        self.with_parser(|parser| parser.parse_stmt_list_item())
    }

    /// Parse an expression.
    pub fn parse_expr(&self) -> Result<Box<Expr>, TsSynError> {
        self.with_parser(|parser| parser.parse_expr())
    }

    /// Parse a module (useful for parsing complete TypeScript files).
    pub fn parse_module(&self) -> Result<Module, TsSynError> {
        self.with_parser(|parser| parser.parse_module())
    }
}

/// A trait for types that can be parsed from TypeScript source, analogous to `syn::parse::Parse`.
///
/// Implement this trait to enable parsing your custom types with [`TsStream::parse`]
/// and [`parse_ts_str`].
///
/// # Built-in Implementations
///
/// The following SWC types already implement `ParseTs`:
/// - `Ident` - A single identifier
/// - `Stmt` - Any statement
/// - `Box<Expr>` - Any expression
/// - `Module` - A complete module
///
/// # Implementing ParseTs
///
/// ```rust,no_run
/// use macroforge_ts_syn::{ParseTs, TsStream, TsSynError, parse_ts_str};
///
/// struct MyType {
///     name: String,
///     value: i32,
/// }
///
/// impl ParseTs for MyType {
///     fn parse(input: &mut TsStream) -> Result<Self, TsSynError> {
///         // Use TsStream's parsing methods
///         let ident = input.parse_ident()?;
///
///         // Or parse sub-expressions
///         let _expr = input.parse_expr()?;
///
///         // Return the parsed type
///         Ok(MyType {
///             name: ident.sym.to_string(),
///             value: 42,
///         })
///     }
/// }
///
/// fn main() -> Result<(), TsSynError> {
///     // Now you can use it with parse_ts_str
///     let _my_value: MyType = parse_ts_str("identifier")?;
///     Ok(())
/// }
/// ```
///
/// # Using with DeriveInput
///
/// [`DeriveInput`](crate::DeriveInput) implements `ParseTs`, allowing it to be
/// parsed from a `TsStream` using the `parse_ts_macro_input!` macro.
pub trait ParseTs: Sized {
    /// Parse a value of this type from a parsing stream.
    ///
    /// # Errors
    ///
    /// Returns [`TsSynError::Parse`] if the source code doesn't match the
    /// expected syntax for this type.
    fn parse(input: &mut TsStream) -> Result<Self, TsSynError>;
}

// Implement ParseTs for common SWC types
#[cfg(feature = "swc")]
impl ParseTs for Ident {
    fn parse(input: &mut TsStream) -> Result<Self, TsSynError> {
        input.parse_ident()
    }
}

#[cfg(feature = "swc")]
impl ParseTs for Stmt {
    fn parse(input: &mut TsStream) -> Result<Self, TsSynError> {
        input.parse_stmt()
    }
}

#[cfg(feature = "swc")]
impl ParseTs for Box<Expr> {
    fn parse(input: &mut TsStream) -> Result<Self, TsSynError> {
        input.parse_expr()
    }
}

#[cfg(feature = "swc")]
impl ParseTs for Module {
    fn parse(input: &mut TsStream) -> Result<Self, TsSynError> {
        input.parse_module()
    }
}

/// Parse a string of TypeScript code into a specific type.
/// This is analogous to `syn::parse_str`.
///
/// # Example
/// ```ignore
/// let ident: Ident = parse_ts_str("myVariable")?;
/// let expr: Box<Expr> = parse_ts_str("x + y")?;
/// let stmt: Stmt = parse_ts_str("const x = 5;")?;
/// ```
#[cfg(feature = "swc")]
pub fn parse_ts_str<T: ParseTs>(code: &str) -> Result<T, TsSynError> {
    let mut stream = TsStream::new(code, "input.ts")?;
    stream.parse()
}

/// Parse a snippet of TypeScript code as an expression.
#[cfg(feature = "swc")]
pub fn parse_ts_expr(code: &str) -> Result<Box<Expr>, TsSynError> {
    parse_ts_str(code)
}

/// Parse a snippet of TypeScript code as a statement.
#[cfg(feature = "swc")]
pub fn parse_ts_stmt(code: &str) -> Result<Stmt, TsSynError> {
    parse_ts_str(code)
}

/// Parse a snippet of TypeScript code as a module.
#[cfg(feature = "swc")]
pub fn parse_ts_module(code: &str) -> Result<Module, TsSynError> {
    parse_ts_str(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "swc")]
    #[test]
    fn test_parse_ident() {
        let result: Result<Ident, _> = parse_ts_str("myVariable");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().sym.as_ref(), "myVariable");
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_parse_expr() {
        let result = parse_ts_expr("1 + 2");
        assert!(result.is_ok());
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_parse_stmt() {
        let result = parse_ts_stmt("const x = 5;");
        assert!(result.is_ok(), "parse_ts_stmt failed: {:?}", result.err());
    }
}
