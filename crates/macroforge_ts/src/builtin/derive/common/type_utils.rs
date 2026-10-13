use convert_case::{Case, Casing};

use super::value_type::rendered;
use crate::builtin::derive::endec::{TypeCategory, get_foreign_types};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::class::MemberKey;
use crate::ts_syn::abi::ir::interface::InterfaceFieldIR;
use crate::ts_syn::abi::ir::type_registry::TypeDefinitionIR;
use crate::ts_syn::abi::ir::{
    EnumVariantIR, FileImportEntry, TypeRegistry, alias_default_member, is_primitive_keyword,
    resolve_generic_aliases, split_top_level_commas, split_top_level_union,
};
use crate::ts_syn::{TsStream, ts_ident};

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
/// -1, 0 or 1: strings by UTF-16 code units, so the order is `0` exactly when
/// `===` holds. `NaN` is equal to itself; against a number it is unordered,
/// `null`, with `partial`, and after every number without.
pub fn primitive_compare_statements(base: &str, partial: bool) -> String {
    rendered(match base {
        "boolean" => ts_template! { return a === b ? 0 : a ? 1 : -1; },
        "number" if partial => ts_template! {
            return a < b ? -1 : a > b ? 1 : a === b || (a !== a && b !== b) ? 0 : null;
        },
        // `NaN` after every number, and equal to itself.
        "number" => ts_template! {
            return a < b ? -1 : a > b ? 1 : a === b ? 0 : a !== a ? (b !== b ? 0 : 1) : -1;
        },
        _ => ts_template! { return a < b ? -1 : a > b ? 1 : 0; },
    })
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
    let mut steps = Vec::new();
    for (index, source) in elements.iter().enumerate() {
        let element = tuple_element(source);
        let (left, right) = (ts_ident!("left{}", index), ts_ident!("right{}", index));
        let cmp = ts_ident!("cmp{}", index);
        let typed = compare(element.ts_type, &left.sym, &right.sym);
        // An absent optional element orders before a present one.
        let comparison = if element.optional {
            rendered(ts_template! {
                @{&left} === undefined || @{&right} === undefined
                    ? (@{&left} === @{&right} ? 0 : @{&left} === undefined ? -1 : 1)
                    : @{typed}
            })
        } else {
            typed
        };
        steps.push(if element.rest {
            ts_template! {
                const @{&left} = a.slice(@{index});
                const @{&right} = b.slice(@{index});
                const @{&cmp} = @{comparison};
                if (@{&cmp} !== 0) return @{&cmp};
                return 0;
            }
        } else {
            ts_template! {
                const @{&left} = a[@{index}];
                const @{&right} = b[@{index}];
                const @{&cmp} = @{comparison};
                if (@{&cmp} !== 0) return @{&cmp};
            }
        });
        if element.rest {
            return rendered(TsStream::merge_all(steps));
        }
    }
    steps.push(ts_template! { return a.length < b.length ? -1 : a.length > b.length ? 1 : 0; });
    rendered(TsStream::merge_all(steps))
}

/// Check if a TypeScript type is numeric
pub fn is_numeric_type(ts_type: &str) -> bool {
    matches!(ts_type.trim(), "number" | "bigint")
}

/// Check if a TypeScript type is nullable: a union with `null` or `undefined`
/// among its own members. A collection of nullable elements is not.
pub fn is_nullable_type(ts_type: &str) -> bool {
    has_union_member(ts_type, "null") || has_union_member(ts_type, "undefined")
}

/// Whether `ts_type` is a union with `member` among its own members.
fn has_union_member(ts_type: &str, member: &str) -> bool {
    split_top_level_union(ts_type.trim()).is_some_and(|parts| parts.contains(&member))
}

/// Which kinds of value an enum's members have, so a generator emits only the
/// branches the enum's type has values for.
pub struct EnumValueKinds {
    /// A member with a string literal value.
    pub strings: bool,
    /// A member with a numeric value: a literal, an auto-incremented one, or
    /// a computed one, which TypeScript types as a number.
    pub numbers: bool,
}

