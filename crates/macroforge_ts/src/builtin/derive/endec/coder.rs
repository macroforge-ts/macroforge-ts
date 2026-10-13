//! How a value of any type is encoded and decoded, by the structure of its
//! type: a collection through its elements, a nullable type through its
//! present member, a user type through that type's own coders.
//!
//! Both directions return `None` for a type whose values are their own
//! encoding, so a caller keeps the value as it is.

use std::collections::HashMap;

use crate::builtin::derive::common::{
    Declared, EnumValueKinds, Member, declared_type_has_derive, fields_from_definition,
    is_identifier_name, js_string, parse_generic_type, property_key, record_key_texts, rendered,
    tuple_body, tuple_element, undefined_as_null, ungrouped,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::class::MemberKey;
use crate::ts_syn::abi::ir::interface::InterfaceFieldIR;
use crate::ts_syn::abi::ir::type_alias::TypeBody;
use crate::ts_syn::abi::ir::type_registry::{FileImportEntry, TypeDefinitionIR, TypeRegistry};
use crate::ts_syn::abi::ir::{
    Unadopted, adopt_type_scope, resolve_generic_aliases_where, split_top_level_commas,
    split_top_level_union, substitute_tokens, typeof_primitive,
};
use crate::ts_syn::config::ForeignHandler;

use super::decode::helpers::{primitive_accepts, structural_accepts};
use super::nested::NestedType;
use super::type_arguments::param_coder;
use super::uncodable::uncodable;
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

    /// `ts_type` as a nested type coded in `direction`, with the coder of
    /// each of its type arguments and, when it is decoded, the keys a
    /// `Required` demands of it.
    pub(crate) fn nested(
        &self,
        ts_type: &str,
        direction: Direction,
        depth: usize,
    ) -> Result<Option<NestedType>, String> {
        let defaulted = self.with_default_arguments(ts_type);
        let ts_type = defaulted.as_deref().unwrap_or(ts_type);
        let Some(mut nested) = NestedType::of(ts_type, &|name| self.is_param(name))? else {
            return Ok(None);
        };
        let mut arguments = Vec::new();
        for argument in nested.type_arguments() {
            arguments.push(match direction {
                Direction::Encode => {
                    let held = format!("__arg{depth}");
                    encode_expr(argument, &held, self, depth + 1)?
                        .map(|encoded| rendered(ts_template! { (@{&held}) => @{encoded} }))
                }
                Direction::Decode => own_decoder(argument, self, depth)?,
            });
        }
        nested.arguments = arguments;
        if nested.required && matches!(direction, Direction::Decode) {
            nested.demanded = self.optional_keys(&nested);
        }
        Ok(Some(nested))
    }

    /// `ts_type` with the type arguments it leaves to their defaults written
    /// out, when the registry holds the type and it leaves any: `Box` is
    /// `Box<string>` for `Box<T = string>`. A default may name a parameter
    /// before it.
    fn with_default_arguments(&self, ts_type: &str) -> Option<String> {
        let ts_type = ts_type.trim();
        let (base, written) = match parse_generic_type(ts_type).filter(|_| ts_type.ends_with('>')) {
            Some((base, arguments)) => (base, split_top_level_commas(arguments)),
            None => (ts_type, Vec::new()),
        };
        if !is_identifier_name(base) {
            return None;
        }
        let entry = self
            .registry
            .resolve_in_file(base, self.file_path, self.file_imports)?;
        let params = match &entry.definition {
            TypeDefinitionIR::Class(class) => &class.type_params,
            TypeDefinitionIR::Interface(interface) => &interface.type_params,
            TypeDefinitionIR::TypeAlias(alias) => &alias.type_params,
            TypeDefinitionIR::Enum(_) => return None,
        };
        if written.len() >= params.len() {
            return None;
        }
        let mut arguments: Vec<String> = written
            .iter()
            .map(|argument| argument.trim().to_string())
            .collect();
        for param in params.iter().skip(arguments.len()) {
            let default = param.default.as_deref()?;
            let earlier: HashMap<&str, &str> = params
                .iter()
                .map(|param| param.name.as_str())
                .zip(arguments.iter().map(String::as_str))
                .collect();
            let default = substitute_tokens(default, &earlier);
            arguments.push(default);
        }
        Some(format!("{base}<{}>", arguments.join(", ")))
    }

    /// The fields `nested` keeps that its type lets a value leave out, each
    /// as its JSON key and its name: an optional field with no default to
    /// fill it. Empty when the registry does not hold the type, whose fields
    /// are then unknown.
    fn optional_keys(&self, nested: &NestedType) -> Vec<(String, String)> {
        let base = nested
            .name
            .split_once('<')
            .map_or(nested.name.as_str(), |(base, _)| base);
        let Some(entry) = self
            .registry
            .resolve_in_file(base, self.file_path, self.file_imports)
        else {
            return Vec::new();
        };
        let decorators = match &entry.definition {
            TypeDefinitionIR::Class(class) => &class.decorators,
            TypeDefinitionIR::Interface(interface) => &interface.decorators,
            TypeDefinitionIR::TypeAlias(alias) => &alias.decorators,
            TypeDefinitionIR::Enum(_) => return Vec::new(),
        };
        let container = EndecContainerOptions::from_decorators(decorators);
        fields_from_definition(&entry.definition, self.registry)
            .unwrap_or_default()
            .into_iter()
            .filter(|field| field.optional && nested.keeps(&field.name))
            .filter_map(|field| {
                let options =
                    EndecFieldOptions::from_decorators(&field.decorators, &field.name).options;
                let left_out = options.should_decode()
                    && !options.flatten
                    && !options.default
                    && options.default_expr.is_none();
                left_out.then(|| {
                    let key = options
                        .rename
                        .unwrap_or_else(|| container.rename_all.apply(&field.name));
                    (key, field.name.clone())
                })
            })
            .collect()
    }

    /// `ts_type` with each generic alias it instantiates written out, where
    /// what the alias stands for can be coded in place. An alias deriving
    /// `derive` whose union a field could not tell apart keeps its name, and
    /// is coded by its own coders.
    pub(crate) fn resolved(&self, ts_type: &str, direction: Direction) -> String {
        let derive = match direction {
            Direction::Encode => "Encode",
            Direction::Decode => "Decode",
        };
        resolve_generic_aliases_where(
            ts_type,
            self.registry,
            self.file_path,
            self.file_imports,
            &|alias, expanded| {
                !self.has_derive(alias, derive)
                    || untold_member(expanded, self).is_none()
                    || matches!(self.guard(expanded, direction), Ok(Some(_)))
            },
        )
    }

    /// The type `ts_type` reads off a type this project declares, when it is
    /// `keyof Name` or `Name['field']` and the fields of `Name` are known:
    /// the names of its fields, or the type of the one named, whose own
    /// named types are imported from where they are declared. An error when
    /// one of those cannot be imported here.
    fn operator_type(&self, ts_type: &str) -> Result<Option<String>, String> {
        let ts_type = ts_type.trim();
        let declared = |name: &str| {
            self.registry
                .resolve_in_file(name.trim(), self.file_path, self.file_imports)
        };
        if let Some(name) = ts_type.strip_prefix("keyof ") {
            let keys: Vec<String> = declared(name)
                .and_then(|entry| fields_from_definition(&entry.definition, self.registry))
                .unwrap_or_default()
                .iter()
                .map(|field| js_string(&field.name))
                .collect();
            return Ok((!keys.is_empty()).then(|| keys.join(" | ")));
        }
        let Some((name, key)) = ts_type
            .strip_suffix(']')
            .and_then(|indexed| indexed.split_once('['))
        else {
            return Ok(None);
        };
        let key = key.trim();
        let Some(key) = key
            .strip_prefix('\'')
            .and_then(|key| key.strip_suffix('\''))
            .or_else(|| key.strip_prefix('"').and_then(|key| key.strip_suffix('"')))
        else {
            return Ok(None);
        };
        let Some(entry) = declared(name) else {
            return Ok(None);
        };
        let Some(field) = fields_from_definition(&entry.definition, self.registry)
            .unwrap_or_default()
            .into_iter()
            .find(|field| field.name == key)
        else {
            return Ok(None);
        };
        // The field's type is written where `name` is declared, so the types
        // it names are brought in from there, with their coders.
        if let Err(unadopted) =
            adopt_type_scope(&field.ts_type, entry, self.registry, self.file_path)
        {
            let (named, why) = match &unadopted {
                Unadopted::Private(named) => (
                    named,
                    format!("a type its module does not export; export `{named}`"),
                ),
                Unadopted::Taken(named) => (
                    named,
                    format!(
                        "and this file has another `{named}`; give one of the two another name"
                    ),
                ),
                Unadopted::Unplaced(named) => (
                    named,
                    "a type whose module cannot be named from this file".to_string(),
                ),
            };
            return Err(format!(
                "`{ts_type}` is `{}`, which names `{named}`, {why}, or write the field's type out",
                field.ts_type
            ));
        }
        Ok(Some(if field.optional {
            format!("{} | undefined", field.ts_type)
        } else {
            field.ts_type
        }))
    }

    /// `ts_type` with each member an operator reads off a declared type
    /// written as what the operator yields, when it has such a member.
    pub(crate) fn operators_resolved(&self, ts_type: &str) -> Result<Option<String>, String> {
        let members = split_top_level_union(ts_type).unwrap_or_else(|| vec![ts_type]);
        let yielded: Vec<Option<String>> = members
            .iter()
            .map(|member| self.operator_type(member))
            .collect::<Result<_, _>>()?;
        Ok(yielded.iter().any(Option::is_some).then(|| {
            members
                .iter()
                .zip(yielded)
                .map(|(member, yielded)| yielded.unwrap_or_else(|| member.to_string()))
                .collect::<Vec<_>>()
                .join(" | ")
        }))
    }

    /// The coder parameter in scope for `ts_type`, when it is one of the
    /// type parameters.
    fn param_coder(&self, ts_type: &str, direction: Direction) -> Option<String> {
        self.is_param(ts_type)
            .then(|| param_coder(direction, ts_type))
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
            && self.has_derive(name, derive))
        .then(|| name.to_string())
    }

    /// Whether the type this scope's file knows as `name` derives `derive`.
    fn has_derive(&self, name: &str, derive: &str) -> bool {
        let declared = Declared {
            registry: self.registry,
            file_path: self.file_path,
            file_imports: self.file_imports,
        };
        declared_type_has_derive(declared, name, derive)
    }

    /// Why `ts_type` cannot be coded, when it names a type through a
    /// namespace the file imports (`models.User` after `import * as models`):
    /// a type's coders are found by its own name, and without them its
    /// values would pass uncoded and unchecked. `None` for any other type,
    /// a foreign type that is configured under such a name among them.
    fn through_namespace(&self, ts_type: &str) -> Option<String> {
        let name = parse_generic_type(ts_type).map_or(ts_type, |(base, _)| base);
        let (namespace, member) = name.split_once('.')?;
        // An array or a union of such a type is read down to the name.
        if !name.split('.').all(is_identifier_name) {
            return None;
        }
        let import = self.file_imports.iter().find(|import| {
            import.local_name == namespace && import.original_name.as_deref() == Some("*")
        })?;
        if TypeCategory::match_foreign_type(name, &get_foreign_types())
            .config
            .is_some()
        {
            return None;
        }
        // What a module exports is one name: of `Space.Type`, the `Space`.
        let exported = member.split('.').next().unwrap_or(member);
        let written = if exported == member {
            String::new()
        } else {
            format!(", and write `{member}`")
        };
        Some(format!(
            "`{name}` names a type through the namespace `{namespace}`, and a type's coders are found by its own name; import it by name, as in `import type {{ {exported} }} from \"{}\"`{written}",
            import.module_specifier
        ))
    }

    fn guard(
        &self,
        ts_type: &str,
        direction: Direction,
    ) -> Result<Option<(UnionGuard, String)>, String> {
        union_guard(ts_type, direction, |name| self.is_param(name))
    }
}

