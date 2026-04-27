use convert_case::{Case, Casing};

use crate::builtin::derive_common::{
    DefaultFieldOptions, flatten_intersection_fields, get_type_default_with_registry,
    has_known_default,
};
use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::{Expr, Ident};
use crate::ts_syn::abi::ir::{TypeBody, TypeDefinitionIR, TypeMemberKind, TypeRegistry};
use crate::ts_syn::ts_ident;
use crate::ts_syn::{
    Data, DeriveInput, MacroforgeError, TsStream, emit_expr, parse_ts_expr, parse_ts_macro_input,
};

use super::types::{DefaultField, validate_default_fields};

/// Resolve the default expression for a field, wrapping a user-provided
/// string into `{ [tag]: "value" }` when the field type is a registered
/// internally-tagged union (e.g. `@default("Fixed")` on a field typed
/// `PricingMode = { variant: "Fixed" } | ({ variant: "PerWeight" } & ...)`).
/// Without the wrap the bare string fails type-checking.
fn resolve_default_value(
    opts_value: Option<String>,
    ts_type: &str,
    type_registry: Option<&TypeRegistry>,
) -> String {
    if let Some(v) = opts_value {
        if let Some(wrapped) = wrap_string_for_tagged_union(&v, ts_type, type_registry) {
            return wrapped;
        }
        return v;
    }
    get_type_default_with_registry(ts_type, type_registry)
}

/// Returns `Some(wrapped)` when `value` is a string literal and `ts_type`
/// is a type alias whose body is an internally-tagged union containing a
/// variant with that discriminant value. The wrap takes the form
/// `({ [tag]: "value" })` for unit variants, or
/// `({ [tag]: "value", ...payloadDefaultValue() })` for variants whose
/// member is `{ [tag]: "value" } & PayloadType`.
fn wrap_string_for_tagged_union(
    value: &str,
    ts_type: &str,
    registry: Option<&TypeRegistry>,
) -> Option<String> {
    let trimmed = value.trim();
    let is_string_literal = (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''));
    if !is_string_literal || trimmed.len() < 2 {
        return None;
    }
    let variant_name = &trimmed[1..trimmed.len() - 1];

    let registry = registry?;
    let alias_name = ts_type.trim().split('<').next()?.trim();
    let entry = registry.get(alias_name)?;
    let alias = match &entry.definition {
        TypeDefinitionIR::TypeAlias(a) => a,
        _ => return None,
    };
    let members = match &alias.body {
        TypeBody::Union(m) => m,
        _ => return None,
    };

    let tag = alias
        .decorators
        .iter()
        .find_map(|d| (d.name == "serde").then(|| extract_tag_from_serde_args(&d.args_src)))
        .flatten()?;

    for member in members {
        let (tag_fields, payload_type): (
            Option<&[crate::ts_syn::InterfaceFieldIR]>,
            Option<String>,
        ) = match &member.kind {
            TypeMemberKind::Object { fields } => (Some(fields.as_slice()), None),
            TypeMemberKind::Intersection(parts) => {
                let mut tf: Option<&[crate::ts_syn::InterfaceFieldIR]> = None;
                let mut pt: Option<String> = None;
                for p in parts {
                    if let Some(fields) = p.as_object() {
                        tf = Some(fields);
                    }
                    if let Some(tr) = p.as_type_ref() {
                        pt = Some(tr.trim().to_string());
                    }
                }
                (tf, pt)
            }
            _ => continue,
        };
        let fields = tag_fields?;
        let tag_field = fields.iter().find(|f| f.name == tag)?;
        let lit = tag_field
            .ts_type
            .trim()
            .trim_matches('"')
            .trim_matches('\'');
        if lit == variant_name {
            return Some(if let Some(p) = payload_type {
                let camel = p
                    .split('<')
                    .next()
                    .unwrap_or(&p)
                    .trim()
                    .to_case(Case::Camel);
                format!(
                    "({{ \"{}\": \"{}\", ...{}DefaultValue() }})",
                    tag, variant_name, camel
                )
            } else {
                format!("({{ \"{}\": \"{}\" }})", tag, variant_name)
            });
        }
    }
    None
}

