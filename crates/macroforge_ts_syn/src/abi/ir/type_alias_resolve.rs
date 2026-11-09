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
use crate::scan::{
    Token, angle_unwrapped, ends_in_its_last_part, is_ident_continue, is_ident_start, tokens,
    top_level_letters,
};

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
    resolve_generic_aliases_where(
        ts_type,
        registry,
        caller_file_path,
        file_imports,
        &|_, _| true,
    )
}

/// [`resolve_generic_aliases`], expanding an instantiation only where
/// `expands` says so, given the alias's name and what it expands to. One left
/// alone keeps its name, with its arguments resolved.
pub fn resolve_generic_aliases_where(
    ts_type: &str,
    registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
    expands: &dyn Fn(&str, &str) -> bool,
) -> String {
    let source = AliasSource {
        registry,
        caller_file_path,
        file_imports,
        expands,
    };
    resolve_recursive(ts_type, &source, 0)
}

/// Where instantiations are resolved from, and which of them are expanded.
struct AliasSource<'a> {
    registry: &'a TypeRegistry,
    caller_file_path: &'a str,
    file_imports: &'a [FileImportEntry],
    expands: &'a dyn Fn(&str, &str) -> bool,
}

fn resolve_recursive(ts_type: &str, source: &AliasSource<'_>, depth: u8) -> String {
    if depth >= MAX_DEPTH {
        return ts_type.to_string();
    }
    let trimmed = ts_type.trim();

    // Top-level union: resolve each member, rejoin with " | ".
    if let Some(parts) = split_top_level_union(trimmed) {
        let resolved: Vec<String> = parts
            .iter()
            .map(|part| resolve_recursive(part, source, depth + 1))
            .collect();
        return resolved.join(" | ");
    }

    // Top-level intersection: same shape.
    if let Some(parts) = split_top_level_intersection(trimmed) {
        let resolved: Vec<String> = parts
            .iter()
            .map(|part| resolve_recursive(part, source, depth + 1))
            .collect();
        return resolved.join(" & ");
    }

    // `T[]`: recurse into element type.
    if let Some(inner) = trimmed
        .strip_suffix("[]")
        .filter(|_| !ends_in_its_last_part(trimmed))
    {
        let element = resolve_recursive(inner, source, depth + 1);
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
            .map(|argument| resolve_recursive(argument, source, depth + 1))
            .collect();

        // A name the registry does not hold as an alias, a built-in
        // container included, is rebuilt around its resolved arguments. So
        // is an alias the caller keeps by name.
        if let Some(expanded) = try_expand_alias(
            base,
            &resolved_args,
            source.registry,
            source.caller_file_path,
            source.file_imports,
        ) && (source.expands)(base, &expanded)
        {
            return resolve_recursive(&expanded, source, depth + 1);
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
        .map(|arg| resolve_generic_aliases(arg, registry, caller_file_path, file_imports))
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

/// Replace each identifier in `s` that names a type and is a key in `subs`
/// with its substitute. Everything else passes through untouched: a string,
/// the text of a template literal, a key, a label, a parameter's name and a
/// member of a qualified name.
pub fn substitute_tokens(s: &str, subs: &HashMap<&str, &str>) -> String {
    let tokens = tokens(s);
    tokens
        .iter()
        .enumerate()
        .map(|(index, token)| match token {
            Token::Ident(ident) if names_type(&tokens, index) => {
                subs.get(ident).copied().unwrap_or(ident)
            }
            Token::Ident(text) | Token::Other(text) => text,
        })
        .collect()
}

/// Each identifier in `s` that names a type, in order of appearance.
pub(super) fn identifiers(s: &str) -> Vec<&str> {
    let tokens = tokens(s);
    tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| match token {
            Token::Ident(ident) if names_type(&tokens, index) => Some(*ident),
            Token::Ident(_) | Token::Other(_) => None,
        })
        .collect()
}

/// Whether the identifier at `index` of `tokens` names a type: not a member
/// of a qualified name (`NS.T`), and not a key, label or parameter name,
/// which a `:` follows unless it ends a conditional's branch (`? A | T :`).
fn names_type(tokens: &[Token<'_>], index: usize) -> bool {
    let significant = |token: &&Token<'_>| !token.text().trim().is_empty();
    let before: Vec<&str> = tokens
        .get(..index)
        .unwrap_or_default()
        .iter()
        .rev()
        .filter(significant)
        .take(3)
        .map(Token::text)
        .collect();
    let mut after = tokens
        .get(index + 1..)
        .unwrap_or_default()
        .iter()
        .filter(significant)
        .map(Token::text);
    let spread = before == [".", ".", "."];
    let member = before.first() == Some(&".") && !spread;
    let named = match after.next() {
        Some(":") => true,
        Some("?") => after.next() == Some(":"),
        _ => false,
    };
    // What stands after one of these is a type, a `:` after it or not.
    let typed = matches!(
        before.first().copied(),
        Some("?" | ":" | "|" | "&" | ">" | "keyof" | "typeof" | "infer" | "unique")
    );
    !member && (!named || typed)
}

/// Parse a generic instantiation `Base<args>` into `("Base", "args")`.
fn parse_generic(type_name: &str) -> Option<(&str, &str)> {
    let open = type_name.find('<')?;
    let base = type_name.get(..open)?.trim();
    let named = base
        .split('.')
        .all(|part| part.starts_with(is_ident_start) && part.chars().all(is_ident_continue));
    let args = angle_unwrapped(type_name.get(open..)?)?.trim();
    (named && !args.is_empty()).then_some((base, args))
}

/// Split a generic-argument list, or a tuple's elements, on top-level commas.
/// Nested `<>`, `()`, `[]`, `{}` depth is tracked so `Map<string, number>`
/// yields one member.
pub fn split_top_level_commas(args: &str) -> Vec<&str> {
    split_on_top_level(args, ',')
        .into_iter()
        .map(str::trim)
        .collect()
}

/// Split a type on its top-level `|`, ignoring any inside `<>`, `()`, `[]`,
/// `{}` or a string literal. `None` when there is no top-level `|`, and for a
/// function or conditional type, whose last part takes a `|` in it as its own.
pub fn split_top_level_union(s: &str) -> Option<Vec<&str>> {
    split_type_operands(s, '|')
}

/// Split a type on its top-level `&`, like [`split_top_level_union`].
pub fn split_top_level_intersection(s: &str) -> Option<Vec<&str>> {
    split_type_operands(s, '&')
}

fn split_type_operands(s: &str, operator: char) -> Option<Vec<&str>> {
    let parts = split_on_top_level(s, operator);
    // A `&` binds tighter than a `|`.
    let looser = operator == '&' && top_level_letters(s).any(|(_, letter)| letter == '|');
    if parts.len() < 2 || ends_in_its_last_part(s) || looser {
        return None;
    }
    Some(parts.into_iter().map(str::trim).collect())
}

fn split_on_top_level(s: &str, separator: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    for (at, _) in top_level_letters(s).filter(|(_, letter)| *letter == separator) {
        parts.push(s.get(start..at).unwrap_or_default());
        start = at + separator.len_utf8();
    }
    parts.push(s.get(start..).unwrap_or_default());
    parts
}

#[cfg(test)]
#[path = "type_alias_resolve_tests.rs"]
mod tests;
