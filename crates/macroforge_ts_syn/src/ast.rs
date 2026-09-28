//! Owned, source-backed expression and identifier values.
//!
//! Derive macros compute these ahead of a template and interpolate them with
//! `@{...}`; they carry source text, not an arena-bound AST node.

use crate::{ToExprSource, ToIdentSource, TsSynError, expr_to_string};

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Ident {
    pub sym: String,
}

impl Ident {
    pub fn new<S: Into<String>>(sym: S) -> Self {
        Self { sym: sym.into() }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.sym)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    Ident(Ident),
    Source(String),
}

impl Expr {
    /// Parse `code` as a single expression.
    pub fn parse(code: &str) -> Result<Self, TsSynError> {
        let allocator = oxc::allocator::Allocator::default();
        Ok(match crate::parse_expr(&allocator, code)? {
            oxc::ast::ast::Expression::Identifier(ident) => {
                Self::Ident(Ident::new(ident.name.as_str()))
            }
            other => Self::Source(expr_to_string(&other)),
        })
    }

    pub fn source(&self) -> &str {
        match self {
            Self::Ident(ident) => ident.sym.as_str(),
            Self::Source(source) => source.as_str(),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.source())
    }
}

impl From<Ident> for Expr {
    fn from(value: Ident) -> Self {
        Self::Ident(value)
    }
}

impl From<&Ident> for Expr {
    fn from(value: &Ident) -> Self {
        Self::Ident(value.clone())
    }
}

impl From<String> for Expr {
    fn from(value: String) -> Self {
        Self::Source(value)
    }
}

impl From<&str> for Expr {
    fn from(value: &str) -> Self {
        Self::Source(value.to_string())
    }
}

impl ToIdentSource for Ident {
    fn to_ident_source(&self) -> String {
        self.sym.clone()
    }
}

impl ToExprSource for Expr {
    fn to_expr_source(&self) -> String {
        self.source().to_string()
    }
}

impl ToExprSource for Ident {
    fn to_expr_source(&self) -> String {
        self.sym.clone()
    }
}
