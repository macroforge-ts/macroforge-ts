#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{swc_ast, DecoratorIR, SpanIR};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ClassIR {
    pub name: String,
    pub span: SpanIR,
    pub body_span: SpanIR,
    pub is_abstract: bool,
    pub type_params: Vec<String>,
    pub heritage: Vec<String>,
    pub decorators: Vec<DecoratorIR>,
    pub decorators_ast: Vec<swc_ast::Decorator>,
    pub fields: Vec<FieldIR>,
    pub methods: Vec<MethodSigIR>,
    pub members: Vec<swc_ast::ClassMember>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String, // keep as string in v0
    pub type_ann: Option<Box<swc_ast::TsType>>,
    pub optional: bool,
    pub readonly: bool,
    pub visibility: Visibility,
    pub decorators: Vec<DecoratorIR>,
    pub prop_ast: Option<swc_ast::ClassProp>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct MethodSigIR {
    pub name: String,
    pub span: SpanIR,
    pub type_params_src: String, // e.g., "<T>", "<T, U>", or ""
    pub params_src: String,
    pub return_type_src: String,
    pub is_static: bool,
    pub is_async: bool,
    pub visibility: Visibility,
    pub decorators: Vec<DecoratorIR>,
    pub member_ast: Option<MethodAstIR>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum MethodAstIR {
    Method(swc_ast::ClassMethod),
    Constructor(swc_ast::Constructor),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Protected,
    Private,
}
