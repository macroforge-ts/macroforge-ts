//! A union alias's members, sorted by how each one decodes.

use crate::ast::{Expr, Ident};
use crate::ts_syn::config::ForeignHandler;
use std::rc::Rc;

use crate::builtin::derive::common::{js_string, rendered};
use crate::host::ForeignTypeConfig;
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::interface::InterfaceFieldIR;
use crate::ts_syn::abi::ir::type_alias::{TypeMember, TypeMemberKind};
use crate::ts_syn::abi::ir::type_registry::{FileImportEntry, TypeRegistry};
use crate::ts_syn::{MacroforgeError, MacroforgeErrors, ts_ident};

use super::super::AliasDecode;
use crate::builtin::derive::endec::decode::helpers::{extract_base_type, nested_decode_fn_name};
use crate::builtin::derive::endec::decode::types::EncodableTypeRef;
use crate::builtin::derive::endec::validators::ValidatorSpec;
use crate::builtin::derive::endec::{
    EndecContainerOptions, TaggingMode, TypeCategory, decorator_validators, get_foreign_types,
};

// Collect inline object variants (e.g., { variant: 'GlobalAdmin' } | { variant: 'AppRoles'; ... })
// These are tagged by a discriminant field (typically the endec tag field).
pub(super) struct ObjectVariant {
    pub(super) tag_value: String,
    pub(super) fields: Vec<InterfaceFieldIR>,
}

// Collect intersection variants (e.g., { variant: 'AppRoles' } & AppRoles)
// These have a tag in the inline object part and delegate to a type ref for the data.
pub(super) struct IntersectionVariant {
    pub(super) tag_value: String,
    pub(super) type_ref: String,
    pub(super) is_encodable: bool,
}

// Inline object members without a tag field (e.g. `string | { id: string }`)
// can only be told apart by shape: every required field must be present.
pub(super) struct UntaggedObjectVariant {
    /// JS condition that is true when `value` has this member's shape.
    pub(super) shape_condition: String,
    pub(super) required_fields: Vec<String>,
    pub(super) optional_fields: Vec<String>,
}

pub(super) struct ExternalObjectVariant {
    pub(super) name: String,
    /// If the variant payload (`fields[0].ts_type`) is a configured foreign
    /// type (e.g. `DateTime.Utc`), this is the inline decode
    /// expression to invoke on `__inner`. Without this, primitive
    /// passthrough returns the raw JSON (e.g. an ISO string) and downstream
    /// code expecting a `DateTime` object breaks.
    pub(super) inner_foreign_decode_callee: Option<String>,
    /// Set when the payload is a generated type instead: without dispatching
    /// to its decoder the payload is kept verbatim, so decimals stay
    /// strings and dates stay ISO text inside the variant.
    pub(super) payload_decode_fn: Option<crate::ast::Ident>,
}

/// A union alias's members, sorted by how each decodes, with everything the
/// generated functions need to name and report them.
pub(super) struct Union<'a> {
    pub(super) type_name: &'a str,
    pub(super) decode_context_ident: Ident,
    pub(super) decode_context_expr: Expr,
    pub(super) decode_error_expr: Expr,
    pub(super) pending_ref_ident: Ident,
    pub(super) pending_ref_expr: Expr,
    pub(super) decode_options_ident: Ident,
    pub(super) generic_decl: &'a str,
    pub(super) generic_args: &'a str,
    pub(super) full_type_name: &'a str,
    pub(super) full_type_ident: Ident,
    pub(super) type_registry: &'a TypeRegistry,
    pub(super) caller_file_path: &'a str,
    pub(super) file_imports: &'a [FileImportEntry],
    pub(super) foreign_types_config: Rc<[ForeignTypeConfig]>,
    pub(super) tag_field: String,
    pub(super) content_field: String,
    pub(super) is_externally_tagged: bool,
    pub(super) is_adjacently_tagged: bool,
    pub(super) is_untagged: bool,
    pub(super) literals: Vec<String>,
    pub(super) primitive_types: Vec<String>,
    pub(super) primitive_arms: Vec<(String, Vec<ValidatorSpec>)>,
    pub(super) has_arm_validators: bool,
    pub(super) encodable_types: Vec<EncodableTypeRef>,
    pub(super) has_primitives: bool,
    pub(super) has_encodables: bool,
    pub(super) has_dates: bool,
    pub(super) has_generic_params: bool,
    pub(super) regular_encodables: Vec<EncodableTypeRef>,
    pub(super) foreign_encodables: Vec<EncodableTypeRef>,
    pub(super) object_variants: Vec<ObjectVariant>,
    pub(super) untagged_object_variants: Vec<UntaggedObjectVariant>,
    pub(super) intersection_variants: Vec<IntersectionVariant>,
    pub(super) external_object_variants: Vec<ExternalObjectVariant>,
    pub(super) has_object_variants: bool,
    pub(super) has_intersection_variants: bool,
    pub(super) is_literal_only: bool,
    pub(super) is_primitive_only: bool,
    pub(super) is_encodable_only: bool,
    pub(super) has_literals: bool,
    pub(super) expected_types_str: String,
    pub(super) primitive_check_condition: String,
    pub(super) encodable_type_check_condition: String,
}

