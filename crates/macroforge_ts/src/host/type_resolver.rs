//! Resolves string type references to entries in the [`TypeRegistry`].
//!
//! Given a field type string like `"User[]"` or `"Map<string, Order>"`,
//! extracts the base type names and resolves them against the registry.

use std::collections::HashMap;

use crate::ts_syn::abi::TargetIR;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};

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

        // Try direct lookup
        if let Some(entry) = self.registry.get(name) {
            // Reconstruct the qualified key from the entry
            // (This is a simplified approach - the qualified key is in qualified_types)
            return Some(format!("{}::{}", entry.file_path, entry.name));
        }

        None
    }
}

/// Resolve all field types for a target declaration.
///
/// Returns a map of field name -> [`ResolvedTypeRef`].
pub fn resolve_target_fields(
    target: &TargetIR,
    resolver: &TypeResolver,
) -> HashMap<String, ResolvedTypeRef> {
    let mut resolved = HashMap::new();

    match target {
        TargetIR::Class(class) => {
            for field in &class.fields {
                resolved.insert(field.name.clone(), resolver.resolve(&field.ts_type));
            }
        }
        TargetIR::Interface(iface) => {
            for field in &iface.fields {
                resolved.insert(field.name.clone(), resolver.resolve(&field.ts_type));
            }
        }
        TargetIR::TypeAlias(alias) => {
            if let Some(fields) = alias.body.as_object() {
                for field in fields {
                    resolved.insert(field.name.clone(), resolver.resolve(&field.ts_type));
                }
            }
        }
        _ => {} // Enums don't have typed fields in the same way
    }

    resolved
}

/// Strip trailing `[]` from a type string, returning the inner type.
fn strip_array_suffix(s: &str) -> Option<&str> {
    if let Some(inner) = s.strip_suffix("[]") {
        Some(inner.trim())
    } else if let Some(inner) = s.strip_prefix("Array<").and_then(|s| s.strip_suffix('>')) {
        Some(inner.trim())
    } else if let Some(inner) = s
        .strip_prefix("ReadonlyArray<")
        .and_then(|s| s.strip_suffix('>'))
    {
        Some(inner.trim())
    } else {
        None
    }
}

/// Strip `| undefined` or `| null` from the end of a type string.
fn strip_optional(s: &str) -> Option<&str> {
    // Check for " | undefined" at the end
    if let Some(pos) = s.rfind(" | undefined")
        && pos + " | undefined".len() == s.len()
    {
        return Some(s[..pos].trim());
    }
    // Check for " | null" at the end
    if let Some(pos) = s.rfind(" | null")
        && pos + " | null".len() == s.len()
    {
        return Some(s[..pos].trim());
    }
    None
}

/// Split a generic type into its base name and type arguments.
///
/// `"Map<string, User>"` -> `("Map", ["string", "User"])`
fn split_generic(s: &str) -> Option<(&str, Vec<&str>)> {
    let open = s.find('<')?;
    if !s.ends_with('>') {
        return None;
    }
    let base = &s[..open];
    let args_str = &s[open + 1..s.len() - 1];
    let args = split_type_args(args_str);
    Some((base, args))
}

/// Split comma-separated type arguments, respecting nested generics.
fn split_type_args(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in s.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => {
                let arg = s[start..i].trim();
                if !arg.is_empty() {
                    result.push(arg);
                }
                start = i + 1;
            }
            _ => {}
        }
    }

    let last = s[start..].trim();
    if !last.is_empty() {
        result.push(last);
    }

    result
}

