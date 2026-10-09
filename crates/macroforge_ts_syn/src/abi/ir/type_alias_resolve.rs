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
//! This exists so that endec codegen does not need runtime helpers like
//! `recordLinkDecodeWithContext<T>` or `recordLinkDefaultValue<T>` :
//! the alias disappears at expansion time and the classifier sees the real
//! structural shape.
//!
//! Non-alias types, missing registry entries, type-parameter mismatches,
//! and types that do not contain generic arguments are all returned
//! unchanged so the caller can keep its existing code paths for them.

use std::collections::HashMap;

use super::type_alias::{TypeAliasIR, TypeBody, TypeMember, TypeMemberKind};
use super::type_alias_scope::adopt_alias_scope;
use super::type_registry::{FileImportEntry, TypeDefinitionIR, TypeRegistry};

const MAX_DEPTH: u8 = 16;

/// Recursively expand every generic type-alias instantiation reachable from
/// `ts_type`, substituting type parameters. Returns the input unchanged if
/// no alias is found or if substitution is not possible (arity mismatch,
/// object-body alias, etc.).
///
/// `caller_file_path` and `file_imports` come from the file *referencing*
/// the type. Both feed [`TypeRegistry::resolve_in_file`]: the file path
/// disambiguates types declared in the caller's own file (typical of
/// generated aggregators that re-declare types alongside their canonical
/// definitions); the imports disambiguate types pulled in by name. Pass an
/// empty path/slice when the caller has no such context: unambiguous names
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

    // `T[]`: recurse into element type.
    if let Some(inner) = trimmed.strip_suffix("[]") {
        let element = resolve_recursive(inner, registry, caller_file_path, file_imports, depth + 1);
        // `[]` binds tighter than `|` and `&`, so a compound element keeps
        // its parentheses.
        let compound = split_top_level_union(&element).is_some()
            || split_top_level_intersection(&element).is_some();
        return if compound {
            format!("({element})[]")
        } else {
            format!("{element}[]")
        };
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
    instantiate_alias(
        base,
        resolved_args,
        registry,
        caller_file_path,
        file_imports,
        |alias, subs| render_body(&alias.body, subs),
    )
}

/// The member of the generic alias instantiated by `ts_type` that the alias
/// marks `/** @default */`, with its parameters substituted: `RecordId` for
/// `RecordLink<Employee>` given `type RecordLink<T> = /** @default */ RecordId | T`.
/// `None` when `ts_type` instantiates no such alias.
pub fn alias_default_member(
    ts_type: &str,
    registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
) -> Option<String> {
    let (base, args) = parse_generic(ts_type.trim())?;
    let resolved_args: Vec<String> = split_top_level_commas(args)
        .iter()
        .map(|arg| resolve_recursive(arg, registry, caller_file_path, file_imports, 1))
        .collect();
    instantiate_alias(
        base,
        &resolved_args,
        registry,
        caller_file_path,
        file_imports,
        |alias, subs| match &alias.body {
            TypeBody::Union(members) => members
                .iter()
                .find(|member| member.has_decorator("default"))
                .and_then(|member| render_member(member, subs)),
            _ => None,
        },
    )
}

/// Renders the alias `base` names, through `render`, with its parameters
/// bound to `resolved_args` and its module's names bound as the caller now
/// has them.
fn instantiate_alias(
    base: &str,
    resolved_args: &[String],
    registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
    render: impl FnOnce(&TypeAliasIR, &HashMap<&str, &str>) -> Option<String>,
) -> Option<String> {
    // Use `resolve_in_file` so ambiguous names: e.g. types redeclared in an
    // aggregator file: still hit the canonical entry once we know either
    // the caller's own file (same-file declaration) or the file it imported
    // the name from.
    let entry = registry.resolve_in_file(base, caller_file_path, file_imports)?;
    let TypeDefinitionIR::TypeAlias(alias) = &entry.definition else {
        return None;
    };
    if alias.type_params.is_empty() || resolved_args.len() > alias.type_params.len() {
        return None;
    }

    // Names from the alias's own module, as the caller now has them bound.
    let scope = adopt_alias_scope(alias, entry, registry, caller_file_path)?;
    let mut subs: HashMap<&str, &str> = scope
        .iter()
        .map(|(name, local)| (name.as_str(), local.as_str()))
        .collect();
    // A parameter the reference leaves out takes its declared default.
    for (index, param) in alias.type_params.iter().enumerate() {
        let argument = match resolved_args.get(index) {
            Some(argument) => argument.as_str(),
            None => param.default.as_deref()?,
        };
        subs.insert(param.name.as_str(), argument);
    }

    render(alias, &subs)
}

