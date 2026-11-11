//! `Encode` for a union type alias, per its tagging mode.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::class::MemberKey;
use crate::ts_syn::abi::ir::interface::InterfaceFieldIR;
use crate::ts_syn::abi::ir::type_alias::TypeMemberKind;
use crate::ts_syn::abi::ir::type_registry::TypeDefinitionIR;
use crate::ts_syn::abi::{Diagnostic, DiagnosticCollector};
use crate::ts_syn::config::ForeignHandler;
use crate::ts_syn::{MacroforgeError, MacroforgeErrors, TsStream, ts_ident};

use crate::builtin::derive::common::{
    entry_derives, fields_from_definition, function_prefix, js_string, rendered,
};

use super::super::field::prepare::to_encode_field;
use super::super::field::statements::encode_fields;
use super::super::text::encode_text_body;
use super::super::types::EncodeField;
use super::AliasEncode;
use crate::builtin::derive::endec::coder::{TypeScope, endec_helper, object_shape_test};
use crate::builtin::derive::endec::type_arguments::{member_encoder, param_coder};
use crate::builtin::derive::endec::union_guard::{Direction, tagged_member};
use crate::builtin::derive::endec::{
    EndecContainerOptions, TaggingMode, TypeCategory, get_foreign_types,
};
use crate::builtin::return_types::ENCODE_CONTEXT;

