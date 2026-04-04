use crate::{ToOxcExprSource, ToOxcIdentSource, TsSynError, oxc_expr_to_string, parse_oxc_expr};

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

impl ToOxcIdentSource for Ident {
    fn to_oxc_ident_source(&self) -> String {
        self.sym.clone()
    }
}

impl ToOxcExprSource for Expr {
    fn to_oxc_expr_source(&self) -> String {
        self.source().to_string()
    }
}

impl ToOxcExprSource for Ident {
    fn to_oxc_expr_source(&self) -> String {
        self.sym.clone()
    }
}

pub fn parse_ts_expr(code: &str) -> Result<Box<Expr>, TsSynError> {
    let expr = parse_oxc_expr(code)?;
    let expr = match expr {
        oxc_ast::ast::Expression::Identifier(ident) => Expr::Ident(Ident::new(ident.name)),
        other => Expr::Source(oxc_expr_to_string(&other)),
    };
    Ok(Box::new(expr))
}

pub fn emit_expr(expr: &Expr) -> String {
    expr.source().to_string()
}
