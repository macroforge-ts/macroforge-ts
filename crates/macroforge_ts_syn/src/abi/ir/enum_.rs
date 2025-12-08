use serde::{Deserialize, Serialize};

use crate::abi::{DecoratorIR, SpanIR};

/// Represents the value of an enum variant initializer
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq, Default)]
pub enum EnumValue {
    /// String literal: `"ACTIVE"`
    String(String),
    /// Numeric literal: `42`
    Number(f64),
    /// No initializer, auto-incremented from previous value
    #[default]
    Auto,
    /// Any expression as raw source: `someFunction()`, `A + B`
    Expr(String),
}

impl EnumValue {
    /// Returns true if this is a string literal value
    pub fn is_string(&self) -> bool {
        matches!(self, EnumValue::String(_))
    }

    /// Returns true if this is a numeric literal value
    pub fn is_number(&self) -> bool {
        matches!(self, EnumValue::Number(_))
    }

    /// Returns true if this is auto-incremented
    pub fn is_auto(&self) -> bool {
        matches!(self, EnumValue::Auto)
    }

    /// Returns true if this is a computed expression
    pub fn is_expr(&self) -> bool {
        matches!(self, EnumValue::Expr(_))
    }

    /// Returns the string value if this is a String variant
    pub fn as_string(&self) -> Option<&str> {
        match self {
            EnumValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the numeric value if this is a Number variant
    pub fn as_number(&self) -> Option<f64> {
        match self {
            EnumValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns the expression source if this is an Expr variant
    pub fn as_expr(&self) -> Option<&str> {
        match self {
            EnumValue::Expr(s) => Some(s),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct EnumIR {
    pub name: String,
    pub span: SpanIR,
    /// Span of the enum body (between { and })
    pub body_span: SpanIR,
    pub decorators: Vec<DecoratorIR>,
    pub variants: Vec<EnumVariantIR>,
    /// Whether this is a `const enum`
    pub is_const: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
    /// The initializer value for this variant
    pub value: EnumValue,
    /// Field-level decorators from JSDoc comments
    pub decorators: Vec<DecoratorIR>,
}
