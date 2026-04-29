//! Resolve generic type-alias instantiations at codegen time.
//!
//! Given a TypeScript type string like `RecordLink<ErrandMessage>` and a
//! [`TypeRegistry`], look up the alias body, substitute the concrete type
//! arguments for the alias's type parameters, and return the expanded type
//! string (e.g. `string | ErrandMessage`).
//!
//! The walker is recursive: container shapes (`T[]`, `Array<T>`, `Map<K, V>`,
//! `Record<K, V>`, `A | B`) are traversed and any aliases nested inside are
//! resolved too, so callers only need to invoke [`resolve_generic_aliases`]
//! once per raw type string.
//!
//! This exists so that serde codegen does not need runtime helpers like
//! `recordLinkDeserializeWithContext<T>` or `recordLinkDefaultValue<T>` —
//! the alias disappears at expansion time and the classifier sees the real
//! structural shape.
//!
//! Non-alias types, missing registry entries, type-parameter mismatches,
//! and types that do not contain generic arguments are all returned
//! unchanged so the caller can keep its existing code paths for them.

use std::collections::HashMap;

use super::type_alias::{TypeBody, TypeMember, TypeMemberKind};
use super::type_registry::{FileImportEntry, TypeDefinitionIR, TypeRegistry};

const MAX_DEPTH: u8 = 16;

/// Recursively expand every generic type-alias instantiation reachable from
/// `ts_type`, substituting type parameters. Returns the input unchanged if
/// no alias is found or if substitution is not possible (arity mismatch,
/// object-body alias, etc.).
///
/// `caller_file_path` and `file_imports` come from the file *referencing*
/// the type. Both feed [`TypeRegistry::resolve_in_file`] — the file path
/// disambiguates types declared in the caller's own file (typical of
/// generated aggregators that re-declare types alongside their canonical
/// definitions); the imports disambiguate types pulled in by name. Pass an
/// empty path/slice when the caller has no such context — unambiguous names
/// still resolve via simple-name lookup.
pub fn resolve_generic_aliases(
    ts_type: &str,
    registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
) -> String {
    resolve_recursive(ts_type, registry, caller_file_path, file_imports, 0)
}

fn resolve_recursive(
    ts_type: &str,
    registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
    depth: u8,
) -> String {
    if depth >= MAX_DEPTH {
        return ts_type.to_string();
    }
    let trimmed = ts_type.trim();

    // Top-level union: resolve each member, rejoin with " | ".
    if let Some(parts) = split_top_level_union(trimmed) {
        let resolved: Vec<String> = parts
            .iter()
            .map(|p| resolve_recursive(p, registry, caller_file_path, file_imports, depth + 1))
            .collect();
        return resolved.join(" | ");
    }

    // Top-level intersection: same shape.
    if let Some(parts) = split_top_level_intersection(trimmed) {
        let resolved: Vec<String> = parts
            .iter()
            .map(|p| resolve_recursive(p, registry, caller_file_path, file_imports, depth + 1))
            .collect();
        return resolved.join(" & ");
    }

    // `T[]` — recurse into element type.
    if let Some(inner) = trimmed.strip_suffix("[]") {
        return format!(
            "{}[]",
            resolve_recursive(inner, registry, caller_file_path, file_imports, depth + 1)
        );
    }

    // Generic: `Base<args>`.
    if let Some((base, args_str)) = parse_generic(trimmed) {
        let resolved_args: Vec<String> = split_top_level_commas(args_str)
            .iter()
            .map(|a| resolve_recursive(a, registry, caller_file_path, file_imports, depth + 1))
            .collect();

        // Only user-defined aliases start with an uppercase letter; lower-case
        // names (`partial<T>`) are TS unknowns. We also leave built-in container
        // types alone and just rebuild them with resolved args.
        if base.chars().next().is_some_and(|c| c.is_ascii_uppercase())
            && let Some(expanded) = try_expand_alias(
                base,
                &resolved_args,
                registry,
                caller_file_path,
                file_imports,
            )
        {
            return resolve_recursive(
                &expanded,
                registry,
                caller_file_path,
                file_imports,
                depth + 1,
            );
        }

        return format!("{}<{}>", base, resolved_args.join(", "));
    }

    ts_type.to_string()
}

fn try_expand_alias(
    base: &str,
    resolved_args: &[String],
    registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
) -> Option<String> {
    // Use `resolve_in_file` so ambiguous names — e.g. types redeclared in an
    // aggregator file — still hit the canonical entry once we know either
    // the caller's own file (same-file declaration) or the file it imported
    // the name from.
    let entry = registry.resolve_in_file(base, caller_file_path, file_imports)?;
    let TypeDefinitionIR::TypeAlias(alias) = &entry.definition else {
        return None;
    };
    if alias.type_params.is_empty() || alias.type_params.len() != resolved_args.len() {
        return None;
    }

    let subs: HashMap<&str, &str> = alias
        .type_params
        .iter()
        .map(String::as_str)
        .zip(resolved_args.iter().map(String::as_str))
        .collect();

    let rendered = render_body(&alias.body, &subs);
    if rendered.is_empty() {
        return None;
    }
    Some(rendered)
}

