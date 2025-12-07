
use serde::{Deserialize, Serialize};

use crate::abi::{DecoratorIR, SpanIR};

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct EnumIR {
    pub name: String,
    pub span: SpanIR,
    pub decorators: Vec<DecoratorIR>,
    pub variants: Vec<EnumVariantIR>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
}
