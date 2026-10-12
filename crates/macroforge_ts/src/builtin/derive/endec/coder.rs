//! How a value of any type is encoded and decoded, by the structure of its
//! type: a collection through its elements, a nullable type through its
//! present member, a user type through that type's own coders.
//!
//! Both directions return `None` for a type whose values are their own
//! encoding, so a caller keeps the value as it is.

use crate::builtin::derive::common::{
    EnumValueKinds, Member, fields_from_definition, is_identifier_name, js_string, property_key,
    rendered, tuple_body, tuple_element, type_has_derive,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::interface::InterfaceFieldIR;
use crate::ts_syn::abi::ir::type_registry::{FileImportEntry, TypeDefinitionIR, TypeRegistry};
use crate::ts_syn::abi::ir::{split_top_level_commas, split_top_level_union, typeof_primitive};
use crate::ts_syn::config::ForeignHandler;

use super::decode::helpers::primitive_accepts;
use super::nested::NestedType;
use super::union_guard::{Direction, UnionGuard, union_guard};
use super::{EndecContainerOptions, EndecFieldOptions, TypeCategory, get_foreign_types};

/// Where a field's type is read: the registry and file that resolve the names
/// in it, and the type parameters in scope, whose values pass through uncoded.
pub(crate) struct TypeScope<'a> {
    pub(crate) registry: &'a TypeRegistry,
    pub(crate) file_path: &'a str,
    pub(crate) file_imports: &'a [FileImportEntry],
    pub(crate) type_params: &'a [String],
}

impl TypeScope<'_> {
    pub(crate) fn is_param(&self, name: &str) -> bool {
        self.type_params.iter().any(|param| param == name)
    }

    fn nested(&self, ts_type: &str) -> Result<Option<NestedType>, String> {
        NestedType::of(ts_type, &|name| self.is_param(name))
    }

    /// `name` when it names a type the registry holds deriving the coder
    /// `direction` calls. A type named in lowercase is only told from an
    /// unknown word this way.
    pub(crate) fn derived(&self, name: &str, direction: Direction) -> Option<String> {
        let name = name.trim();
        let derive = match direction {
            Direction::Encode => "Encode",
            Direction::Decode => "Decode",
        };
        (is_identifier_name(name)
            && !self.is_param(name)
            && self
                .registry
                .resolve_in_file(name, self.file_path, self.file_imports)
                .is_some()
            && type_has_derive(self.registry, name, derive))
        .then(|| name.to_string())
    }

    fn guard(
        &self,
        ts_type: &str,
        direction: Direction,
    ) -> Result<Option<(UnionGuard, String)>, String> {
        union_guard(ts_type, direction, |name| self.is_param(name))
    }
}

/// `ts_type` without the parentheses that only group it.
fn ungrouped(ts_type: &str) -> &str {
    let trimmed = ts_type.trim();
    match trimmed
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
    {
        Some(inner) if !inner.contains("=>") && balanced(inner) => ungrouped(inner),
        _ => trimmed,
    }
}

/// Whether every `)` in `text` closes a `(` in it, so the pair stripped
/// around it was one pair.
fn balanced(text: &str) -> bool {
    let mut depth = 0usize;
    for byte in text.bytes() {
        match byte {
            b'(' => depth += 1,
            b')' => match depth.checked_sub(1) {
                Some(open) => depth = open,
                None => return false,
            },
            _ => {}
        }
    }
    depth == 0
}

/// The element sources of a tuple type, `readonly` or not.
fn tuple_elements(ts_type: &str) -> Option<Vec<&str>> {
    Some(
        split_top_level_commas(tuple_body(ts_type)?)
            .into_iter()
            .filter(|source| !source.is_empty())
            .collect(),
    )
}

/// The members of an object type written out in place, such as
/// `{ at: Date; note?: string }`. `None` for any other type, and for one
/// with a member that is not a plain property, an index signature included:
/// its values pass through as they are.
pub(crate) fn object_members(ts_type: &str) -> Option<Vec<InterfaceFieldIR>> {
    use oxc::ast::ast::{Statement, TSType};

    if !ts_type.trim_start().starts_with('{') {
        return None;
    }
    let allocator = oxc::allocator::Allocator::default();
    let source = format!("type __T = {ts_type};");
    let parsed = oxc::parser::Parser::new(&allocator, &source, oxc::span::SourceType::ts()).parse();
    let [Statement::TSTypeAliasDeclaration(declared)] = parsed.program.body.as_slice() else {
        return None;
    };
    let TSType::TSTypeLiteral(literal) = &declared.type_annotation else {
        return None;
    };
    let aliases = crate::ts_syn::lower_type_aliases(&parsed.program, &source).ok()?;
    let [alias] = aliases.as_slice() else {
        return None;
    };
    let fields = alias.body.as_object()?;
    (fields.len() == literal.members.len()).then(|| fields.to_vec())
}

