use convert_case::{Case, Casing};

use std::collections::HashMap;

use crate::ast::Expr;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::common::{
    field_value_type, fixed_tuple, is_primitive_union, js_string, rendered, structural_call,
};
use crate::macros::{ts_macro_derive, ts_template};
use crate::ts_syn::abi::ir::TypeBody;
use crate::ts_syn::abi::ir::interface::InterfaceFieldIR;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input, ts_ident};

use super::clone_generation::{generate_clone_expr, generate_value_clone, tuple_clone_expr};

/// Generates a `clone()` method for creating copies of objects.
///
/// This macro implementation handles four TypeScript data types:
///
/// - **Classes**: Generates a static `clone(value)` wrapper method plus a standalone
///   `{className}Clone` function that creates a new object via `Object.create()`
///   and copies all fields
/// - **Enums**: Generates a standalone function that returns the value unchanged
/// - **Interfaces**: Generates a standalone function that creates a new object literal
/// - **Type Aliases**: Generates a standalone function with appropriate copying strategy
///
/// # Arguments
///
/// * `input` - The parsed derive input containing the type information
///
/// # Returns
///
/// Returns a `TsStream` containing the generated clone method or function,
/// or a `MacroforgeError` if code generation fails.
///
/// # Generated Signatures
///
/// - Classes: `static clone(value): ClassName` + `{className}Clone(value): ClassName`
/// - Enums: `{enumName}Clone(value): EnumName`
/// - Interfaces: `{ifaceName}Clone(value): InterfaceName`
/// - Type Aliases: `{typeName}Clone(value): TypeName`
#[ts_macro_derive(Clone, description = "Generates a clone() method for deep cloning")]
pub fn derive_clone_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    let resolved_fields = input.context.resolved_fields.as_ref();
    let type_registry = &input.context.type_registry;

    match &input.data {
        Data::Class(class) => {
            let class_name = class.inner.name.clone();
            let class_names = TypeNames::new(&class_name, class.type_params());
            let class_ident = class_names.full_type_ident();

            // Generate identifier for function name (Ident for declaration, Expr for call)
            let fn_name_ident = class_names.generic_function("Clone");
            let fn_name_expr: Expr = ts_ident!(class_names.function("Clone")).into();

            // Generate type-aware clone assignments
            let mut assignments = Vec::new();
            for field in class.fields() {
                let resolved = resolved_fields.and_then(|rf| rf.get(&field.name));
                let expr = generate_clone_expr(
                    &field.name,
                    &field_value_type(&field.ts_type, field.optional),
                    "value",
                    resolved,
                    type_registry,
                );
                assignments.push(ts_template! { cloned.@{&field.name} = @{expr}; });
            }
            let clone_body = TsStream::merge_all(assignments);

            // Generate standalone function with value parameter
            let standalone = ts_template! {
                export function @{fn_name_ident}(value: @{class_ident}): @{class_ident} {
                    const cloned = Object.create(Object.getPrototypeOf(value));
                    {$typescript clone_body}
                    return cloned;
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = ts_template!(Within {
                static @{class_names.generic_method("clone")}(value: @{class_ident}): @{class_ident} {
                    return @{fn_name_expr}(value);
                }
            });

            // Combine standalone function with class body using {$typescript}
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            Ok(standalone.merge(class_body))
        }
        Data::Enum(_) => {
            // Enums are primitive values, cloning is just returning the value
            let enum_name = input.name();
            let fn_name_ident = ts_ident!("{}Clone", enum_name.to_case(Case::Camel));
            Ok(ts_template! {
                export function @{fn_name_ident}(value: @{ts_ident!(enum_name)}): @{ts_ident!(enum_name)} {
                    return value;
                }
            })
        }
        Data::Interface(interface) => {
            let interface_names = TypeNames::new(input.name(), interface.type_params());
            let interface_ident = interface_names.full_type_ident();
            let fn_name_ident = interface_names.generic_function("Clone");

            let literal = object_clone_literal(interface.fields(), resolved_fields, type_registry);
            let clone = parse_clone_expr(&literal, &input)?;
            Ok(ts_template! {
                export function @{fn_name_ident}(value: @{interface_ident}): @{interface_ident} {
                    return @{clone};
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_names = TypeNames::new(input.name(), type_alias.type_params());
            let fn_name_ident = type_names.generic_function("Clone");

            let effective_fields =
                crate::builtin::derive::common::get_effective_fields(type_alias, type_registry);
            if let Some(ref effective_fields) = effective_fields {
                // Object-like type (Object or flattened Intersection): type-aware clone
                let literal =
                    object_clone_literal(effective_fields, resolved_fields, type_registry);
                let clone = parse_clone_expr(&literal, &input)?;
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{type_names.full_type_ident()}): @{type_names.full_type_ident()} {
                        return @{clone};
                    }
                })
            } else {
                let clone_src = alias_clone_expr(type_alias.body(), type_registry);
                let clone = parse_clone_expr(&clone_src, &input)?;
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{type_names.full_type_ident()}): @{type_names.full_type_ident()} {
                        return @{clone};
                    }
                })
            }
        }
    }
}

/// An object literal copying `value`'s fields, each cloned by its type. An
/// optional field absent from `value` stays absent, so the copy has the same
/// keys as its source.
fn object_clone_literal(
    fields: &[InterfaceFieldIR],
    resolved_fields: Option<&HashMap<String, ResolvedTypeRef>>,
    registry: &TypeRegistry,
) -> String {
    let entries: Vec<String> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let resolved = resolved_fields.and_then(|fields| fields.get(name));
            let ts_type = field_value_type(&field.ts_type, field.optional);
            let clone = generate_clone_expr(name, &ts_type, "value", resolved, registry);
            if field.optional {
                let key = js_string(name);
                rendered(ts_template! { ...(@{key} in value ? { @{name}: @{clone} } : {}) })
            } else {
                rendered(ts_template! { @{name}: @{clone} })
            }
        })
        .collect();
    let entries = entries.join(", ");
    rendered(ts_template! { { @{entries} } })
}

/// A clone of `value`, a value of a non-object alias with `body`.
fn alias_clone_expr(body: &TypeBody, registry: &TypeRegistry) -> String {
    // Primitives, branded or not, are immutable.
    if body.primitive_base().is_some() || is_primitive_union(body) {
        return "value".to_string();
    }
    if let Some(elements) = body.as_tuple().filter(|elements| fixed_tuple(elements)) {
        return tuple_clone_expr(elements, registry);
    }
    match body.as_alias() {
        Some(ts_type) => generate_value_clone(ts_type, "value", None, registry),
        None => structural_call("structuralClone", "value"),
    }
}

fn parse_clone_expr(source: &str, input: &DeriveInput) -> Result<Expr, MacroforgeError> {
    Expr::parse(source).map_err(|err| {
        MacroforgeError::new(
            input.decorator_span(),
            format!("@derive(Clone): invalid clone expression: {err:?}"),
        )
    })
}