impl EnumValueKinds {
    pub fn of(variants: &[EnumVariantIR]) -> Self {
        Self {
            strings: variants.iter().any(|variant| variant.value.is_string()),
            numbers: variants.iter().any(|variant| !variant.value.is_string()),
        }
    }
}

/// A runtime test that `value` is of the primitive type `keyword` names:
/// `typeof` where the keyword is its own `typeof` result, and identity for
/// `null` and `undefined`, which have no `typeof` of their own name.
pub fn primitive_keyword_test(keyword: &str, value: &str) -> String {
    let keyword = keyword.trim();
    if matches!(keyword, "null" | "undefined") {
        return rendered(ts_template! { @{value} === @{keyword} });
    }
    let kind = super::js_string(keyword);
    rendered(ts_template! { typeof @{value} === @{kind} })
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

/// The element type of an array type: `T[]`, `readonly T[]`, `Array<T>` or
/// `ReadonlyArray<T>`. `None` for anything else, including a union such as
/// `string | number[]`, which merely ends with an array member.
pub fn array_element_type(ts_type: &str) -> Option<&str> {
    let trimmed = ts_type.trim();
    if split_top_level_union(trimmed).is_some() {
        return None;
    }
    let trimmed = trimmed
        .strip_prefix("readonly ")
        .map_or(trimmed, str::trim_start);
    if let Some(element) = trimmed.strip_suffix("[]") {
        let element = element.trim();
        return Some(
            element
                .strip_prefix('(')
                .and_then(|inner| inner.strip_suffix(')'))
                .map_or(element, str::trim),
        );
    }
    let (base, element) = parse_generic_type(trimmed)?;
    (trimmed.ends_with('>') && matches!(base, "Array" | "ReadonlyArray")).then_some(element)
}

/// The text between the brackets of a tuple type, `readonly` or not. `None`
/// for any other type, an array of tuples such as `[A, B][]` included.
pub fn tuple_body(ts_type: &str) -> Option<&str> {
    let trimmed = ts_type.trim();
    let tuple = trimmed
        .strip_prefix("readonly ")
        .map_or(trimmed, str::trim_start);
    let body = tuple.strip_prefix('[')?.strip_suffix(']')?;
    // The bracket that opens the type has to be the one that closes it.
    let mut depth = 0usize;
    for byte in body.bytes() {
        match byte {
            b'[' => depth += 1,
            b']' => depth = depth.checked_sub(1)?,
            _ => {}
        }
    }
    (depth == 0).then_some(body)
}

/// Returns whether a type can have a default value generated for it.
///
/// Primitives and collections have built-in defaults, and custom types are
/// assumed to provide a `{typeName}DefaultValue()` standalone function
/// (following Rust's `derive(Default)` philosophy). A function or constructor
/// type has none, as a Rust `fn` pointer does not implement `Default`.
pub fn has_known_default(ts_type: &str) -> bool {
    !is_function_type(ts_type)
}

/// Whether `ts_type` is a function or constructor type, parenthesized or not.
fn is_function_type(ts_type: &str) -> bool {
    use oxc::allocator::Allocator;
    use oxc::ast::ast::{Statement, TSType};

    let allocator = Allocator::default();
    let source = format!("type __T = {ts_type};");
    let parsed = oxc::parser::Parser::new(&allocator, &source, oxc::span::SourceType::ts()).parse();
    let Some(Statement::TSTypeAliasDeclaration(alias)) = parsed.program.body.first() else {
        return false;
    };
    let mut ty = &alias.type_annotation;
    while let TSType::TSParenthesizedType(inner) = ty {
        ty = &inner.type_annotation;
    }
    matches!(ty, TSType::TSFunctionType(_) | TSType::TSConstructorType(_))
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
    // A generic alias that marks its default member defaults to that member.
    if let Some(member) = alias_default_member(ts_type, registry, caller_file_path, file_imports) {
        return get_type_default_with_registry(&member, registry, caller_file_path, file_imports);
    }
    let resolved = resolve_generic_aliases(ts_type, registry, caller_file_path, file_imports);
    get_type_default_resolved(
        &resolved,
        &Names {
            registry,
            file_path: caller_file_path,
            file_imports,
        },
    )
}

/// What resolves the names in a type whose default is being written.
struct Names<'a> {
    registry: &'a TypeRegistry,
    file_path: &'a str,
    file_imports: &'a [FileImportEntry],
}

