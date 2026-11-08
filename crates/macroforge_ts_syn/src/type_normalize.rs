//! Classifies the base names of TypeScript type-string snippets, for macros
//! that read a field's type as text and must tell a built-in generic from a
//! type the user declared.

/// PascalCase TS utility types whose runtime values are plain objects.
///
/// Used by code that needs to emit object-shaped defaults or clones for these
/// generics (`Record<K, V>` -> `({})` / `({ ...x })`).
pub fn is_ts_object_utility_type(base: &str) -> bool {
    matches!(
        base,
        "Record"
            | "Partial"
            | "Required"
            | "Readonly"
            | "Pick"
            | "Omit"
            | "NonNullable"
            | "Exclude"
            | "Extract"
    )
}

/// PascalCase TS built-in and utility type names that must not be treated as
/// user-defined types when interpreting a TS type-string.
///
/// Includes the object-utility set, the collection wrappers
/// (`Map`/`Set`/`WeakMap`/...), `Promise`, and the type-level operators
/// (`Awaited`/`Parameters`/`ReturnType`/`InstanceType`).
///
/// Does not include `Array` / `ReadonlyArray`: array forms are detected by
/// dedicated array predicates and the `[]` suffix path.
pub fn is_ts_builtin_type(base: &str) -> bool {
    if is_ts_object_utility_type(base) {
        return true;
    }
    matches!(
        base,
        "Map"
            | "ReadonlyMap"
            | "Set"
            | "ReadonlySet"
            | "WeakMap"
            | "WeakSet"
            | "WeakRef"
            | "Promise"
            | "Awaited"
            | "Parameters"
            | "ReturnType"
            | "InstanceType"
    )
}

#[cfg(test)]
mod tests {
    use super::{is_ts_builtin_type, is_ts_object_utility_type};

    #[test]
    fn object_utilities_are_built_in() {
        assert!(is_ts_object_utility_type("Record"));
        assert!(is_ts_builtin_type("Record"));
        assert!(is_ts_builtin_type("Omit"));
    }

    #[test]
    fn collections_and_operators_are_built_in_but_not_object_utilities() {
        for name in ["Map", "WeakRef", "Promise", "ReturnType"] {
            assert!(is_ts_builtin_type(name), "{name}");
            assert!(!is_ts_object_utility_type(name), "{name}");
        }
    }

    #[test]
    fn arrays_and_declared_types_are_not_built_in() {
        for name in ["Array", "ReadonlyArray", "Customer", "record"] {
            assert!(!is_ts_builtin_type(name), "{name}");
        }
    }
}
