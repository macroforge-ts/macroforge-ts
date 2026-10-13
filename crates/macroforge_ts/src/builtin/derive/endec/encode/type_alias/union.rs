//! `Encode` for a union type alias, per its tagging mode.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_alias::TypeMemberKind;
use crate::ts_syn::abi::{Diagnostic, DiagnosticCollector};
use crate::ts_syn::config::ForeignHandler;
use crate::ts_syn::{MacroforgeError, MacroforgeErrors, TsStream, ts_ident};

use convert_case::{Case, Casing};

use super::super::field::prepare::to_encode_field;
use super::super::field::statements::encode_fields;
use super::super::text::encode_text_body;
use super::super::types::EncodeField;
use super::AliasEncode;
use crate::builtin::derive::common::rendered;
use crate::builtin::derive::endec::coder::{TypeScope, object_shape_test};
use crate::builtin::derive::endec::type_arguments::{member_encoder, param_coder};
use crate::builtin::derive::endec::union_guard::Direction;
use crate::builtin::derive::endec::{
    EndecContainerOptions, TaggingMode, TypeCategory, get_foreign_types,
};

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

    // Enumerate variants that expose a dedicated per-variant encoder so the
    // union encoder can DISPATCH to them rather than relying on the value
    // carrying a `encodeWithContext` method. Plain objects (values not
    // constructed through a generated factory) have no such method, so without
    // dispatch their variant fields that need non-identity encoding leak
    // as raw values instead of passing through the per-variant encoder.
    struct SerVariant {
        tag_value: String,
        ser_fn: Expr,
        has_shape_fn: crate::ast::Ident,
    }
    let mut ser_variants: Vec<SerVariant> = Vec::new();
    // A member that is a user type is told by its tag when the value
    // carries one, as a decoded value does, and otherwise by its shape: a
    // plain object says no other way which member it is.
    let mut ref_variants: Vec<SerVariant> = Vec::new();
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
    // What encodes a member that is a user type, with the coders of its
    // type arguments handed on.
    let member_encoder_of = |ts_type: &str| -> Result<Expr, MacroforgeError> {
        member_encoder(ts_type, &scope)
            .map(Expr::Source)
            .map_err(|message| {
                MacroforgeError::new(type_alias.inner.span, format!("@derive(Encode): {message}"))
            })
    };
    // One parameter among members that are otherwise told by `null`, a tag
    // or a shape is the member a value is when it is no other, so the coder
    // handed in for it encodes that value.
    let param_encoder: Option<(String, String)> = type_alias
        .body()
        .unbranded()
        .as_union()
        .and_then(|members| {
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
                }
                TypeMemberKind::Object { .. } | TypeMemberKind::Intersection(_) => true,
                TypeMemberKind::Brand(_) => false,
            });
            told_apart.then(|| (param_coder(Direction::Encode, &param), param))
        });
    // Externally-tagged single-key object variants (`{ Name: Payload }`)
    // whose payload is a configured foreign type (BigDecimal, DateTime):
    // without the inline encode expression, the runtime instance
    // passes through and JSON.stringify emits its internal object
    // shape. Mirrors `external_object_variants` on the decode side.
    struct ExternalSerVariant {
        name: String,
        foreign_encode_callee: Option<String>,
        /// Set when the payload is a generated type instead: without
        /// dispatching to its encoder the payload is emitted
        /// verbatim, so every field inside it that needs a non-identity
        /// encoding (decimals, dates, record links) leaks its
        /// runtime shape.
        payload_ser_fn: Option<Expr>,
    }
    let mut external_ser_variants: Vec<ExternalSerVariant> = Vec::new();
    if let Some(members) = type_alias.body().unbranded().as_union() {
        for m in members {
            match &m.kind {
                // Tagged intersection variant: `{ variant: 'X' } & TypeRef`
                TypeMemberKind::Intersection(subs) => {
                    let mut tag_value: Option<String> = None;
                    let mut ref_type: Option<String> = None;
                    for sub in subs {
                        match &sub.kind {
                            TypeMemberKind::Object { fields } => {
                                tag_value = fields.iter().find_map(|f| {
                                    if f.name == tag_field {
                                        Some(
                                            f.ts_type
                                                .trim()
                                                .trim_matches('\'')
                                                .trim_matches('"')
                                                .to_string(),
                                        )
                                    } else {
                                        None
                                    }
                                });
                            }
                            TypeMemberKind::TypeRef(t) => ref_type = Some(t.clone()),
                            _ => {}
                        }
                    }
                    if let (Some(tv), Some(rt)) = (tag_value, ref_type) {
                        let camel = rt.to_case(Case::Camel);
                        ser_variants.push(SerVariant {
                            tag_value: tv,
                            ser_fn: member_encoder_of(&rt)?,
                            has_shape_fn: ts_ident!("{}HasShape", camel),
                        });
                    }
                }
                TypeMemberKind::TypeRef(t) => {
                    if let TypeCategory::Encodable(name) = TypeCategory::from_ts_type(t)
                        && !is_type_param(&name)
                        && TypeCategory::match_foreign_type(t, &get_foreign_types())
                            .config
                            .is_none()
                    {
                        let camel = name.to_case(Case::Camel);
                        ref_variants.push(SerVariant {
                            tag_value: name,
                            ser_fn: member_encoder_of(t)?,
                            has_shape_fn: ts_ident!("{}HasShape", camel),
                        });
                    }
                }
                TypeMemberKind::Object { fields } if is_externally_tagged && !fields.is_empty() => {
                    let foreign_types = get_foreign_types();
                    let payload_ts_type = fields[0].ts_type.as_str();
                    let foreign_encode_callee =
                        TypeCategory::match_foreign_type(payload_ts_type, &foreign_types)
                            .config
                            .and_then(|ft| ft.handler_callee(ForeignHandler::Encode));
                    let payload_ser_fn = if foreign_encode_callee.is_some() {
                        None
                    } else if let TypeCategory::Encodable(_) =
                        TypeCategory::from_ts_type(payload_ts_type)
                    {
                        Some(member_encoder_of(payload_ts_type)?)
                    } else {
                        None
                    };
                    if foreign_encode_callee.is_some() || payload_ser_fn.is_some() {
                        external_ser_variants.push(ExternalSerVariant {
                            name: fields[0].name.clone(),
                            foreign_encode_callee,
                            payload_ser_fn,
                        });
                    }
                }
                _ => {}
            }
        }
    }
    let has_ser_variants = !ser_variants.is_empty();
    let has_ref_variants = !ref_variants.is_empty();
    let has_external_ser_variants = !external_ser_variants.is_empty();
    let dispatch_by_tag = !is_untagged;

    let encoded_as: Expr = ts_ident!(crate::builtin::derive::endec::coder::endec_helper(
        "encodedAs"
    ))
    .into();
    let (object_dispatch, object_diagnostics) = object_variant_dispatch(alias, &tag_field)?;
    let encode_text = encode_text_body(&fn_encode_internal_expr, "__type", passed_coders);

    let mut result = if let Some(params) = &type_params_ident {
        ts_template! {
            /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
            export function @{fn_encode_ident}<@{params}>(value: @{full_type_ident}, keepMetadata?: boolean@{coders}): string {
                {$typescript encode_text}
            }

            /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
            export function @{fn_encode_internal_ident}<@{params}>(value: @{full_type_ident}, ctx: @{encode_context_ident}@{coders}): unknown {
                let __variant: unknown;
                let __matched = false;
                const __encoder = __mf_derivedEncoder(value);
                if (__encoder) {
                    __variant = __encoder(value, ctx);
                    __matched = true;
                }
                {#if has_ser_variants}
                if (!__matched && value !== null && typeof value === "object") {
                    {#if dispatch_by_tag}
                    const __tag = (value as any)["@{tag_field}"];
                    {#for v in &ser_variants}
                    if (!__matched && __tag === "@{v.tag_value}") { __variant = @{v.ser_fn}(value as any, ctx); __matched = true; }
                    {/for}
                    {:else}
                    {#for v in &ser_variants}
                    if (!__matched && @{v.has_shape_fn}(value)) { __variant = @{v.ser_fn}(value as any, ctx); __matched = true; }
                    {/for}
                    {/if}
                }
                {/if}
                {#if has_ref_variants}
                if (!__matched && value !== null && typeof value === "object") {
                    const __refTag = (value as any)["@{tag_field}"];
                    {#for v in &ref_variants}
                    if (!__matched && __refTag === "@{v.tag_value}") { __variant = @{v.ser_fn}(value as any, ctx); __matched = true; }
                    {/for}
                    {#for v in &ref_variants}
                    if (!__matched && @{v.has_shape_fn}(value)) { __variant = @{v.ser_fn}(value as any, ctx); __matched = true; }
                    {/for}
                }
                {/if}
                {$typescript object_dispatch}
                {#if has_external_ser_variants}
                if (!__matched && value !== null && typeof value === "object") {
                    const __exName = Object.keys(value as object)[0];
                    {#for ov in &external_ser_variants}
                    {#if let Some(ref encode_callee) = ov.foreign_encode_callee}
                    {$let foreign_ser_expr: Expr = Expr::parse(encode_callee).expect("inner foreign encode expr should parse")}
                    if (!__matched && __exName === "@{ov.name}") {
                        __variant = ({ "@{ov.name}": @{foreign_ser_expr}((value as any)["@{ov.name}"]) });
                        __matched = true;
                    }
                    {/if}
                    {#if let Some(ref payload_ser_fn) = ov.payload_ser_fn}
                    if (!__matched && __exName === "@{ov.name}") {
                        __variant = ({ "@{ov.name}": @{payload_ser_fn}((value as any)["@{ov.name}"], ctx) });
                        __matched = true;
                    }
                    {/if}
                    {/for}
                }
                {/if}
                {#if let Some((encode_param, param)) = &param_encoder}
                if (!__matched && @{encode_param} !== undefined && value !== null && value !== undefined) {
                    __variant = @{encode_param}(value as @{param}, ctx);
                    __matched = true;
                }
                {/if}
                if (!__matched) {
                    __variant = value;
                }

                {#if is_internally_tagged}
                    // Internally tagged: keep the tag field. A dispatched
                    // per-variant result carries the inner struct's `__type`
                    // (e.g. `PartialUser`), which differs from the union's
                    // variant label (e.g. `User`) whenever the variant name and
                    // its payload type name differ: so restore the discriminator
                    // from the value's own tag field, falling back to `__type`
                    // only when the value carries no tag. A passed-through value
                    // already carries the tag field.
                    if (__variant !== null && typeof __variant === "object" && "__type" in (__variant as any)) {
                        const { __type: __typeName, ...fields } = __variant as any;
                        const __tagged = { "@{tag_field}": (value as any)["@{tag_field}"] ?? __typeName, ...fields };
                        // What a later reference to this value stands for is
                        // the member as the union tags it.
                        return typeof fields.__id === "number" ? @{&encoded_as}(ctx, fields.__id, __tagged) : __tagged;
                    }
                    return __variant;
                {:else}
                    // Non-internally-tagged modes: restructure the variant output
                    if (typeof __variant !== "object" || __variant === null) return __variant;
                    const { __type: __typeName, __id: __idVal, ...fields } = __variant as any;
                    if (typeof __typeName !== "string") return __variant;

                    {#if is_externally_tagged}
                        // Externally tagged: { "TypeName": { ...fields } }
                        const __outer: Record<string, unknown> = { [__typeName]: fields };
                        if (__idVal !== undefined) @{&encoded_as}(ctx, __idVal, __outer);
                        return __outer;
                    {:else if is_adjacently_tagged}
                        // Adjacently tagged: { tag: "TypeName", content: { ...fields } }
                        const __outer: Record<string, unknown> = { "@{tag_field}": __typeName, "@{content_field}": fields };
                        if (__idVal !== undefined) @{&encoded_as}(ctx, __idVal, __outer);
                        return __outer;
                    {:else if is_untagged}
                        // Untagged: just the raw fields
                        const __outer: Record<string, unknown> = { ...fields };
                        if (__idVal !== undefined) @{&encoded_as}(ctx, __idVal, __outer);
                        return __outer;
                    {/if}
                {/if}
            }
        }
    } else {
        ts_template! {
            /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
            export function @{fn_encode_ident}(value: @{full_type_ident}, keepMetadata?: boolean): string {
                {$typescript encode_text}
            }

            /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
            export function @{fn_encode_internal_ident}(value: @{full_type_ident}, ctx: @{encode_context_ident}): unknown {
                let __variant: unknown;
                let __matched = false;
                const __encoder = __mf_derivedEncoder(value);
                if (__encoder) {
                    __variant = __encoder(value, ctx);
                    __matched = true;
                }
                {#if has_ser_variants}
                if (!__matched && value !== null && typeof value === "object") {
                    {#if dispatch_by_tag}
                    const __tag = (value as any)["@{tag_field}"];
                    {#for v in &ser_variants}
                    if (!__matched && __tag === "@{v.tag_value}") { __variant = @{v.ser_fn}(value as any, ctx); __matched = true; }
                    {/for}
                    {:else}
                    {#for v in &ser_variants}
                    if (!__matched && @{v.has_shape_fn}(value)) { __variant = @{v.ser_fn}(value as any, ctx); __matched = true; }
                    {/for}
                    {/if}
                }
                {/if}
                {#if has_ref_variants}
                if (!__matched && value !== null && typeof value === "object") {
                    const __refTag = (value as any)["@{tag_field}"];
                    {#for v in &ref_variants}
                    if (!__matched && __refTag === "@{v.tag_value}") { __variant = @{v.ser_fn}(value as any, ctx); __matched = true; }
                    {/for}
                    {#for v in &ref_variants}
                    if (!__matched && @{v.has_shape_fn}(value)) { __variant = @{v.ser_fn}(value as any, ctx); __matched = true; }
                    {/for}
                }
                {/if}
                {$typescript object_dispatch}
                {#if has_external_ser_variants}
                if (!__matched && value !== null && typeof value === "object") {
                    const __exName = Object.keys(value as object)[0];
                    {#for ov in &external_ser_variants}
                    {#if let Some(ref encode_callee) = ov.foreign_encode_callee}
                    {$let foreign_ser_expr: Expr = Expr::parse(encode_callee).expect("inner foreign encode expr should parse")}
                    if (!__matched && __exName === "@{ov.name}") {
                        __variant = ({ "@{ov.name}": @{foreign_ser_expr}((value as any)["@{ov.name}"]) });
                        __matched = true;
                    }
                    {/if}
                    {#if let Some(ref payload_ser_fn) = ov.payload_ser_fn}
                    if (!__matched && __exName === "@{ov.name}") {
                        __variant = ({ "@{ov.name}": @{payload_ser_fn}((value as any)["@{ov.name}"], ctx) });
                        __matched = true;
                    }
                    {/if}
                    {/for}
                }
                {/if}
                if (!__matched) {
                    __variant = value;
                }

                {#if is_internally_tagged}
                    // Internally tagged: keep the tag field. A dispatched
                    // per-variant result carries the inner struct's `__type`
                    // (e.g. `PartialUser`), which differs from the union's
                    // variant label (e.g. `User`) whenever the variant name and
                    // its payload type name differ: so restore the discriminator
                    // from the value's own tag field, falling back to `__type`
                    // only when the value carries no tag. A passed-through value
                    // already carries the tag field.
                    if (__variant !== null && typeof __variant === "object" && "__type" in (__variant as any)) {
                        const { __type: __typeName, ...fields } = __variant as any;
                        const __tagged = { "@{tag_field}": (value as any)["@{tag_field}"] ?? __typeName, ...fields };
                        // What a later reference to this value stands for is
                        // the member as the union tags it.
                        return typeof fields.__id === "number" ? @{&encoded_as}(ctx, fields.__id, __tagged) : __tagged;
                    }
                    return __variant;
                {:else}
                    // Non-internally-tagged modes: restructure the variant output
                    if (typeof __variant !== "object" || __variant === null) return __variant;
                    const { __type: __typeName, __id: __idVal, ...fields } = __variant as any;
                    if (typeof __typeName !== "string") return __variant;

                    {#if is_externally_tagged}
                        // Externally tagged: { "TypeName": { ...fields } }
                        const __outer: Record<string, unknown> = { [__typeName]: fields };
                        if (__idVal !== undefined) @{&encoded_as}(ctx, __idVal, __outer);
                        return __outer;
                    {:else if is_adjacently_tagged}
                        // Adjacently tagged: { tag: "TypeName", content: { ...fields } }
                        const __outer: Record<string, unknown> = { "@{tag_field}": __typeName, "@{content_field}": fields };
                        if (__idVal !== undefined) @{&encoded_as}(ctx, __idVal, __outer);
                        return __outer;
                    {:else if is_untagged}
                        // Untagged: just the raw fields
                        const __outer: Record<string, unknown> = { ...fields };
                        if (__idVal !== undefined) @{&encoded_as}(ctx, __idVal, __outer);
                        return __outer;
                    {/if}
                {/if}
            }
        }
    };
    result.add_aliased_import("EncodeContext", crate::package::ENDEC);
    result.add_aliased_import("derivedEncoder", crate::package::ENDEC);
    result.add_diagnostics(object_diagnostics);
    Ok(result)
}

/// Blocks encoding each inline object member of an internally tagged union
/// field by field into `__variant`, chosen by its tag. A member passed through
/// instead keeps the runtime shape of every field needing a non-identity
/// encoding: a `Map` becomes `{}`, a nested type skips its encoder.
fn object_variant_dispatch(
    alias: &AliasEncode,
    tag_field: &str,
) -> Result<(TsStream, Vec<Diagnostic>), MacroforgeError> {
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
        _ => return Ok((TsStream::merge_all([]), Vec::new())),
    };
    let param_names: Vec<String> = alias
        .type_params
        .iter()
        .map(|param| param.name.clone())
        .collect();
    let mut diagnostics = DiagnosticCollector::new();
    let mut blocks = Vec::new();
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
        blocks.push(ts_template! {
            if (!__matched && @{is_member}) {
                const result: Record<string, unknown> = {};
                {$typescript statements}
                __variant = result;
                __matched = true;
            }
        });
    }
    if diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
    }
    Ok((TsStream::merge_all(blocks), diagnostics.into_vec()))
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