/// The member of the union `ts_type` that a field cannot tell from the
/// others: one whose values are coded, where nothing says which member a
/// value is. `None` for any other type, and for a union whose members are
/// their own coding, which is carried as it is.
fn untold_member<'a>(ts_type: &'a str, scope: &TypeScope<'_>) -> Option<&'a str> {
    // A function type's `|` belongs to what it returns.
    if ts_type.contains("=>") {
        return None;
    }
    // `null` and `undefined` are told by being themselves, which leaves a
    // union of one other member no union at all.
    let members: Vec<&str> = split_top_level_union(ts_type)?
        .into_iter()
        .filter(|member| !matches!(ungrouped(member), "null" | "undefined"))
        .collect();
    if members.len() < 2 {
        return None;
    }
    let holds_text = members.contains(&"string");
    members.into_iter().find(|member| {
        let member = ungrouped(member);
        if scope.is_param(member) {
            return false;
        }
        match TypeCategory::from_ts_type(member) {
            TypeCategory::Encodable(_) | TypeCategory::Wrapper(_) => true,
            TypeCategory::Set(_) | TypeCategory::Map(_, _) => true,
            // A date is its text once encoded, which a string member takes.
            TypeCategory::Date => !holds_text,
            TypeCategory::Array(_) | TypeCategory::Record(_, _) | TypeCategory::Unknown => {
                scope.derived(member, Direction::Encode).is_some()
                    || scope.derived(member, Direction::Decode).is_some()
                    || matches!(
                        encode_expr(member, "__probe", scope, 0),
                        Ok(Some(_)) | Err(_)
                    )
            }
            TypeCategory::Primitive | TypeCategory::Optional(_) | TypeCategory::Nullable(_) => {
                false
            }
        }
    })
}