/// Whether a member of an object type may be absent from its encoding: one
/// declared optional, or typed to admit `undefined`, which JSON leaves out.
fn member_may_be_absent(member: &InterfaceFieldIR) -> bool {
    member.optional
        || matches!(
            TypeCategory::from_ts_type(&member.ts_type),
            TypeCategory::Optional(_)
        )
}

fn encode_object(
    members: &[InterfaceFieldIR],
    value: &str,
    scope: &TypeScope<'_>,
    depth: usize,
) -> Result<Option<String>, String> {
    let mut encoded_members = Vec::new();
    for member in members {
        let held = rendered(ts_template! { @{value}@{Member::of(&member.name)} });
        let Some(encoded) = encode_expr(&member.ts_type, &held, scope, depth)? else {
            continue;
        };
        let key = property_key(&member.name);
        encoded_members.push(if member.optional {
            rendered(ts_template! { @{key}: (@{&held} === undefined ? undefined : @{encoded}) })
        } else {
            rendered(ts_template! { @{key}: @{encoded} })
        });
    }
    if encoded_members.is_empty() {
        return Ok(None);
    }
    let encoded_members = encoded_members.join(", ");
    Ok(Some(rendered(
        ts_template! { ({ ...@{value}, @{encoded_members} }) },
    )))
}

/// A call decoding `raw` as the object type with `members`, each member by
/// its own type. `None` when no member's type has a decoder, so the object
/// is taken as it is.
fn decode_object(
    members: &[InterfaceFieldIR],
    raw: &str,
    scope: &TypeScope<'_>,
    depth: usize,
) -> Result<Option<String>, String> {
    for member in members {
        if decode_expr(&member.ts_type, "__probe", scope, depth + 1)?.is_some() {
            return decode_members(members, raw, scope, depth).map(Some);
        }
    }
    Ok(None)
}

/// A call decoding `raw` into an object of exactly `members`, each by its own
/// type, and a member whose type has no decoder as it is.
pub(crate) fn decode_members(
    members: &[InterfaceFieldIR],
    raw: &str,
    scope: &TypeScope<'_>,
    depth: usize,
) -> Result<String, String> {
    let mut decoders = Vec::with_capacity(members.len());
    for member in members {
        let key = property_key(&member.name);
        let decode = decoder(&member.ts_type, scope, depth)?;
        decoders.push(rendered(ts_template! { @{key}: @{decode} }));
    }
    let optional: Vec<String> = members
        .iter()
        .filter(|member| member_may_be_absent(member))
        .map(|member| js_string(&member.name))
        .collect();
    let decode_object = endec_helper("decodeObject");
    let decoders = decoders.join(", ");
    if optional.is_empty() {
        return Ok(rendered(ts_template! {
            @{decode_object}(@{raw}, ctx, { @{decoders} })
        }));
    }
    let optional = optional.join(", ");
    Ok(rendered(ts_template! {
        @{decode_object}(@{raw}, ctx, { @{decoders} }, [@{optional}])
    }))
}

/// The handler a configured foreign type codes its values with.
fn foreign_callee(ts_type: &str, handler: ForeignHandler) -> Option<String> {
    let foreign_types = get_foreign_types();
    TypeCategory::match_foreign_type(ts_type, &foreign_types)
        .config
        .and_then(|config| config.handler_callee(handler))
}

/// What a `Map`'s keys are in the JSON object the map is encoded as, where
/// every key is text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KeyBase {
    Text,
    Number,
    BigInt,
    Boolean,
    Date,
}

/// A `Map`'s key type: what its values are spelled from, and the user type
/// that validates them, when it derives `Decode`.
struct MapKey {
    base: KeyBase,
    decoder: Option<NestedType>,
}

fn unsupported_key(key_type: &str) -> String {
    format!(
        "a `Map` keyed by `{key_type}` has no JSON encoding: a key has to be a string, a number, a bigint, a boolean, a `Date`, or an enum or alias of one of those"
    )
}

