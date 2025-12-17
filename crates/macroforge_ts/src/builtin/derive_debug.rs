//! # Debug Macro Implementation
//!
//! The `Debug` macro generates a human-readable `toString()` method for
//! TypeScript classes, interfaces, enums, and type aliases.
//!
//! ## Generated Output
//!
//! **Classes**: Generates a standalone function `classNameToString(value)` and a static wrapper
//! method `static toString(value)` returning a string like `"ClassName { field1: value1, field2: value2 }"`.
//!
//! **Enums**: Generates a standalone function `enumNameToString(value)` that performs
//! reverse lookup on numeric enums.
//!
//! **Interfaces**: Generates a standalone function `interfaceNameToString(value)`.
//!
//! **Type Aliases**: Generates a standalone function using JSON.stringify for
//! complex types, or field enumeration for object types.
//!
//!
//! ## Field-Level Options
//!
//! The `@debug` decorator supports:
//!
//! - `skip` - Exclude the field from debug output
//! - `rename = "label"` - Use a custom label instead of the field name
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Debug) */
//! class User {
//!     /** @debug({ rename: "id" }) */
//!     userId: number;
//!
//!     /** @debug({ skip: true }) */
//!     password: string;
//!
//!     email: string;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class User {
//!     userId: number;
//! 
//!     password: string;
//! 
//!     email: string;
//! 
//!     static toString(value: User): string {
//!         return userToString(value);
//!     }
//! }
//! 
//! export function userToString(value: User): string {
//!     const parts: string[] = [];
//!     parts.push('id: ' + value.userId);
//!     parts.push('email: ' + value.email);
//!     return 'User { ' + parts.join(', ') + ' }';
//! }
//! ```

use convert_case::{Case, Casing};

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

/// Options parsed from @Debug decorator on fields
#[derive(Default)]
struct DebugFieldOptions {
    skip: bool,
    rename: Option<String>,
}

impl DebugFieldOptions {
    fn from_decorators(decorators: &[crate::ts_syn::abi::DecoratorIR]) -> Self {
        let mut opts = DebugFieldOptions::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("debug") {
                continue;
            }

            let args = decorator.args_src.trim();
            if args.is_empty() {
                continue;
            }

            if has_flag(args, "skip") {
                opts.skip = true;
            }

            if let Some(rename) = extract_named_string(args, "rename") {
                opts.rename = Some(rename);
            }
        }
        opts
    }
}

fn has_flag(args: &str, flag: &str) -> bool {
    if flag_explicit_false(args, flag) {
        return false;
    }

    args.split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|token| token.eq_ignore_ascii_case(flag))
}

fn flag_explicit_false(args: &str, flag: &str) -> bool {
    let lower = args.to_ascii_lowercase();
    let condensed: String = lower.chars().filter(|c| !c.is_whitespace()).collect();
    condensed.contains(&format!("{flag}:false")) || condensed.contains(&format!("{flag}=false"))
}

fn extract_named_string(args: &str, name: &str) -> Option<String> {
    let lower = args.to_ascii_lowercase();
    let idx = lower.find(name)?;
    let remainder = &args[idx + name.len()..];
    let remainder = remainder.trim_start();

    if remainder.starts_with(':') || remainder.starts_with('=') {
        let value = remainder[1..].trim_start();
        return parse_string_literal(value);
    }

    if remainder.starts_with('(')
        && let Some(close) = remainder.rfind(')')
    {
        let inner = remainder[1..close].trim();
        return parse_string_literal(inner);
    }

    None
}

