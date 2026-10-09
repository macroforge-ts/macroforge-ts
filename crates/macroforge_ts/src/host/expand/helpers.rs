use convert_case::{Case, Casing};

use crate::ts_syn::abi::{ClassIR, Patch, SpanIR};

use super::derive_targets::DeriveTargetIR;

/// Extracts exported function names from patch code.
/// Returns a vector of (full_fn_name, short_name) pairs.
pub(super) fn extract_function_names_from_patches(
    patches: &[Patch],
    type_name: &str,
) -> Vec<(String, String)> {
    let mut functions: Vec<(String, String)> = Vec::new();
    let camel_type_name = type_name.to_case(Case::Camel);

    for patch in patches {
        let code = match patch {
            Patch::Insert { code, .. } | Patch::Replace { code, .. } => code,
            _ => continue,
        };

        // Each `export function <name>(` or `export function <name><T>(`.
        let mut search_start = 0;
        while let Some(pos) = code[search_start..].find("export function ") {
            let start = search_start + pos + "export function ".len();
            let rest = &code[start..];
            let name_len = rest
                .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
                .unwrap_or(rest.len());
            let fn_name = &rest[..name_len];
            let declares_function = rest[name_len..].trim_start().starts_with(['(', '<']);
            if !fn_name.is_empty()
                && declares_function
                && let Some(short_name) = extract_short_name(fn_name, &camel_type_name)
                && !functions
                    .iter()
                    .any(|(existing_name, ..)| existing_name == fn_name)
            {
                functions.push((fn_name.to_string(), short_name));
            }
            search_start = start + name_len;
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
                    format!("  export const {short_name}: typeof {full_name} = {full_name};")
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
            // Interfaces and type aliases use a const object. Its type is written
            // out so `--isolatedDeclarations` (and JSR) can emit it unaided.
            let members: Vec<String> = functions
                .iter()
                .map(|(full_name, short_name)| {
                    format!("  readonly {short_name}: typeof {full_name};")
                })
                .collect();
            let entries: Vec<String> = functions
                .iter()
                .map(|(full_name, short_name)| format!("  {short_name}: {full_name}"))
                .collect();

            format!(
                "{export_keyword}const {type_name}: {{\n{}\n}} = {{\n{}\n}};",
                members.join("\n"),
                entries.join(",\n")
            )
        }
    }
}

/// Checks if the source already has a namespace or const declaration with the given name.
/// This prevents generating a convenience const that would conflict with existing declarations.
pub(super) fn has_existing_namespace_or_const(source: &str, type_name: &str) -> bool {
    if type_name.is_empty() {
        return false;
    }
    source.match_indices(type_name).any(|(at, _)| {
        let before = source[..at].trim_end_matches([' ', '\t', '\n']);
        if before.len() == at {
            return false;
        }
        let next = source[at + type_name.len()..].chars().next();
        let declares = |keyword: &str, followers: &[char]| {
            before
                .strip_suffix(keyword)
                .is_some_and(|head| !head.chars().next_back().is_some_and(is_ident_char))
                && next.is_none_or(|c| followers.contains(&c))
        };
        declares("namespace", &['{', ' ', '\t', '\n', '<'])
            || declares("const", &['=', ' ', '\t', '\n', ':', '<'])
    })
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

fn contains_identifier(haystack: &str, ident: &str) -> bool {
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

/// The identifiers of one text, for checking many names against it without
/// rescanning it for each.
pub(super) struct Identifiers<'text> {
    text: &'text str,
    words: std::collections::HashSet<&'text str>,
}

impl<'text> Identifiers<'text> {
    pub(super) fn of(text: &'text str) -> Self {
        Self {
            text,
            words: text
                .split(|c: char| !is_ident_char(c))
                .filter(|word| !word.is_empty())
                .collect(),
        }
    }

    /// Whether `ident` occurs in the text as a whole identifier, exactly as
    /// [`contains_identifier`] answers.
    pub(super) fn contains(&self, ident: &str) -> bool {
        if ident.chars().all(is_ident_char) {
            self.words.contains(ident)
        } else {
            contains_identifier(self.text, ident)
        }
    }
}

pub(super) fn derive_insert_pos(class_ir: &ClassIR, source: &str) -> u32 {
    let end = class_ir.span.end as usize;
    let search = &source[..end.min(source.len())];
    search
        .rfind('}')
        .map(|idx| idx as u32 + 1)
        .unwrap_or_else(|| class_ir.body_span.end.max(class_ir.span.start))
}

/// Members for a class body, laid out to go before its closing brace at the
/// 1-based position `at`: on their own lines, one level deeper than the
/// brace's line, with the brace back on a line of its own. Code holding a
/// template literal keeps its lines as they are, since re-indenting one
/// would change the string.
pub(crate) fn class_body_payload(code: &str, source: &str, at: u32) -> String {
    let brace = (at as usize).saturating_sub(1).min(source.len());
    let line_start = source
        .get(..brace)
        .and_then(|before| before.rfind('\n'))
        .map_or(0, |newline| newline + 1);
    let brace_indent: String = source
        .get(line_start..brace)
        .unwrap_or_default()
        .chars()
        .take_while(|character| *character == ' ' || *character == '\t')
        .collect();
    let members = code.trim_matches('\n').trim_end();
    let body = if members.contains('`') {
        members.to_string()
    } else {
        let member_indent = format!("{brace_indent}    ");
        members
            .lines()
            .map(|line| {
                if line.trim().is_empty() {
                    String::new()
                } else {
                    format!("{member_indent}{line}")
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    // An indented brace on its own line already has its indentation before
    // `at`; the first member takes that place instead of a new line.
    let brace_on_own_line = !brace_indent.is_empty()
        && source
            .get(line_start..brace)
            .is_some_and(|before| before.trim().is_empty());
    match body.strip_prefix(brace_indent.as_str()) {
        Some(first_line_onward) if brace_on_own_line => {
            format!("{first_line_onward}\n{brace_indent}")
        }
        Some(_) | None => format!("\n{body}\n{brace_indent}"),
    }
}

pub(super) fn find_macro_comment_span(source: &str, target_start: u32) -> Option<SpanIR> {
    let block =
        crate::ts_syn::jsdoc::adjacent_jsdoc(source, target_start.saturating_sub(1) as usize)?;
    Some(SpanIR::new(block.start as u32 + 1, block.end as u32 + 1))
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
