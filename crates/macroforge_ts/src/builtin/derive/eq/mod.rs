//! # Eq Macro Implementation
//!
//! The `Eq` macro marks a type's `PartialEq` equality as a total equivalence
//! relation, as Rust's `Eq` trait does: every value equals itself, so `equals`
//! never treats a value as incomparable. Like Rust's, it adds no methods and
//! generates no code.
//!
//! ## Requirements
//!
//! `Eq` requires `PartialEq`, and `Ord` requires `Eq`. Deriving one without
//! what it requires is an expansion error, as it is a compile error in Rust.
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(PartialEq, Eq, PartialOrd, Ord) */
//! class Version {
//!     major: number;
//!     minor: number;
//! }
//! ```
//!
//! `Eq` contributes nothing to the output; `Version` gets `equals` from
//! `PartialEq`, `partialCompare` from `PartialOrd` and `compare` from `Ord`.

mod core;
#[cfg(test)]
mod tests;