fn map_key(key_type: &str, scope: &TypeScope<'_>) -> Result<MapKey, String> {
    let key_type = ungrouped(key_type);
    let plain = |base| {
        Ok(MapKey {
            base,
            decoder: None,
        })
    };
    match key_type {
        "string" => return plain(KeyBase::Text),
        "number" => return plain(KeyBase::Number),
        "bigint" => return plain(KeyBase::BigInt),
        "boolean" | "true" | "false" => return plain(KeyBase::Boolean),
        "Date" => return plain(KeyBase::Date),
        _ => {}
    }
    // A literal, or a union of literals of one kind.
    let members = split_top_level_union(key_type).unwrap_or_else(|| vec![key_type]);
    let literal_base = |member: &str| {
        if member.starts_with(['"', '\'', '`']) {
            Some(KeyBase::Text)
        } else if member.parse::<f64>().is_ok() {
            Some(KeyBase::Number)
        } else {
            match member {
                "string" => Some(KeyBase::Text),
                "number" => Some(KeyBase::Number),
                "true" | "false" | "boolean" => Some(KeyBase::Boolean),
                _ => None,
            }
        }
    };
    let bases: Option<Vec<KeyBase>> = members.iter().map(|member| literal_base(member)).collect();
    if let Some(bases) = bases {
        return match bases.split_first() {
            Some((first, rest)) if rest.iter().all(|base| base == first) => plain(*first),
            _ => Err(unsupported_key(key_type)),
        };
    }
    if scope.is_param(key_type) {
        return plain(KeyBase::Text);
    }
    let TypeCategory::Encodable(name) = TypeCategory::from_ts_type(key_type) else {
        return Err(unsupported_key(key_type));
    };
    // A type the registry does not hold is taken at its word as text.
    let Some(entry) = scope
        .registry
        .resolve_in_file(&name, scope.file_path, scope.file_imports)
    else {
        return plain(KeyBase::Text);
    };
    let base = match &entry.definition {
        TypeDefinitionIR::Enum(declared) => {
            let kinds = EnumValueKinds::of(&declared.variants);
            match (kinds.strings, kinds.numbers) {
                (true, false) => KeyBase::Text,
                (false, true) => KeyBase::Number,
                (true, true) | (false, false) => return Err(unsupported_key(key_type)),
            }
        }
        TypeDefinitionIR::TypeAlias(alias) => match alias.body.primitive_base() {
            Some("string") => KeyBase::Text,
            Some("number") => KeyBase::Number,
            Some("bigint") => KeyBase::BigInt,
            Some("boolean") => KeyBase::Boolean,
            _ if alias.body.as_union().is_some() => {
                return map_key_of_literals(key_type, alias.body.as_union(), &name);
            }
            _ => return Err(unsupported_key(key_type)),
        },
        TypeDefinitionIR::Class(_) | TypeDefinitionIR::Interface(_) => {
            return Err(unsupported_key(key_type));
        }
    };
    Ok(MapKey {
        base,
        decoder: type_has_derive(scope.registry, &name, "Decode").then(|| NestedType::whole(name)),
    })
}

/// The key of a map keyed by `name`, an alias of a union: text when every
/// member is a string literal, which is all a JSON key can tell apart.
fn map_key_of_literals(
    key_type: &str,
    members: Option<&[crate::ts_syn::abi::ir::type_alias::TypeMember]>,
    name: &str,
) -> Result<MapKey, String> {
    let all_text = members.is_some_and(|members| {
        members.iter().all(|member| {
            member
                .as_literal()
                .is_some_and(|literal| literal.starts_with(['"', '\'', '`']))
        })
    });
    if all_text {
        Ok(MapKey {
            base: KeyBase::Text,
            decoder: Some(NestedType::whole(name.to_string())),
        })
    } else {
        Err(unsupported_key(key_type))
    }
}

