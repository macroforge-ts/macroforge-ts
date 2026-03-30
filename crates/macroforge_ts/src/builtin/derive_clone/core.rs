use convert_case::{Case, Casing};

use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::Expr;
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input, ts_ident};

use super::clone_generation::generate_clone_expr;

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
