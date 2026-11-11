use crate::builtin::derive::common::function_prefix;

use std::collections::HashMap;

use crate::ast::Expr;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::common::parameter_name;
use crate::builtin::derive::common::{
    ClassUse, derive_class_fields, derive_interface_fields, inherited_statics,
};
use crate::builtin::derive::common::{
    field_value_type, fixed_tuple, is_primitive_union, js_string, property_key, rendered,
    structural_call, structural_helper,
};
use crate::macros::{ts_macro_derive, ts_template};
use crate::ts_syn::abi::ir::TypeBody;
use crate::ts_syn::abi::ir::class::FieldIR;
use crate::ts_syn::abi::ir::interface::InterfaceFieldIR;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input, ts_ident};

use super::clone_generation::{generate_clone_expr, generate_value_clone, tuple_clone_expr};

/// Generates a `clone()` method for creating copies of objects.
///
/// This macro implementation handles four TypeScript data types:
///
/// - **Classes**: Generates a static `clone(value)` method that copies all fields
///   onto a new object with the value's prototype, without running the
///   constructor, plus a standalone `{className}Clone` function that runs it
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
            let class_fields = derive_class_fields(&input, class, "Clone", ClassUse::Builds)?;
            let class_name = class.inner.name.clone();
            let class_names = TypeNames::new(&class_name, class.type_params())
                .inheriting(inherited_statics(&input, class, "Clone"));
            let class_ident = class_names.full_type_ident();

            let literal = object_clone_literal(
                class_fields.iter().map(CloneField::from),
                resolved_fields,
                type_registry,
            )
            .map_err(|message| refused(&input, message))?;
            let fields = parse_clone_expr(&literal, &input)?;
            let cloned_with: Expr = ts_ident!(structural_helper("clonedWith")).into();
            let body = ts_template! {
                return @{cloned_with}(value, () => @{fields});
            };
            let value = rendered(ts_template! { value: @{&class_ident} });
            Ok(class_names.class_function(
                "clone",
                "Clone",
                &value,
                "value",
                &class_names.full_type,
                body,
            ))
        }
        Data::Enum(_) => {
            // Enums are primitive values, cloning is just returning the value
            let enum_name = input.name();
            let fn_name_ident = ts_ident!("{}Clone", function_prefix(enum_name));
            Ok(ts_template! {
                export function @{fn_name_ident}(value: @{ts_ident!(enum_name)}): @{ts_ident!(enum_name)} {
                    return value;
                }
            })
        }
        Data::Interface(interface) => {
            let interface_fields = derive_interface_fields(&input, interface, "Clone")?;
            let interface_names = TypeNames::new(input.name(), interface.type_params());
            let interface_ident = interface_names.full_type_ident();
            let fn_name_ident = interface_names.generic_function("Clone");

            let literal = object_clone_literal(
                interface_fields.iter().map(CloneField::from),
                resolved_fields,
                type_registry,
            )
            .map_err(|message| refused(&input, message))?;
            let clone = parse_clone_expr(&literal, &input)?;
            let cloned_with: Expr = ts_ident!(structural_helper("clonedWith")).into();
            Ok(ts_template! {
                export function @{fn_name_ident}(value: @{&interface_ident}): @{&interface_ident} {
                    return @{cloned_with}(value, (): @{&interface_ident} => @{clone});
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_names = TypeNames::new(input.name(), type_alias.type_params());
            let fn_name_ident = type_names.generic_function("Clone");

            let effective_fields =
                crate::builtin::derive::common::derive_alias_fields(&input, type_alias);
            if let Some(ref effective_fields) = effective_fields {
                // Object-like type (Object or flattened Intersection): type-aware clone
                let literal = object_clone_literal(
                    effective_fields.iter().map(CloneField::from),
                    resolved_fields,
                    type_registry,
                )
                .map_err(|message| refused(&input, message))?;
                let clone = parse_clone_expr(&literal, &input)?;
                let cloned_with: Expr = ts_ident!(structural_helper("clonedWith")).into();
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{type_names.full_type_ident()}): @{type_names.full_type_ident()} {
                        return @{cloned_with}(value, (): @{type_names.full_type_ident()} => @{clone});
                    }
                })
            } else {
                let clone_src = alias_clone_expr(type_alias.body(), type_registry)
                    .map_err(|message| refused(&input, message))?;
                let clone = parse_clone_expr(&clone_src, &input)?;
                let parameter = ts_ident!(parameter_name("value", &clone_src));
                Ok(ts_template! {
                    export function @{fn_name_ident}(@{parameter}: @{type_names.full_type_ident()}): @{type_names.full_type_ident()} {
                        return @{clone};
                    }
                })
            }
        }
    }
}

/// A field to copy, of a class or of an object shape.
struct CloneField<'a> {
    name: &'a str,
    ts_type: &'a str,
    optional: bool,
}

impl<'a> From<&'a FieldIR> for CloneField<'a> {
    fn from(field: &'a FieldIR) -> Self {
        Self {
            name: &field.name,
            ts_type: &field.ts_type,
            optional: field.optional,
        }
    }
}

impl<'a> From<&'a InterfaceFieldIR> for CloneField<'a> {
    fn from(field: &'a InterfaceFieldIR) -> Self {
        Self {
            name: &field.name,
            ts_type: &field.ts_type,
            optional: field.optional,
        }
    }
}

/// An object literal copying `value`'s fields, each cloned by its type. An
/// optional field absent from `value` stays absent, so the copy has the same
/// keys as its source.
fn object_clone_literal<'a>(
    fields: impl Iterator<Item = CloneField<'a>>,
    resolved_fields: Option<&HashMap<String, ResolvedTypeRef>>,
    registry: &TypeRegistry,
) -> Result<String, String> {
    let entries: Vec<String> = fields
        .map(|field| {
            let name = field.name;
            let resolved = resolved_fields.and_then(|fields| fields.get(name));
            let ts_type = field_value_type(field.ts_type, field.optional);
            let clone = generate_clone_expr(name, &ts_type, "value", resolved, registry)?;
            let property = property_key(name);
            Ok(if field.optional {
                let key = js_string(name);
                rendered(ts_template! { ...(@{key} in value ? { @{property}: @{clone} } : {}) })
            } else {
                rendered(ts_template! { @{property}: @{clone} })
            })
        })
        .collect::<Result<_, String>>()?;
    let entries = entries.join(", ");
    Ok(rendered(ts_template! { { @{entries} } }))
}

/// A clone of `value`, a value of a non-object alias with `body`.
fn alias_clone_expr(body: &TypeBody, registry: &TypeRegistry) -> Result<String, String> {
    // Primitives, branded or not, are immutable.
    if body.primitive_base().is_some() || is_primitive_union(body) {
        return Ok("value".to_string());
    }
    if let Some(elements) = body.as_tuple().filter(|elements| fixed_tuple(elements)) {
        return tuple_clone_expr(elements, registry);
    }
    match body.as_alias() {
        Some(ts_type) => generate_value_clone(ts_type, "value", None, registry),
        None => Ok(structural_call("structuralClone", "value")),
    }
}

/// The error `@derive(Clone)` on `input` gives for what it cannot copy.
fn refused(input: &DeriveInput, message: String) -> MacroforgeError {
    MacroforgeError::new(input.decorator_span(), format!("@derive(Clone): {message}"))
}

fn parse_clone_expr(source: &str, input: &DeriveInput) -> Result<Expr, MacroforgeError> {
    Expr::parse(source).map_err(|err| {
        MacroforgeError::new(
            input.decorator_span(),
            format!("@derive(Clone): invalid clone expression: {err}"),
        )
    })
}
