//! /** @derive(Eq) */ macro implementation

use crate::macros::{ts_macro_derive, body, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(Eq, description = "Generates equals() and hashCode() methods")]
pub fn derive_eq_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let field_names: Vec<&str> = class.field_names().collect();
            let has_fields = !field_names.is_empty();

            // Build comparison expression
            let comparison = if field_names.is_empty() {
                "true".to_string()
            } else {
                field_names
                    .iter()
                    .map(|f| format!("this.{f} === typedOther.{f}"))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };

            Ok(body! {
                equals(other: unknown): boolean {
                    if (this === other) return true;
                    if (!(other instanceof @{class_name})) return false;
                    const typedOther = other as @{class_name};
                    return @{comparison};
                }

                hashCode(): number {
                    let hash = 0;

                    {#if has_fields}
                        {#for field in field_names}
                            hash = (hash * 31 + (this.@{field} ? this.@{field}.toString().charCodeAt(0) : 0)) | 0;
                        {/for}
                    {/if}

                    return hash;
                }
            })
        }
        Data::Enum(_) => {
            // Enums: direct comparison with ===
            let enum_name = input.name();
            Ok(ts_template! {
                export namespace @{enum_name} {
                    export function equals(a: @{enum_name}, b: @{enum_name}): boolean {
                        return a === b;
                    }

                    export function hashCode(value: @{enum_name}): number {
                        // For numeric enums, use the value directly
                        // For string enums, hash the string
                        if (typeof value === "string") {
                            let hash = 0;
                            for (let i = 0; i < value.length; i++) {
                                hash = (hash * 31 + value.charCodeAt(i)) | 0;
                            }
                            return hash;
                        }
                        return value as number;
                    }
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let field_names: Vec<&str> = interface.field_names().collect();
            let has_fields = !field_names.is_empty();

            // Build comparison expression for interfaces
            let comparison = if field_names.is_empty() {
                "true".to_string()
            } else {
                field_names
                    .iter()
                    .map(|f| format!("self.{f} === other.{f}"))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };

            Ok(ts_template! {
                export namespace @{interface_name} {
                    export function equals(self: @{interface_name}, other: @{interface_name}): boolean {
                        if (self === other) return true;
                        return @{comparison};
                    }

                    export function hashCode(self: @{interface_name}): number {
                        let hash = 0;

                        {#if has_fields}
                            {#for field in field_names}
                                hash = (hash * 31 + (self.@{field} ? self.@{field}.toString().charCodeAt(0) : 0)) | 0;
                            {/for}
                        {/if}

                        return hash;
                    }
                }
            })
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            if type_alias.is_object() {
                // Object type: field-by-field comparison
                let field_names: Vec<&str> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|f| f.name.as_str())
                    .collect();
                let has_fields = !field_names.is_empty();

                let comparison = if field_names.is_empty() {
                    "true".to_string()
                } else {
                    field_names
                        .iter()
                        .map(|f| format!("a.{f} === b.{f}"))
                        .collect::<Vec<_>>()
                        .join(" && ")
                };

                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function equals(a: @{type_name}, b: @{type_name}): boolean {
                            if (a === b) return true;
                            return @{comparison};
                        }

                        export function hashCode(value: @{type_name}): number {
                            let hash = 0;

                            {#if has_fields}
                                {#for field in field_names}
                                    hash = (hash * 31 + (value.@{field} ? value.@{field}.toString().charCodeAt(0) : 0)) | 0;
                                {/for}
                            {/if}

                            return hash;
                        }
                    }
                })
            } else {
                // Union, tuple, or simple alias: use strict equality and JSON hash
                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function equals(a: @{type_name}, b: @{type_name}): boolean {
                            if (a === b) return true;
                            if (typeof a === "object" && typeof b === "object" && a !== null && b !== null) {
                                return JSON.stringify(a) === JSON.stringify(b);
                            }
                            return false;
                        }

                        export function hashCode(value: @{type_name}): number {
                            const str = JSON.stringify(value);
                            let hash = 0;
                            for (let i = 0; i < str.length; i++) {
                                hash = (hash * 31 + str.charCodeAt(i)) | 0;
                            }
                            return hash;
                        }
                    }
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_macro_output() {
        // This test just checks that the template compiles and runs
        let class_name = "User";
        let field_names: Vec<&str> = vec!["name"];
        let has_fields = !field_names.is_empty();

        let comparison = field_names
            .iter()
            .map(|f| format!("this.{f} === typedOther.{f}"))
            .collect::<Vec<_>>()
            .join(" && ");

        let output = body! {
            equals(other: unknown): boolean {
                if (this === other) return true;
                if (!(other instanceof @{class_name})) return false;
                const typedOther = other as @{class_name};
                return @{comparison};
            }

            hashCode(): number {
                let hash = 0;

                {#if has_fields}
                    {#for field in field_names}
                        hash = (hash * 31 + (this.@{field} ? this.@{field}.toString().charCodeAt(0) : 0)) | 0;
                    {/for}
                {/if}

                return hash;
            }
        };

        let source = output.source();

        // The source now includes the body marker, strip it for parsing test
        let body_content = source
            .strip_prefix("/* @macroforge:body */")
            .unwrap_or(source);

        // Try to parse as class members
        let wrapped = format!("class __Temp {{ {} }}", body_content);

        assert!(
            macroforge_ts_syn::parse_ts_stmt(&wrapped).is_ok(),
            "Generated Eq macro output should parse as class members"
        );

        assert!(source.contains("equals"), "Should contain equals method");
        assert!(
            source.contains("hashCode"),
            "Should contain hashCode method"
        );
    }
}
