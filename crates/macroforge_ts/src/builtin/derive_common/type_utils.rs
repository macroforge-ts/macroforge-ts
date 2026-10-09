use convert_case::{Case, Casing};

use crate::builtin::endec::{TypeCategory, get_foreign_types};
use crate::ts_syn::abi::ir::{
    FileImportEntry, TypeRegistry, is_primitive_keyword, resolve_generic_aliases,
    split_top_level_union,
};

/// Check if a TypeScript type is a primitive or opaque built-in keyword, whose
/// values are compared, hashed and copied by reference or value as they are
pub fn is_primitive_type(ts_type: &str) -> bool {
    is_primitive_keyword(ts_type)
        || matches!(
            ts_type.trim(),
            "unknown" | "any" | "void" | "never" | "object" | "symbol" | "Function"
        )
}

/// Statements comparing `a` and `b` of one primitive base and returning
/// -1, 0 or 1. With `partial`, an unordered pair (NaN) returns `null` and a
/// string comparison returns `localeCompare` unclamped.
pub fn primitive_compare_statements(base: &str, partial: bool) -> String {
    match base {
        "string" if partial => "return a.localeCompare(b);",
        "string" => "const cmp = a.localeCompare(b);\nreturn cmp < 0 ? -1 : cmp > 0 ? 1 : 0;",
        "boolean" => "return a === b ? 0 : a ? 1 : -1;",
        _ if partial => "return a < b ? -1 : a > b ? 1 : a === b ? 0 : null;",
        _ => "return a < b ? -1 : a > b ? 1 : 0;",
    }
    .to_string()
}

/// One element of a tuple type, as written in the tuple's source.
pub struct TupleElement<'a> {
    /// The element's type, without its label, `?` or `...`.
    pub ts_type: &'a str,
    /// Whether this is a rest element (`...T[]`), which takes every remaining
    /// position.
    pub rest: bool,
    /// Whether the element may be absent (`T?` or `name?: T`).
    pub optional: bool,
}

/// Reads a tuple element such as `number`, `name: string`, `count?: number`
/// or `...rest: string[]`.
pub fn tuple_element(source: &str) -> TupleElement<'_> {
    let trimmed = source.trim();
    let (rest, unspread) = match trimmed.strip_prefix("...") {
        Some(inner) => (true, inner.trim_start()),
        None => (false, trimmed),
    };
    let labeled_optional = unspread
        .split_once(':')
        .is_some_and(|(label, _)| label.trim().ends_with('?'));
    let unlabeled = unspread
        .split_once(':')
        .filter(|(label, _)| {
            let name = label.trim().trim_end_matches('?');
            !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
        })
        .map_or(unspread, |(_, ty)| ty.trim());
    TupleElement {
        ts_type: unlabeled.strip_suffix('?').unwrap_or(unlabeled).trim(),
        rest,
        optional: labeled_optional || unlabeled.ends_with('?'),
    }
}

/// Statements comparing tuples `a` and `b` element by element, returning the
/// first non-zero result. `compare` renders one comparison of a type between
/// two expressions; a `null` from it (a partial order) is returned as is. A
/// rest element compares the remaining slices as arrays, and tuples equal on
/// every fixed element order by length.
pub fn tuple_compare_statements(
    elements: &[String],
    compare: impl Fn(&str, &str, &str) -> String,
) -> String {
    let mut body = String::new();
    for (index, source) in elements.iter().enumerate() {
        let element = tuple_element(source);
        let (left, right) = if element.rest {
            (format!("a.slice({index})"), format!("b.slice({index})"))
        } else {
            (format!("a[{index}]"), format!("b[{index}]"))
        };
        body.push_str(&format!(
            "const left{index} = {left};\nconst right{index} = {right};\n"
        ));
        let typed = compare(
            element.ts_type,
            &format!("left{index}"),
            &format!("right{index}"),
        );
        // An absent optional element orders before a present one.
        let comparison = if element.optional {
            format!(
                "left{index} === undefined || right{index} === undefined \
                 ? (left{index} === right{index} ? 0 : left{index} === undefined ? -1 : 1) \
                 : {typed}"
            )
        } else {
            typed
        };
        body.push_str(&format!(
            "const cmp{index} = {comparison};\nif (cmp{index} !== 0) return cmp{index};\n"
        ));
        if element.rest {
            return body + "return 0;";
        }
    }
    body + "return a.length < b.length ? -1 : a.length > b.length ? 1 : 0;"
}

/// Check if a TypeScript type is numeric
pub fn is_numeric_type(ts_type: &str) -> bool {
    matches!(ts_type.trim(), "number" | "bigint")
}