impl<'a> Union<'a> {
    /// Sorts `members` and reports validators written on arms that cannot
    /// carry them.
    pub(super) fn new(
        alias: &'a AliasDecode<'a>,
        members: &[TypeMember],
    ) -> Result<Self, MacroforgeError> {
        let AliasDecode {
            type_alias,
            type_name,
            full_type_name,
            ..
        } = alias;
        // Union type - could be literal union, type ref union, or mixed
        let container_opts = EndecContainerOptions::from_decorators(&type_alias.inner.decorators);
        let tag_field = container_opts.tag_field_or_default();

        // Tagging mode variables for template branching
        let is_externally_tagged = matches!(container_opts.tagging, TaggingMode::ExternallyTagged);
        let is_adjacently_tagged =
            matches!(container_opts.tagging, TaggingMode::AdjacentlyTagged { .. });
        let is_untagged = matches!(container_opts.tagging, TaggingMode::Untagged);
        let content_field = container_opts.content_field().unwrap_or("").to_string();

        // Create a set of type parameter names for filtering
        let type_param_set: std::collections::HashSet<&str> = type_alias
            .type_params()
            .iter()
            .map(|param| param.name.as_str())
            .collect();

        let literals: Vec<String> = members
            .iter()
            .filter_map(|m| m.as_literal().map(|s| s.to_string()))
            .collect();
        let type_refs: Vec<String> = members
            .iter()
            .filter_map(|m| m.as_type_ref().map(|s| s.to_string()))
            .collect();

        // Separate primitives, generic type params, and encodable types
        let primitive_types: Vec<String> = type_refs
            .iter()
            .filter(|t| matches!(TypeCategory::from_ts_type(t), TypeCategory::Primitive))
            .cloned()
            .collect();

        // Validators written on an arm (`| /** @endec(email) */ string`) run inside
        // that arm's `typeof` branch. No other arm has a single value to validate.
        let mut arm_diagnostics = DiagnosticCollector::new();
        let mut primitive_arms: Vec<(String, Vec<ValidatorSpec>)> = primitive_types
            .iter()
            .map(|prim| (prim.clone(), Vec::new()))
            .collect();
        for member in members {
            let arm = member.type_name().unwrap_or("object");
            let validators = decorator_validators(
                &member.decorators,
                &format!("type '{type_name}' arm '{arm}'"),
                &mut arm_diagnostics,
            );
            if validators.is_empty() {
                continue;
            }
            match primitive_arms
                .iter_mut()
                .find(|(prim, _)| member.as_type_ref() == Some(prim.as_str()))
            {
                Some((_, arm_validators)) => arm_validators.extend(validators),
                None => arm_diagnostics.error(
                    member.decorators.first().map_or(type_alias.inner.span, |d| d.span),
                    format!(
                        "@endec validators on arm '{arm}' of type '{type_name}' only apply to primitive arms; put them on the variant's fields instead"
                    ),
                ),
            }
        }
        if arm_diagnostics.has_errors() {
            return Err(MacroforgeErrors::new(arm_diagnostics.into_vec()).into());
        }
        let has_arm_validators = primitive_arms
            .iter()
            .any(|(_, validators)| !validators.is_empty());

        // Generic type parameters (like T, U) - these are passed through as-is
        let generic_type_params: Vec<String> = type_refs
            .iter()
            .filter(|t| type_param_set.contains(t.as_str()))
            .cloned()
            .collect();

        // Build EncodableTypeRef with both full type and base type for runtime access.
        // Foreign types (from macroforge.config.ts) are detected here so the union
        // template can use their configured expressions instead of generating
        // broken `{camelCase}DecodeWithContext()` function calls.
        let foreign_types_config = get_foreign_types();
        let encodable_types: Vec<EncodableTypeRef> = type_refs
            .iter()
            .filter(|t| {
                !matches!(
                    TypeCategory::from_ts_type(t),
                    TypeCategory::Primitive | TypeCategory::Date
                ) && !type_param_set.contains(t.as_str())
            })
            .map(|t| {
                let ft_match = TypeCategory::match_foreign_type(t, &foreign_types_config);
                let foreign_decode_callee = ft_match
                    .config
                    .and_then(|ft| ft.handler_callee(ForeignHandler::Decode));
                let foreign_has_shape_callee = ft_match
                    .config
                    .and_then(|ft| ft.handler_callee(ForeignHandler::HasShape));
                // Strip surrounding quotes from string literal types (e.g., "\"Foo\"" -> "Foo")
                // to prevent double-quoting in template string comparisons.
                let clean_type = if (t.starts_with('"') && t.ends_with('"'))
                    || (t.starts_with('\'') && t.ends_with('\''))
                {
                    t[1..t.len() - 1].to_string()
                } else {
                    t.clone()
                };
                EncodableTypeRef {
                    full_type: clean_type,
                    is_foreign: ft_match.config.is_some(),
                    foreign_decode_callee,
                    foreign_has_shape_callee,
                }
            })
            .collect();

        let date_types: Vec<String> = type_refs
            .iter()
            .filter(|t| matches!(TypeCategory::from_ts_type(t), TypeCategory::Date))
            .cloned()
            .collect();

        let has_primitives = !primitive_types.is_empty();
        let has_encodables = !encodable_types.is_empty();
        let has_dates = !date_types.is_empty();
        let has_generic_params = !generic_type_params.is_empty();

        // Separate regular and foreign encodable types for different code generation
        let regular_encodables: Vec<&EncodableTypeRef> =
            encodable_types.iter().filter(|t| !t.is_foreign).collect();
        let foreign_encodables: Vec<&EncodableTypeRef> =
            encodable_types.iter().filter(|t| t.is_foreign).collect();

        let mut object_variants: Vec<ObjectVariant> = Vec::new();
        let mut untagged_object_variants: Vec<UntaggedObjectVariant> = Vec::new();
        let mut intersection_variants: Vec<IntersectionVariant> = Vec::new();

        let mut external_object_variants: Vec<ExternalObjectVariant> = Vec::new();

        for m in members {
            match &m.kind {
                TypeMemberKind::Object { fields } if is_externally_tagged && !fields.is_empty() => {
                    let payload_ts_type = fields[0].ts_type.as_str();
                    let inner_foreign_decode_callee =
                        TypeCategory::match_foreign_type(payload_ts_type, &foreign_types_config)
                            .config
                            .and_then(|ft| ft.handler_callee(ForeignHandler::Decode));
                    let payload_decode_fn = if inner_foreign_decode_callee.is_some() {
                        None
                    } else if let TypeCategory::Encodable(base) =
                        TypeCategory::from_ts_type(payload_ts_type)
                    {
                        Some(ts_ident!(nested_decode_fn_name(&extract_base_type(&base))))
                    } else {
                        None
                    };
                    external_object_variants.push(ExternalObjectVariant {
                        name: fields[0].name.clone(),
                        inner_foreign_decode_callee,
                        payload_decode_fn,
                    });
                }
                TypeMemberKind::Object { fields } => {
                    if let Some(tag_value) = fields.iter().find_map(|f| {
                        if f.name == tag_field {
                            let t = f.ts_type.trim().trim_matches('\'').trim_matches('"');
                            Some(t.to_string())
                        } else {
                            None
                        }
                    }) {
                        object_variants.push(ObjectVariant {
                            tag_value,
                            fields: fields.clone(),
                        });
                    } else {
                        let (optional, required): (Vec<_>, Vec<_>) =
                            fields.iter().partition(|f| f.optional);
                        let required_fields: Vec<String> =
                            required.iter().map(|f| f.name.clone()).collect();
                        let shape_condition = std::iter::once(rendered(ts_template! {
                            typeof value === "object" && value !== null && !Array.isArray(value)
                        }))
                        .chain(required_fields.iter().map(|name| {
                            let key = js_string(name);
                            rendered(ts_template! { @{key} in value })
                        }))
                        .collect::<Vec<_>>()
                        .join(" && ");
                        untagged_object_variants.push(UntaggedObjectVariant {
                            shape_condition,
                            required_fields,
                            optional_fields: optional.iter().map(|f| f.name.clone()).collect(),
                        });
                    }
                }
                TypeMemberKind::Intersection(sub_members) => {
                    // Look for { tag: 'Value' } & TypeRef pattern
                    let mut tag_value = None;
                    let mut ref_type = None;

                    for sub in sub_members {
                        match &sub.kind {
                            TypeMemberKind::Object { fields } => {
                                tag_value = fields.iter().find_map(|f| {
                                    if f.name == tag_field {
                                        let t =
                                            f.ts_type.trim().trim_matches('\'').trim_matches('"');
                                        Some(t.to_string())
                                    } else {
                                        None
                                    }
                                });
                            }
                            TypeMemberKind::TypeRef(t) => {
                                ref_type = Some(t.clone());
                            }
                            _ => {}
                        }
                    }

                    if let (Some(tv), Some(rt)) = (tag_value, ref_type) {
                        // A `{ tag: 'X' } & TypeRef` variant's payload must be decoded
                        // through `TypeRef`'s own decoder so nested fields (dates,
                        // records, links) are reconstructed: a shallow `{ ...value, tag }`
                        // spread leaves them as raw JSON. `encodable_types` only holds
                        // DIRECT type-ref union members, so an intersection's inner type is
                        // absent from it; recognize any non-primitive / non-date /
                        // non-type-param / non-foreign type ref (which has a generated
                        // `*DecodeWithContext`) as encodable here.
                        let is_ser = encodable_types.iter().any(|s| s.full_type == rt)
                            || (!matches!(
                                TypeCategory::from_ts_type(&rt),
                                TypeCategory::Primitive | TypeCategory::Date
                            ) && !type_param_set.contains(rt.as_str())
                                && TypeCategory::match_foreign_type(&rt, &foreign_types_config)
                                    .config
                                    .is_none());
                        intersection_variants.push(IntersectionVariant {
                            tag_value: tv,
                            type_ref: rt,
                            is_encodable: is_ser,
                        });
                    }
                }
                _ => {}
            }
        }
        let has_object_variants = !object_variants.is_empty();
        let has_untagged_object_variants = !untagged_object_variants.is_empty();
        let has_intersection_variants = !intersection_variants.is_empty();
        let has_tagged_variants = has_object_variants
            || has_untagged_object_variants
            || has_intersection_variants
            || !external_object_variants.is_empty();
        let is_literal_only = !literals.is_empty() && type_refs.is_empty() && !has_tagged_variants;
        let is_primitive_only = has_primitives
            && !has_encodables
            && !has_dates
            && !has_generic_params
            && literals.is_empty()
            && !has_tagged_variants;
        let is_encodable_only = !has_primitives
            && !has_dates
            && !has_generic_params
            && has_encodables
            && literals.is_empty()
            && !has_tagged_variants;
        let has_literals = !literals.is_empty();

        // Pre-compute the expected types string for error messages
        let expected_types_str = {
            let mut parts: Vec<String> = Vec::new();
            if has_encodables {
                parts.extend(encodable_types.iter().map(|t| t.full_type.clone()));
            } else if !type_refs.is_empty() {
                parts.extend(type_refs.iter().cloned());
            }
            for ov in &object_variants {
                parts.push(format!("{{ {}: '{}' }}", tag_field, ov.tag_value));
            }
            for ov in &external_object_variants {
                parts.push(format!("{{ {}: any }}", ov.name));
            }
            for iv in &intersection_variants {
                parts.push(format!(
                    "{{ {}: '{}' }} & {}",
                    tag_field, iv.tag_value, iv.type_ref
                ));
            }
            if parts.is_empty() {
                literals.join(", ")
            } else {
                parts.join(", ")
            }
        };

        let primitive_check_condition: String = if primitive_types.is_empty() {
            "false".to_string()
        } else {
            primitive_types
                .iter()
                .map(|prim| {
                    let kind = js_string(prim);
                    rendered(ts_template! { typeof value === @{kind} })
                })
                .collect::<Vec<_>>()
                .join(" || ")
        };

        let encodable_type_check_condition: String = if encodable_types.is_empty() {
            "false".to_string()
        } else {
            encodable_types
                .iter()
                .map(|type_ref| {
                    let name = js_string(&type_ref.full_type);
                    rendered(ts_template! { __typeName === @{name} })
                })
                .collect::<Vec<_>>()
                .join(" || ")
        };
        Ok(Self {
            type_name,
            decode_context_ident: alias.decode_context_ident.clone(),
            decode_context_expr: alias.decode_context_expr.clone(),
            decode_error_expr: alias.decode_error_expr.clone(),
            pending_ref_ident: alias.pending_ref_ident.clone(),
            pending_ref_expr: alias.pending_ref_expr.clone(),
            decode_options_ident: alias.decode_options_ident.clone(),
            generic_decl: &alias.generic_decl,
            generic_args: &alias.generic_args,
            full_type_name,
            full_type_ident: ts_ident!(full_type_name.as_str()),
            type_registry: alias.type_registry,
            caller_file_path: alias.caller_file_path,
            file_imports: alias.file_imports,
            foreign_types_config,
            tag_field: tag_field.to_string(),
            content_field,
            is_externally_tagged,
            is_adjacently_tagged,
            is_untagged,
            literals,
            primitive_types,
            primitive_arms,
            has_arm_validators,
            regular_encodables: regular_encodables.into_iter().cloned().collect(),
            foreign_encodables: foreign_encodables.into_iter().cloned().collect(),
            encodable_types,
            has_primitives,
            has_encodables,
            has_dates,
            has_generic_params,
            object_variants,
            untagged_object_variants,
            intersection_variants,
            external_object_variants,
            has_object_variants,
            has_intersection_variants,
            is_literal_only,
            is_primitive_only,
            is_encodable_only,
            has_literals,
            expected_types_str,
            primitive_check_condition,
            encodable_type_check_condition,
        })
    }
}
