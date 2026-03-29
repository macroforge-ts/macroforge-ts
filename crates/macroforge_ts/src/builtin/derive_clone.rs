//! # Clone Macro Implementation
//!
//! The `Clone` macro generates a `clone()` method for deep copying objects.
//! This is analogous to Rust's `Clone` trait, providing a way to create
//! independent copies of values.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `{className}Clone(value)` + `static clone(value)` | Standalone function + static wrapper method |
//! | Enum | `{enumName}Clone(value): EnumName` | Standalone function (enums are primitives, returns value as-is) |
//! | Interface | `{ifaceName}Clone(value): InterfaceName` | Standalone function creating a new object literal |
//! | Type Alias | `{typeName}Clone(value): TypeName` | Standalone function with spread copy for objects |
//!
//! Names use **camelCase** conversion (e.g., `Point` → `pointClone`).
//!
//!
//! ## Cloning Strategy
//!
//! The generated clone is **type-aware** when a type registry is available:
//!
//! - **Primitives** (`string`, `number`, `boolean`, `bigint`): Copied by value
//! - **`Date`**: Deep cloned via `new Date(x.getTime())`
//! - **Arrays**: Spread copy `[...arr]`, or deep map if element type has `Clone`
//! - **`Map`/`Set`**: New collection, deep copy if value type has `Clone`
//! - **Objects with `@derive(Clone)`**: Deep cloned via their standalone clone function
//! - **Optional fields**: Null-checked — `null`/`undefined` pass through unchanged
//! - **Other objects**: Shallow copy (reference)
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Clone) */
//! class Point {
//!     x: number;
//!     y: number;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class Point {
//!     x: number;
//!     y: number;
//!
//!     static clone(value: Point): Point {
//!         return pointClone(value);
//!     }
//! }
//!
//! export function pointClone(value: Point): Point {
//!     const cloned = Object.create(Object.getPrototypeOf(value));
//!     cloned.x = value.x;
//!     cloned.y = value.y;
//!     return cloned;
//! }
//! ```
//!
//! ## Implementation Notes
//!
//! - **Classes**: Uses `Object.create(Object.getPrototypeOf(value))` to preserve
//!   the prototype chain, ensuring `instanceof` checks work correctly
//! - **Enums**: Simply returns the value (enums are primitives in TypeScript)
//! - **Interfaces/Type Aliases**: Creates new object literals with spread operator
//!   for union/tuple types, or field-by-field copy for object types

use convert_case::{Case, Casing};

use crate::builtin::derive_common::{
    collection_element_type, is_primitive_type, standalone_fn_name, type_has_derive,
};
use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::Expr;
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input, ts_ident};

/// Generate a type-aware clone expression for a single field.
///
/// When the type registry is available and confirms the field's type has `@derive(Clone)`,
/// generates a direct function call (e.g., `userClone(value.field)`).
/// Otherwise falls back to shallow copy.
fn generate_clone_expr(
    field_name: &str,
    ts_type: &str,
    var: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: Option<&TypeRegistry>,
) -> String {
    let access = format!("{var}.{field_name}");

    // If we have resolved type info and a registry, generate optimized clones
    if let (Some(resolved), Some(registry)) = (resolved, registry) {
        // Handle optional types: clone inner if present, else pass through
        if resolved.is_optional {
            let inner_expr =
                generate_clone_for_resolved(field_name, ts_type, var, resolved, registry);
            if inner_expr != access {
                return format!("{access} != null ? {inner_expr} : {access}");
            }
            return access;
        }

        return generate_clone_for_resolved(field_name, ts_type, var, resolved, registry);
    }

    // Fallback: no registry available — shallow copy (backward compatible)
    generate_clone_expr_fallback(field_name, ts_type, var)
}

/// Generate a clone expression using resolved type information.
fn generate_clone_for_resolved(
    field_name: &str,
    ts_type: &str,
    var: &str,
    resolved: &ResolvedTypeRef,
    registry: &TypeRegistry,
) -> String {
    let access = format!("{var}.{field_name}");

    // Direct type with Clone derive → call standalone clone function
    if !resolved.is_collection
        && resolved.registry_key.is_some()
        && type_has_derive(registry, &resolved.base_type_name, "Clone")
    {
        let fn_name = standalone_fn_name(&resolved.base_type_name, "Clone");
        return format!("{fn_name}({access})");
    }

    // Date → deep copy
    if resolved.base_type_name == "Date" && !resolved.is_collection {
        return format!("new Date({access}.getTime())");
    }

    // Collection types
    if resolved.is_collection
        && let Some(elem) = collection_element_type(resolved)
    {
        let base = resolved.base_type_name.as_str();

        match base {
            // Array/User[] — map elements
            _ if base != "Map" && base != "Set" => {
                let elem_clone = element_clone_expr(elem, registry, "v");
                if elem_clone == "v" {
                    // Primitive or unknown element — spread copy
                    return format!("[...{access}]");
                }
                return format!("{access}.map(v => {elem_clone})");
            }
            // Set<T> — always copy, clone elements if they have Clone
            "Set" => {
                let elem_clone = element_clone_expr(elem, registry, "v");
                if elem_clone == "v" {
                    return format!("new Set({access})");
                }
                return format!("new Set(Array.from({access}).map(v => {elem_clone}))");
            }
            // Map<K, V> — clone values
            "Map" => {
                let value_clone = element_clone_expr(elem, registry, "v");
                if value_clone == "v" {
                    return format!("new Map({access})");
                }
                return format!(
                    "new Map(Array.from({access}.entries()).map(([k, v]) => [k, {value_clone}]))"
                );
            }
            _ => {}
        }
    }

    // Fallback for non-registry types
    generate_clone_expr_fallback(field_name, ts_type, var)
}

