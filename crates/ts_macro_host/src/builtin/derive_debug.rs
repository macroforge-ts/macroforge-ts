//! @Derive(Debug) macro implementation

use crate::traits::TsMacro;
use ts_macro_abi::{FieldIR, MacroContextIR, MacroKind, MacroResult, Patch, SpanIR};

pub struct DeriveDebugMacro;

impl TsMacro for DeriveDebugMacro {
    fn name(&self) -> &str {
        "Debug"
    }

    fn kind(&self) -> MacroKind {
        MacroKind::Derive
    }

    fn run(&self, ctx: MacroContextIR) -> MacroResult {
        let class = match ctx.as_class() {
            Some(class) => class,
            None => {
                return MacroResult {
                    runtime_patches: vec![],
                    type_patches: vec![],
                    diagnostics: vec![],
                    debug: Some("@Derive(Debug) can only be applied to classes".into()),
                };
            }
        };

        let debug_impl = generate_debug_implementation(class);
        let decorator_removals = collect_debug_decorator_removals(class);

        // The body_span includes the enclosing braces. We want to insert inside, before the '}'.
        let class_insert_point = SpanIR {
            start: class.body_span.end.saturating_sub(1),
            end: class.body_span.end.saturating_sub(1),
        };
        let post_class_insert_point = SpanIR {
            start: ctx.target_span.end,
            end: ctx.target_span.end,
        };

        let mut runtime_patches = decorator_removals.clone();
        runtime_patches.push(Patch::Insert {
            at: post_class_insert_point,
            code: debug_impl,
        });

        let mut type_patches = decorator_removals;
        type_patches.push(Patch::Insert {
            at: class_insert_point,
            code: generate_debug_signature(),
        });

        MacroResult {
            runtime_patches,
            type_patches,
            diagnostics: vec![],
            debug: None,
        }
    }

    fn description(&self) -> &str {
        "Generates a toString() method for debugging"
    }
}

fn generate_debug_implementation(class: &ts_macro_abi::ClassIR) -> String {
    let mut fields_debug: Vec<String> = Vec::new();
    for field in &class.fields {
        let opts = DebugFieldOptions::from_field(field);
        if opts.skip {
            continue;
        }

        let label = opts.rename.as_deref().unwrap_or(&field.name);
        fields_debug.push(format!("{label}: ${{this.{}}}", field.name));
    }

    let formatted_body = if fields_debug.is_empty() {
        format!("return `{} {{}}`;", class.name)
    } else {
        format!("return `{} {{ {} }}`;", class.name, fields_debug.join(", "))
    };

    format!(
        r#"
{class_name}.prototype.toString = function () {{
        {formatted_body}
}};
"#,
        class_name = class.name
    )
}

fn generate_debug_signature() -> String {
    "\n    toString(): string;\n".to_string()
}

fn collect_debug_decorator_removals(class: &ts_macro_abi::ClassIR) -> Vec<Patch> {
    let mut patches = Vec::new();
    for field in &class.fields {
        for decorator in &field.decorators {
            // Support both @Debug({...}) and @Derive({...}) patterns on fields
            if decorator.name == "Debug" || decorator.name == "Derive" {
                patches.push(Patch::Delete {
                    span: decorator.span,
                });
            }
        }
    }
    patches
}

#[derive(Default)]
struct DebugFieldOptions {
    skip: bool,
    rename: Option<String>,
}

impl DebugFieldOptions {
    fn from_field(field: &FieldIR) -> Self {
        let mut opts = DebugFieldOptions::default();
        for decorator in &field.decorators {
            if decorator.name != "Debug" {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ts_macro_abi::{ClassIR, DecoratorIR, SpanIR, Visibility};

    fn span() -> SpanIR {
        SpanIR::new(0, 0)
    }

    fn field(name: &str, decorator: Option<DecoratorIR>) -> FieldIR {
        FieldIR {
            name: name.into(),
            span: span(),
            ts_type: "string".into(),
            optional: false,
            readonly: false,
            visibility: Visibility::Public,
            decorators: decorator.into_iter().collect(),
        }
    }

    fn base_class(fields: Vec<FieldIR>) -> ClassIR {
        ClassIR {
            name: "User".into(),
            span: span(),
            body_span: span(),
            is_abstract: false,
            type_params: vec![],
            heritage: vec![],
            decorators: vec![],
            fields,
            methods: vec![],
        }
    }

    #[test]
    fn skips_fields_with_debug_attribute() {
        let decorator = DecoratorIR {
            name: "Debug".into(),
            args_src: "skip".into(),
            span: span(),
        };

        let class = base_class(vec![field("password", Some(decorator))]);
        let impl_code = generate_debug_implementation(&class);
        assert!(
            !impl_code.contains("password"),
            "password should be omitted from debug output"
        );
    }

    #[test]
    fn renames_fields_with_attribute() {
        let decorator = DecoratorIR {
            name: "Debug".into(),
            args_src: r#"{ rename: "identifier" }"#.into(),
            span: span(),
        };

        let class = base_class(vec![field("id", Some(decorator))]);
        let impl_code = generate_debug_implementation(&class);
        assert!(
            impl_code.contains("identifier: ${this.id}"),
            "field label should use rename option"
        );
    }

    #[test]
    fn skip_false_keeps_field() {
        let decorator = DecoratorIR {
            name: "Debug".into(),
            args_src: r#"{ skip: false }"#.into(),
            span: span(),
        };

        let class = base_class(vec![field("email", Some(decorator))]);
        let impl_code = generate_debug_implementation(&class);
        assert!(
            impl_code.contains("email: ${this.email}"),
            "skip: false should not remove the field"
        );
    }
}
