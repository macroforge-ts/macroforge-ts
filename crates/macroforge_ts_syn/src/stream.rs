//! A `syn`-like parsing API for TypeScript using SWC.
//!
//! This module provides a `TsStream` type and `ParseTs` trait that abstracts
//! over SWC's parser, making it feel more like Rust's syn crate.

#[cfg(feature = "swc")]
use swc_core::common::{FileName, SourceMap, sync::Lrc};
#[cfg(feature = "swc")]
use swc_core::ecma::ast::*;
#[cfg(feature = "swc")]
use swc_core::ecma::codegen::{Config, Emitter, text_writer::JsWriter};
#[cfg(feature = "swc")]
use swc_core::ecma::parser::{PResult, Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

use crate::TsSynError;

/// A parsing stream that wraps SWC's parser.
/// This is analogous to `syn::parse::ParseBuffer`.
///
/// # Example
/// ```ignore
/// let mut stream = TsStream::new("const x = 5;", "input.ts")?;
/// let stmt = stream.parse::<Stmt>()?;
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

/// The Parse trait for TypeScript AST nodes.
/// This is analogous to `syn::parse::Parse`.
///
/// # Example
/// ```ignore
/// impl ParseTs for MyType {
///     fn parse(input: &mut TsStream) -> Result<Self, TsSynError> {
///         let ident = input.parse_ident()?;
///         // ... more parsing
///         Ok(MyType { /* ... */ })
///     }
/// }
/// ```
pub trait ParseTs: Sized {
    /// Parse a value of this type from a parsing stream.
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
