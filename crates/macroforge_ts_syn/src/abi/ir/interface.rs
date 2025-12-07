
use serde::{Deserialize, Serialize};

use crate::abi::{DecoratorIR, SpanIR};

/// Interface IR for derive macros targeting TypeScript interfaces.
/// Similar to ClassIR but for interface declarations.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub type_params: Vec<String>,
    /// Heritage clauses (extends)
    pub heritage: Vec<String>,
    pub decorators: Vec<DecoratorIR>,
    pub fields: Vec<InterfaceFieldIR>,
    pub methods: Vec<InterfaceMethodIR>,
}

/// A field/property in an interface
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceFieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String,
    pub optional: bool,
    pub readonly: bool,
    pub decorators: Vec<DecoratorIR>,
}

/// A method signature in an interface
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceMethodIR {
    pub name: String,
    pub span: SpanIR,
    pub type_params_src: String,
    pub params_src: String,
    pub return_type_src: String,
    pub optional: bool,
    pub decorators: Vec<DecoratorIR>,
}
