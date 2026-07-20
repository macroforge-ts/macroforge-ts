//! Helpers for parsing TS type-string snippets into structural pieces.
//!
//! Several consumers across the macroforge crates decide what to emit by
//! string-prefix matching on a field's TypeScript type snippet (e.g.
//! `starts_with("Record<")`). Some snippets have multiple equivalent
//! surface forms — for example `Record<K, V>` and `{ [k: K]: V }` are
//! the same TypeScript shape — so a small set of shared parsers here
//! lets each consumer recognise either form without re-implementing the
//! string handling.

/// Returns the `(K, V)` slices from a TS inline index-signature
/// `{ [name: K]: V }`. Returns `None` for anything that isn't a simple
/// index signature — struct literals like `{ id: string }`, mapped
/// types `{ [P in K]: V }`, or non-`{...}` inputs.
///
/// The slices borrow into the input `ts_type` so callers can use them
/// for further parsing without an extra allocation.
pub fn parse_index_signature(ts_type: &str) -> Option<(&str, &str)> {
    let trimmed = ts_type.trim();
    let inner_with_braces = trimmed
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))?;
    let inner = inner_with_braces.trim();
    let rest_after_lbr = inner.strip_prefix('[')?;
    let rbr_pos = rest_after_lbr.find(']')?;
    let key_part = rest_after_lbr[..rbr_pos].trim();
    // Index signatures use `name: K` (e.g. `key: string`). Mapped types
    // use `P in K`, which we explicitly decline — they have different
    // runtime semantics that consumers handle elsewhere.
    if key_part.contains(" in ") {
        return None;
    }
    let colon_pos = key_part.find(':')?;
    let key_type = key_part[colon_pos + 1..].trim();

    let after_rbr = rest_after_lbr[rbr_pos + 1..].trim_start();
    let value_with_terminator = after_rbr.strip_prefix(':')?;
    let value_type = value_with_terminator
        .trim_start()
        .trim_end_matches(';')
        .trim_end_matches(',')
        .trim();

    if key_type.is_empty() || value_type.is_empty() {
        return None;
    }
    Some((key_type, value_type))
}

/// Returns the base name of a generic TS type-string
/// (`Record<K, V>` -> `Record`). For non-generic types, returns the
/// trimmed input unchanged.
pub fn base_type_name(ts_type: &str) -> &str {
    let trimmed = ts_type.trim();
    match trimmed.find('<') {
        Some(pos) => trimmed[..pos].trim(),
        None => trimmed,
    }
}

/// PascalCase TS utility types whose runtime values are plain objects.
///
/// Used by code that needs to emit object-shaped defaults / clones for
/// these generics (`Record<K, V>` -> `({})` / `({ ...x })`).
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

/// PascalCase TS built-in / utility type names that consumers must NOT
/// treat as user-defined types when interpreting a TS type-string.
///
/// Includes the object-utility set, the collection wrappers
/// (`Map`/`Set`/`WeakMap`/...), `Promise`, and the type-level operators
/// (`Awaited`/`Parameters`/`ReturnType`/`InstanceType`).
///
/// Does not include `Array` / `ReadonlyArray` — array forms are detected
/// by dedicated array predicates and the `[]` suffix path.
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
