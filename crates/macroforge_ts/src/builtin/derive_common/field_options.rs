use super::args::{extract_named_string, has_flag, js_string, parse_string_literal};
use crate::ts_syn::abi::DecoratorIR;

/// Options parsed from field-level decorators for comparison macros
/// Supports @partialEq(skip), @hash(skip), @ord(skip)
#[derive(Default, Clone)]
pub struct CompareFieldOptions {
    pub skip: bool,
}

impl CompareFieldOptions {
    /// Parse field options from decorators for a specific attribute name
    pub fn from_decorators(decorators: &[DecoratorIR], attr_name: &str) -> Self {
        let mut opts = Self::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case(attr_name) {
                continue;
            }
            let args = decorator.args_src.trim();
            if has_flag(args, "skip") {
                opts.skip = true;
            }
        }
        opts
    }
}

// ============================================================================
// Field Options for Default Macro
// ============================================================================

/// Options parsed from @default decorator on fields
#[derive(Default, Clone)]
pub struct DefaultFieldOptions {
    /// The default value expression (e.g., "0", "\"\"", "[]")
    pub value: Option<String>,
    /// Whether this field has a @default decorator
    pub has_default: bool,
}

impl DefaultFieldOptions {
    pub fn from_decorators(decorators: &[DecoratorIR]) -> Self {
        let mut opts = Self::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("default") {
                continue;
            }
            opts.has_default = true;
            let args = decorator.args_src.trim();

            // Check for @default("value") or @default({ value: "..." })
            if let Some(value) = extract_default_value(args) {
                opts.value = Some(value);
            } else if !args.is_empty() {
                // Treat the args directly as the value if not empty
                // This handles @default(0), @default([]), @default(false), etc.
                opts.value = Some(args.to_string());
            }
        }
        opts
    }
}

/// Extract default value from decorator arguments
fn extract_default_value(args: &str) -> Option<String> {
    // Try named form: { value: "..." }
    if let Some(value) = extract_named_string(args, "value") {
        return Some(value);
    }

    // Try direct string literal: "..."
    if let Some(value) = parse_string_literal(args) {
        return Some(js_string(&value));
    }

    None
}
