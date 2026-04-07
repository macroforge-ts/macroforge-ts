use convert_case::{Case, Casing};

use crate::ts_syn::abi::{ClassIR, Patch, PatchCode, SpanIR};
#[cfg(feature = "swc")]
use swc_core::{
    common::{FileName, SourceMap, sync::Lrc},
    ecma::{
        ast::{ClassMember, EsVersion},
        parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer},
    },
};

use super::derive_targets::DeriveTargetIR;

/// Extracts exported function names from patch code.
/// Returns a vector of (full_fn_name, short_name) pairs.
pub(super) fn extract_function_names_from_patches(
    patches: &[Patch],
    type_name: &str,
) -> Vec<(String, String)> {
    let mut functions = Vec::new();
    let camel_type_name = type_name.to_case(Case::Camel);

    for patch in patches {
        let code = match patch {
            Patch::Insert {
                code: PatchCode::Text(text),
                ..
            } => text,
            Patch::Replace {
                code: PatchCode::Text(text),
                ..
            } => text,
            _ => continue,
        };

        // Find "export function <name>(" patterns
        let mut search_start = 0;
        while let Some(pos) = code[search_start..].find("export function ") {
            let start = search_start + pos + "export function ".len();
            if let Some(paren_pos) = code[start..].find('(') {
                let fn_name = code[start..start + paren_pos].trim();
                if !fn_name.is_empty()
                    && fn_name
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_')
                    && let Some(short_name) = extract_short_name(fn_name, &camel_type_name)
                {
                    functions.push((fn_name.to_string(), short_name));
                }
                search_start = start + paren_pos;
            } else {
                break;
            }
        }
    }

    functions
}

/// Extracts the short function name from the full function name.
/// Uses Prefix naming style: userClone -> clone, userDefaultValue -> defaultValue
fn extract_short_name(full_name: &str, camel_type_name: &str) -> Option<String> {
    if let Some(rest) = full_name.strip_prefix(camel_type_name) {
        if rest.is_empty() {
            return None;
        }
        // Convert first char to lowercase (UserClone prefix removed, rest is Clone -> clone)
        Some(rest.to_case(Case::Camel))
    } else {
        None
    }
}

/// Generates a convenience export that groups all generated functions for a type.
/// For enums, uses namespace merging (valid TS). For other types, uses const object.
pub(super) fn generate_convenience_export(
    target: &DeriveTargetIR,
    type_name: &str,
    functions: &[(String, String)],
    is_exported: bool,
) -> String {
    if functions.is_empty() {
        return String::new();
    }

    let export_keyword = if is_exported { "export " } else { "" };

    match target {
        DeriveTargetIR::Enum(_) => {
            // Enums require namespace merging - const redeclaration is invalid TS
            let entries: Vec<String> = functions
                .iter()
                .map(|(full_name, short_name)| {
                    format!("  export const {} = {};", short_name, full_name)
                })
                .collect();

            format!(
                "{}namespace {} {{\n{}\n}}",
                export_keyword,
                type_name,
                entries.join("\n")
            )
        }
        _ => {
            // Interfaces and type aliases use const object
            let entries: Vec<String> = functions
                .iter()
                .map(|(full_name, short_name)| format!("  {}: {}", short_name, full_name))
                .collect();

            format!(
                "{}const {} = {{\n{}\n}} as const;",
                export_keyword,
                type_name,
                entries.join(",\n")
            )
        }
    }
}

/// Checks if the source already has a namespace or const declaration with the given name.
/// This prevents generating a convenience const that would conflict with existing declarations.
pub(super) fn has_existing_namespace_or_const(source: &str, type_name: &str) -> bool {
    // Check for `namespace TypeName` or `const TypeName`
    // Look for common patterns with various whitespace
    let namespace_patterns = [
        format!("namespace {}", type_name),
        format!("namespace  {}", type_name),
        format!("namespace\t{}", type_name),
        format!("namespace\n{}", type_name),
    ];

    let const_patterns = [
        format!("const {}", type_name),
        format!("const  {}", type_name),
        format!("const\t{}", type_name),
        format!("const\n{}", type_name),
    ];

    for pattern in &namespace_patterns {
        if let Some(pos) = source.find(pattern.as_str()) {
            // Check that this is followed by whitespace, { or end of string
            let after_pos = pos + pattern.len();
            if after_pos >= source.len() {
                return true;
            }
            let next_char = source[after_pos..].chars().next();
            if matches!(
                next_char,
                Some('{') | Some(' ') | Some('\t') | Some('\n') | Some('<')
            ) {
                return true;
            }
        }
    }

    for pattern in &const_patterns {
        if let Some(pos) = source.find(pattern.as_str()) {
            // Check that this is followed by whitespace, = or end of string
            let after_pos = pos + pattern.len();
            if after_pos >= source.len() {
                return true;
            }
            let next_char = source[after_pos..].chars().next();
            if matches!(
                next_char,
                Some('=') | Some(' ') | Some('\t') | Some('\n') | Some(':') | Some('<')
            ) {
                return true;
            }
        }
    }

    false
}

