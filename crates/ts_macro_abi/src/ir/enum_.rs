#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{DecoratorIR, SpanIR};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct EnumIR {
    pub name: String,
    pub span: SpanIR,
    pub decorators: Vec<DecoratorIR>,
    pub variants: Vec<EnumVariantIR>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariantIR {
    pub name: String,
    pub span: SpanIR,
}