/// Render an alias body with its type parameters substituted. Bodies that
/// cannot round-trip through a string (inline objects, newtypes and other
/// symbol brands) are `None`, so the caller keeps the generic-instantiation
/// form.
pub(super) fn render_body(body: &TypeBody, subs: &HashMap<&str, &str>) -> Option<String> {
    match body {
        TypeBody::Union(members) => render_members(members, subs, " | "),
        TypeBody::Intersection(members) => render_members(members, subs, " & "),
        TypeBody::Alias(target) => Some(substitute_tokens(target, subs)),
        TypeBody::Tuple(elems) => {
            let inner: Vec<String> = elems.iter().map(|e| substitute_tokens(e, subs)).collect();
            Some(format!("[{}]", inner.join(", ")))
        }
        TypeBody::Object { .. } | TypeBody::Newtype(_) => None,
        TypeBody::Other(raw) => Some(substitute_tokens(raw, subs)),
    }
}

fn render_members(
    members: &[TypeMember],
    subs: &HashMap<&str, &str>,
    separator: &str,
) -> Option<String> {
    let rendered = members
        .iter()
        .map(|member| render_member(member, subs))
        .collect::<Option<Vec<_>>>()?;
    Some(rendered.join(separator))
}

fn render_member(member: &TypeMember, subs: &HashMap<&str, &str>) -> Option<String> {
    match &member.kind {
        TypeMemberKind::Literal(literal) => Some(literal.clone()),
        TypeMemberKind::TypeRef(reference) => Some(substitute_tokens(reference, subs)),
        TypeMemberKind::Intersection(members) => render_members(members, subs, " & "),
        TypeMemberKind::Object { .. } | TypeMemberKind::Brand(_) => None,
    }
}

/// Replace identifier tokens in `s` that match a key in `subs` with the
/// substituted value. Everything else, string literals included, passes
/// through untouched.
fn substitute_tokens(s: &str, subs: &HashMap<&str, &str>) -> String {
    let mut out = String::with_capacity(s.len());
    for token in tokens(s) {
        match token {
            Token::Ident(ident) => out.push_str(subs.get(ident).copied().unwrap_or(ident)),
            Token::Other(text) => out.push_str(text),
        }
    }
    out
}

/// Each identifier token in `s`, in order of appearance, outside string
/// literals.
pub(super) fn identifiers(s: &str) -> Vec<&str> {
    tokens(s)
        .filter_map(|token| match token {
            Token::Ident(ident) => Some(ident),
            Token::Other(_) => None,
        })
        .collect()
}

enum Token<'a> {
    Ident(&'a str),
    /// Anything that is not an identifier: punctuation, numbers, whitespace
    /// and whole string literals.
    Other(&'a str),
}

/// `s` split into identifiers and the text between them.
fn tokens(s: &str) -> impl Iterator<Item = Token<'_>> {
    let mut rest = s;
    std::iter::from_fn(move || {
        let first = rest.chars().next()?;
        let len = if is_ident_start(first) {
            rest.find(|c: char| !is_ident_continue(c))
                .unwrap_or(rest.len())
        } else if matches!(first, '"' | '\'' | '`') {
            string_literal_len(rest, first)
        } else {
            first.len_utf8()
        };
        let (token, tail) = rest.split_at(len);
        rest = tail;
        Some(if is_ident_start(first) {
            Token::Ident(token)
        } else {
            Token::Other(token)
        })
    })
}

/// The length of the string literal `s` opens with `quote`, through its
/// closing quote, or all of `s` when it is unterminated.
fn string_literal_len(s: &str, quote: char) -> usize {
    let mut escaped = false;
    for (index, c) in s.char_indices().skip(1) {
        match c {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            _ if c == quote => return index + c.len_utf8(),
            _ => {}
        }
    }
    s.len()
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || c == '$'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '$'
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

/// Split a type on its top-level `|`, ignoring any inside `<>`, `()`, `[]`,
/// `{}` or a string literal. `None` when there is no top-level `|`.
pub fn split_top_level_union(s: &str) -> Option<Vec<&str>> {
    let parts = split_on_top_level(s, b'|');
    if parts.len() < 2 {
        return None;
    }
    Some(parts.into_iter().map(str::trim).collect())
}

/// Split a type on its top-level `&`, like [`split_top_level_union`].
pub fn split_top_level_intersection(s: &str) -> Option<Vec<&str>> {
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
                // The `>` of an arrow (`=>`) closes nothing.
                b'>' if i > 0 && bytes[i - 1] == b'=' => {}
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
#[path = "type_alias_resolve_tests.rs"]
mod tests;