/// Check if a TypeScript type is nullable (contains `| null` or `| undefined`)
/// Like Rust's Option<T>, these types default to null.
pub fn is_nullable_type(ts_type: &str) -> bool {
    let normalized = ts_type.replace(' ', "");
    normalized.contains("|null") || normalized.contains("|undefined")
}

/// Check if a type name contains generic parameters (e.g., "RecordLink<Service>")
/// This is used to detect generic type instantiations that need special handling.
pub fn is_generic_type(type_name: &str) -> bool {
    type_name.contains('<') && type_name.contains('>')
}

/// Detect the resolved shape of `RecordLink<T> = string | T` and similar
/// primitive-plus-user-type unions. Returns `(primitive, encodable)` when
/// `ts_type` is a two-member top-level union where exactly one side is a
/// primitive and the other is a user-defined type (uppercase, non-primitive).
///
/// This is the structural shape left behind when `resolve_generic_aliases`
/// expands `RecordLink<ErrandMessage>` into `string | ErrandMessage`.
pub fn detect_primitive_encodable_union(ts_type: &str) -> Option<(String, String)> {
    let parts = split_top_level_union(ts_type.trim())?;
    if parts.len() != 2 {
        return None;
    }
    let left = parts[0].trim();
    let right = parts[1].trim();
    match (
        TypeCategory::from_ts_type(left),
        TypeCategory::from_ts_type(right),
    ) {
        (TypeCategory::Primitive, TypeCategory::Encodable(name)) => Some((left.to_string(), name)),
        (TypeCategory::Encodable(name), TypeCategory::Primitive) => Some((right.to_string(), name)),
        _ => None,
    }
}

/// Extracts base type and type arguments from a generic type.
/// "RecordLink<Service>" -> Some(("RecordLink", "Service"))
/// "Map<string, number>" -> Some(("Map", "string, number"))
/// "User" -> None
pub fn parse_generic_type(type_name: &str) -> Option<(&str, &str)> {
    let open = type_name.find('<')?;
    let close = type_name.rfind('>')?;
    if open < close {
        let base = &type_name[..open];
        let args = &type_name[open + 1..close];
        Some((base.trim(), args.trim()))
    } else {
        None
    }
}

/// Returns whether a type can have a default value generated for it.
///
/// Always returns `true` because all types are assumed to implement Default:
/// primitives and collections have built-in defaults, and custom types are
/// assumed to provide a `{typeName}DefaultValue()` standalone function
/// (following Rust's `derive(Default)` philosophy). This function exists
/// as a named predicate for readability.
pub fn has_known_default(_ts_type: &str) -> bool {
    true
}

/// Get default value for a TypeScript type with no project registry.
/// Used by tests and primitive-only fixtures; production code should always
/// call [`get_type_default_with_registry`] so generic aliases like
/// `RecordLink<T>` expand against the actual project context.
pub fn get_type_default(ts_type: &str) -> String {
    let registry = TypeRegistry::default();
    get_type_default_with_registry(ts_type, &registry, "", &[])
}

/// Get default value for a TypeScript type, resolving generic aliases against
/// the project's [`TypeRegistry`] first. `RecordLink<T>` (and any other
/// user-defined generic alias) expands to its body before the default is
/// chosen, so the emitter never references a nonexistent
/// `{alias}DefaultValue<T>()` helper.
///
/// `caller_file_path` is the file referencing `ts_type` (used to resolve
/// types declared in the same file when the simple name is ambiguous, as
/// happens in generated aggregator files), and `file_imports` come from
/// that same file's import statements.
pub fn get_type_default_with_registry(
    ts_type: &str,
    registry: &TypeRegistry,
    caller_file_path: &str,
    file_imports: &[FileImportEntry],
) -> String {
    let resolved = resolve_generic_aliases(ts_type, registry, caller_file_path, file_imports);
    get_type_default_resolved(&resolved)
}