/// An expression encoding `value`, of type `ts_type`, with `ctx` in scope.
/// `depth` numbers the locals of nested callbacks apart.
pub(crate) fn encode_expr(
    ts_type: &str,
    value: &str,
    scope: &TypeScope<'_>,
    depth: usize,
) -> Result<Option<String>, String> {
    let ts_type = ungrouped(ts_type);
    if let Some(callee) = foreign_callee(ts_type, ForeignHandler::Encode) {
        return Ok(Some(rendered(ts_template! { @{callee}(@{value}) })));
    }
    if let Some(elements) = tuple_elements(ts_type) {
        return encode_tuple(&elements, value, scope, depth);
    }
    if let Some(members) = object_members(ts_type) {
        return encode_object(&members, value, scope, depth);
    }
    let item = format!("__item{depth}");
    let key = format!("__key{depth}");
    let encoded_item = |item_type: &str| encode_expr(item_type, &item, scope, depth + 1);
    Ok(match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Primitive => None,
        TypeCategory::Date => Some(rendered(ts_template! { @{value}.toISOString() })),
        TypeCategory::Array(element) => encoded_item(&element)?
            .map(|encoded| rendered(ts_template! { @{value}.map((@{&item}) => @{encoded}) })),
        TypeCategory::Set(element) => Some(match encoded_item(&element)? {
            Some(encoded) => {
                rendered(ts_template! { Array.from(@{value}, (@{&item}) => @{encoded}) })
            }
            None => rendered(ts_template! { Array.from(@{value}) }),
        }),
        TypeCategory::Map(key_type, value_type) => {
            let dated = map_key(&key_type, scope)?.base == KeyBase::Date;
            Some(match (dated, encoded_item(&value_type)?) {
                (false, None) => rendered(ts_template! { Object.fromEntries(@{value}.entries()) }),
                (dated, encoded) => {
                    let encoded_key = if dated {
                        rendered(ts_template! { @{&key}.toISOString() })
                    } else {
                        key.clone()
                    };
                    let encoded = encoded.unwrap_or_else(|| item.clone());
                    rendered(ts_template! {
                        Object.fromEntries(Array.from(@{value}, ([@{&key}, @{&item}]) => [@{encoded_key}, @{encoded}]))
                    })
                }
            })
        }
        TypeCategory::Record(_, value_type) => encoded_item(&value_type)?.map(|encoded| {
            rendered(ts_template! {
                Object.fromEntries(Object.entries(@{value}).map(([@{&key}, @{&item}]) => [@{&key}, @{encoded}]))
            })
        }),
        TypeCategory::Optional(inner) => encode_expr(&inner, value, scope, depth)?
            .map(|encoded| rendered(ts_template! { (@{value} === undefined ? undefined : @{encoded}) })),
        TypeCategory::Nullable(inner) => encode_expr(&inner, value, scope, depth)?
            .map(|encoded| rendered(ts_template! { (@{value} === null ? null : @{encoded}) })),
        TypeCategory::Encodable(_) | TypeCategory::Wrapper(_) => scope
            .nested(ts_type)?
            .map(|nested| nested.encode_call(value)),
        TypeCategory::Unknown if scope.derived(ts_type, Direction::Encode).is_some() => {
            Some(NestedType::whole(ts_type.to_string()).encode_call(value))
        }
        TypeCategory::Unknown => match scope.guard(ts_type, Direction::Encode)? {
            Some((guard, name)) => {
                let value_expr = crate::ast::Expr::Source(value.to_string());
                let (is_member, member_value) =
                    (guard.test(&value_expr), guard.convert(&value_expr));
                let encoded = NestedType::whole(name).encode_call(value);
                Some(rendered(
                    ts_template! { (@{is_member} ? @{member_value} : @{encoded}) },
                ))
            }
            None => None,
        },
    })
}

fn encode_tuple(
    elements: &[&str],
    value: &str,
    scope: &TypeScope<'_>,
    depth: usize,
) -> Result<Option<String>, String> {
    let mut encoded_elements = Vec::with_capacity(elements.len());
    let mut any_encoded = false;
    for (index, source) in elements.iter().enumerate() {
        let element = tuple_element(source);
        if element.rest {
            let rest = rendered(ts_template! { @{value}.slice(@{index}) });
            let encoded = encode_expr(element.ts_type, &rest, scope, depth)?;
            any_encoded |= encoded.is_some();
            let encoded = encoded.unwrap_or(rest);
            encoded_elements.push(rendered(ts_template! { ...@{encoded} }));
            continue;
        }
        let at = rendered(ts_template! { @{value}[@{index}] });
        let encoded = encode_expr(element.ts_type, &at, scope, depth)?;
        any_encoded |= encoded.is_some();
        encoded_elements.push(match encoded {
            Some(encoded) if element.optional => {
                rendered(ts_template! { (@{&at} === undefined ? undefined : @{encoded}) })
            }
            Some(encoded) => encoded,
            None => at,
        });
    }
    let encoded_elements = encoded_elements.join(", ");
    Ok(any_encoded.then(|| rendered(ts_template! { [@{encoded_elements}] })))
}