/// Generate a clone expression for a collection element value.
/// Returns `"v"` if no cloning is needed (primitive/unknown).
fn element_clone_expr(elem: &ResolvedTypeRef, registry: &TypeRegistry, var: &str) -> String {
    // Known Clone type → direct call
    if elem.registry_key.is_some() && type_has_derive(registry, &elem.base_type_name, "Clone") {
        return format!(
            "{}({var})",
            standalone_fn_name(&elem.base_type_name, "Clone")
        );
    }

    // Date → deep copy
    if elem.base_type_name == "Date" {
        return format!("new Date({var}.getTime())");
    }

    // Primitive → identity
    if is_primitive_type(&elem.base_type_name) {
        return var.to_string();
    }

    // Unknown type → identity (shallow)
    var.to_string()
}

/// Fallback clone expression when no type registry is available.
/// Handles known built-in types; everything else is shallow copy.
fn generate_clone_expr_fallback(field_name: &str, ts_type: &str, var: &str) -> String {
    let access = format!("{var}.{field_name}");
    let t = ts_type.trim();

    if is_primitive_type(t) {
        return access;
    }

    if t == "Date" {
        return format!("new Date({access}.getTime())");
    }

    if t.ends_with("[]") || t.starts_with("Array<") {
        return format!("[...{access}]");
    }

    if t.starts_with("Set<") {
        return format!("new Set({access})");
    }

    if t.starts_with("Map<") {
        return format!("new Map({access})");
    }

    // Unknown type → shallow copy
    access
}

/// Generates a `clone()` method for creating copies of objects.
///
/// This macro implementation handles four TypeScript data types:
///
/// - **Classes**: Generates an instance method that creates a new object via
///   `Object.create()` and copies all fields
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
    let type_registry = input.context.type_registry.as_ref();

    match &input.data {
        Data::Class(class) => {
            let class_name = class.inner.name.clone();
            let class_ident = ts_ident!(class_name.clone());

            // Generate identifier for function name (Ident for declaration, Expr for call)
            let fn_name_ident = ts_ident!("{}Clone", class_name.to_case(Case::Camel));
            let fn_name_expr: Expr = fn_name_ident.clone().into();

            // Generate type-aware clone assignments
            let mut clone_body = String::new();
            for field in class.fields() {
                let resolved = resolved_fields.and_then(|rf| rf.get(&field.name));
                let expr = generate_clone_expr(
                    &field.name,
                    &field.ts_type,
                    "value",
                    resolved,
                    type_registry,
                );
                clone_body.push_str(&format!("cloned.{} = {};\n", field.name, expr));
            }

            // Generate standalone function with value parameter
            let standalone = ts_template! {
                export function @{fn_name_ident}(value: @{class_ident}): @{class_ident} {
                    const cloned = Object.create(Object.getPrototypeOf(value));
                    {$typescript TsStream::from_string(clone_body)}
                    return cloned;
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = ts_template!(Within {
                static clone(value: @{class_ident}): @{class_ident} {
                    return @{fn_name_expr}(value);
                }
            });

            if std::env::var("MF_DEBUG_CLONE").is_ok() {
                eprintln!("[MF_DEBUG_CLONE] standalone:\n{}", standalone.source());
                eprintln!("[MF_DEBUG_CLONE] class_body:\n{}", class_body.source());
            }

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
            let interface_ident = ts_ident!(interface.inner.name.clone());
            let fn_name_ident =
                ts_ident!("{}Clone", interface.inner.name.clone().to_case(Case::Camel));

            // Generate type-aware clone assignments
            let mut clone_body = String::new();
            for field in interface.fields() {
                let resolved = resolved_fields.and_then(|rf| rf.get(&field.name));
                let expr = generate_clone_expr(
                    &field.name,
                    &field.ts_type,
                    "value",
                    resolved,
                    type_registry,
                );
                clone_body.push_str(&format!("result.{} = {};\n", field.name, expr));
            }

            Ok(ts_template! {
                export function @{fn_name_ident}(value: @{interface_ident}): @{interface_ident} {
                    const result = {} as any;
                    {$typescript TsStream::from_string(clone_body)}
                    return result as @{interface_ident};
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let fn_name_ident = ts_ident!("{}Clone", type_name.to_case(Case::Camel));

            if type_alias.is_object() {
                // Object type: type-aware clone
                let mut clone_body = String::new();
                for field in type_alias.as_object().unwrap() {
                    let resolved = resolved_fields.and_then(|rf| rf.get(&field.name));
                    let expr = generate_clone_expr(
                        &field.name,
                        &field.ts_type,
                        "value",
                        resolved,
                        type_registry,
                    );
                    clone_body.push_str(&format!("result.{} = {};\n", field.name, expr));
                }

                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{ts_ident!(type_name)}): @{ts_ident!(type_name)} {
                        const result = {} as any;
                        {$typescript TsStream::from_string(clone_body)}
                        return result as @{ts_ident!(type_name)};
                    }
                })
            } else {
                // Union, tuple, or simple alias: use spread for objects, or return as-is
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{ts_ident!(type_name)}): @{ts_ident!(type_name)} {
                        if (typeof value === "object" && value !== null) {
                            return { ...value } as @{ts_ident!(type_name)};
                        }
                        return value;
                    }
                })
            }
        }
    }
}
