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
//! | Class | `clone(): ClassName` | Instance method creating a new instance with copied fields |
//! | Enum | `cloneEnumName(value: EnumName): EnumName` | Standalone function (enums are primitives, returns value as-is) |
//! | Interface | `cloneInterfaceName(value: InterfaceName): InterfaceName` | Standalone function creating a new object literal |
//! | Type Alias | `cloneTypeName(value: TypeName): TypeName` | Standalone function with spread copy for objects |
//!
//! ## Configuration
//!
//! The `functionNamingStyle` option in `macroforge.json` controls naming:
//! - `"suffix"` (default): Suffixes with type name (e.g., `cloneMyType`)
//! - `"prefix"`: Prefixes with type name (e.g., `myTypeClone`)
//! - `"generic"`: Uses TypeScript generics (e.g., `clone<T extends MyType>`)
//! - `"namespace"`: Legacy namespace wrapping
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
//! @derive(Clone)
//! class Point {
//!     x: number;
//!     y: number;
//! }
//!
//! // Generated:
//! // clone(): Point {
//! //     const cloned = Object.create(Object.getPrototypeOf(this));
//! //     cloned.x = this.x;
//! //     cloned.y = this.y;
//! //     return cloned;
//! // }
//!
//! const p1 = new Point();
//! const p2 = p1.clone(); // Creates a new Point with same values
//! ```
//!
//! ## Implementation Notes
//!
//! - **Classes**: Uses `Object.create(Object.getPrototypeOf(this))` to preserve
//!   the prototype chain, ensuring `instanceof` checks work correctly
//! - **Enums**: Simply returns the value (enums are primitives in TypeScript)
//! - **Interfaces/Type Aliases**: Creates new object literals with spread operator
//!   for union/tuple types, or field-by-field copy for object types

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::abi::FunctionNamingStyle;
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

            Ok(body! {
                clone(): @{class_name} {
                    const cloned = Object.create(Object.getPrototypeOf(this));

                    {#if has_fields}
                        {#for field in field_names}
                            cloned.@{field} = this.@{field};
                        {/for}
                    {/if}

                    return cloned;
                }
            })
        }
        Data::Enum(_) => {
            // Enums are primitive values, cloning is just returning the value
            let enum_name = input.name();
            let naming_style = input.context.function_naming_style;

            match naming_style {
                FunctionNamingStyle::Namespace => Ok(ts_template! {
                    export namespace @{enum_name} {
                        export function clone(value: @{enum_name}): @{enum_name} {
                            return value;
                        }
                    }
                }),
                FunctionNamingStyle::Generic => Ok(ts_template! {
                    export function clone<T extends @{enum_name}>(value: T): T {
                        return value;
                    }
                }),
                FunctionNamingStyle::Prefix => {
                    let fn_name = format!("{}Clone", to_camel_case(enum_name));
                    Ok(ts_template! {
                        export function @{fn_name}(value: @{enum_name}): @{enum_name} {
                            return value;
                        }
                    })
                }
                FunctionNamingStyle::Suffix => {
                    let fn_name = format!("clone{}", enum_name);
                    Ok(ts_template! {
                        export function @{fn_name}(value: @{enum_name}): @{enum_name} {
                            return value;
                        }
                    })
                }
            }
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let field_names: Vec<&str> = interface.field_names().collect();
            let has_fields = !field_names.is_empty();
            let naming_style = input.context.function_naming_style;

            match naming_style {
                FunctionNamingStyle::Namespace => Ok(ts_template! {
                    export namespace @{interface_name} {
                        export function clone(self: @{interface_name}): @{interface_name} {
                            return {
                                {#if has_fields}
                                    {#for field in field_names}
                                        @{field}: self.@{field},
                                    {/for}
                                {/if}
                            };
                        }
                    }
                }),
                FunctionNamingStyle::Generic => Ok(ts_template! {
                    export function clone<T extends @{interface_name}>(value: T): T {
                        return {
                            {#if has_fields}
                                {#for field in field_names}
                                    @{field}: value.@{field},
                                {/for}
                            {/if}
                        } as T;
                    }
                }),
                FunctionNamingStyle::Prefix => {
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
                FunctionNamingStyle::Suffix => {
                    let fn_name = format!("clone{}", interface_name);
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
            }
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let naming_style = input.context.function_naming_style;

            if type_alias.is_object() {
                // Object type: spread copy
                let field_names: Vec<&str> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|f| f.name.as_str())
                    .collect();
                let has_fields = !field_names.is_empty();

                match naming_style {
                    FunctionNamingStyle::Namespace => Ok(ts_template! {
                        export namespace @{type_name} {
                            export function clone(value: @{type_name}): @{type_name} {
                                return {
                                    {#if has_fields}
                                        {#for field in field_names}
                                            @{field}: value.@{field},
                                        {/for}
                                    {/if}
                                };
                            }
                        }
                    }),
                    FunctionNamingStyle::Generic => Ok(ts_template! {
                        export function clone<T extends @{type_name}>(value: T): T {
                            return {
                                {#if has_fields}
                                    {#for field in field_names}
                                        @{field}: value.@{field},
                                    {/for}
                                {/if}
                            } as T;
                        }
                    }),
                    FunctionNamingStyle::Prefix => {
                        let fn_name = format!("{}Clone", to_camel_case(type_name));
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
                    }
                    FunctionNamingStyle::Suffix => {
                        let fn_name = format!("clone{}", type_name);
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
                    }
                }
            } else {
                // Union, tuple, or simple alias: use spread for objects, or return as-is
                match naming_style {
                    FunctionNamingStyle::Namespace => Ok(ts_template! {
                        export namespace @{type_name} {
                            export function clone(value: @{type_name}): @{type_name} {
                                if (typeof value === "object" && value !== null) {
                                    return { ...value } as @{type_name};
                                }
                                return value;
                            }
                        }
                    }),
                    FunctionNamingStyle::Generic => Ok(ts_template! {
                        export function clone<T extends @{type_name}>(value: T): T {
                            if (typeof value === "object" && value !== null) {
                                return { ...value } as T;
                            }
                            return value;
                        }
                    }),
                    FunctionNamingStyle::Prefix => {
                        let fn_name = format!("{}Clone", to_camel_case(type_name));
                        Ok(ts_template! {
                            export function @{fn_name}(value: @{type_name}): @{type_name} {
                                if (typeof value === "object" && value !== null) {
                                    return { ...value } as @{type_name};
                                }
                                return value;
                            }
                        })
                    }
                    FunctionNamingStyle::Suffix => {
                        let fn_name = format!("clone{}", type_name);
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
    }
}
