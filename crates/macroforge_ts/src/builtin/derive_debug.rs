//! /** @derive(Debug) */ macro implementation

use crate::macros::{ts_macro_derive, body, ts_template};
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
    attributes(debug)
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

            Ok(body! {
                toString(): string {
                    {#if has_fields}
                        const parts: string[] = [];
                        {#for (label, name) in debug_fields}
                            parts.push("@{label}: " + this.@{name});
                        {/for}
                        return "@{class_name} { " + parts.join(", ") + " }";
                    {:else}
                        return "@{class_name} {}";
                    {/if}
                }
            })
        }
        Data::Enum(_) => Err(MacroforgeError::new(
            input.error_span(),
            "/** @derive(Debug) */ can only be applied to classes or interfaces",
        )),
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

            Ok(ts_template! {
                export namespace @{interface_name} {
                    export function toString(self: @{interface_name}): string {
                        {#if has_fields}
                            const parts: string[] = [];
                            {#for (label, name) in debug_fields}
                                parts.push("@{label}: " + self.@{name});
                            {/for}
                            return "@{interface_name} { " + parts.join(", ") + " }";
                        {:else}
                            return "@{interface_name} {}";
                        {/if}
                    }
                }
            })
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