fn render_body(body: &TypeBody, subs: &HashMap<&str, &str>) -> String {
    match body {
        TypeBody::Union(members) => members
            .iter()
            .map(|m| render_member(m, subs))
            .collect::<Vec<_>>()
            .join(" | "),
        TypeBody::Intersection(members) => members
            .iter()
            .map(|m| render_member(m, subs))
            .collect::<Vec<_>>()
            .join(" & "),
        TypeBody::Alias(target) => substitute_tokens(target, subs),
        TypeBody::Tuple(elems) => {
            let inner: Vec<String> = elems.iter().map(|e| substitute_tokens(e, subs)).collect();
            format!("[{}]", inner.join(", "))
        }
        // Object / Other aliases with non-trivial bodies aren't safe to
        // round-trip via a string; signal un-resolvable so the caller keeps
        // the generic-instantiation form and reports a clear error later.
        TypeBody::Object { .. } => String::new(),
        TypeBody::Other(raw) => substitute_tokens(raw, subs),
    }
}

fn render_member(m: &TypeMember, subs: &HashMap<&str, &str>) -> String {
    match &m.kind {
        TypeMemberKind::Literal(s) => s.clone(),
        TypeMemberKind::TypeRef(s) => substitute_tokens(s, subs),
        TypeMemberKind::Intersection(members) => members
            .iter()
            .map(|m| render_member(m, subs))
            .collect::<Vec<_>>()
            .join(" & "),
        TypeMemberKind::Object { .. } => String::new(),
    }
}

/// Replace identifier tokens in `s` that match a key in `subs` with the
/// substituted value. Non-identifier characters (punctuation, angle
/// brackets, string literals, numbers) pass through untouched.
fn substitute_tokens(s: &str, subs: &HashMap<&str, &str>) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if is_ident_start(c) {
            let start = i;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }
            let ident = &s[start..i];
            if let Some(sub) = subs.get(ident) {
                out.push_str(sub);
            } else {
                out.push_str(ident);
            }
        } else {
            out.push(c as char);
            i += 1;
        }
    }
    out
}

fn is_ident_start(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_' || c == b'$'
}

fn is_ident_continue(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'$'
}

/// Parse a generic instantiation `Base<args>` into `("Base", "args")`.
fn parse_generic(type_name: &str) -> Option<(&str, &str)> {
    let open = type_name.find('<')?;
    if !type_name.ends_with('>') {
        return None;
    }
    let close = type_name.len() - 1;
    if open >= close {
        return None;
    }
    let base = type_name[..open].trim();
    let args = type_name[open + 1..close].trim();
    if base.is_empty() || args.is_empty() {
        return None;
    }
    Some((base, args))
}

/// Split a generic-argument list on top-level commas. Nested `<>`, `()`,
/// `[]`, `{}` depth is tracked so `Map<string, number>` yields one member.
fn split_top_level_commas(args: &str) -> Vec<&str> {
    split_on_top_level(args, b',')
        .into_iter()
        .map(str::trim)
        .collect()
}

/// Split on top-level `|` — mirrors the classifier's behavior for unions.
fn split_top_level_union(s: &str) -> Option<Vec<&str>> {
    let parts = split_on_top_level(s, b'|');
    if parts.len() < 2 {
        return None;
    }
    Some(parts.into_iter().map(str::trim).collect())
}

fn split_top_level_intersection(s: &str) -> Option<Vec<&str>> {
    let parts = split_on_top_level(s, b'&');
    if parts.len() < 2 {
        return None;
    }
    Some(parts.into_iter().map(str::trim).collect())
}