fn parse_string_literal(input: &str) -> Option<String> {
    let trimmed = input.trim();
    let mut chars = trimmed.chars();
    let quote = chars.next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let mut escaped = false;
    let mut buf = String::new();
    for c in chars {
        if escaped {
            buf.push(c);
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if c == quote {
            return Some(buf);
        }
        buf.push(c);
    }
    None
}

/// Debug field info: (label, field_name)
type DebugField = (String, String);

#[ts_macro_derive(
    Debug,
    description = "Generates a toString() method for debugging",
    attributes((debug, "Configure debug output for this field. Options: skip (exclude from output), rename (custom label)"))
)]
pub fn derive_debug_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

            // Collect fields that should be included in debug output
            let debug_fields: Vec<DebugField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = DebugFieldOptions::from_decorators(&field.decorators);
                    if opts.skip {
                        return None;
                    }
                    let label = opts.rename.unwrap_or_else(|| field.name.clone());
                    Some((label, field.name.clone()))
                })
                .collect();

            let has_fields = !debug_fields.is_empty();

            // Generate function name (always prefix style)
            let fn_name = format!("{}ToString", class_name.to_case(Case::Camel));

            // Generate standalone function with value parameter
            let standalone = ts_template! {
                export function @{fn_name}(value: @{class_name}): string {
                    {#if has_fields}
                        const parts: string[] = [];
                        {#for (label, name) in debug_fields}
                            parts.push("@{label}: " + value.@{name});
                        {/for}
                        return "@{class_name} { " + parts.join(", ") + " }";
                    {:else}
                        return "@{class_name} {}";
                    {/if}
                }
            };

            // Generate static wrapper method that delegates to standalone function
            let class_body = body! {
                static toString(value: @{class_name}): string {
                    return @{fn_name}(value);
                }
            };

            // Combine standalone function with class body by concatenating sources
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            // The body! output has /* @macroforge:body */ marker for class body insertion
            let combined_source = format!("{}\n{}", standalone.source(), class_body.source());
            let mut combined = TsStream::from_string(combined_source);
            combined.runtime_patches = standalone.runtime_patches;
            combined.runtime_patches.extend(class_body.runtime_patches);

            Ok(combined)
        }
        Data::Enum(enum_data) => {
            let enum_name = input.name();
            let variants: Vec<String> = enum_data
                .variants()
                .iter()
                .map(|v| v.name.clone())
                .collect();
            let has_variants = !variants.is_empty();

            let fn_name = format!("{}ToString", enum_name.to_case(Case::Camel));
            Ok(ts_template! {
                export function @{fn_name}(value: @{enum_name}): string {
                    {#if has_variants}
                        const key = @{enum_name}[value as unknown as keyof typeof @{enum_name}];
                        if (key !== undefined) {
                            return "@{enum_name}." + key;
                        }
                        return "@{enum_name}(" + String(value) + ")";
                    {:else}
                        return "@{enum_name}(" + String(value) + ")";
                    {/if}
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();

            // Collect fields that should be included in debug output
            let debug_fields: Vec<DebugField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = DebugFieldOptions::from_decorators(&field.decorators);
                    if opts.skip {
                        return None;
                    }
                    let label = opts.rename.unwrap_or_else(|| field.name.clone());
                    Some((label, field.name.clone()))
                })
                .collect();

            let has_fields = !debug_fields.is_empty();
            let fn_name = format!("{}ToString", interface_name.to_case(Case::Camel));

            Ok(ts_template! {
                export function @{fn_name}(value: @{interface_name}): string {
                    {#if has_fields}
                        const parts: string[] = [];
                        {#for (label, name) in debug_fields}
                            parts.push("@{label}: " + value.@{name});
                        {/for}
                        return "@{interface_name} { " + parts.join(", ") + " }";
                    {:else}
                        return "@{interface_name} {}";
                    {/if}
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            // Generate different output based on type body
            if type_alias.is_object() {
                // Object type: show fields
                let debug_fields: Vec<DebugField> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter_map(|field| {
                        let opts = DebugFieldOptions::from_decorators(&field.decorators);
                        if opts.skip {
                            return None;
                        }
                        let label = opts.rename.unwrap_or_else(|| field.name.clone());
                        Some((label, field.name.clone()))
                    })
                    .collect();

                let has_fields = !debug_fields.is_empty();
                let fn_name = format!("{}ToString", type_name.to_case(Case::Camel));

                Ok(ts_template! {
                    export function @{fn_name}(value: @{type_name}): string {
                        {#if has_fields}
                            const parts: string[] = [];
                            {#for (label, name) in debug_fields}
                                parts.push("@{label}: " + value.@{name});
                            {/for}
                            return "@{type_name} { " + parts.join(", ") + " }";
                        {:else}
                            return "@{type_name} {}";
                        {/if}
                    }
                })
            } else {
                // Union, intersection, tuple, or simple alias: use JSON.stringify
                let fn_name = format!("{}ToString", type_name.to_case(Case::Camel));

                Ok(ts_template! {
                    export function @{fn_name}(value: @{type_name}): string {
                        return "@{type_name}(" + JSON.stringify(value) + ")";
                    }
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ts_syn::abi::{DecoratorIR, SpanIR};

    fn span() -> SpanIR {
        SpanIR::new(0, 0)
    }

    #[test]
    fn test_skip_flag() {
        let decorator = DecoratorIR {
            name: "Debug".into(),
            args_src: "skip".into(),
            span: span(),
            node: None,
        };

        let opts = DebugFieldOptions::from_decorators(&[decorator]);
        assert!(opts.skip, "skip flag should be true");
    }

    #[test]
    fn test_skip_false_keeps_field() {
        let decorator = DecoratorIR {
            name: "Debug".into(),
            args_src: r#"{ skip: false }"#.into(),
            span: span(),
            node: None,
        };

        let opts = DebugFieldOptions::from_decorators(&[decorator]);
        assert!(!opts.skip, "skip: false should not skip the field");
    }

    #[test]
    fn test_rename_option() {
        let decorator = DecoratorIR {
            name: "Debug".into(),
            args_src: r#"{ rename: "identifier" }"#.into(),
            span: span(),
            node: None,
        };

        let opts = DebugFieldOptions::from_decorators(&[decorator]);
        assert_eq!(opts.rename.as_deref(), Some("identifier"));
    }
}