/// Gets the type name from a DeriveTargetIR.
/// Returns None for classes (they use instance methods).
pub(super) fn get_derive_target_name(target: &DeriveTargetIR) -> Option<&str> {
    match target {
        DeriveTargetIR::Class(_) => None,
        DeriveTargetIR::Interface(i) => Some(&i.name),
        DeriveTargetIR::Enum(e) => Some(&e.name),
        DeriveTargetIR::TypeAlias(t) => Some(&t.name),
    }
}

/// Gets the end span position for a DeriveTargetIR.
pub(super) fn get_derive_target_end_span(target: &DeriveTargetIR) -> u32 {
    match target {
        DeriveTargetIR::Class(c) => c.span.end,
        DeriveTargetIR::Interface(i) => i.span.end,
        DeriveTargetIR::Enum(e) => e.span.end,
        DeriveTargetIR::TypeAlias(t) => t.span.end,
    }
}

/// Gets the start span position for a DeriveTargetIR.
pub(super) fn get_derive_target_start_span(target: &DeriveTargetIR) -> u32 {
    match target {
        DeriveTargetIR::Class(c) => c.span.start,
        DeriveTargetIR::Interface(i) => i.span.start,
        DeriveTargetIR::Enum(e) => e.span.start,
        DeriveTargetIR::TypeAlias(t) => t.span.start,
    }
}

/// Checks if a declaration at the given position is exported.
/// Looks for the `export` keyword before the declaration start.
pub(super) fn is_declaration_exported(source: &str, decl_start: u32) -> bool {
    let start = decl_start as usize;
    if start == 0 || start > source.len() {
        return false;
    }

    // Look at the text before the declaration (up to 50 chars should be enough)
    let look_back = start.min(50);
    let prefix = &source[start - look_back..start];

    // Find "export" keyword - must be followed by whitespace and not be part of another word
    if let Some(pos) = prefix.rfind("export") {
        let after_export = pos + 6;
        // Check that "export" is followed by whitespace (or is at the end of prefix)
        if after_export >= prefix.len() {
            return true;
        }
        let next_char = prefix[after_export..].chars().next();
        if matches!(next_char, Some(' ') | Some('\t') | Some('\n')) {
            // Also check it's not part of a larger word (e.g., "reexport")
            if pos == 0 {
                return true;
            }
            let prev_char = prefix[..pos].chars().last();
            if !matches!(prev_char, Some(c) if c.is_ascii_alphanumeric() || c == '_') {
                return true;
            }
        }
    }

    false
}

pub(super) fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '$'
}

pub(super) fn contains_identifier(haystack: &str, ident: &str) -> bool {
    if ident.is_empty() {
        return false;
    }

    let mut search_start = 0;
    while let Some(pos) = haystack[search_start..].find(ident) {
        let start = search_start + pos;
        let end = start + ident.len();

        let prev_ok = start == 0
            || !haystack[..start]
                .chars()
                .next_back()
                .is_some_and(is_ident_char);
        let next_ok =
            end >= haystack.len() || !haystack[end..].chars().next().is_some_and(is_ident_char);

        if prev_ok && next_ok {
            return true;
        }

        search_start = end;
    }

    false
}

pub(super) fn derive_insert_pos(class_ir: &ClassIR, source: &str) -> u32 {
    let end = class_ir.span.end as usize;
    let search = &source[..end.min(source.len())];
    search
        .rfind('}')
        .map(|idx| idx as u32 + 1)
        .unwrap_or_else(|| class_ir.body_span.end.max(class_ir.span.start))
}

pub(super) fn find_macro_comment_span(source: &str, target_start: u32) -> Option<SpanIR> {
    let start = target_start.saturating_sub(1) as usize;
    if start == 0 || start > source.len() {
        return None;
    }
    let search_area = &source[..start];
    let start_idx = search_area.rfind("/**")?;
    let rest = &search_area[start_idx..];
    let end_rel = rest.find("*/")?;
    let end_idx = start_idx + end_rel + 2;

    let between = &search_area[end_idx..];
    if !between.trim().is_empty() {
        return None;
    }

    Some(SpanIR::new(start_idx as u32 + 1, end_idx as u32 + 1))
}

/// Convert InsertPos enum to the string location used internally.
pub(super) fn insert_pos_to_location(pos: crate::ts_syn::InsertPos) -> &'static str {
    match pos {
        crate::ts_syn::InsertPos::Top => "top",
        crate::ts_syn::InsertPos::Above => "above",
        crate::ts_syn::InsertPos::Within => "body",
        crate::ts_syn::InsertPos::Below => "below",
        crate::ts_syn::InsertPos::Bottom => "bottom",
    }
}