/// Encodes whichever member the value is, tagged as the union's tagging mode
/// says.
pub(super) fn encode_union_alias(alias: &AliasEncode) -> Result<TsStream, MacroforgeError> {
    let AliasEncode {
        type_alias,
        type_params_ident,
        full_type_ident,
        fn_encode_ident,
        fn_encode_internal_ident,
        encode_context_ident,
        coders,
        passed_coders,
        ..
    } = alias;
    // Union type: tagging-mode-aware encoding
    let container_opts = EndecContainerOptions::from_decorators(&type_alias.inner.decorators);
    let fn_encode_internal_expr: Expr = fn_encode_internal_ident.clone().into();

    let is_internally_tagged =
        matches!(container_opts.tagging, TaggingMode::InternallyTagged { .. });
    let is_externally_tagged = matches!(container_opts.tagging, TaggingMode::ExternallyTagged);
    let is_adjacently_tagged =
        matches!(container_opts.tagging, TaggingMode::AdjacentlyTagged { .. });
    let is_untagged = matches!(container_opts.tagging, TaggingMode::Untagged);
    let tag_field = container_opts.tag_field_or_default().to_string();
    let content_field = container_opts.content_field().unwrap_or("").to_string();

    let is_type_param = |name: &str| {
        type_alias
            .type_params()
            .iter()
            .any(|param| param.name == name)
    };
    let param_names: Vec<String> = type_alias
        .type_params()
        .iter()
        .map(|param| param.name.clone())
        .collect();
    let scope = TypeScope {
        registry: alias.type_registry,
        file_path: alias.caller_file_path,
        file_imports: alias.file_imports,
        type_params: &param_names,
    };
    let refused = |message: String| {
        MacroforgeError::new(
            type_alias.inner.span,
            format!("@derive(Encode) on '{}': {message}", type_alias.inner.name),
        )
    };
    // What encodes a member that is a user type, with the coders of its
    // type arguments handed on.
    let member_encoder_of = |ts_type: &str| -> Result<Expr, MacroforgeError> {
        member_encoder(ts_type, &scope)
            .map(Expr::Source)
            .map_err(refused)
    };
    let union_type = rendered(ts_template! { @{full_type_ident} });
    let member_coder = endec_helper("memberCoder");
    let member =
        |key: &str| rendered(ts_template! { @{endec_helper("member")}(member, @{js_string(key)}) });
    // A coder for the member `member_type` that `test` tells, encoded by
    // `encoder`; the test narrows the union to the member it names.
    let coder = |member_type: &str, test: &str, encoder: &Expr| {
        ts_template! {
            @{&member_coder}((member: @{&union_type}): member is @{member_type} => @{test}, (member) => @{encoder}(member, ctx))
        }
    };

    // The coders of the members a value may be, tried in turn: members tagged
    // in place by their tag, then user types by the tag a decoded value
    // carries, then by their fields, as a plain object says no other way
    // which member it is.
    let mut tagged_coders: Vec<TsStream> = Vec::new();
    let mut named_by_tag: Vec<TsStream> = Vec::new();
    let mut named_by_shape: Vec<TsStream> = Vec::new();
    let mut foreign_coders: Vec<TsStream> = Vec::new();
    let mut external_coders: Vec<TsStream> = Vec::new();
    if let Some(members) = type_alias.body().unbranded().as_union() {
        for m in members {
            match &m.kind {
                // Tagged intersection variant: `{ variant: 'X' } & TypeRef`
                TypeMemberKind::Intersection(subs) => {
                    let (tag_value, type_ref) = tagged_member(subs, &tag_field).map_err(refused)?;
                    let member_type = format!(
                        "{{ {}: {} }} & {type_ref}",
                        js_string(&tag_field),
                        js_string(&tag_value)
                    );
                    let test = format!("{} === {}", member(&tag_field), js_string(&tag_value));
                    // The member's own encoder writes no tag of the union's.
                    let encoder = member_encoder_of(type_ref)?;
                    let tagged = Expr::Source(rendered(ts_template! {
                        ((__member: @{type_ref}, ctx: @{ENCODE_CONTEXT}) => @{endec_helper("withTag")}(@{encoder}(__member, ctx), @{js_string(&tag_field)}, @{js_string(&tag_value)}))
                    }));
                    tagged_coders.push(coder(&member_type, &test, &tagged));
                }
                TypeMemberKind::TypeRef(t) => {
                    let foreign_types = get_foreign_types();
                    let foreign = TypeCategory::match_foreign_type(t, &foreign_types).config;
                    if let Some(config) = foreign
                        && !is_type_param(t.trim())
                    {
                        // Told by its `is`: otherwise a value of it is taken
                        // for whichever member comes after.
                        let is = config.handler_callee(ForeignHandler::Is).ok_or_else(|| {
                            refused(format!(
                                "the foreign type '{}' configures no `is` handler, which tells its values from the union's other members",
                                config.name
                            ))
                        })?;
                        let is = Expr::parse(&is).map_err(|err| {
                            refused(format!("a foreign `is` that does not parse: {err}"))
                        })?;
                        let encoded = match config.handler_callee(ForeignHandler::Encode) {
                            Some(callee) => {
                                let callee = Expr::parse(&callee).map_err(|err| {
                                    refused(format!("a foreign encoder that does not parse: {err}"))
                                })?;
                                rendered(ts_template! { @{callee}(member) })
                            }
                            None => "member".to_string(),
                        };
                        foreign_coders.push(ts_template! {
                            @{&member_coder}((member: @{&union_type}): member is @{t} => @{is}(member), (member) => @{encoded})
                        });
                    } else if let TypeCategory::Encodable(name) = TypeCategory::from_ts_type(t)
                        && !is_type_param(&name)
                    {
                        // An enum's values are written as they are.
                        if let Some(by_shape) = named_member_test(&name, &scope).map_err(refused)? {
                            let encoder = member_encoder_of(t)?;
                            let by_tag = format!("{} === {}", member(&tag_field), js_string(&name));
                            named_by_tag.push(coder(t, &by_tag, &encoder));
                            named_by_shape.push(coder(t, &by_shape, &encoder));
                        }
                    }
                }
                // Externally-tagged single-key object variants (`{ Name: Payload }`)
                // whose payload is a configured foreign type or a generated
                // type: passed through, the payload would be written as its
                // runtime shape.
                TypeMemberKind::Object { fields } if is_externally_tagged && !fields.is_empty() => {
                    let foreign_types = get_foreign_types();
                    let payload_ts_type = fields[0].ts_type.as_str();
                    let key = js_string(&fields[0].name);
                    let payload = rendered(ts_template! { value[@{&key}] });
                    let foreign_encode_callee =
                        TypeCategory::match_foreign_type(payload_ts_type, &foreign_types)
                            .config
                            .and_then(|ft| ft.handler_callee(ForeignHandler::Encode));
                    let encoded = if let Some(callee) = foreign_encode_callee {
                        let callee = Expr::parse(&callee).map_err(|err| {
                            refused(format!("a foreign encoder that does not parse: {err}"))
                        })?;
                        Some(rendered(ts_template! { @{callee}(@{&payload}) }))
                    } else if let TypeCategory::Encodable(_) =
                        TypeCategory::from_ts_type(payload_ts_type)
                    {
                        let encoder = member_encoder_of(payload_ts_type)?;
                        Some(rendered(ts_template! { @{encoder}(@{&payload}, ctx) }))
                    } else {
                        None
                    };
                    if let Some(encoded) = encoded {
                        external_coders.push(ts_template! {
                            (value: @{&union_type}): unknown => {
                                if (!(typeof value === "object" && value !== null && @{&key} in value && Object.keys(value)[0] === @{&key})) {
                                    return @{endec_helper("notTheMember")};
                                }
                                return { @{&key}: @{encoded} };
                            }
                        });
                    }
                }
                _ => {}
            }
        }
    }
    let (object_coders, object_diagnostics) =
        object_variant_dispatch(alias, &tag_field, &union_type)?;
    // One parameter among members that are otherwise told by `null`, a tag
    // or a shape is the member a value is when it is no other, so the coder
    // handed in for it encodes that value.
    let param_coder_of: Option<TsStream> = type_alias.body().unbranded().as_union().and_then(|members| {
        let mut params = members.iter().filter_map(|member| match &member.kind {
            TypeMemberKind::TypeRef(reference) if is_type_param(reference.trim()) => {
                Some(reference.trim().to_string())
            }
            _ => None,
        });
        let param = params.next().filter(|_| params.next().is_none())?;
        let told_apart = members.iter().all(|member| match &member.kind {
            TypeMemberKind::Literal(literal) => matches!(literal.trim(), "null" | "undefined"),
            TypeMemberKind::TypeRef(reference) => {
                is_type_param(reference.trim())
                    || matches!(reference.trim(), "null" | "undefined")
                    || matches!(
                        TypeCategory::from_ts_type(reference),
                        TypeCategory::Encodable(_)
                    )
                    // Told by the `is` its coder above requires.
                    || TypeCategory::match_foreign_type(reference, &get_foreign_types())
                        .config
                        .is_some()
            }
            TypeMemberKind::Object { .. } | TypeMemberKind::Intersection(_) => true,
            TypeMemberKind::Brand(_) => false,
        });
        let encode_param = param_coder(Direction::Encode, &param);
        told_apart.then(|| {
            ts_template! {
                (value: @{&union_type}): unknown => @{&encode_param} !== undefined && value !== null && value !== undefined ? @{&encode_param}(value as @{&param}, ctx) : @{endec_helper("notTheMember")}
            }
        })
    });
    let coders_list: Vec<TsStream> = tagged_coders
        .into_iter()
        .chain(named_by_tag)
        .chain(named_by_shape)
        .chain(foreign_coders)
        .chain(object_coders)
        .chain(external_coders)
        .chain(param_coder_of)
        .collect();
    let has_coders = !coders_list.is_empty();
    let member_coders = TsStream::merge_all(coders_list.into_iter().map(|coder| {
        ts_template! {
            {$typescript coder},
        }
    }));
    let encoded_member = endec_helper("encodedMember");
    let generics = type_params_ident
        .as_ref()
        .map(|params| rendered(ts_template! { <@{params}> }))
        .unwrap_or_default();

    let encoded_as: Expr = ts_ident!(crate::builtin::derive::endec::coder::endec_helper(
        "encodedAs"
    ))
    .into();
    let encode_text = encode_text_body(&fn_encode_internal_expr, "__type", passed_coders);

    let mut result = ts_template! {
        /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
        export function @{fn_encode_ident}@{&generics}(value: @{full_type_ident}, keepMetadata?: boolean@{coders}): string {
            {$typescript encode_text}
        }

        /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
        export function @{fn_encode_internal_ident}@{&generics}(value: @{full_type_ident}, ctx: @{encode_context_ident}@{coders}): unknown {
            const __encoder = __mf_derivedEncoder(value);
            {#if !has_coders}
                const __variant: unknown = __encoder !== undefined ? __encoder(value, ctx) : value;
            {:else}
                // Each member is told and encoded in a function of its own, so
                // the checker reads each on its own however many there are.
                const __variant: unknown = __encoder !== undefined ? __encoder(value, ctx) : @{&encoded_member}(value, [
                    {$typescript member_coders}
                ]);
            {/if}

            {#if is_internally_tagged}
                // Internally tagged: keep the tag field. A dispatched
                // per-variant result carries the inner struct's `__type`
                // (e.g. `PartialUser`), which differs from the union's
                // variant label (e.g. `User`) whenever the variant name and
                // its payload type name differ: so restore the discriminator
                // from the value's own tag field, falling back to `__type`
                // only when the value carries no tag. A passed-through value
                // already carries the tag field.
                if (__variant !== null && typeof __variant === "object" && "__type" in __variant) {
                    const { __type: __typeName, ...fields } = __variant as Record<string, unknown>;
                    const __tagged = { "@{tag_field}": @{crate::builtin::derive::endec::coder::endec_call("member")}(value, "@{tag_field}") ?? __typeName, ...fields };
                    // What a later reference to this value stands for is
                    // the member as the union tags it.
                    const __idVal = fields["__id"];
                    return typeof __idVal === "number" ? @{&encoded_as}(ctx, __idVal, __tagged) : __tagged;
                }
                return __variant;
            {:else}
                // Non-internally-tagged modes: restructure the variant output
                if (typeof __variant !== "object" || __variant === null) return __variant;
                const { __type: __typeName, __id: __idVal, ...fields } = __variant as Record<string, unknown>;
                if (typeof __typeName !== "string") return __variant;

                {#if is_externally_tagged}
                    // Externally tagged: { "TypeName": { ...fields } }
                    const __outer: Record<string, unknown> = { [__typeName]: fields };
                    if (typeof __idVal === "number") @{&encoded_as}(ctx, __idVal, __outer);
                    return __outer;
                {:else if is_adjacently_tagged}
                    // Adjacently tagged: { tag: "TypeName", content: { ...fields } }
                    const __outer: Record<string, unknown> = { "@{tag_field}": __typeName, "@{content_field}": fields };
                    if (typeof __idVal === "number") @{&encoded_as}(ctx, __idVal, __outer);
                    return __outer;
                {:else if is_untagged}
                    // Untagged: just the raw fields
                    const __outer: Record<string, unknown> = { ...fields };
                    if (typeof __idVal === "number") @{&encoded_as}(ctx, __idVal, __outer);
                    return __outer;
                {/if}
            {/if}
        }
    };
    result.add_aliased_import("EncodeContext", crate::package::ENDEC);
    result.add_aliased_import("derivedEncoder", crate::package::ENDEC);
    result.add_diagnostics(object_diagnostics);
    Ok(result)
}

/// Coders encoding each inline object member of an internally tagged union
/// field by field, each told by its tag. A member passed through
/// instead keeps the runtime shape of every field needing a non-identity
/// encoding: a `Map` becomes `{}`, a nested type skips its encoder.
fn object_variant_dispatch(
    alias: &AliasEncode,
    tag_field: &str,
    union_type: &str,
) -> Result<(Vec<TsStream>, Vec<Diagnostic>), MacroforgeError> {
    let container_opts = EndecContainerOptions::from_decorators(&alias.type_alias.inner.decorators);
    // An adjacently tagged member, `{ tag: 'X'; content: T }`, is an object
    // variant like an internally tagged one: encoding each of its fields
    // sends the content through its own encoder, and the tag is copied.
    // An externally tagged member is its one key, dispatched on its own.
    let (members, tagged) = match (
        &container_opts.tagging,
        alias.type_alias.body().unbranded().as_union(),
    ) {
        (
            TaggingMode::InternallyTagged { .. } | TaggingMode::AdjacentlyTagged { .. },
            Some(members),
        ) => (members, true),
        (TaggingMode::Untagged, Some(members)) => (members, false),
        _ => return Ok((Vec::new(), Vec::new())),
    };
    let param_names: Vec<String> = alias
        .type_params
        .iter()
        .map(|param| param.name.clone())
        .collect();
    let mut diagnostics = DiagnosticCollector::new();
    let mut coders = Vec::new();
    for member in members {
        let TypeMemberKind::Object { fields } = &member.kind else {
            continue;
        };
        let tag_value = fields
            .iter()
            .find(|field| tagged && field.name == tag_field)
            .and_then(|field| string_literal(&field.ts_type));
        let encode_fields_list: Vec<EncodeField> = fields
            .iter()
            .filter_map(|field| {
                to_encode_field(
                    field.into(),
                    &container_opts,
                    &mut diagnostics,
                    alias.type_registry,
                    alias.caller_file_path,
                    alias.file_imports,
                    &param_names,
                )
            })
            .collect();
        // A member with the union's tag is told by it, and any other by
        // its shape: the keys it has to have and the literals it holds.
        let is_member = match tag_value {
            Some(tag_value) => rendered(ts_template! {
                typeof value === "object" && value !== null && "@{tag_field}" in value && value["@{tag_field}"] === "@{tag_value}"
            }),
            None => object_shape_test(fields, "value"),
        };
        let statements = encode_fields(&encode_fields_list, tag_field);
        coders.push(ts_template! {
            (value: @{union_type}): unknown => {
                if (!(@{is_member})) {
                    return @{endec_helper("notTheMember")};
                }
                const result: Record<string, unknown> = {};
                {$typescript statements}
                return result;
            }
        });
    }
    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }
    Ok((coders, diagnostics.into_vec()))
}

/// A test that `member`, a value of the union, is of the user type `name`:
/// by the keys and literals of the fields its declaration has, or by the
/// `hasShape` its `Decode` writes when it has no fields to tell it by. `None`
/// for an enum, whose values encode as themselves. A type the registry does
/// not hold, as in a file expanded without a scan of its project, is taken to
/// derive the `Decode` that writes its `hasShape`.
fn named_member_test(name: &str, scope: &TypeScope<'_>) -> Result<Option<String>, String> {
    let has_shape = || {
        let has_shape = ts_ident!("{}HasShape", function_prefix(name));
        rendered(ts_template! { @{has_shape}(member) })
    };
    let Some(entry) = scope
        .registry
        .resolve_in_file(name, scope.file_path, scope.file_imports)
    else {
        return Ok(Some(has_shape()));
    };
    if matches!(entry.definition, TypeDefinitionIR::Enum(_)) {
        return Ok(None);
    }
    if let Some(fields) = fields_from_definition(entry, scope.registry) {
        // A `#private` member is no key of the value.
        let keyed: Vec<InterfaceFieldIR> = fields
            .into_iter()
            .filter(|field| field.key != MemberKey::Private)
            .collect();
        return Ok(Some(object_shape_test(&keyed, "member")));
    }
    if entry_derives(entry, "Decode") {
        return Ok(Some(has_shape()));
    }
    Err(format!(
        "nothing tells a value of the member `{name}` from the other members': it has no fields of its own, and derives no `Decode`, whose `hasShape` would; derive `Decode` on `{name}`"
    ))
}

/// The text of a quoted string literal type, such as `circle` for `'circle'`.
fn string_literal(ts_type: &str) -> Option<&str> {
    let trimmed = ts_type.trim();
    ['\'', '"'].into_iter().find_map(|quote| {
        trimmed
            .strip_prefix(quote)
            .and_then(|inner| inner.strip_suffix(quote))
    })
}

#[cfg(test)]
mod tests {
    use super::named_member_test;
    use crate::builtin::derive::endec::coder::TypeScope;
    use crate::ts_syn::abi::ir::type_registry::{
        TypeDefinitionIR, TypeRegistry, TypeRegistryEntry,
    };
    use crate::ts_syn::lower::{lower_enums, lower_interfaces, lower_type_aliases};
    use crate::ts_syn::oxc::allocator::Allocator;
    use crate::ts_syn::oxc::parser::Parser;
    use crate::ts_syn::oxc::span::SourceType;

    const FILE: &str = "/project/src/pets.ts";

    fn registry_of(source: &str) -> TypeRegistry {
        let allocator = Allocator::default();
        let program = Parser::new(&allocator, source, SourceType::ts())
            .parse()
            .program;
        let definitions = lower_interfaces(&program, source, None)
            .expect("the interfaces lower")
            .into_iter()
            .map(TypeDefinitionIR::Interface)
            .chain(
                lower_enums(&program, source, None)
                    .expect("the enums lower")
                    .into_iter()
                    .map(TypeDefinitionIR::Enum),
            )
            .chain(
                lower_type_aliases(&program, source)
                    .expect("the aliases lower")
                    .into_iter()
                    .map(TypeDefinitionIR::TypeAlias),
            );
        let mut registry = TypeRegistry::new();
        for definition in definitions {
            let name = match &definition {
                TypeDefinitionIR::Class(class) => class.name.clone(),
                TypeDefinitionIR::Interface(interface) => interface.name.clone(),
                TypeDefinitionIR::TypeAlias(alias) => alias.name.clone(),
                TypeDefinitionIR::Enum(declared) => declared.name.clone(),
            };
            registry.insert(
                TypeRegistryEntry {
                    name,
                    file_path: FILE.to_string(),
                    is_exported: true,
                    exported_as: Vec::new(),
                    re_exported: Vec::new(),
                    definition,
                    file_imports: vec![],
                },
                "/project",
            );
        }
        registry
    }

    #[test]
    fn a_named_member_is_told_by_its_own_fields_or_its_shape_test() {
        let registry = registry_of(
            "/** @derive(Encode) */ export interface Cat { kind: 'cat'; born: Date; nickname?: string; }\n\
             /** @derive(Encode) */ export enum Mood { Calm = 'calm' }\n\
             /** @derive(Encode) */ export type Stamp = Date | string;\n\
             /** @derive(Encode, Decode) */ export type Mark = Date | number;\n",
        );
        let scope = TypeScope {
            registry: &registry,
            file_path: FILE,
            file_imports: &[],
            type_params: &[],
        };
        let test = |name: &str| named_member_test(name, &scope);
        assert_eq!(
            test("Cat").expect("Cat has fields").as_deref(),
            Some(
                r#"typeof member === "object" && member !== null && !Array.isArray(member) && "kind" in member && member["kind"] === 'cat' && "born" in member"#
            )
        );
        assert_eq!(test("Mood").expect("an enum encodes as itself"), None);
        assert_eq!(
            test("Mark").expect("Mark derives Decode").as_deref(),
            Some("markHasShape(member)")
        );
        assert_eq!(
            test("Elsewhere")
                .expect("an unknown type keeps its shape test")
                .as_deref(),
            Some("elsewhereHasShape(member)")
        );
        let refused = test("Stamp").expect_err("nothing tells Stamp apart");
        assert!(refused.contains("derive `Decode` on `Stamp`"), "{refused}");
    }
}