/// An error for a union with a member a field cannot tell from the others.
fn told_apart(ts_type: &str, scope: &TypeScope<'_>) -> Result<(), String> {
    match untold_member(ts_type, scope) {
        Some(member) => Err(format!(
            "`{ts_type}` holds `{}`, whose values are coded, and nothing in a field says which member of the union a value is; name the union with an alias, `type Named = {ts_type}`, and derive on that",
            member.trim()
        )),
        None => Ok(()),
    }
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
/// `{ at: Date; note?: string }`, an index signature among them. `None` for
/// any other type, and for one with a method or a call signature.
pub(crate) fn inline_object_fields(ts_type: &str) -> Option<Vec<InterfaceFieldIR>> {
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

/// The members of an object type written out in place that are coded one
/// by one. `None` for any other type, for one whose values pass through as
/// they are, and for one that is a single index signature, which is coded as
/// the `Record` it describes. An error for one with a computed key, or an
/// index signature beside named members, which names no member to code.
pub(crate) fn object_members(ts_type: &str) -> Result<Option<Vec<InterfaceFieldIR>>, String> {
    let Some(fields) = inline_object_fields(ts_type) else {
        return Ok(None);
    };
    if dictionary(&fields).is_some() {
        return Ok(None);
    }
    if let Some(unnamed) = fields.iter().find(|field| field.key == MemberKey::Unnamed) {
        return Err(format!(
            "`{ts_type}` has the member `{}`, which no name reads: a computed key or an index signature is known only as the code runs. Write the entries as a `Record`",
            unnamed.name
        ));
    }
    Ok(Some(fields))
}

/// The value type of an object type that is one index signature over
/// strings, `{ [key: string]: V }`, which is `Record<string, V>` written
/// another way.
fn dictionary(fields: &[InterfaceFieldIR]) -> Option<&str> {
    let [only] = fields else {
        return None;
    };
    let parameter = only.name.strip_prefix('[')?.strip_suffix(']')?;
    let (_, key_type) = parameter.split_once(':')?;
    (only.key == MemberKey::Unnamed && key_type.trim() == "string").then_some(only.ts_type.as_str())
}

/// `ts_type` as the `Record` it is, when it is an object type of one index
/// signature over strings.
pub(crate) fn dictionary_record(ts_type: &str) -> Option<String> {
    let fields = inline_object_fields(ts_type)?;
    let value_type = dictionary(&fields)?;
    Some(format!("Record<string, {value_type}>"))
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

/// A test that `value` has the shape of the object type with `members`: an
/// object holding each required key, and each member typed by one literal
/// holding that literal, which is what tells apart the variants of a union
/// that carries no tag.
pub(crate) fn object_shape_test(members: &[InterfaceFieldIR], value: &str) -> String {
    let is_literal = |ts_type: &str| {
        ts_type.starts_with(['"', '\''])
            || crate::ts_syn::abi::ir::number_literal(ts_type).is_some()
            || matches!(ts_type, "true" | "false")
    };
    std::iter::once(rendered(ts_template! {
        typeof @{value} === "object" && @{value} !== null && !Array.isArray(@{value})
    }))
    .chain(
        members
            .iter()
            .filter(|member| !member_may_be_absent(member))
            .flat_map(|member| {
                let key = js_string(&member.name);
                let literal = member.ts_type.trim();
                std::iter::once(rendered(ts_template! { @{&key} in @{value} })).chain(
                    is_literal(literal)
                        .then(|| rendered(ts_template! { @{value}[@{&key}] === @{literal} })),
                )
            }),
    )
    .collect::<Vec<_>>()
    .join(" && ")
}

/// [`object_shape_test`] for a raw value being decoded, which is told by the
/// keys it holds itself: every object answers to `constructor` and the rest
/// of `Object.prototype`.
pub(crate) fn raw_shape_test(members: &[InterfaceFieldIR], value: &str) -> String {
    let is_literal = |ts_type: &str| {
        ts_type.starts_with(['"', '\''])
            || crate::ts_syn::abi::ir::number_literal(ts_type).is_some()
            || matches!(ts_type, "true" | "false")
    };
    let holds = endec_helper("holds");
    std::iter::once(rendered(ts_template! {
        typeof @{value} === "object" && @{value} !== null && !Array.isArray(@{value})
    }))
    .chain(
        members
            .iter()
            .filter(|member| !member_may_be_absent(member))
            .flat_map(|member| {
                let key = js_string(&member.name);
                let literal = member.ts_type.trim();
                std::iter::once(rendered(ts_template! { @{&holds}(@{value}, @{&key}) })).chain(
                    is_literal(literal).then(|| {
                        rendered(ts_template! { Reflect.get(@{value}, @{&key}) === @{literal} })
                    }),
                )
            }),
    )
    .collect::<Vec<_>>()
    .join(" && ")
}

/// What a coder says of the intersection alias `name` when a member's fields
/// are not known: there is nothing to code or check its values by.
pub(crate) fn unread_intersection(name: &str) -> String {
    format!(
        "type '{name}' is an intersection with a member whose fields are not known here, so its values cannot be coded field by field; declare the member in this project, or write its fields out"
    )
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
    let mut decoders = Vec::with_capacity(members.len());
    for member in members {
        decoders.push(own_decoder(&member.ts_type, scope, depth)?);
    }
    if decoders.iter().all(Option::is_none) {
        return Ok(None);
    }
    let decoders = members
        .iter()
        .zip(decoders)
        .map(|(member, own)| own.unwrap_or_else(|| kept_as(&member.ts_type, depth)))
        .collect();
    Ok(Some(members_call(members, decoders, raw)))
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
        decoders.push(decoder(&member.ts_type, scope, depth)?);
    }
    Ok(members_call(members, decoders, raw))
}

/// The call decoding `raw` into an object of `members`, the decoder of each
/// given in their order.
fn members_call(members: &[InterfaceFieldIR], decoders: Vec<String>, raw: &str) -> String {
    let decoders: Vec<String> = members
        .iter()
        .zip(decoders)
        .map(|(member, decode)| {
            let key = property_key(&member.name);
            rendered(ts_template! { @{key}: @{decode} })
        })
        .collect();
    let optional: Vec<String> = members
        .iter()
        .filter(|member| member_may_be_absent(member))
        .map(|member| js_string(&member.name))
        .collect();
    let decode_object = endec_helper("decodeObject");
    let decoders = decoders.join(", ");
    if optional.is_empty() {
        return rendered(ts_template! {
            @{decode_object}(@{raw}, ctx, { @{decoders} })
        });
    }
    let optional = optional.join(", ");
    rendered(ts_template! {
        @{decode_object}(@{raw}, ctx, { @{decoders} }, [@{optional}])
    })
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
        } else if let Some(kind) = crate::ts_syn::abi::ir::number_literal(member) {
            Some(match kind {
                crate::ts_syn::abi::ir::NumberLiteral::Number => KeyBase::Number,
                crate::ts_syn::abi::ir::NumberLiteral::BigInt => KeyBase::BigInt,
            })
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
        decoder: scope
            .has_derive(&name, "Decode")
            .then(|| NestedType::whole(name)),
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
    // A value of a type parameter is encoded by the coder handed in for it.
    if let Some(encode) = scope.param_coder(ts_type, Direction::Encode) {
        return Ok(Some(rendered(ts_template! {
            (@{&encode} === undefined ? @{value} : @{&encode}(@{value}, ctx))
        })));
    }
    if let Some(callee) = foreign_callee(ts_type, ForeignHandler::Encode) {
        return Ok(Some(rendered(ts_template! { @{callee}(@{value}) })));
    }
    if let Some(reason) = scope.through_namespace(ts_type) {
        return Err(reason);
    }
    if let Some(yielded) = scope.operators_resolved(ts_type)? {
        return encode_expr(&yielded, value, scope, depth);
    }
    if let Some(written) = wrapper_over_structure(ts_type).transpose()? {
        return encode_expr(&written, value, scope, depth);
    }
    if let Some(reason) = uncodable(ts_type) {
        return Err(reason);
    }
    if let Some(elements) = tuple_elements(ts_type) {
        return encode_tuple(&elements, value, scope, depth);
    }
    if let Some(record) = dictionary_record(ts_type) {
        return encode_expr(&record, value, scope, depth);
    }
    if let Some(members) = object_members(ts_type)? {
        return encode_object(&members, value, scope, depth);
    }
    let item = format!("__item{depth}");
    let key = format!("__key{depth}");
    let encoded_item = |item_type: &str| encode_expr(item_type, &item, scope, depth + 1);
    // The value of an entry, which JSON drops with its key when it is
    // `undefined`: such a value is written as `null`, and the entry stays.
    let encoded_entry = |value_type: &str| -> Result<Option<String>, String> {
        let encoded = encoded_item(value_type)?;
        if !undefined_as_null(value_type) {
            return Ok(encoded);
        }
        let held = encoded.unwrap_or_else(|| item.clone());
        Ok(Some(rendered(
            ts_template! { (@{&item} === undefined ? null : @{held}) },
        )))
    };
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
            Some(match (dated, encoded_entry(&value_type)?) {
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
        // An entry's value is of the record's value type, which a stricter
        // `Object.entries` does not say of a record over named keys.
        TypeCategory::Record(_, value_type) => encoded_entry(&value_type)?.map(|encoded| {
            rendered(ts_template! {
                Object.fromEntries((Object.entries(@{value}) as [string, @{&value_type}][]).map(([@{&key}, @{&item}]) => [@{&key}, @{encoded}]))
            })
        }),
        TypeCategory::Optional(inner) => encode_expr(&inner, value, scope, depth)?
            .map(|encoded| rendered(ts_template! { (@{value} === undefined ? undefined : @{encoded}) })),
        TypeCategory::Nullable(inner) => encode_expr(&inner, value, scope, depth)?
            .map(|encoded| rendered(ts_template! { (@{value} === null ? null : @{encoded}) })),
        TypeCategory::Encodable(_) | TypeCategory::Wrapper(_) => scope
            .nested(ts_type, Direction::Encode, depth)?
            .map(|nested| nested.encode_call(value)),
        TypeCategory::Unknown if scope.derived(ts_type, Direction::Encode).is_some() => {
            Some(NestedType::whole(ts_type.to_string()).encode_call(value))
        }
        TypeCategory::Unknown => match scope.guard(ts_type, Direction::Encode)? {
            Some((guard, name)) => {
                let value_expr = crate::ast::Expr::Source(value.to_string());
                let (is_member, member_value) =
                    (guard.test(&value_expr, Direction::Encode), guard.convert(&value_expr));
                let encoded = scope
                    .nested(&name, Direction::Encode, depth)?
                    .unwrap_or_else(|| NestedType::whole(name))
                    .encode_call(value);
                Some(rendered(
                    ts_template! { (@{is_member} ? @{member_value} : @{encoded}) },
                ))
            }
            None => {
                told_apart(ts_type, scope)?;
                None
            }
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

/// Why a field of type `ts_type` cannot be flattened, when it cannot: only
/// a type with fields of its own, or a `Record`, has members the enclosing
/// object can hold as its own.
pub(crate) fn unflattenable(
    ts_type: &str,
    scope: &TypeScope<'_>,
    direction: Direction,
) -> Option<String> {
    let mut present = ts_type.trim().to_string();
    while let TypeCategory::Optional(inner) | TypeCategory::Nullable(inner) =
        TypeCategory::from_ts_type(&present)
    {
        present = inner;
    }
    let named = match TypeCategory::from_ts_type(&present) {
        TypeCategory::Record(_, _) => return None,
        TypeCategory::Encodable(name) => Some(name),
        // A type named in lowercase is known for one by the registry alone.
        TypeCategory::Unknown => scope.derived(&present, direction),
        TypeCategory::Primitive
        | TypeCategory::Array(_)
        | TypeCategory::Optional(_)
        | TypeCategory::Nullable(_)
        | TypeCategory::Date
        | TypeCategory::Map(_, _)
        | TypeCategory::Set(_)
        | TypeCategory::Wrapper(_) => None,
    };
    let has_fields = named.is_some_and(|name| {
        let declared = scope
            .registry
            .resolve_in_file(&name, scope.file_path, scope.file_imports)
            .map(|entry| &entry.definition);
        !scope.is_param(&name)
            && match declared {
                // A type the project does not declare is taken at its word.
                Some(TypeDefinitionIR::Class(_) | TypeDefinitionIR::Interface(_)) | None => true,
                Some(TypeDefinitionIR::TypeAlias(alias)) => matches!(
                    alias.body.unbranded(),
                    TypeBody::Object { .. } | TypeBody::Intersection(_)
                ),
                Some(TypeDefinitionIR::Enum(_)) => false,
            }
    });
    (!has_fields).then(|| {
        format!(
            "`flatten` holds a value's fields as the enclosing object's own, and `{present}` has none to hold: only a class, an interface, an object type alias or a `Record` can be flattened"
        )
    })
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

/// The local name of an `@macroforge/core/endec` type, requesting its
/// import for the file being expanded.
pub(crate) fn endec_type(name: &str) -> String {
    let alias = format!("__mf_{name}");
    crate::host::import_registry::with_registry_mut(|registry| {
        registry.request_import(&alias, Some(name), crate::package::ENDEC, true);
    });
    alias
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
    Ok(own_decoder(ts_type, scope, depth)?.unwrap_or_else(|| kept_as(ts_type, depth)))
}

/// `decoder`, and `None` for a type whose values decode as themselves, which
/// leaves a collection of them to be taken as it is. The type is read once:
/// reading it again for each of its uses would double the work at each level
/// it nests.
fn own_decoder(
    ts_type: &str,
    scope: &TypeScope<'_>,
    depth: usize,
) -> Result<Option<String>, String> {
    let ts_type = ungrouped(ts_type);
    if foreign_callee(ts_type, ForeignHandler::Decode).is_none()
        && let Some(leaf) = leaf_decoder(ts_type)
    {
        return Ok(Some(endec_helper(leaf)));
    }
    let raw = format!("__raw{depth}");
    // An `undefined` held in a collection is `null` once encoded.
    let read = if undefined_as_null(ts_type) {
        format!("({raw} ?? undefined)")
    } else {
        raw.clone()
    };
    Ok(decode_expr(ts_type, &read, scope, depth + 1)?
        .map(|decoded| rendered(ts_template! { (@{&raw}) => @{decoded} })))
}

/// A function handing back a raw value of `ts_type`, which decodes as itself.
fn kept_as(ts_type: &str, depth: usize) -> String {
    let (raw, ts_type) = (format!("__raw{depth}"), ungrouped(ts_type));
    rendered(ts_template! { (@{&raw}) => @{&raw} as @{ts_type} })
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
    // A value of a type parameter is decoded by the coder handed in for it.
    if let Some(decode) = scope.param_coder(ts_type, Direction::Decode) {
        return Ok(Some(rendered(ts_template! {
            (@{&decode} === undefined ? @{raw} as @{ts_type} : @{&decode}(@{raw}, ctx))
        })));
    }
    if let Some(callee) = foreign_callee(ts_type, ForeignHandler::Decode) {
        return Ok(Some(rendered(ts_template! { @{callee}(@{raw}) })));
    }
    if let Some(reason) = scope.through_namespace(ts_type) {
        return Err(reason);
    }
    if let Some(yielded) = scope.operators_resolved(ts_type)? {
        return decode_expr(&yielded, raw, scope, depth);
    }
    if let Some(written) = wrapper_over_structure(ts_type).transpose()? {
        return Ok(decode_expr(&written, raw, scope, depth)?
            .map(|decoded| rendered(ts_template! { (@{decoded} as @{ts_type}) })));
    }
    if let Some(reason) = uncodable(ts_type) {
        return Err(reason);
    }
    if let Some(elements) = tuple_elements(ts_type) {
        return decode_tuple(ts_type, &elements, raw, scope, depth);
    }
    if let Some(record) = dictionary_record(ts_type) {
        return Ok(decode_expr(&record, raw, scope, depth)?
            .map(|decoded| rendered(ts_template! { (@{decoded} as @{ts_type}) })));
    }
    if let Some(members) = object_members(ts_type)? {
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
    Ok(match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Primitive | TypeCategory::Date => None,
        TypeCategory::Array(element) => own_decoder(&element, scope, depth)?.map(|item| {
            let decode_array = endec_helper("decodeArray");
            rendered(ts_template! { @{decode_array}(@{raw}, ctx, @{item}) })
        }),
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
        TypeCategory::Record(key_type, value_type) => {
            let keys = record_key_arguments(&key_type, scope, depth)?;
            let own = own_decoder(&value_type, scope, depth)?;
            if keys.is_none() && own.is_none() {
                None
            } else {
                let decode_record = endec_helper("decodeRecord");
                let item = own.unwrap_or_else(|| kept_as(&value_type, depth));
                let keys = keys.unwrap_or_default();
                Some(rendered(
                    ts_template! { (@{decode_record}(@{raw}, ctx, @{item}@{keys}) as @{ts_type}) },
                ))
            }
        }
        TypeCategory::Optional(inner) => decode_expr(&inner, raw, scope, depth)?.map(|decoded| {
            rendered(ts_template! { (@{raw} === undefined ? undefined : @{decoded}) })
        }),
        TypeCategory::Nullable(inner) => decode_expr(&inner, raw, scope, depth)?
            .map(|decoded| rendered(ts_template! { (@{raw} === null ? null : @{decoded}) })),
        TypeCategory::Encodable(_) | TypeCategory::Wrapper(_) => scope
            .nested(ts_type, Direction::Decode, depth)?
            .map(|nested| nested.decode_call(raw)),
        TypeCategory::Unknown if scope.derived(ts_type, Direction::Decode).is_some() => {
            Some(NestedType::whole(ts_type.to_string()).decode_call(raw))
        }
        TypeCategory::Unknown => match scope.guard(ts_type, Direction::Decode)? {
            Some((guard, name)) => {
                let raw_expr = crate::ast::Expr::Source(raw.to_string());
                let (is_member, member_value) = (
                    guard.test(&raw_expr, Direction::Decode),
                    guard.convert(&raw_expr),
                );
                let decoded = scope
                    .nested(&name, Direction::Decode, depth)?
                    .unwrap_or_else(|| NestedType::whole(name))
                    .decode_call(raw);
                Some(rendered(
                    ts_template! { (@{is_member} ? @{member_value} : @{decoded}) },
                ))
            }
            // A union left to be carried as it is holds values that are
            // their own encoding, which are still checked to be of it.
            None => {
                told_apart(ts_type, scope)?;
                split_top_level_union(ts_type)
                    .and_then(|_| structural_accepts(ts_type, raw, depth))
                    .map(|accepts| {
                        let checked = endec_helper("decodeChecked");
                        let expected = js_string(ts_type);
                        rendered(ts_template! {
                            (@{checked}(@{raw}, @{accepts}, @{expected}) as @{ts_type})
                        })
                    })
            }
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
        // An optional element that is there and `undefined` is one too.
        let left_out = format!("{item_type} | undefined");
        let item_type = if element.optional && undefined_as_null(&left_out) {
            left_out.as_str()
        } else {
            item_type
        };
        let own = own_decoder(item_type, scope, depth)?;
        any_decoded |= own.is_some();
        let item = own.unwrap_or_else(|| kept_as(item_type, depth));
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

/// The type a wrapper over a type written in place is coded as:
/// `Readonly<Record<K, V>>` as the record, `NonNullable<T | null>` as `T`,
/// and `Partial<Record<'a' | 'b', V>>` as an object whose members may be
/// missing. `None` for any other type, a wrapper of a named type among them,
/// which that type's own coders take. An error for a wrapper that picks
/// members out of a type written in place.
pub(crate) fn wrapper_over_structure(ts_type: &str) -> Option<Result<String, String>> {
    let ts_type = ts_type.trim();
    let (base, arguments) = parse_generic_type(ts_type).filter(|_| ts_type.ends_with('>'))?;
    if !matches!(
        base,
        "Readonly" | "Required" | "NonNullable" | "Partial" | "Pick" | "Omit"
    ) {
        return None;
    }
    let arguments = split_top_level_commas(arguments);
    let inner = arguments.first()?.trim();
    let category = TypeCategory::from_ts_type(inner);
    if matches!(
        category,
        TypeCategory::Encodable(_) | TypeCategory::Wrapper(_)
    ) {
        return None;
    }
    let is_literal = |member: &str| {
        member.starts_with(['"', '\'']) || crate::ts_syn::abi::ir::number_literal(member).is_some()
    };
    Some(match (base, category) {
        ("NonNullable", _) => Ok(split_top_level_union(inner)
            .unwrap_or_else(|| vec![inner])
            .into_iter()
            .filter(|member| !matches!(member.trim(), "null" | "undefined"))
            .collect::<Vec<_>>()
            .join(" | ")),
        ("Readonly", _) => Ok(inner.to_string()),
        ("Required", _) if !inner.starts_with('{') => Ok(inner.to_string()),
        ("Partial", TypeCategory::Record(key_type, value_type)) => {
            let keys = split_top_level_union(&key_type).unwrap_or_else(|| vec![&key_type]);
            if keys.iter().all(|key| is_literal(key.trim())) {
                // Over a finite set of keys, any of them may be missing.
                let members: Vec<String> = keys
                    .iter()
                    .map(|key| format!("{}?: {value_type}", key.trim()))
                    .collect();
                Ok(format!("{{ {} }}", members.join("; ")))
            } else if matches!(key_type.trim(), "string" | "number")
                || key_type.trim().starts_with('`')
            {
                // An open key type demands no key to begin with.
                Ok(inner.to_string())
            } else {
                Err(unpicked(ts_type))
            }
        }
        _ => Err(unpicked(ts_type)),
    })
}

fn unpicked(ts_type: &str) -> String {
    format!(
        "`{ts_type}` picks members out of a type written in place, which a derive does not rewrite; write the type out with the members it has, or name it with an alias and derive on that"
    )
}

/// The arguments `decodeRecord` takes for the keys of a `Record<K, V>`: a
/// check of each key, and the keys the record has to have when `K` is a
/// finite set of them. `None` for `string` and for a type parameter, which
/// say nothing of the keys.
fn record_key_arguments(
    key_type: &str,
    scope: &TypeScope<'_>,
    depth: usize,
) -> Result<Option<String>, String> {
    let key_type = ungrouped(key_type);
    if key_type == "string" || scope.is_param(key_type) {
        return Ok(None);
    }
    let check = key_decoder(key_type, scope, depth)?;
    let required = record_key_texts(
        key_type,
        scope.registry,
        scope.file_path,
        scope.file_imports,
    )
    .map(|keys| keys.join(", "))
    .unwrap_or_default();
    Ok(Some(format!(", {check}, [{required}]")))
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