/// Check if a type name represents a collection type.
fn is_collection_type(name: &str) -> bool {
    matches!(
        name,
        "Array" | "Set" | "Map" | "WeakMap" | "WeakSet" | "ReadonlyArray"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ts_syn::abi::ClassIR;
    use crate::ts_syn::abi::ir::type_registry::{
        TypeDefinitionIR, TypeRegistry, TypeRegistryEntry,
    };

    fn make_test_registry() -> TypeRegistry {
        let mut registry = TypeRegistry::new();

        let zero_span = crate::ts_syn::abi::SpanIR { start: 0, end: 0 };

        // Add a User class
        let user_entry = TypeRegistryEntry {
            name: "User".to_string(),
            file_path: "/project/src/models/user.ts".to_string(),
            is_exported: true,
            definition: TypeDefinitionIR::Class(ClassIR {
                name: "User".to_string(),
                span: zero_span,
                body_span: zero_span,
                is_abstract: false,
                type_params: vec![],
                heritage: vec![],
                decorators: vec![],
                decorators_ast: vec![],
                fields: vec![],
                methods: vec![],
                members: vec![],
            }),
            file_imports: vec![],
        };
        registry.insert(user_entry, "/project");

        // Add an Order interface
        let order_entry = TypeRegistryEntry {
            name: "Order".to_string(),
            file_path: "/project/src/models/order.ts".to_string(),
            is_exported: true,
            definition: TypeDefinitionIR::Interface(crate::ts_syn::abi::InterfaceIR {
                name: "Order".to_string(),
                span: zero_span,
                body_span: zero_span,
                type_params: vec![],
                heritage: vec![],
                decorators: vec![],
                fields: vec![],
                methods: vec![],
            }),
            file_imports: vec![],
        };
        registry.insert(order_entry, "/project");

        registry
    }

    #[test]
    fn test_resolve_primitive() {
        let registry = make_test_registry();
        let resolver = TypeResolver::new(&registry);

        let resolved = resolver.resolve("string");
        assert_eq!(resolved.base_type_name, "string");
        assert!(resolved.registry_key.is_none());
        assert!(!resolved.is_collection);
        assert!(!resolved.is_optional);
    }

    #[test]
    fn test_resolve_known_type() {
        let registry = make_test_registry();
        let resolver = TypeResolver::new(&registry);

        let resolved = resolver.resolve("User");
        assert_eq!(resolved.base_type_name, "User");
        assert!(resolved.registry_key.is_some());
        assert!(!resolved.is_collection);
    }

    #[test]
    fn test_resolve_array() {
        let registry = make_test_registry();
        let resolver = TypeResolver::new(&registry);

        let resolved = resolver.resolve("User[]");
        assert_eq!(resolved.base_type_name, "User");
        assert!(resolved.registry_key.is_some());
        assert!(resolved.is_collection);
        assert_eq!(resolved.type_args.len(), 1);
    }

    #[test]
    fn test_resolve_optional() {
        let registry = make_test_registry();
        let resolver = TypeResolver::new(&registry);

        let resolved = resolver.resolve("User | undefined");
        assert_eq!(resolved.base_type_name, "User");
        assert!(resolved.registry_key.is_some());
        assert!(resolved.is_optional);
    }

    #[test]
    fn test_resolve_generic() {
        let registry = make_test_registry();
        let resolver = TypeResolver::new(&registry);

        let resolved = resolver.resolve("Map<string, User>");
        assert_eq!(resolved.base_type_name, "Map");
        assert!(resolved.registry_key.is_none()); // Map is a primitive
        assert!(resolved.is_collection);
        assert_eq!(resolved.type_args.len(), 2);
        assert_eq!(resolved.type_args[0].base_type_name, "string");
        assert!(resolved.type_args[1].registry_key.is_some()); // User resolves
    }

    #[test]
    fn test_resolve_unknown_type() {
        let registry = make_test_registry();
        let resolver = TypeResolver::new(&registry);

        let resolved = resolver.resolve("UnknownType");
        assert_eq!(resolved.base_type_name, "UnknownType");
        assert!(resolved.registry_key.is_none());
    }

    #[test]
    fn test_split_type_args() {
        assert_eq!(split_type_args("string, number"), vec!["string", "number"]);
        assert_eq!(
            split_type_args("Map<string, number>, User"),
            vec!["Map<string, number>", "User"]
        );
        assert_eq!(split_type_args("string"), vec!["string"]);
    }
}
