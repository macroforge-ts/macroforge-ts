//! A field's path through `Encode`: its preparation from the source field,
//! and the statements that encode it.

mod flatten;
pub(crate) mod prepare;
pub(super) mod statements;
mod value;