/// The JSON keys the type `name` reads its fields from, flattened ones
/// included: what a field flattening it takes from the enclosing object.
/// `None` when the registry does not hold the type or cannot say.
pub(crate) fn json_keys_of(name: &str, scope: &TypeScope<'_>) -> Option<Vec<String>> {
    json_keys_visiting(name, scope, &mut Vec::new())
}

fn json_keys_visiting(
    name: &str,
    scope: &TypeScope<'_>,
    visiting: &mut Vec<String>,
) -> Option<Vec<String>> {
    if visiting.iter().any(|visited| visited == name) {
        return None;
    }
    let entry = scope
        .registry
        .resolve_in_file(name, scope.file_path, scope.file_imports)?;
    let decorators = match &entry.definition {
        TypeDefinitionIR::Class(class) => &class.decorators,
        TypeDefinitionIR::Interface(interface) => &interface.decorators,
        TypeDefinitionIR::TypeAlias(alias) => &alias.decorators,
        TypeDefinitionIR::Enum(_) => return None,
    };
    let container = EndecContainerOptions::from_decorators(decorators);
    visiting.push(name.to_string());
    let mut keys = Vec::new();
    for field in fields_from_definition(&entry.definition, scope.registry)? {
        let options = EndecFieldOptions::from_decorators(&field.decorators, &field.name).options;
        if !options.should_decode() {
            continue;
        }
        if options.flatten {
            let TypeCategory::Encodable(flattened) = TypeCategory::from_ts_type(&field.ts_type)
            else {
                return None;
            };
            keys.extend(json_keys_visiting(&flattened, scope, visiting)?);
        } else {
            keys.push(
                options
                    .rename
                    .unwrap_or_else(|| container.rename_all.apply(&field.name)),
            );
        }
    }
    visiting.pop();
    Some(keys)
}

/// A function decoding the raw value of `ts_type` when that is a foreign
/// type that may also be `null` or `undefined`, which its handler is not
/// handed. `None` for any other type.
pub(crate) fn nullable_foreign_decoder(
    ts_type: &str,
    scope: &TypeScope<'_>,
) -> Result<Option<String>, String> {
    let mut present = ungrouped(ts_type).to_string();
    while let TypeCategory::Optional(inner) | TypeCategory::Nullable(inner) =
        TypeCategory::from_ts_type(&present)
    {
        present = inner;
    }
    if present == ungrouped(ts_type) || foreign_callee(&present, ForeignHandler::Decode).is_none() {
        return Ok(None);
    }
    Ok(decode_expr(ts_type, "__raw", scope, 0)?
        .map(|decoded| rendered(ts_template! { ((__raw: unknown) => @{decoded}) })))
}

/// The `@macroforge/core/endec` decoder of a primitive keyword or `Date`.
fn leaf_decoder(ts_type: &str) -> Option<&'static str> {
    Some(match ts_type {
        "string" => "decodeString",
        "number" => "decodeNumber",
        "boolean" => "decodeBoolean",
        "bigint" => "decodeBigInt",
        "Date" => "decodeDate",
        _ => return None,
    })
}

/// The local name of an `@macroforge/core/endec` helper, requesting its
/// import for the file being expanded.
pub(crate) fn endec_helper(name: &str) -> String {
    let alias = format!("__mf_{name}");
    crate::host::import_registry::with_registry_mut(|registry| {
        registry.request_import(&alias, Some(name), crate::package::ENDEC, false);
    });
    alias
}

/// A function decoding one raw value of `ts_type`, to hand to a collection's
/// decoder. A type whose values decode as themselves is cast to it.
fn decoder(ts_type: &str, scope: &TypeScope<'_>, depth: usize) -> Result<String, String> {
    let ts_type = ungrouped(ts_type);
    if foreign_callee(ts_type, ForeignHandler::Decode).is_none()
        && let Some(leaf) = leaf_decoder(ts_type)
    {
        return Ok(endec_helper(leaf));
    }
    let raw = format!("__raw{depth}");
    Ok(match decode_expr(ts_type, &raw, scope, depth + 1)? {
        Some(decoded) => rendered(ts_template! { (@{&raw}) => @{decoded} }),
        None => rendered(ts_template! { (@{&raw}) => @{&raw} as @{ts_type} }),
    })
}