fn get_type_default_resolved(ts_type: &str) -> String {
    let t = ts_type.trim();

    // Check for foreign type default first
    let foreign_types = get_foreign_types();
    let ft_match = TypeCategory::match_foreign_type(t, &foreign_types);
    // Note: Warnings from near-matches are handled by encode/decode macros
    // which have access to diagnostics
    if let Some(ft) = ft_match.config
        && let Some(ref default_expr) = ft.default_expr
    {
        // Wrap the expression in an IIFE if it's a function
        // Foreign type defaults are expected to be functions: () => DateTime.now()
        // Rewrite namespace references to use generated aliases
        let rewritten = crate::builtin::endec::rewrite_expression_namespaces(default_expr);
        return format!("({})()", rewritten);
    }

    // Nullable first (like Rust's Option::default() -> None)
    if is_nullable_type(t) {
        return "null".to_string();
    }

    // Object literal types: { [key: string]: number }, { foo: string }, etc.
    // Must be checked before union splitting since braces can contain pipes.
    if t.starts_with('{') {
        return "{}".to_string();
    }

    // Handle union types (e.g., string | Account, "Estimate" | "Invoice")
    // Nullable unions (T | null, T | undefined) are already handled above.
    if let Some(parts) = split_top_level_union(t) {
        // 1. If any member is a primitive, use that primitive's default
        for part in &parts {
            if is_primitive_type(part) {
                return get_type_default_resolved(part);
            }
        }
        // 2. If any member is a literal, use the first literal
        for part in &parts {
            let p = part.trim();
            if (p.starts_with('"') && p.ends_with('"'))
                || (p.starts_with('\'') && p.ends_with('\''))
                || (p.starts_with('`') && p.ends_with('`'))
                || p.parse::<f64>().is_ok()
                || matches!(p, "true" | "false")
            {
                return get_type_default_resolved(p);
            }
        }
        // 3. Union of only custom types: default via first member
        return get_type_default_resolved(parts[0]);
    }

    match t {
        "string" => r#""""#.to_string(),
        "number" => "0".to_string(),
        "boolean" => "false".to_string(),
        "bigint" => "0n".to_string(),
        t if t.ends_with("[]") => "[]".to_string(),
        t if t.starts_with("Array<") => "[]".to_string(),
        t if t.starts_with("Map<") => "new Map()".to_string(),
        t if t.starts_with("Set<") => "new Set()".to_string(),
        // Builtin object types
        "Date" => "new Date()".to_string(),
        "RegExp" => "new RegExp(\"\")".to_string(),
        "Error" | "TypeError" | "RangeError" | "SyntaxError" | "ReferenceError" | "URIError"
        | "EvalError" => format!("new {}()", t),
        "Blob" => "new Blob()".to_string(),
        "FormData" => "new FormData()".to_string(),
        "Headers" => "new Headers()".to_string(),
        "URLSearchParams" => "new URLSearchParams()".to_string(),
        "AbortController" => "new AbortController()".to_string(),
        "ArrayBuffer" => "new ArrayBuffer(0)".to_string(),
        "SharedArrayBuffer" => "new SharedArrayBuffer(0)".to_string(),
        // Typed arrays
        "Uint8Array" | "Int8Array" | "Uint16Array" | "Int16Array" | "Uint32Array"
        | "Int32Array" | "Float32Array" | "Float64Array" | "BigInt64Array" | "BigUint64Array"
        | "Uint8ClampedArray" => format!("new {}()", t),
        // Primitive wrappers and special types
        "unknown" | "any" => "undefined".to_string(),
        "void" | "never" => "undefined".to_string(),
        "object" => "({})".to_string(),
        "symbol" => "Symbol()".to_string(),
        "Function" => "(() => {})".to_string(),
        // Built-in generic collection types
        t if t.starts_with("ReadonlyArray<") => "[]".to_string(),
        t if t.starts_with("WeakMap<") => "new WeakMap()".to_string(),
        t if t.starts_with("WeakSet<") => "new WeakSet()".to_string(),
        t if t.starts_with("Promise<") => "Promise.resolve()".to_string(),
        // Built-in generic utility types (all produce object-like values)
        t if crate::ts_syn::type_normalize::is_ts_object_utility_type(
            crate::ts_syn::type_normalize::base_type_name(t),
        ) =>
        {
            "({})".to_string()
        }
        // Generic type instantiations (`RecordLink<T>`, etc.) should have
        // been resolved to their body by `resolve_generic_aliases` before we
        // got here. Any instantiation that reaches this branch is either an
        // unregistered alias or a type we cannot introspect: emit `undefined`
        // rather than a call to a nonexistent `xDefaultValue<T>()` helper.
        t if is_generic_type(t) => "undefined".to_string(),
        // String literal types: "active", 'pending', `template`
        t if (t.starts_with('"') && t.ends_with('"'))
            || (t.starts_with('\'') && t.ends_with('\''))
            || (t.starts_with('`') && t.ends_with('`')) =>
        {
            t.to_string()
        }
        // Number literal types: 42, 3.14
        t if t.parse::<f64>().is_ok() => t.to_string(),
        // Boolean literal types
        "true" | "false" => t.to_string(),
        // Unknown types: assume they implement Default trait
        type_name => format_default_call(type_name),
    }
}

fn format_default_call(type_name: &str) -> String {
    format!("{}DefaultValue()", type_name.to_case(Case::Camel))
}
