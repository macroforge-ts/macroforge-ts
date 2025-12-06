//! /** @derive(Eq) */ macro implementation

use ts_macro_derive::ts_macro_derive;
use ts_quote::body;
use ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

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
        Data::Enum(_) => Err(MacroforgeError::new(
            input.error_span(),
            "/** @derive(Eq) */ can only be applied to classes",
        )),
        Data::Interface(_) => Err(MacroforgeError::new(
            input.error_span(),
            "/** @derive(Eq) */ can only be applied to classes, not interfaces",
        )),
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
            .unwrap_or(&source);

        // Try to parse as class members
        let wrapped = format!("class __Temp {{ {} }}", body_content);

        assert!(
            ts_syn::parse_ts_stmt(&wrapped).is_ok(),
            "Generated Eq macro output should parse as class members"
        );

        assert!(source.contains("equals"), "Should contain equals method");
        assert!(
            source.contains("hashCode"),
            "Should contain hashCode method"
        );
    }
}