/// An expression decoding `raw`, an `unknown` value, as `ts_type`, with
/// `ctx` in scope. `depth` numbers the locals of nested callbacks apart.
pub(crate) fn decode_expr(
    ts_type: &str,
    raw: &str,
    scope: &TypeScope<'_>,
    depth: usize,
) -> Result<Option<String>, String> {
    let ts_type = ungrouped(ts_type);
    if let Some(callee) = foreign_callee(ts_type, ForeignHandler::Decode) {
        return Ok(Some(rendered(ts_template! { @{callee}(@{raw}) })));
    }
    if let Some(elements) = tuple_elements(ts_type) {
        return decode_tuple(ts_type, &elements, raw, scope, depth);
    }
    if let Some(members) = object_members(ts_type) {
        return Ok(decode_object(&members, raw, scope, depth)?
            .map(|decoded| rendered(ts_template! { (@{decoded} as @{ts_type}) })));
    }
    if let Some(leaf) = leaf_decoder(ts_type) {
        let leaf = endec_helper(leaf);
        return Ok(Some(rendered(ts_template! { @{leaf}(@{raw}) })));
    }
    // A literal, or a union of literals and primitives, takes only its own
    // values. `null` and `undefined` alone are for their unions to pass by.
    // A member a foreign type's handler decodes is no value of its own.
    let has_foreign_member = split_top_level_union(ts_type)
        .unwrap_or_else(|| vec![ts_type])
        .iter()
        .any(|member| foreign_callee(member, ForeignHandler::Decode).is_some());
    if !matches!(ts_type, "null" | "undefined")
        && !has_foreign_member
        && let Some(accepts) = primitive_accepts(ts_type, raw)
    {
        let checked = endec_helper("decodeChecked");
        let expected = js_string(ts_type);
        return Ok(Some(rendered(ts_template! {
            (@{checked}(@{raw}, @{accepts}, @{expected}) as @{ts_type})
        })));
    }
    // Whether an element decodes as itself decides whether its collection
    // does; a `Set` or `Map` is built from its encoding either way.
    let item_is_itself = |item_type: &str| -> Result<bool, String> {
        Ok(decode_expr(item_type, "__probe", scope, depth + 1)?.is_none())
    };
    Ok(match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Primitive | TypeCategory::Date => None,
        TypeCategory::Array(element) => {
            if item_is_itself(&element)? {
                None
            } else {
                let decode_array = endec_helper("decodeArray");
                let item = decoder(&element, scope, depth)?;
                Some(rendered(
                    ts_template! { @{decode_array}(@{raw}, ctx, @{item}) },
                ))
            }
        }
        TypeCategory::Set(element) => {
            let decode_set = endec_helper("decodeSet");
            let item = decoder(&element, scope, depth)?;
            Some(rendered(
                ts_template! { @{decode_set}(@{raw}, ctx, @{item}) },
            ))
        }
        TypeCategory::Map(key_type, value_type) => {
            let decode_map = endec_helper("decodeMap");
            let key = key_decoder(&key_type, scope, depth)?;
            let item = decoder(&value_type, scope, depth)?;
            Some(rendered(
                ts_template! { @{decode_map}(@{raw}, ctx, @{key}, @{item}) },
            ))
        }
        TypeCategory::Record(_, value_type) => {
            if item_is_itself(&value_type)? {
                None
            } else {
                let decode_record = endec_helper("decodeRecord");
                let item = decoder(&value_type, scope, depth)?;
                Some(rendered(
                    ts_template! { (@{decode_record}(@{raw}, ctx, @{item}) as @{ts_type}) },
                ))
            }
        }
        TypeCategory::Optional(inner) => decode_expr(&inner, raw, scope, depth)?.map(|decoded| {
            rendered(ts_template! { (@{raw} === undefined ? undefined : @{decoded}) })
        }),
        TypeCategory::Nullable(inner) => decode_expr(&inner, raw, scope, depth)?
            .map(|decoded| rendered(ts_template! { (@{raw} === null ? null : @{decoded}) })),
        TypeCategory::Encodable(_) | TypeCategory::Wrapper(_) => {
            scope.nested(ts_type)?.map(|nested| nested.decode_call(raw))
        }
        TypeCategory::Unknown if scope.derived(ts_type, Direction::Decode).is_some() => {
            Some(NestedType::whole(ts_type.to_string()).decode_call(raw))
        }
        TypeCategory::Unknown => match scope.guard(ts_type, Direction::Decode)? {
            Some((guard, name)) => {
                let raw_expr = crate::ast::Expr::Source(raw.to_string());
                let (is_member, member_value) = (guard.test(&raw_expr), guard.convert(&raw_expr));
                let decoded = NestedType::whole(name).decode_call(raw);
                Some(rendered(
                    ts_template! { (@{is_member} ? @{member_value} : @{decoded}) },
                ))
            }
            None => None,
        },
    })
}

