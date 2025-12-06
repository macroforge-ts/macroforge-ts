
use serde::{Deserialize, Serialize};

use crate::abi::{swc_ast, SpanIR};

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct DecoratorIR {
    pub name: String,     // e.g. "Derive"
    pub args_src: String, // raw args text "Debug, Clone"
    pub span: SpanIR,
    #[serde(skip)]
    pub node: Option<swc_ast::Decorator>,
}