/// A key of a `Record` over a finite set of keys.
enum RecordKey {
    /// A literal, as its type writes it: `'a'`, `1`.
    Literal(String),
    /// An enum's member, as an expression: `Status.Active`.
    Member(String),
}

/// The text of each key a `Record<K, V>` has to have when `K` is a finite
/// set of keys, as an expression: a union of literals, an alias of one, or
/// an enum. `None` for a key type such as `string`, which names no key.
pub fn record_key_texts(
    key_type: &str,
    registry: &TypeRegistry,
    file_path: &str,
    file_imports: &[FileImportEntry],
) -> Option<Vec<String>> {
    let names = Names {
        registry,
        file_path,
        file_imports,
    };
    Some(
        finite_record_keys(key_type, &names)?
            .into_iter()
            .map(|key| match key {
                RecordKey::Literal(value) | RecordKey::Member(value) => {
                    rendered(ts_template! { String(@{value}) })
                }
            })
            .collect(),
    )
}

/// [`finite_record_keys`], each as it is written in an object literal.
fn record_keys(key_type: &str, names: &Names<'_>) -> Option<Vec<String>> {
    Some(
        finite_record_keys(key_type, names)?
            .into_iter()
            .map(|key| match key {
                RecordKey::Literal(literal) => literal,
                RecordKey::Member(member) => rendered(ts_template! { [@{member}] }),
            })
            .collect(),
    )
}

/// The keys of `Record<K, V>` when `K` is a finite set of them: a union of
/// literals, an alias of one, or an enum. `None` for a key type such as
/// `string`, which names no key.
fn finite_record_keys(key_type: &str, names: &Names<'_>) -> Option<Vec<RecordKey>> {
    let key_type = key_type.trim();
    let literal_key = |member: &str| {
        let member = member.trim();
        let is_literal = member.starts_with(['"', '\''])
            || member.parse::<f64>().is_ok()
            || matches!(member, "true" | "false");
        is_literal.then(|| RecordKey::Literal(member.to_string()))
    };
    let members = split_top_level_union(key_type).unwrap_or_else(|| vec![key_type]);
    if let Some(keys) = members.iter().map(|member| literal_key(member)).collect() {
        return Some(keys);
    }
    let entry = names
        .registry
        .resolve_in_file(key_type, names.file_path, names.file_imports)?;
    match &entry.definition {
        TypeDefinitionIR::Enum(declared) => Some(
            declared
                .variants
                .iter()
                .map(|variant| {
                    let member = super::Member::of(&variant.name);
                    RecordKey::Member(rendered(ts_template! { @{key_type}@{member} }))
                })
                .collect(),
        ),
        TypeDefinitionIR::TypeAlias(alias) => alias
            .body
            .as_union()?
            .iter()
            .map(|member| member.as_literal().and_then(literal_key))
            .collect(),
        TypeDefinitionIR::Class(_) | TypeDefinitionIR::Interface(_) => None,
    }
}

