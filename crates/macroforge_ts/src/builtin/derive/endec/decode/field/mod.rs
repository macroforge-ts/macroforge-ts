//! A field's path through `Decode`: its preparation from the source field,
//! and the statements that decode it.

pub(crate) mod prepare;
pub(super) mod statements;
mod value;