/// True when `s` contains a `|` at top level (depth 0 with respect to
/// matching parens / brackets / braces / angle brackets). Used to tell a
/// parenthesized union (`(string | T)`) — which we reject — from a
/// parenthesized intersection (`({ tag } & T)`) — which is fine.
fn contains_top_level_pipe(s: &str) -> bool {
    let mut depth: i32 = 0;
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth -= 1,
            '|' if depth <= 1 => return true,
            _ => {}
        }
    }
    false
}

/// Extract `"value"` out of `tag: "value"` or `tag = "value"` inside the
/// raw arguments of `@serde(...)`. Returns the unquoted variant tag name.
fn extract_tag_from_serde_args(args: &str) -> Option<String> {
    let s = args.trim();
    let idx = s.find("tag")?;
    let after = &s[idx + 3..];
    let after = after.trim_start();
    let after = after
        .strip_prefix(':')
        .or_else(|| after.strip_prefix('='))?
        .trim_start();
    let quote = after.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let rest = &after[1..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

#[ts_macro_derive(
    Default,
    description = "Generates a static defaultValue() factory method",
    attributes(default)
)]
pub fn derive_default_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let type_registry = input.context.type_registry.as_ref();

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let class_ident = ts_ident!(class_name);
            let class_expr: Expr = class_ident.clone().into();

            // Validate fields against type registry (emit warnings for types missing Default)
            let fields_to_validate: Vec<(String, String)> = class
                .fields()
                .iter()
                .filter(|f| {
                    !f.optional && !DefaultFieldOptions::from_decorators(&f.decorators).has_default
                })
                .map(|f| (f.name.clone(), f.ts_type.clone()))
                .collect();
            validate_default_fields(&fields_to_validate, class_name, type_registry);

            // Check for required non-primitive fields missing @default (like Rust's derive(Default))
            let missing_defaults: Vec<&str> = class
                .fields()
                .iter()
                .filter(|field| {
                    // Skip optional fields
                    if field.optional {
                        return false;
                    }
                    // Skip if has explicit @default
                    if DefaultFieldOptions::from_decorators(&field.decorators).has_default {
                        return false;
                    }
                    // Skip if type has known default (primitives, collections, nullable)
                    if has_known_default(&field.ts_type) {
                        return false;
                    }
                    // This field needs @default but doesn't have it
                    true
                })
                .map(|f| f.name.as_str())
                .collect();

            if !missing_defaults.is_empty() {
                return Err(MacroforgeError::new(
                    input.decorator_span(),
                    format!(
                        "@derive(Default) cannot determine default for non-primitive fields. Add @default(value) to: {}",
                        missing_defaults.join(", ")
                    ),
                ));
            }

            // Build defaults for ALL non-optional fields by parsing expressions and generating class body
            // Parse all field default expressions upfront for validation (before template generation)
            let field_data: Vec<(Ident, Expr)> = class
                .fields()
                .iter()
                .filter(|field| !field.optional)
                .map(|field| {
                    let opts = DefaultFieldOptions::from_decorators(&field.decorators);
                    let default_value =
                        resolve_default_value(opts.value, &field.ts_type, type_registry);

                    let value_expr = parse_ts_expr(&default_value).map_err(|err| {
                        MacroforgeError::new(
                            input.decorator_span(),
                            format!(
                                "@derive(Default): invalid default expression for '{}': {err:?}",
                                field.name
                            ),
                        )
                    })?;
                    Ok((ts_ident!(field.name.as_str()), *value_expr))
                })
                .collect::<Result<_, MacroforgeError>>()?;

            // Generate the method body using parsed field data
            // Note: field_data is consumed by the body! macro below
            let _ = &field_data; // Explicitly mark as used to satisfy clippy
            let class_body = ts_template!(Within {
                static defaultValue(): @{class_ident.clone()} {
                    const instance = new @{class_expr.clone()}();
                    {#for (name_ident, value_expr) in field_data}
                        instance.@{name_ident} = @{value_expr};
                    {/for}
                    return instance;
                }
            });

            // Also generate standalone function for consistency
            // Using {$typescript} to compose TsStream objects
            let fn_name_ident = ts_ident!("{}DefaultValue", class_name.to_case(Case::Camel));
            Ok(ts_template! {
                {$typescript class_body}

                export function @{fn_name_ident}(): @{class_ident.clone()} {
                    return @{class_expr.clone()}.defaultValue();
                }
            })
        }
        Data::Enum(enum_data) => {
            let enum_name = input.name();
            let enum_ident = ts_ident!(enum_name);

            // Find variant with @default attribute (like Rust's #[default] on enums)
            let default_variant = enum_data.variants().iter().find(|v| {
                v.decorators
                    .iter()
                    .any(|d| d.name.eq_ignore_ascii_case("default"))
            });

            match default_variant {
                Some(variant) => {
                    let variant_name = &variant.name;
                    let fn_name_ident = ts_ident!("{}DefaultValue", enum_name.to_case(Case::Camel));
                    let enum_expr: Expr = ts_ident!(enum_name).into();
                    let variant_ident = ts_ident!(variant_name.as_str());
                    Ok(ts_template! {
                        export function @{fn_name_ident}(): @{enum_ident} {
                            return @{enum_expr}.@{variant_ident};
                        }
                    })
                }
                None => Err(MacroforgeError::new(
                    input.decorator_span(),
                    format!(
                        "@derive(Default) on enum requires exactly one variant with @default attribute. \
                        Add @default to one variant of {}",
                        enum_name
                    ),
                )),
            }
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let interface_ident = ts_ident!(interface_name);

            // Validate fields against type registry (emit warnings for types missing Default)
            let fields_to_validate: Vec<(String, String)> = interface
                .fields()
                .iter()
                .filter(|f| {
                    !f.optional && !DefaultFieldOptions::from_decorators(&f.decorators).has_default
                })
                .map(|f| (f.name.clone(), f.ts_type.clone()))
                .collect();
            validate_default_fields(&fields_to_validate, interface_name, type_registry);

            // Check for required non-primitive fields missing @default (like Rust's derive(Default))
            let missing_defaults: Vec<&str> = interface
                .fields()
                .iter()
                .filter(|field| {
                    // Skip optional fields
                    if field.optional {
                        return false;
                    }
                    // Skip if has explicit @default
                    if DefaultFieldOptions::from_decorators(&field.decorators).has_default {
                        return false;
                    }
                    // Skip if type has known default (primitives, collections, nullable)
                    if has_known_default(&field.ts_type) {
                        return false;
                    }
                    // This field needs @default but doesn't have it
                    true
                })
                .map(|f| f.name.as_str())
                .collect();

            if !missing_defaults.is_empty() {
                return Err(MacroforgeError::new(
                    input.decorator_span(),
                    format!(
                        "@derive(Default) cannot determine default for non-primitive fields. Add @default(value) to: {}",
                        missing_defaults.join(", ")
                    ),
                ));
            }

            // Build defaults for ALL non-optional fields
            let default_fields: Vec<DefaultField> = interface
                .fields()
                .iter()
                .filter(|field| !field.optional)
                .map(|field| {
                    let opts = DefaultFieldOptions::from_decorators(&field.decorators);
                    DefaultField {
                        name: field.name.clone(),
                        value: resolve_default_value(opts.value, &field.ts_type, type_registry),
                    }
                })
                .collect();

            let has_defaults = !default_fields.is_empty();

            let fn_name_ident = ts_ident!("{}DefaultValue", interface_name.to_case(Case::Camel));

            if has_defaults {
                let object_fields: Vec<(Ident, Expr)> = default_fields
                    .iter()
                    .map(|f| {
                        let value_expr = parse_ts_expr(&f.value).map_err(|err| {
                            MacroforgeError::new(
                                input.decorator_span(),
                                format!(
                                    "@derive(Default): invalid default expression for '{}': {err:?}",
                                    f.name
                                ),
                            )
                        })?;
                        Ok((ts_ident!(f.name.as_str()), *value_expr))
                    })
                    .collect::<Result<_, MacroforgeError>>()?;

                let mut props = String::new();
                for (name_ident, value_expr) in &object_fields {
                    let name: &str = name_ident.sym.as_ref();
                    let value = emit_expr(value_expr);
                    props.push_str(&format!("{name}: {value},\n"));
                }

                let return_stmt = format!("return {{\n{props}}};");
                let return_stmt_stream = TsStream::from_string(return_stmt);

                Ok(ts_template! {
                    export function @{fn_name_ident}(): @{interface_ident.clone()} {
                        {$typescript return_stmt_stream}
                    }
                })
            } else {
                let return_stmt = "return {};".to_string();
                let return_stmt_stream = TsStream::from_string(return_stmt);

                Ok(ts_template! {
                    export function @{fn_name_ident}(): @{interface_ident.clone()} {
                        {$typescript return_stmt_stream}
                    }
                })
            }
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            // Build generic type signature if type has type params
            let type_params = type_alias.type_params();
            let (generic_decl, generic_args) = if type_params.is_empty() {
                (String::new(), String::new())
            } else {
                let params = type_params.join(", ");
                (format!("<{}>", params), format!("<{}>", params))
            };
            let full_type_name = format!("{}{}", type_name, generic_args);
            let full_type_ident = ts_ident!(full_type_name.as_str());
            let generic_decl_ident = ts_ident!(generic_decl.as_str());

            let effective_fields =
                crate::builtin::derive_common::get_effective_fields(type_alias, type_registry);
            if let Some(ref fields) = effective_fields {
                let fields = fields.as_slice();

                // Validate fields against type registry (emit warnings for types missing Default)
                let fields_to_validate: Vec<(String, String)> = fields
                    .iter()
                    .filter(|f| {
                        !f.optional
                            && !DefaultFieldOptions::from_decorators(&f.decorators).has_default
                    })
                    .map(|f| (f.name.clone(), f.ts_type.clone()))
                    .collect();
                validate_default_fields(&fields_to_validate, type_name, type_registry);

                // Check for required non-primitive fields missing @default (like Rust's derive(Default))
                let missing_defaults: Vec<&str> = fields
                    .iter()
                    .filter(|field| {
                        // Skip optional fields
                        if field.optional {
                            return false;
                        }
                        // Skip if has explicit @default
                        if DefaultFieldOptions::from_decorators(&field.decorators).has_default {
                            return false;
                        }
                        // Skip if type has known default (primitives, collections, nullable)
                        if has_known_default(&field.ts_type) {
                            return false;
                        }
                        // This field needs @default but doesn't have it
                        true
                    })
                    .map(|f| f.name.as_str())
                    .collect();

                if !missing_defaults.is_empty() {
                    return Err(MacroforgeError::new(
                        input.decorator_span(),
                        format!(
                            "@derive(Default) cannot determine default for non-primitive fields. Add @default(value) to: {}",
                            missing_defaults.join(", ")
                        ),
                    ));
                }

                // Build defaults for ALL non-optional fields
                let default_fields: Vec<DefaultField> = fields
                    .iter()
                    .filter(|field| !field.optional)
                    .map(|field| {
                        let opts = DefaultFieldOptions::from_decorators(&field.decorators);
                        DefaultField {
                            name: field.name.clone(),
                            value: resolve_default_value(opts.value, &field.ts_type, type_registry),
                        }
                    })
                    .collect();

                let has_defaults = !default_fields.is_empty();

                let fn_name_ident = ts_ident!("{}DefaultValue", type_name.to_case(Case::Camel));

                if has_defaults {
                    let object_fields: Vec<(Ident, Expr)> = default_fields
                        .iter()
                        .map(|f| {
                            let value_expr = parse_ts_expr(&f.value).map_err(|err| {
                                MacroforgeError::new(
                                    input.decorator_span(),
                                    format!(
                                        "@derive(Default): invalid default expression for '{}': {err:?}",
                                        f.name
                                    ),
                                )
                            })?;
                            Ok((ts_ident!(f.name.as_str()), *value_expr))
                        })
                        .collect::<Result<_, MacroforgeError>>()?;

                    let mut props = String::new();
                    for (name_ident, value_expr) in &object_fields {
                        let name: &str = name_ident.sym.as_ref();
                        let value = emit_expr(value_expr);
                        props.push_str(&format!("{name}: {value},\n"));
                    }

                    let return_stmt = format!("return {{\n{props}}};");
                    let return_stmt_stream = TsStream::from_string(return_stmt);

                    Ok(ts_template! {
                        export function @{fn_name_ident}@{generic_decl_ident}(): @{full_type_ident.clone()} {
                            {$typescript return_stmt_stream}
                        }
                    })
                } else {
                    let return_stmt = "return {};".to_string();
                    let return_stmt_stream = TsStream::from_string(return_stmt);

                    Ok(ts_template! {
                        export function @{fn_name_ident}@{generic_decl_ident}(): @{full_type_ident.clone()} {
                            {$typescript return_stmt_stream}
                        }
                    })
                }
            } else if type_alias.is_union() {
                // Union type: check for @default on a variant OR @default(...) on the type
                let members = type_alias.as_union().unwrap();

                // Helper: build an object literal default from an inline object variant's fields
                fn build_object_default(
                    fields: &[crate::ts_syn::InterfaceFieldIR],
                    registry: Option<&crate::ts_syn::abi::ir::TypeRegistry>,
                ) -> String {
                    let props: Vec<String> = fields
                        .iter()
                        .map(|f| {
                            let opts = DefaultFieldOptions::from_decorators(&f.decorators);
                            let value = resolve_default_value(opts.value, &f.ts_type, registry);
                            format!("{}: {}", f.name, value)
                        })
                        .collect();
                    format!("({{ {} }})", props.join(", "))
                }

                // Helper: build an intersection variant default by spreading
                // each TypeRef's `{type}DefaultValue()` and inlining the object
                // part's fields. Avoids flattening foreign-typed fields like
                // `BigDecimal.BigDecimal` whose per-field `get_type_default`
                // can't resolve them in this codepath and emits bogus camelCase
                // calls (`bigDecimal.bigDecimalDefaultValue()`).
                fn build_intersection_default(
                    intersection_members: &[crate::ts_syn::TypeMember],
                    registry: Option<&crate::ts_syn::abi::ir::TypeRegistry>,
                ) -> Option<String> {
                    use convert_case::{Case, Casing};
                    let mut parts: Vec<String> = Vec::new();
                    for member in intersection_members {
                        if let Some(type_name) = member.as_type_ref() {
                            let camel = type_name.trim().to_case(Case::Camel);
                            parts.push(format!("...{}DefaultValue()", camel));
                        } else if let Some(fields) = member.as_object() {
                            for f in fields {
                                let opts = DefaultFieldOptions::from_decorators(&f.decorators);
                                let value = resolve_default_value(opts.value, &f.ts_type, registry);
                                parts.push(format!("{}: {}", f.name, value));
                            }
                        }
                        // Literals and nested intersections fall through —
                        // we only handle the common `{tag} & TypeRef` shape.
                    }
                    if parts.is_empty() {
                        None
                    } else {
                        Some(format!("({{ {} }})", parts.join(", ")))
                    }
                }

                // Check for parenthesized union members - can't place @default inside parens
                // e.g., `(string | Product) | (string | Service)` is not allowed.
                // Parenthesized intersections like `({ kind: 'A' } & ADetail)` are
                // fine — they preserve doc-comment placement unambiguously and are
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
                        // Named type (TypeRef or Literal) — use the type name
                        if let Some(name) = member.type_name() {
                            return Some(name.to_string());
                        }
                        // Object type (tagged union variant) — build an object literal
                        // with default values for each field
                        if let Some(fields) = member.as_object() {
                            return Some(build_object_default(fields, type_registry));
                        }
                        // Intersection type (tagged union with struct payload).
                        // Prefer spreading each TypeRef's `xxxDefaultValue()`
                        // so foreign-typed fields like `BigDecimal.BigDecimal`
                        // get their proper default expression instead of a
                        // bogus camelCase fallback.
                        if let Some(intersection_members) = member.as_intersection_members() {
                            if let Some(literal) =
                                build_intersection_default(intersection_members, type_registry)
                            {
                                return Some(literal);
                            }
                            // Fallback: full flattening with registry
                            if let Some(fields) =
                                flatten_intersection_fields(intersection_members, type_registry)
                            {
                                return Some(build_object_default(&fields, type_registry));
                            }
                            // Last resort: inline object fields only
                            let inline_fields: Vec<_> = intersection_members
                                .iter()
                                .filter_map(|m| m.as_object())
                                .flat_map(|fields| fields.iter().cloned())
                                .collect();
                            if !inline_fields.is_empty() {
                                return Some(build_object_default(&inline_fields, type_registry));
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
                        members.first().and_then(|m| {
                            if let Some(fields) = m.as_object() {
                                Some(build_object_default(fields, type_registry))
                            } else if let Some(intersection_members) = m.as_intersection_members() {
                                build_intersection_default(intersection_members, type_registry)
                                    .or_else(|| {
                                        flatten_intersection_fields(
                                            intersection_members,
                                            type_registry,
                                        )
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
                                        .map(|fields| build_object_default(&fields, type_registry))
                                    })
                            } else {
                                None
                            }
                        })
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
                    default_opts.value
                });

                if let Some(variant) = default_variant {
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
                    // Determine the default expression based on variant type
                    // Use as-is if it's already an expression, a literal, or a primitive value
                    let is_expression = variant.contains('.') || variant.contains('(');
                    let is_string_literal = variant.starts_with('"')
                        || variant.starts_with('\'')
                        || variant.starts_with('`');
                    let is_primitive_value = variant.parse::<f64>().is_ok()
                        || variant == "true"
                        || variant == "false"
                        || variant == "null";

                    let default_expr = if is_expression || is_string_literal || is_primitive_value {
                        variant // Use as-is
                    } else {
                        // Resolves generic aliases via the type registry, then
                        // falls back to primitives / named-type defaults.
                        get_type_default_with_registry(&variant, type_registry)
                    };

                    // Handle generic type aliases (e.g., type RecordLink<T> = ...)
                    let type_params = type_alias.type_params();
                    let has_generics = !type_params.is_empty();
                    let generic_params = if has_generics {
                        format!("<{}>", type_params.join(", "))
                    } else {
                        String::new()
                    };
                    let return_type = if has_generics {
                        format!("{}<{}>", type_name, type_params.join(", "))
                    } else {
                        type_name.to_string()
                    };
                    let return_type_ident = ts_ident!(return_type.as_str());
                    let generic_params_ident = ts_ident!(generic_params.as_str());

                    let fn_name_ident = ts_ident!("{}DefaultValue", type_name.to_case(Case::Camel));
                    let return_expr = parse_ts_expr(&default_expr).map_err(|err| {
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
            } else {
                // Tuple or simple alias: check for explicit @default(value)
                let default_opts = DefaultFieldOptions::from_decorators(
                    &input
                        .attrs
                        .iter()
                        .map(|a| a.inner.clone())
                        .collect::<Vec<_>>(),
                );

                if let Some(default_variant) = default_opts.value {
                    let fn_name_ident = ts_ident!("{}DefaultValue", type_name.to_case(Case::Camel));
                    let return_expr = parse_ts_expr(&default_variant).map_err(|err| {
                        MacroforgeError::new(
                            input.decorator_span(),
                            format!(
                                "@derive(Default): invalid default expression for '{}': {err:?}",
                                type_name
                            ),
                        )
                    })?;
                    Ok(ts_template! {
                        export function @{fn_name_ident}@{generic_decl_ident}(): @{full_type_ident.clone()} {
                            return @{return_expr};
                        }
                    })
                } else {
                    Err(MacroforgeError::new(
                        input.decorator_span(),
                        format!(
                            "@derive(Default) on type '{}' requires @default(value) to specify the default.",
                            type_name
                        ),
                    ))
                }
            }
        }
    }
}
