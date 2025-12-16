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

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

/// Convert a PascalCase name to camelCase (for prefix naming style)
fn to_camel_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
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
            let class_name = input.name();
            let field_names: Vec<&str> = class.field_names().collect();
            let has_fields = !field_names.is_empty();

            // Generate function name (always prefix style)
            let fn_name = format!("{}Clone", to_camel_case(class_name));

            // Generate standalone function with value parameter
            let standalone = ts_template! {
                export function @{fn_name}(value: @{class_name}): @{class_name} {
                    const cloned = Object.create(Object.getPrototypeOf(value));

                    {#if has_fields}
                        {#for field in field_names}
                            cloned.@{field} = value.@{field};
                        {/for}
                    {/if}

                    return cloned;
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = body! {
                static clone(value: @{class_name}): @{class_name} {
                    return @{fn_name}(value);
                }
            };

            // Combine standalone function with class body by concatenating sources
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            let combined_source = format!("{}\n{}", standalone.source(), class_body.source());
            let mut combined = TsStream::from_string(combined_source);
            combined.runtime_patches = standalone.runtime_patches;
            combined.runtime_patches.extend(class_body.runtime_patches);

            Ok(combined)
        }
        Data::Enum(_) => {
            // Enums are primitive values, cloning is just returning the value
            let enum_name = input.name();
            let fn_name = format!("{}Clone", to_camel_case(enum_name));
            Ok(ts_template! {
                export function @{fn_name}(value: @{enum_name}): @{enum_name} {
                    return value;
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let field_names: Vec<&str> = interface.field_names().collect();
            let has_fields = !field_names.is_empty();
            let fn_name = format!("{}Clone", to_camel_case(interface_name));
            Ok(ts_template! {
                export function @{fn_name}(value: @{interface_name}): @{interface_name} {
                    return {
                        {#if has_fields}
                            {#for field in field_names}
                                @{field}: value.@{field},
                            {/for}
                        {/if}
                    };
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let fn_name = format!("{}Clone", to_camel_case(type_name));

            if type_alias.is_object() {
                // Object type: spread copy
                let field_names: Vec<&str> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|f| f.name.as_str())
                    .collect();
                let has_fields = !field_names.is_empty();

                Ok(ts_template! {
                    export function @{fn_name}(value: @{type_name}): @{type_name} {
                        return {
                            {#if has_fields}
                                {#for field in field_names}
                                    @{field}: value.@{field},
                                {/for}
                            {/if}
                        };
                    }
                })
            } else {
                // Union, tuple, or simple alias: use spread for objects, or return as-is
                Ok(ts_template! {
                    export function @{fn_name}(value: @{type_name}): @{type_name} {
                        if (typeof value === "object" && value !== null) {
                            return { ...value } as @{type_name};
                        }
                        return value;
                    }
                })
            }
        }
    }
}
