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
        return Some(format!("\"{}\"", value));
    }

    None
}

// ============================================================================
// Helper functions (shared with other modules)
// ============================================================================

/// Check if a decorator argument string contains the given flag.
///
/// Splits the argument string on non-alphanumeric characters and checks if
/// any token matches `flag` (case-insensitive). Returns `false` if the flag
/// is explicitly set to `false` (e.g., `skip: false` or `skip=false`).
pub fn has_flag(args: &str, flag: &str) -> bool {
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

/// Extract a named string value from a decorator argument string.
///
/// Looks for patterns like `name: "value"`, `name = "value"`, or `name("value")`
/// and returns the unquoted string. The name match is case-insensitive.
///
/// Returns `None` if the name is not found or the value is not a string literal.
pub fn extract_named_string(args: &str, name: &str) -> Option<String> {
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

pub(crate) fn parse_string_literal(input: &str) -> Option<String> {
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
