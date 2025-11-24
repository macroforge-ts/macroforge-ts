#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{DecoratorIR, SpanIR};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ClassIR {
    pub name: String,
    pub span: SpanIR,
    pub is_abstract: bool,
    pub type_params: Vec<String>,
    pub heritage: Vec<String>,
    pub decorators: Vec<DecoratorIR>,
    pub fields: Vec<FieldIR>,
    pub methods: Vec<MethodSigIR>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct FieldIR {
    pub name: String,
    pub span: SpanIR,
    pub ts_type: String, // keep as string in v0
    pub optional: bool,
    pub readonly: bool,
    pub visibility: Visibility,
    pub decorators: Vec<DecoratorIR>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct MethodSigIR {
    pub name: String,
    pub span: SpanIR,
    pub params_src: String,
    pub return_type_src: String,
    pub is_static: bool,
    pub visibility: Visibility,
    pub decorators: Vec<DecoratorIR>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Protected,
    Private,
}