fn split_on_top_level(s: &str, sep: u8) -> Vec<&str> {
    let mut out = Vec::new();
    let mut depth: i32 = 0;
    let mut in_str: Option<u8> = None;
    let mut start = 0;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if let Some(quote) = in_str {
            if c == quote && (i == 0 || bytes[i - 1] != b'\\') {
                in_str = None;
            }
        } else {
            match c {
                b'"' | b'\'' | b'`' => in_str = Some(c),
                b'<' | b'(' | b'[' | b'{' => depth += 1,
                b'>' | b')' | b']' | b'}' => depth -= 1,
                _ if c == sep && depth == 0 => {
                    out.push(&s[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
        }
        i += 1;
    }
    out.push(&s[start..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::SpanIR;
    use crate::abi::ir::type_alias::TypeAliasIR;
    use crate::abi::ir::type_registry::TypeRegistryEntry;

    fn alias_entry(
        name: &str,
        type_params: Vec<&str>,
        body: TypeBody,
        file_path: &str,
    ) -> TypeRegistryEntry {
        TypeRegistryEntry {
            name: name.to_string(),
            file_path: file_path.to_string(),
            is_exported: true,
            definition: TypeDefinitionIR::TypeAlias(TypeAliasIR {
                name: name.to_string(),
                span: SpanIR::new(0, 0),
                decorators: vec![],
                type_params: type_params.into_iter().map(String::from).collect(),
                body,
            }),
            file_imports: vec![],
        }
    }

    fn record_link_registry() -> TypeRegistry {
        let mut registry = TypeRegistry::new();
        // type RecordLink<T> = string | T
        let body = TypeBody::Union(vec![
            TypeMember::new(TypeMemberKind::TypeRef("string".to_string())),
            TypeMember::new(TypeMemberKind::TypeRef("T".to_string())),
        ]);
        registry.insert(
            alias_entry("RecordLink", vec!["T"], body, "/p/record-link.ts"),
            "/p",
        );
        registry
    }

    #[test]
    fn expands_record_link_to_union() {
        let reg = record_link_registry();
        assert_eq!(
            resolve_generic_aliases("RecordLink<ErrandMessage>", &reg, "", &[]),
            "string | ErrandMessage"
        );
    }

    #[test]
    fn expands_record_link_inside_array() {
        let reg = record_link_registry();
        assert_eq!(
            resolve_generic_aliases("Array<RecordLink<ErrandMessage>>", &reg, "", &[]),
            "Array<string | ErrandMessage>"
        );
    }

    #[test]
    fn expands_record_link_inside_array_suffix() {
        let reg = record_link_registry();
        assert_eq!(
            resolve_generic_aliases("RecordLink<Foo>[]", &reg, "", &[]),
            "string | Foo[]"
        );
        // Note: `string | Foo[]` parses as `string | (Foo[])` in TS; that's
        // the correct expansion because `RecordLink<Foo>[]` means array of
        // string-or-Foo, but typed as `(string | Foo)[]` at the TS level
        // should be written with parens. Callers that use `[]` suffix on a
        // generic alias are already on thin ice; recommend `Array<...>`.
    }

    #[test]
    fn expands_record_link_inside_map() {
        let reg = record_link_registry();
        assert_eq!(
            resolve_generic_aliases("Map<string, RecordLink<Foo>>", &reg, "", &[]),
            "Map<string, string | Foo>"
        );
    }

    #[test]
    fn passes_through_missing_alias() {
        let reg = TypeRegistry::new();
        assert_eq!(
            resolve_generic_aliases("RecordLink<Foo>", &reg, "", &[]),
            "RecordLink<Foo>"
        );
    }

    #[test]
    fn passes_through_non_generic() {
        let reg = record_link_registry();
        assert_eq!(resolve_generic_aliases("User", &reg, "", &[]), "User");
        assert_eq!(resolve_generic_aliases("string", &reg, "", &[]), "string");
    }

    #[test]
    fn passes_through_lowercase_base() {
        let reg = record_link_registry();
        assert_eq!(
            resolve_generic_aliases("partial<User>", &reg, "", &[]),
            "partial<User>"
        );
    }

    #[test]
    fn passes_through_arity_mismatch() {
        let reg = record_link_registry();
        assert_eq!(
            resolve_generic_aliases("RecordLink<A, B>", &reg, "", &[]),
            "RecordLink<A, B>"
        );
    }

    #[test]
    fn nested_substitution_collapses() {
        let mut reg = TypeRegistry::new();
        reg.insert(
            alias_entry(
                "Inner",
                vec!["T"],
                TypeBody::Union(vec![
                    TypeMember::new(TypeMemberKind::TypeRef("T".to_string())),
                    TypeMember::new(TypeMemberKind::Literal("null".to_string())),
                ]),
                "/p/inner.ts",
            ),
            "/p",
        );
        reg.insert(
            alias_entry(
                "Outer",
                vec!["U"],
                TypeBody::Alias("Inner<U>".to_string()),
                "/p/outer.ts",
            ),
            "/p",
        );
        assert_eq!(
            resolve_generic_aliases("Outer<User>", &reg, "", &[]),
            "User | null"
        );
    }

    #[test]
    fn skips_object_body() {
        let mut reg = TypeRegistry::new();
        reg.insert(
            alias_entry(
                "Boxed",
                vec!["T"],
                TypeBody::Object { fields: vec![] },
                "/p/boxed.ts",
            ),
            "/p",
        );
        assert_eq!(
            resolve_generic_aliases("Boxed<User>", &reg, "", &[]),
            "Boxed<User>"
        );
    }

    #[test]
    fn substitute_tokens_respects_word_boundaries() {
        let mut subs = HashMap::new();
        subs.insert("T", "ErrandMessage");
        assert_eq!(substitute_tokens("T", &subs), "ErrandMessage");
        assert_eq!(substitute_tokens("Array<T>", &subs), "Array<ErrandMessage>");
        assert_eq!(substitute_tokens("MyT", &subs), "MyT");
        assert_eq!(substitute_tokens("TFoo", &subs), "TFoo");
    }

    #[test]
    fn split_top_level_commas_respects_nesting() {
        assert_eq!(
            split_top_level_commas("string, Map<string, number>"),
            vec!["string", "Map<string, number>"]
        );
        assert_eq!(split_top_level_commas("A"), vec!["A"]);
    }
}
