use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};

use super::helpers::{is_collection_type, split_generic, strip_array_suffix, strip_optional};

/// TypeScript primitive/built-in types that should not be resolved against the registry.
const PRIMITIVES: &[&str] = &[
    "string",
    "number",
    "boolean",
    "bigint",
    "symbol",
    "undefined",
    "null",
    "void",
    "never",
    "any",
    "unknown",
    "object",
    "Date",
    "RegExp",
    "Error",
    "Promise",
    "Array",
    "Set",
    "Map",
    "WeakMap",
    "WeakSet",
    "Record",
    "Partial",
    "Required",
    "Readonly",
    "Pick",
    "Omit",
    "Exclude",
    "Extract",
    "NonNullable",
    "ReturnType",
    "InstanceType",
    "Parameters",
    "ConstructorParameters",
    "Int8Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "Int16Array",
    "Uint16Array",
    "Int32Array",
    "Uint32Array",
    "Float32Array",
    "Float64Array",
    "BigInt64Array",
    "BigUint64Array",
    "ArrayBuffer",
    "SharedArrayBuffer",
    "DataView",
    "ReadonlyArray",
    "Iterable",
    "Iterator",
    "AsyncIterable",
    "AsyncIterator",
    "Generator",
    "AsyncGenerator",
    "PromiseLike",
    "Awaited",
];

/// Resolves opaque field type strings against a [`TypeRegistry`].
pub struct TypeResolver<'a> {
    registry: &'a TypeRegistry,
}

impl<'a> TypeResolver<'a> {
    /// Create a new resolver for the given registry.
    pub fn new(registry: &'a TypeRegistry) -> Self {
        Self { registry }
    }

    /// Resolve a type string to a [`ResolvedTypeRef`].
    pub fn resolve(&self, raw_type: &str) -> ResolvedTypeRef {
        let trimmed = raw_type.trim();

        // Handle array types: "User[]" or "Array<User>"
        if let Some(inner) = strip_array_suffix(trimmed) {
            let inner_resolved = self.resolve(inner);
            return ResolvedTypeRef {
                raw_type: trimmed.to_string(),
                base_type_name: inner_resolved.base_type_name.clone(),
                registry_key: inner_resolved.registry_key.clone(),
                is_collection: true,
                is_optional: false,
                type_args: vec![inner_resolved],
            };
        }

        // Handle optional types: "User | undefined" or "User | null"
        if let Some(inner) = strip_optional(trimmed) {
            let inner_resolved = self.resolve(inner);
            return ResolvedTypeRef {
                raw_type: trimmed.to_string(),
                base_type_name: inner_resolved.base_type_name.clone(),
                registry_key: inner_resolved.registry_key.clone(),
                is_collection: inner_resolved.is_collection,
                is_optional: true,
                type_args: inner_resolved.type_args,
            };
        }

        // Handle generic types: "Map<string, User>"
        if let Some((base, args)) = split_generic(trimmed) {
            let resolved_args: Vec<ResolvedTypeRef> =
                args.iter().map(|arg| self.resolve(arg)).collect();
            let registry_key = self.lookup(base);
            return ResolvedTypeRef {
                raw_type: trimmed.to_string(),
                base_type_name: base.to_string(),
                registry_key,
                is_collection: is_collection_type(base),
                is_optional: false,
                type_args: resolved_args,
            };
        }

        // Simple type reference
        let registry_key = self.lookup(trimmed);
        ResolvedTypeRef {
            raw_type: trimmed.to_string(),
            base_type_name: trimmed.to_string(),
            registry_key,
            is_collection: false,
            is_optional: false,
            type_args: vec![],
        }
    }

    /// Look up a type name in the registry, returning the qualified key if found.
    fn lookup(&self, name: &str) -> Option<String> {
        if PRIMITIVES.contains(&name) {
            return None;
        }

        // Try direct lookup (returns None for ambiguous names)
        if let Some(entry) = self.registry.get(name) {
            return Some(format!("{}::{}", entry.file_path, entry.name));
        }

        // For ambiguous names, pick the first qualified entry
        if let Some(entry) = self.registry.get_all(name).next() {
            return Some(format!("{}::{}", entry.file_path, entry.name));
        }

        None
    }
}