fn get_type_default_resolved(ts_type: &str, names: &Names<'_>) -> String {
    let t = ts_type.trim();
    // A readonly tuple or array defaults as the mutable one does.
    let t = t.strip_prefix("readonly ").map_or(t, str::trim_start);

    // Check for foreign type default first
    let foreign_types = get_foreign_types();
    let ft_match = TypeCategory::match_foreign_type(t, &foreign_types);
    // Note: Warnings from near-matches are handled by encode/decode macros
    // which have access to diagnostics
    // A foreign type's default is a function returning a fresh value.
    if let Some(callee) = ft_match
        .config
        .and_then(|ft| ft.handler_callee(crate::ts_syn::config::ForeignHandler::Default))
    {
        return rendered(ts_template! { @{callee}() });
    }

    // Nullable first, like Rust's `Option::default()`: the absent value the
    // type itself has.
    if has_union_member(t, "null") {
        return "null".to_string();
    }
    if has_union_member(t, "undefined") {
        return "undefined".to_string();
    }

    // A tuple of fixed length defaults element by element. One with an
    // optional or rest element defaults to its required prefix.
    if let Some(elements) = tuple_body(t) {
        let defaults: Vec<String> = split_top_level_commas(elements)
            .into_iter()
            .filter(|source| !source.is_empty())
            .map(tuple_element)
            .take_while(|element| !element.rest && !element.optional)
            .map(|element| get_type_default_resolved(element.ts_type, names))
            .collect();
        let defaults = defaults.join(", ");
        return rendered(ts_template! { [@{defaults}] });
    }

    // An object type written in place: each member it has to have, by its
    // own default. An index signature names no member, so it adds none.
    // Checked before union splitting since braces can contain pipes.
    if t.starts_with('{') {
        return match crate::builtin::derive::endec::inline_object_fields(t) {
            Some(fields) => object_default_literal(
                fields.iter().filter(|field| field.key == MemberKey::Named),
                names,
            ),
            None => "({})".to_string(),
        };
    }

    // Handle union types (e.g., string | Account, "Estimate" | "Invoice")
    // Nullable unions (T | null, T | undefined) are already handled above.
    if let Some(parts) = split_top_level_union(t) {
        // 1. If any member is a primitive, use that primitive's default
        for part in &parts {
            if is_primitive_type(part) {
                return get_type_default_resolved(part, names);
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
                return get_type_default_resolved(p, names);
            }
        }
        // 3. Union of only custom types: default via first member
        return get_type_default_resolved(parts[0], names);
    }

    match t {
        "string" => r#""""#.to_string(),
        "number" => "0".to_string(),
        "boolean" => "false".to_string(),
        "bigint" => "0n".to_string(),
        t if t.ends_with("[]") => "[]".to_string(),
        t if t.starts_with("Array<") => "[]".to_string(),
        t if t.starts_with("Map<") || t.starts_with("ReadonlyMap<") => "new Map()".to_string(),
        t if t.starts_with("Set<") || t.starts_with("ReadonlySet<") => "new Set()".to_string(),
        // Builtin object types
        "Date" => "new Date()".to_string(),
        "RegExp" => "new RegExp(\"\")".to_string(),
        "Error" | "TypeError" | "RangeError" | "SyntaxError" | "ReferenceError" | "URIError"
        | "EvalError" => rendered(ts_template! { new @{t}() }),
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
        | "Uint8ClampedArray" => rendered(ts_template! { new @{t}() }),
        // Primitive wrappers and special types
        "unknown" | "any" => "undefined".to_string(),
        "void" | "never" => "undefined".to_string(),
        "null" => "null".to_string(),
        "undefined" => "undefined".to_string(),
        "object" => "({})".to_string(),
        "symbol" => "Symbol()".to_string(),
        "Function" => "(() => {})".to_string(),
        // Built-in generic collection types
        t if t.starts_with("ReadonlyArray<") => "[]".to_string(),
        t if t.starts_with("WeakMap<") => "new WeakMap()".to_string(),
        t if t.starts_with("WeakSet<") => "new WeakSet()".to_string(),
        t if t.starts_with("Promise<") => "Promise.resolve()".to_string(),
        // A record over a finite set of keys has each of them.
        t if t.starts_with("Record<") && t.ends_with('>') => {
            let arguments = split_top_level_commas(&t["Record<".len()..t.len() - 1]);
            let keys = match arguments.as_slice() {
                [key_type, _] => record_keys(key_type, names),
                _ => None,
            };
            match (keys, arguments.get(1)) {
                (Some(keys), Some(value_type)) => {
                    let value = get_type_default_resolved(value_type, names);
                    let members: Vec<String> = keys
                        .iter()
                        .map(|key| rendered(ts_template! { @{key}: @{&value} }))
                        .collect();
                    let members = members.join(", ");
                    rendered(ts_template! { ({ @{members} }) })
                }
                _ => "({})".to_string(),
            }
        }
        // A readonly view of a type defaults as the type does.
        t if t.starts_with("Readonly<") && t.ends_with('>') => {
            get_type_default_resolved(&t["Readonly<".len()..t.len() - 1], names)
        }
        // `Required<T>` and `NonNullable<T>` default as `T` does, once the
        // `null` and `undefined` a `NonNullable` rules out are set aside.
        t if (t.starts_with("Required<") || t.starts_with("NonNullable<")) && t.ends_with('>') => {
            let inner = &t[t.find('<').map_or(0, |open| open + 1)..t.len() - 1];
            let present: Vec<&str> = split_top_level_union(inner)
                .unwrap_or_else(|| vec![inner])
                .into_iter()
                .filter(|member| !matches!(member.trim(), "null" | "undefined"))
                .collect();
            get_type_default_resolved(&present.join(" | "), names)
        }
        // A `Pick` or `Omit` holds the fields of its type it keeps, each by
        // its own default.
        t if (t.starts_with("Pick<") || t.starts_with("Omit<")) && t.ends_with('>') => {
            match selected_fields(t, names) {
                Some(fields) => object_default_literal(fields.iter(), names),
                None => "({})".to_string(),
            }
        }
        // Built-in generic utility types (all produce object-like values)
        t if crate::ts_syn::type_normalize::is_ts_object_utility_type(
            crate::ts_syn::type_normalize::base_type_name(t),
        ) =>
        {
            "({})".to_string()
        }
        // A generic type the registry holds deriving `Default` defaults
        // through its own function, given the same arguments. An alias that
        // could be written out was, by `resolve_generic_aliases`, before
        // this, and one the registry does not hold has no function to call.
        t if is_generic_type(t) => match parse_generic_type(t) {
            Some((base, arguments))
                if names
                    .registry
                    .resolve_in_file(base, names.file_path, names.file_imports)
                    .is_some()
                    && super::type_has_derive(names.registry, base, "Default") =>
            {
                let callee = ts_ident!("{}DefaultValue", base.to_case(Case::Camel));
                rendered(ts_template! { @{callee}<@{arguments}>() })
            }
            _ => "undefined".to_string(),
        },
        // A template literal type defaults to its text, each hole filled.
        t if t.starts_with('`') && t.ends_with('`') => match template_default(t) {
            Some(Ok(text)) => text,
            Some(Err(_)) | None => t.to_string(),
        },
        // String literal types: "active", 'pending'
        t if (t.starts_with('"') && t.ends_with('"'))
            || (t.starts_with('\'') && t.ends_with('\'')) =>
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

/// An object literal holding each of `fields` that a value has to have, by
/// its own default.
fn object_default_literal<'f>(
    fields: impl Iterator<Item = &'f InterfaceFieldIR>,
    names: &Names<'_>,
) -> String {
    let members: Vec<String> = fields
        .filter(|field| !field.optional)
        .map(|field| {
            let key = super::property_key(&field.name);
            let default = get_type_default_resolved(&field.ts_type, names);
            rendered(ts_template! { @{key}: @{default} })
        })
        .collect();
    if members.is_empty() {
        return "({})".to_string();
    }
    let members = members.join(", ");
    rendered(ts_template! { ({ @{members} }) })
}

/// The fields a `Pick<T, K>` or an `Omit<T, K>` keeps of `T`, when the
/// registry holds `T` and the keys are written as literals.
fn selected_fields(ts_type: &str, names: &Names<'_>) -> Option<Vec<InterfaceFieldIR>> {
    use crate::builtin::derive::endec::WrapperFields;

    let (_, arguments) = parse_generic_type(ts_type)?;
    let inner = split_top_level_commas(arguments).into_iter().next()?;
    let entry =
        names
            .registry
            .resolve_in_file(inner.trim(), names.file_path, names.file_imports)?;
    let fields = super::fields_from_definition(&entry.definition, names.registry)?;
    let keeps: Box<dyn Fn(&str) -> bool> = match WrapperFields::of(ts_type)?.ok()? {
        WrapperFields::Only(kept) => Box::new(move |name| kept.iter().any(|key| key == name)),
        WrapperFields::Except(left) => Box::new(move |name| !left.iter().any(|key| key == name)),
        WrapperFields::Whole | WrapperFields::Any => return None,
    };
    Some(
        fields
            .into_iter()
            .filter(|field| keeps(&field.name))
            .collect(),
    )
}

/// The default text of a template literal type: each `${...}` hole filled
/// with the default of the primitive it names. `None` when `ts_type` is no
/// template literal type, and the hole when one names anything else.
fn template_default(ts_type: &str) -> Option<Result<String, String>> {
    let body = ts_type.strip_prefix('`')?.strip_suffix('`')?;
    let mut text = String::from("`");
    let mut rest = body;
    while let Some((before, after)) = rest.split_once("${") {
        let (hole, tail) = after.split_once('}')?;
        text.push_str(before);
        text.push_str(match hole.trim() {
            "string" => "",
            "number" | "bigint" => "0",
            "boolean" => "false",
            "null" => "null",
            "undefined" => "undefined",
            other => return Some(Err(other.to_string())),
        });
        rest = tail;
    }
    text.push_str(rest);
    text.push('`');
    Some(Ok(text))
}

/// Why a field of type `ts_type` has no default the derive can write, when
/// it has none: a `Required` whose type leaves fields optional, which a
/// default of that type lacks, and a `Pick` or `Omit` of a type whose
/// fields are not known.
pub fn undefaultable(ts_type: &str, registry: &TypeRegistry) -> Option<String> {
    // The field's own file is not known here, so a name resolves as the
    // registry holds it.
    let names = &Names {
        registry,
        file_path: "",
        file_imports: &[],
    };
    let ts_type = ts_type.trim();
    if let Some(Err(hole)) = template_default(ts_type) {
        return Some(format!(
            "has type `{ts_type}`, whose `${{{hole}}}` names no text a default could hold"
        ));
    }
    if !ts_type.ends_with('>') {
        return None;
    }
    let (base, arguments) = parse_generic_type(ts_type)?;
    match base {
        "Pick" | "Omit" => selected_fields(ts_type, names).is_none().then(|| {
            format!("has type `{ts_type}`, a selection of fields that are not known here")
        }),
        "Required" => {
            let entry = names.registry.resolve_in_file(
                arguments.trim(),
                names.file_path,
                names.file_imports,
            )?;
            let fields = super::fields_from_definition(&entry.definition, names.registry)?;
            let optional: Vec<&str> = fields
                .iter()
                .filter(|field| field.optional)
                .map(|field| field.name.as_str())
                .collect();
            (!optional.is_empty()).then(|| {
                format!(
                    "has type `{ts_type}`, whose default would have to hold `{}`, which '{}' leaves optional",
                    optional.join("`, `"),
                    arguments.trim()
                )
            })
        }
        _ => None,
    }
}

fn format_default_call(type_name: &str) -> String {
    let callee = ts_ident!("{}DefaultValue", type_name.to_case(Case::Camel));
    rendered(ts_template! { @{callee}() })
}
