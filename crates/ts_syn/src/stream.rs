//! A `syn`-like parsing API for TypeScript using SWC.
//!
//! This module provides a `TsStream` type and `ParseTs` trait that abstracts
//! over SWC's parser, making it feel more like Rust's syn crate.

#[cfg(feature = "swc")]
use swc_common::{sync::Lrc, FileName, SourceMap};
#[cfg(feature = "swc")]
use swc_ecma_ast::*;
#[cfg(feature = "swc")]
use swc_ecma_parser::{
    lexer::Lexer, Parser, PResult, StringInput, Syntax, TsSyntax,
};

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
    pub ctx: Option<ts_macro_abi::MacroContextIR>,
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
        })
    }

    /// Create a new parsing stream with macro context attached.
    /// This is used by the macro host to provide context to macros.
    pub fn with_context(
        source: &str,
        file_name: &str,
        ctx: ts_macro_abi::MacroContextIR,
    ) -> Result<Self, TsSynError> {
        Ok(TsStream {
            source_map: Lrc::new(Default::default()),
            source: source.to_string(),
            file_name: file_name.to_string(),
            ctx: Some(ctx),
        })
    }

    /// Get the macro context if available
    pub fn context(&self) -> Option<&ts_macro_abi::MacroContextIR> {
        self.ctx.as_ref()
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

        let lexer = Lexer::new(
            syntax,
            EsVersion::latest(),
            StringInput::from(&*fm),
            None,
        );

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
            // Parse as expression and extract identifier
            parser.parse_expr().and_then(|expr| match *expr {
                Expr::Ident(ident) => Ok(ident),
                _ => Err(swc_ecma_parser::error::Error::new(
                    swc_common::DUMMY_SP,
                    swc_ecma_parser::error::SyntaxError::TS1003,
                )),
            })
        })
    }

    /// Parse a statement.
    pub fn parse_stmt(&self) -> Result<Stmt, TsSynError> {
        self.with_parser(|parser| parser.parse_stmt())
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
        assert!(result.is_ok());
    }
}