fn decode_tuple(
    ts_type: &str,
    elements: &[&str],
    raw: &str,
    scope: &TypeScope<'_>,
    depth: usize,
) -> Result<Option<String>, String> {
    let mut any_decoded = false;
    let mut items = Vec::with_capacity(elements.len());
    let mut required = 0usize;
    let mut rest = None;
    for source in elements {
        let element = tuple_element(source);
        // A rest element is written as the array of what remains.
        let element_type = if element.rest {
            TypeCategory::from_ts_type(element.ts_type)
        } else {
            TypeCategory::Unknown
        };
        let item_type = match &element_type {
            TypeCategory::Array(item_type) => item_type.as_str(),
            _ => element.ts_type,
        };
        any_decoded |= decode_expr(item_type, "__probe", scope, depth + 1)?.is_some();
        let item = decoder(item_type, scope, depth)?;
        if element.rest {
            rest = Some(item);
        } else {
            if !element.optional {
                required += 1;
            }
            items.push(item);
        }
    }
    if !any_decoded {
        return Ok(None);
    }
    let decode_tuple = endec_helper("decodeTuple");
    let items = items.join(", ");
    // The decoded array is a new one, so it is cast to the tuple's mutable
    // form, which a `readonly` tuple accepts.
    let tuple_type = ts_type
        .strip_prefix("readonly ")
        .map_or(ts_type, str::trim_start);
    Ok(Some(match rest {
        Some(rest) => rendered(ts_template! {
            (@{decode_tuple}(@{raw}, ctx, [@{items}], @{required}, @{rest}) as @{tuple_type})
        }),
        None => rendered(ts_template! {
            (@{decode_tuple}(@{raw}, ctx, [@{items}], @{required}) as @{tuple_type})
        }),
    }))
}

/// A function giving a `Map`'s key from the text it is spelled as in JSON.
fn key_decoder(key_type: &str, scope: &TypeScope<'_>, depth: usize) -> Result<String, String> {
    let key = map_key(key_type, scope)?;
    let text = format!("__key{depth}");
    let spelled = match key.base {
        KeyBase::Text => text.clone(),
        KeyBase::Number => {
            let decode = endec_helper("decodeNumberKey");
            rendered(ts_template! { @{decode}(@{&text}) })
        }
        KeyBase::BigInt => {
            let decode = endec_helper("decodeBigIntKey");
            rendered(ts_template! { @{decode}(@{&text}) })
        }
        KeyBase::Boolean => {
            let decode = endec_helper("decodeBooleanKey");
            rendered(ts_template! { @{decode}(@{&text}) })
        }
        KeyBase::Date => {
            let decode = endec_helper("decodeDate");
            rendered(ts_template! { @{decode}(@{&text}) })
        }
    };
    let key_type = ungrouped(key_type);
    Ok(match key.decoder {
        Some(nested) => {
            let decoded = nested.decode_call(&spelled);
            rendered(ts_template! { (@{&text}) => @{decoded} })
        }
        None if typeof_primitive(key_type).is_some() || key_type == "Date" => {
            rendered(ts_template! { (@{&text}) => @{spelled} })
        }
        // A key of a literal type is one of its literals.
        None => match primitive_accepts(key_type, &spelled) {
            Some(accepts) => {
                let checked = endec_helper("decodeChecked");
                let expected = js_string(key_type);
                rendered(ts_template! {
                    (@{&text}) => @{checked}(@{&spelled}, @{accepts}, @{expected}) as @{key_type}
                })
            }
            None => rendered(ts_template! { (@{&text}) => @{spelled} as @{key_type} }),
        },
    })
}
