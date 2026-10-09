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
use super::super::types::EncodeField;
use super::AliasEncode;
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
        encode_context_expr,
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
        ser_fn: crate::ast::Ident,
        has_shape_fn: crate::ast::Ident,
    }
    let mut ser_variants: Vec<SerVariant> = Vec::new();
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
        payload_ser_fn: Option<crate::ast::Ident>,
    }
    let mut external_ser_variants: Vec<ExternalSerVariant> = Vec::new();
    if let Some(members) = type_alias.as_union() {
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
                            ser_fn: ts_ident!("{}EncodeWithContext", camel),
                            has_shape_fn: ts_ident!("{}HasShape", camel),
                        });
                    }
                }
                // Untagged encodable type-ref variant: `TypeRef`
                TypeMemberKind::TypeRef(t)
                    if is_untagged
                        && !matches!(
                            TypeCategory::from_ts_type(t),
                            TypeCategory::Primitive | TypeCategory::Date
                        ) =>
                {
                    let camel = t.to_case(Case::Camel);
                    ser_variants.push(SerVariant {
                        tag_value: t.clone(),
                        ser_fn: ts_ident!("{}EncodeWithContext", camel),
                        has_shape_fn: ts_ident!("{}HasShape", camel),
                    });
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
                    } else if let TypeCategory::Encodable(base) =
                        TypeCategory::from_ts_type(payload_ts_type)
                    {
                        Some(ts_ident!("{}EncodeWithContext", base.to_case(Case::Camel)))
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
    let has_external_ser_variants = !external_ser_variants.is_empty();
    let dispatch_by_tag = !is_untagged;

    let (object_dispatch, object_diagnostics) = object_variant_dispatch(alias, &tag_field)?;

    let mut result = if let Some(params) = &type_params_ident {
        ts_template! {
            /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
            export function @{fn_encode_ident}<@{params}>(value: @{full_type_ident}, keepMetadata?: boolean): string {
                const ctx = @{encode_context_expr}.create();
                const __raw = @{fn_encode_internal_expr}(value, ctx);
                if (keepMetadata) return JSON.stringify(__raw);
                return JSON.stringify(__raw, (key, val) => key === "__type" || key === "__id" ? undefined : val);
            }

            /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
            export function @{fn_encode_internal_ident}<@{params}>(value: @{full_type_ident}, ctx: @{encode_context_ident}): unknown {
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
                        return { "@{tag_field}": (value as any)["@{tag_field}"] ?? __typeName, ...fields };
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
                        if (__idVal !== undefined) __outer.__id = __idVal;
                        return __outer;
                    {:else if is_adjacently_tagged}
                        // Adjacently tagged: { tag: "TypeName", content: { ...fields } }
                        const __outer: Record<string, unknown> = { "@{tag_field}": __typeName, "@{content_field}": fields };
                        if (__idVal !== undefined) __outer.__id = __idVal;
                        return __outer;
                    {:else if is_untagged}
                        // Untagged: just the raw fields
                        const __outer: Record<string, unknown> = { ...fields };
                        if (__idVal !== undefined) __outer.__id = __idVal;
                        return __outer;
                    {/if}
                {/if}
            }
        }
    } else {
        ts_template! {
            /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
            export function @{fn_encode_ident}(value: @{full_type_ident}, keepMetadata?: boolean): string {
                const ctx = @{encode_context_expr}.create();
                const __raw = @{fn_encode_internal_expr}(value, ctx);
                if (keepMetadata) return JSON.stringify(__raw);
                return JSON.stringify(__raw, (key, val) => key === "__type" || key === "__id" ? undefined : val);
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
                        return { "@{tag_field}": (value as any)["@{tag_field}"] ?? __typeName, ...fields };
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
                        if (__idVal !== undefined) __outer.__id = __idVal;
                        return __outer;
                    {:else if is_adjacently_tagged}
                        // Adjacently tagged: { tag: "TypeName", content: { ...fields } }
                        const __outer: Record<string, unknown> = { "@{tag_field}": __typeName, "@{content_field}": fields };
                        if (__idVal !== undefined) __outer.__id = __idVal;
                        return __outer;
                    {:else if is_untagged}
                        // Untagged: just the raw fields
                        const __outer: Record<string, unknown> = { ...fields };
                        if (__idVal !== undefined) __outer.__id = __idVal;
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
    let members = match (&container_opts.tagging, alias.type_alias.as_union()) {
        (TaggingMode::InternallyTagged { .. }, Some(members)) => members,
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
        let Some(tag_value) = fields
            .iter()
            .find(|field| field.name == tag_field)
            .and_then(|field| string_literal(&field.ts_type))
        else {
            continue;
        };
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
        let statements = encode_fields(&encode_fields_list, tag_field);
        blocks.push(ts_template! {
            if (!__matched && typeof value === "object" && value !== null && "@{tag_field}" in value && value["@{tag_field}"] === "@{tag_value}") {
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