pub(super) fn split_by_markers(
    source: &str,
    default_pos: crate::ts_syn::InsertPos,
) -> Vec<(&'static str, String)> {
    let markers = [
        ("top", "/* @macroforge:top */"),
        ("above", "/* @macroforge:above */"),
        ("below", "/* @macroforge:below */"),
        ("body", "/* @macroforge:body */"),
        ("bottom", "/* @macroforge:bottom */"),
        // Legacy markers for backward compatibility
        ("body", "/* @macroforge:signature */"),
    ];

    let mut occurrences = Vec::new();
    for (name, pattern) in markers {
        for (idx, _) in source.match_indices(pattern) {
            occurrences.push((idx, pattern.len(), name));
        }
    }
    occurrences.sort_by_key(|k| k.0);

    let default_location = insert_pos_to_location(default_pos);

    if occurrences.is_empty() {
        return vec![(default_location, source.to_string())];
    }

    let mut chunks = Vec::new();

    if occurrences[0].0 > 0 {
        let text = &source[0..occurrences[0].0];
        if !text.trim().is_empty() {
            chunks.push((default_location, text.to_string()));
        }
    }

    for i in 0..occurrences.len() {
        let (start, len, name) = occurrences[i];
        let content_start = start + len;
        let content_end = if i + 1 < occurrences.len() {
            occurrences[i + 1].0
        } else {
            source.len()
        };

        let content = &source[content_start..content_end];
        chunks.push((name, content.to_string()));
    }

    chunks
}

/// A class member with its associated leading JSDoc comment (if any).
#[cfg(feature = "swc")]
pub(super) struct MemberWithComment {
    /// The leading JSDoc comment text (without /** */)
    pub leading_comment: Option<String>,
    /// The class member AST node
    pub member: ClassMember,
}

#[cfg(feature = "swc")]
pub(super) fn parse_members_from_tokens(tokens: &str) -> anyhow::Result<Vec<MemberWithComment>> {
    // First, extract JSDoc comments and their associated code segments
    // The body! macro outputs: /** comment */code /** comment */code ...
    let segments = extract_jsdoc_segments(tokens);

    // Build class body without comments for parsing
    let code_only: String = segments.iter().map(|(_, code)| code.as_str()).collect();
    let wrapped_stmt = format!("class __Temp {{ {} }}", code_only);

    // Parse using standard SWC (comments stripped)
    let cm: Lrc<SourceMap> = Lrc::new(Default::default());
    let fm = cm.new_source_file(
        FileName::Custom("macro_body.ts".into()).into(),
        wrapped_stmt,
    );

    let syntax = Syntax::Typescript(TsSyntax {
        tsx: true,
        decorators: true,
        ..Default::default()
    });

    let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);

    let module = parser
        .parse_module()
        .map_err(|e| anyhow::anyhow!("Failed to parse macro output: {:?}", e))?;

    let class_body = module
        .body
        .into_iter()
        .find_map(|item| {
            if let swc_core::ecma::ast::ModuleItem::Stmt(swc_core::ecma::ast::Stmt::Decl(
                swc_core::ecma::ast::Decl::Class(class_decl),
            )) = item
            {
                Some(class_decl.class.body)
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow::anyhow!("Failed to parse macro output into class members"))?;

    // Match parsed members with their extracted JSDoc comments
    // Use enumerate instead of zip to handle cases where there are more members than segments
    // (e.g., when no JSDoc comments exist, all code is in one segment but parses to multiple members)
    let result = class_body
        .into_iter()
        .enumerate()
        .map(|(i, member)| MemberWithComment {
            leading_comment: segments.get(i).and_then(|(comment, _)| comment.clone()),
            member,
        })
        .collect();

    Ok(result)
}

/// Extract JSDoc comments and their following code segments from body! output.
/// Returns a vec of (Option<comment_text>, code_segment) pairs.
#[cfg(feature = "swc")]
pub(super) fn extract_jsdoc_segments(input: &str) -> Vec<(Option<String>, String)> {
    let mut result = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        remaining = remaining.trim_start();
        if remaining.is_empty() {
            break;
        }

        // Check if starts with JSDoc comment
        if remaining.starts_with("/**") {
            // Find the end of the comment
            if let Some(end_idx) = remaining.find("*/") {
                let comment_text = &remaining[3..end_idx]; // Skip /** and exclude */
                remaining = &remaining[end_idx + 2..]; // Skip past */
                // Now find the code until the next JSDoc or end
                let code_end = remaining.find("/**").unwrap_or(remaining.len());
                let code = remaining[..code_end].to_string();
                remaining = &remaining[code_end..];

                if !code.trim().is_empty() {
                    result.push((Some(comment_text.to_string()), code));
                }
            } else {
                // Malformed comment, treat rest as code
                result.push((None, remaining.to_string()));
                break;
            }
        } else {
            // No JSDoc comment, find code until next JSDoc or end
            let code_end = remaining.find("/**").unwrap_or(remaining.len());
            let code = remaining[..code_end].to_string();
            remaining = &remaining[code_end..];

            if !code.trim().is_empty() {
                result.push((None, code));
            }
        }
    }

    result
}
