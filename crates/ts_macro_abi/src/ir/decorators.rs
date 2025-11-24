#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::SpanIR;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct DecoratorIR {
    pub name: String,     // e.g. "Derive"
    pub args_src: String, // raw args text "Debug, Clone"
    pub span: SpanIR,
}
