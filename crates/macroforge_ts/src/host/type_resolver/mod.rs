//! Resolves string type references to entries in the [`TypeRegistry`].
//!
//! Given a field type string like `"User[]"` or `"Map<string, Order>"`,
//! extracts the base type names and resolves them against the registry.

mod helpers;
mod resolver;

#[cfg(test)]
mod tests;

pub use helpers::resolve_target_fields;
pub use resolver::TypeResolver;
