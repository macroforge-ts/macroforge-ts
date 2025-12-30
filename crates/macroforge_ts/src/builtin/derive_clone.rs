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
//! | Class | `classNameClone(value)` + `static clone(value)` | Standalone function + static wrapper method |
//! | Enum | `enumNameClone(value: EnumName): EnumName` | Standalone function (enums are primitives, returns value as-is) |
//! | Interface | `interfaceNameClone(value: InterfaceName): InterfaceName` | Standalone function creating a new object literal |
//! | Type Alias | `typeNameClone(value: TypeName): TypeName` | Standalone function with spread copy for objects |
//!
//!
//! ## Cloning Strategy
//!
//! The generated clone performs a **shallow copy** of all fields:
//!
//! - **Primitives** (`string`, `number`, `boolean`): Copied by value
//! - **Objects**: Reference is copied (not deep cloned)
//! - **Arrays**: Reference is copied (not deep cloned)
//!
//! For deep cloning of nested objects, those objects should also derive `Clone`
//! and the caller should clone them explicitly.
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

use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::Expr;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input, ts_ident};

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
/// # Generated Signatures (default suffix style)
///
/// - Classes: `clone(): ClassName`
/// - Enums: `cloneEnumName(value: EnumName): EnumName`
/// - Interfaces: `cloneInterfaceName(value: InterfaceName): InterfaceName`
/// - Type Aliases: `cloneTypeName(value: TypeName): TypeName`
#[ts_macro_derive(Clone, description = "Generates a clone() method for deep cloning")]
pub fn derive_clone_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = class.inner.name.clone();
            let class_ident = ts_ident!(class_name.clone());

            // Generate identifier for function name (Ident for declaration, Expr for call)
            let fn_name_ident = ts_ident!("{}Clone", class_name.to_case(Case::Camel));
            let fn_name_expr: Expr = fn_name_ident.clone().into();

            // Generate standalone function with value parameter
            let standalone = ts_template! {
                export function @{fn_name_ident}(value: @{class_ident}): @{class_ident} {
                    const cloned = Object.create(Object.getPrototypeOf(value));
                    {#for field in class.field_names().map(|f| ts_ident!(f))}
                        cloned.@{field.clone()} = value.@{field};
                    {/for}
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

            Ok(ts_template! {
                export function @{fn_name_ident}(value: @{interface_ident}): @{interface_ident} {
                    const result = {} as any;
                    {#for field in interface.field_names().map(|f| ts_ident!(f))}
                        result.@{field.clone()} = value.@{field};
                    {/for}
                    return result as @{interface_ident};
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let fn_name_ident = ts_ident!("{}Clone", type_name.to_case(Case::Camel));

            if type_alias.is_object() {
                // Object type: spread copy
                Ok(ts_template! {
                    export function @{fn_name_ident}(value: @{ts_ident!(type_name)}): @{ts_ident!(type_name)} {
                        const result = {} as any;
                        {#for field in type_alias.as_object().unwrap().iter().map(|f| ts_ident!(f.name.as_str()))}
                            result.@{field.clone()} = value.@{field};
                        {/for}
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
