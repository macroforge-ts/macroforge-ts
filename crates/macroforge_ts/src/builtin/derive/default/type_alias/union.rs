//! `Default` for a union type alias: the member marked `@default`, the first
//! member of an object union, or `@default(expression)` on the alias.

use convert_case::{Case, Casing};

use crate::ast::Expr;
use crate::builtin::derive::common::{
    DefaultFieldOptions, TypeNames, flatten_intersection_fields, get_type_default_with_registry,
    rendered,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_alias::TypeMember;
use crate::ts_syn::{DataTypeAlias, DeriveInput, MacroforgeError, TsStream, ts_ident};

use super::super::values::{DefaultSources, contains_top_level_pipe, resolve_default_value};

/// Where a union's default comes from.
enum UnionDefault {
    /// The member marked `@default`, named by its type.
    Member(String),
    /// An expression: an object variant's default, or `@default(...)` on the
    /// alias.
    Value(String),
}

pub(super) fn union_default(
    input: &DeriveInput,
    type_alias: &DataTypeAlias,
    members: &[TypeMember],
    sources: &DefaultSources,
) -> Result<TsStream, MacroforgeError> {
    let type_name = input.name();
    // Union type: check for @default on a variant OR @default(...) on the type

    // Check for parenthesized union members - can't place @default inside parens
    // e.g., `(string | Product) | (string | Service)` is not allowed.
    // Parenthesized intersections like `({ kind: 'A' } & ADetail)` are
    // fine: they preserve doc-comment placement unambiguously and are
    // already handled by `as_intersection_members` below.
    let parenthesized: Vec<&str> = members
        .iter()
        .filter_map(|m| m.as_type_ref())
        .filter(|t| {
            let trimmed = t.trim();
            trimmed.starts_with('(') && contains_top_level_pipe(trimmed)
        })
        .collect();

    if !parenthesized.is_empty() {
        return Err(MacroforgeError::new(
            input.decorator_span(),
            format!(
                "@derive(Default): Parenthesized union expressions ({}) are not supported. \
                 Formatters cannot preserve doc comments inside parentheses. \
                 Create a named type alias for each variant instead \
                 (e.g., use `RecordLink<Product>` instead of `(string | Product)`).",
                parenthesized.join(", ")
            ),
        ));
    }

    // First, look for a variant with @default decorator
    let default_variant_from_member = members.iter().find_map(|member| {
        if member.has_decorator("default") {
            // Named type (TypeRef or Literal): that type's default
            if let Some(name) = member.type_name() {
                return Some(UnionDefault::Member(name.to_string()));
            }
            // Object type (tagged union variant): build an object literal
            // with default values for each field
            if let Some(fields) = member.as_object() {
                return Some(UnionDefault::Value(build_object_default(fields, sources)));
            }
            // Intersection type (tagged union with struct payload).
            // Prefer spreading each TypeRef's `xxxDefaultValue()`
            // so foreign-typed fields like `BigDecimal.BigDecimal`
            // get their proper default expression instead of a
            // bogus camelCase fallback.
            if let Some(intersection_members) = member.as_intersection_members() {
                if let Some(literal) = build_intersection_default(intersection_members, sources) {
                    return Some(UnionDefault::Value(literal));
                }
                // Fallback: full flattening with registry
                if let Some(fields) =
                    flatten_intersection_fields(intersection_members, sources.registry)
                {
                    return Some(UnionDefault::Value(build_object_default(&fields, sources)));
                }
                // Last resort: inline object fields only
                let inline_fields: Vec<_> = intersection_members
                    .iter()
                    .filter_map(|m| m.as_object())
                    .flat_map(|fields| fields.iter().cloned())
                    .collect();
                if !inline_fields.is_empty() {
                    return Some(UnionDefault::Value(build_object_default(
                        &inline_fields,
                        sources,
                    )));
                }
            }
            None
        } else {
            None
        }
    });

    // Fallback for tagged object/intersection unions where @default may not be
    // attached to the member: use the first variant if all are object-like.
    let default_variant_from_member = default_variant_from_member.or_else(|| {
        let all_object_like = members
            .iter()
            .all(|m| m.is_object() || m.as_intersection_members().is_some());
        if all_object_like {
            members
                .first()
                .and_then(|m| -> Option<String> {
                    if let Some(fields) = m.as_object() {
                        Some(build_object_default(fields, sources))
                    } else if let Some(intersection_members) = m.as_intersection_members() {
                        build_intersection_default(intersection_members, sources).or_else(|| {
                            flatten_intersection_fields(intersection_members, sources.registry)
                                .or_else(|| {
                                    let inline: Vec<_> = intersection_members
                                        .iter()
                                        .filter_map(|im| im.as_object())
                                        .flat_map(|f| f.iter().cloned())
                                        .collect();
                                    if inline.is_empty() {
                                        None
                                    } else {
                                        Some(inline)
                                    }
                                })
                                .map(|fields| build_object_default(&fields, sources))
                        })
                    } else {
                        None
                    }
                })
                .map(UnionDefault::Value)
        } else {
            None
        }
    });

    // Fall back to @default(...) on the type alias itself
    let default_variant = default_variant_from_member.or_else(|| {
        let default_opts = DefaultFieldOptions::from_decorators(
            &input
                .attrs
                .iter()
                .map(|a| a.inner.clone())
                .collect::<Vec<_>>(),
        );
        default_opts.value.map(UnionDefault::Value)
    });

    if let Some(default_variant) = default_variant {
        let (UnionDefault::Member(variant) | UnionDefault::Value(variant)) = &default_variant;
        if variant.is_empty() {
            return Err(MacroforgeError::new(
                input.decorator_span(),
                format!(
                    "@derive(Default): resolved an empty default expression for union type '{}'. \
                     Add @default on a variant or @default(expression) on the type.",
                    type_name
                ),
            ));
        }
        let default_expr = match default_variant {
            // A literal member is its own default; a named one, foreign types
            // and generic aliases included, takes its type's.
            UnionDefault::Member(member) => get_type_default_with_registry(
                &member,
                sources.registry,
                sources.caller_file_path,
                sources.file_imports,
            ),
            UnionDefault::Value(value) => value,
        };

        // Handle generic type aliases (e.g., type RecordLink<T> = ...)
        let names = TypeNames::new(type_name, type_alias.type_params());
        let return_type_ident = names.full_type_ident();
        let generic_params_ident = ts_ident!(names.generic_decl.as_str());

        let fn_name_ident = ts_ident!("{}DefaultValue", type_name.to_case(Case::Camel));
        let return_expr = Expr::parse(&default_expr).map_err(|err| {
            MacroforgeError::new(
                input.decorator_span(),
                format!(
                    "@derive(Default): invalid default expression for '{}': {err:?}",
                    type_name
                ),
            )
        })?;
        Ok(ts_template! {
            export function @{fn_name_ident}@{generic_params_ident}(): @{return_type_ident} {
                return @{return_expr};
            }
        })
    } else {
        Err(MacroforgeError::new(
            input.decorator_span(),
            format!(
                "@derive(Default) on union type '{}' requires @default on one variant \
                or @default(VariantName.defaultValue()) on the type.",
                type_name
            ),
        ))
    }
}

/// build an object literal default from an inline object variant's fields
fn build_object_default(
    fields: &[crate::ts_syn::InterfaceFieldIR],
    sources: &DefaultSources,
) -> String {
    let props: Vec<String> = fields
        .iter()
        .map(|f| {
            let opts = DefaultFieldOptions::from_decorators(&f.decorators);
            let value = resolve_default_value(opts.value, &f.ts_type, sources);
            rendered(ts_template! { @{&f.name}: @{value} })
        })
        .collect();
    let props = props.join(", ");
    rendered(ts_template! { ({ @{props} }) })
}

/// build an intersection variant default by spreading
// each TypeRef's `{type}DefaultValue()` and inlining the object
// part's fields. Avoids flattening foreign-typed fields like
// `BigDecimal.BigDecimal` whose per-field `get_type_default`
// can't resolve them in this codepath and emits bogus camelCase
// calls (`bigDecimal.bigDecimalDefaultValue()`).
fn build_intersection_default(
    intersection_members: &[crate::ts_syn::TypeMember],
    sources: &DefaultSources,
) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    for member in intersection_members {
        if let Some(type_name) = member.as_type_ref() {
            let default_fn = ts_ident!("{}DefaultValue", type_name.trim().to_case(Case::Camel));
            parts.push(rendered(ts_template! { ...@{default_fn}() }));
        } else if let Some(fields) = member.as_object() {
            for f in fields {
                let opts = DefaultFieldOptions::from_decorators(&f.decorators);
                let value = resolve_default_value(opts.value, &f.ts_type, sources);
                parts.push(rendered(ts_template! { @{&f.name}: @{value} }));
            }
        }
        // Literals and nested intersections fall through :
        // we only handle the common `{tag} & TypeRef` shape.
    }
    if parts.is_empty() {
        None
    } else {
        let parts = parts.join(", ");
        Some(rendered(ts_template! { ({ @{parts} }) }))
    }
}
