use std::collections::HashMap;

use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, SpanIR};
use swc_core::ecma::ast::Module;

use super::BUILTIN_MACRO_NAMES;
use super::helpers::contains_identifier;

/// Result of collecting import information from a module.
pub struct ImportCollectionResult {
    /// Maps local identifier names to their module sources.
    pub sources: HashMap<String, String>,
    /// Maps local alias names to their original imported names.
    /// For `import { Option as EffectOption }`, contains `"EffectOption" -> "Option"`.
    pub aliases: HashMap<String, String>,
}

/// Collect import information from a module.
///
/// Returns both:
/// - A map of identifier name -> module source
/// - A map of local alias name -> original imported name
pub fn collect_import_sources(module: &Module, source: &str) -> ImportCollectionResult {
    use swc_core::ecma::ast::{
        ImportDecl, ImportSpecifier, ModuleDecl, ModuleExportName, ModuleItem,
    };

    let mut import_map = HashMap::new();
    let mut alias_map = HashMap::new();

    import_map.extend(collect_macro_import_comments(source));

    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
            specifiers, src, ..
        })) = item
        {
            let module_source = src.value.to_string_lossy().to_string();

            for specifier in specifiers {
                match specifier {
                    ImportSpecifier::Named(named) => {
                        let local_name = named.local.sym.to_string();
                        import_map.insert(local_name.clone(), module_source.clone());

                        // If there's an alias (imported name differs from local name),
                        // record the mapping from local -> original
                        if let Some(imported) = &named.imported {
                            let original_name = match imported {
                                ModuleExportName::Ident(ident) => ident.sym.to_string(),
                                ModuleExportName::Str(s) => {
                                    String::from_utf8_lossy(s.value.as_bytes()).to_string()
                                }
                            };
                            if original_name != local_name {
                                alias_map.insert(local_name, original_name);
                            }
                        }
                    }
                    ImportSpecifier::Default(default) => {
                        let local_name = default.local.sym.to_string();
                        import_map.insert(local_name, module_source.clone());
                    }
                    ImportSpecifier::Namespace(ns) => {
                        let local_name = ns.local.sym.to_string();
                        import_map.insert(local_name, module_source.clone());
                    }
                }
            }
        }
    }

    ImportCollectionResult {
        sources: import_map,
        aliases: alias_map,
    }
}

pub(super) fn collect_macro_import_comments(source: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let mut search_start = 0usize;

    while let Some(idx) = source[search_start..].find("/**") {
        let abs_idx = search_start + idx;
        let remaining = &source[abs_idx + 3..];
        let Some(end_rel) = remaining.find("*/") else {
            break;
        };
        let body = &remaining[..end_rel];
        let normalized = normalize_macro_import_body(body);
        let normalized_lower = normalized.to_ascii_lowercase();

        if normalized_lower.contains("import macro")
            && let (Some(open_brace), Some(close_brace)) =
                (normalized.find('{'), normalized.find('}'))
            && close_brace > open_brace
            && let Some(from_idx) = normalized_lower[close_brace..].find("from")
        {
            let names_src = normalized[open_brace + 1..close_brace].trim();
            let from_section = &normalized[close_brace + from_idx + "from".len()..];
            if let Some(module_src) = extract_quoted_string(from_section) {
                for name in names_src.split(',') {
                    let trimmed = name.trim();
                    if !trimmed.is_empty() {
                        out.insert(trimmed.to_string(), module_src.clone());
                    }
                }
            }
        }

        search_start = abs_idx + 3 + end_rel + 2;
    }

    out
}

fn normalize_macro_import_body(body: &str) -> String {
    let mut normalized = String::new();
    for line in body.lines() {
        let mut trimmed = line.trim();
        if let Some(stripped) = trimmed.strip_prefix('*') {
            trimmed = stripped.trim();
        }
        if trimmed.is_empty() {
            continue;
        }
        if !normalized.is_empty() {
            normalized.push(' ');
        }
        normalized.push_str(trimmed);
    }
    normalized
}

fn extract_quoted_string(input: &str) -> Option<String> {
    for (idx, ch) in input.char_indices() {
        if ch == '"' || ch == '\'' {
            let start = idx + 1;
            let rest = &input[start..];
            if let Some(end) = rest.find(ch) {
                return Some(rest[..end].trim().to_string());
            }
            break;
        }
    }
    None
}

pub(super) fn external_type_function_import_patches(
    tokens: &str,
    import_sources: &HashMap<String, String>,
    extra_suffixes: &[String],
    extra_type_suffixes: &[String],
) -> Vec<Patch> {
    use convert_case::{Case, Casing};

    // Track needed imports: (ident, module_src) -> is_type_only
    let mut needed: std::collections::BTreeMap<(String, String), bool> = Default::default();

    for (type_name, module_src) in import_sources {
        if !type_name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase())
        {
            continue;
        }

        // For now, only add external imports for relative module specifiers.
        // This matches the common `./foo` / `../foo` patterns for sharing types across files.
        if !module_src.starts_with('.') && !module_src.starts_with('$') {
            continue;
        }

        // Strip generic parameters (e.g., "RecordLink<Employee>" -> "RecordLink")
        // before converting to camelCase, since `<>` are not valid in identifiers
        // and would produce broken function names like `recordLink<employee>Serialize`.
        let base_type = if let Some(idx) = type_name.find('<') {
            &type_name[..idx]
        } else {
            type_name.as_str()
        };
        let camel = base_type.to_case(Case::Camel);

        // Built-in suffixes from core macros (Default, Serialize, Deserialize, etc.)
        // These are always camelCase value references (function calls).
        let mut candidates: Vec<(String, bool)> = vec![
            (format!("{camel}SerializeWithContext"), false),
            (format!("{camel}DeserializeWithContext"), false),
            (format!("{camel}DefaultValue"), false),
            (format!("{camel}Serialize"), false),
            (format!("{camel}Deserialize"), false),
            (format!("{camel}ValidateField"), false),
            (format!("{camel}ValidateFields"), false),
            (format!("{camel}HasShape"), false),
        ];

        // Append camelCase suffixes registered by external macros via add_cross_module_suffix()
        for suffix in extra_suffixes {
            candidates.push((format!("{camel}{suffix}"), false));
        }

        // Append PascalCase type suffixes registered via add_cross_module_type_suffix()
        // These resolve {TypeName}{Suffix} references and generate `import type` statements.
        for suffix in extra_type_suffixes {
            candidates.push((format!("{base_type}{suffix}"), true));
        }

        for (ident, is_type) in candidates {
            if import_sources.contains_key(&ident) {
                continue;
            }
            if contains_identifier(tokens, &ident) {
                needed.insert((ident, module_src.clone()), is_type);
            }
        }
    }

    needed
        .into_iter()
        .map(|((ident, module_src), is_type)| {
            let keyword = if is_type { "import type" } else { "import" };
            Patch::InsertRaw {
                at: SpanIR::new(1, 1),
                code: format!("{keyword} {{ {ident} }} from \"{module_src}\";\n"),
                context: Some("import".to_string()),
                source_macro: None,
            }
        })
        .collect()
}

/// Check for imports of built-in macros and return warnings
/// Built-in macros like Debug, Clone, Serialize don't need to be imported
pub(super) fn check_builtin_import_warnings(module: &Module, _source: &str) -> Vec<Diagnostic> {
    use swc_core::ecma::ast::{ImportDecl, ImportSpecifier, ModuleDecl, ModuleItem};

    let mut warnings = Vec::new();

    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
            specifiers, src, ..
        })) = item
        {
            let module_source = src.value.to_string_lossy().to_string();

            // Only warn for imports that look like they're trying to import macros
            // e.g., from "macroforge", "@macroforge/core", or similar macro-related modules
            let is_macro_module = module_source.contains("macroforge")
                || module_source.contains("macro")
                || module_source == "@macro/derive";

            if !is_macro_module {
                continue;
            }

            for specifier in specifiers {
                let (local_name, import_span) = match specifier {
                    ImportSpecifier::Named(named) => (named.local.sym.to_string(), named.span),
                    ImportSpecifier::Default(default) => {
                        (default.local.sym.to_string(), default.span)
                    }
                    ImportSpecifier::Namespace(_) => continue,
                };

                // Check if this is a built-in macro name
                if BUILTIN_MACRO_NAMES.iter().any(|&name| name == local_name) {
                    let span_ir = SpanIR::new(
                        import_span.lo.0.saturating_sub(1),
                        import_span.hi.0.saturating_sub(1),
                    );

                    warnings.push(Diagnostic {
                        level: DiagnosticLevel::Warning,
                        message: format!(
                            "'{}' is a built-in macro and doesn't need to be imported",
                            local_name
                        ),
                        span: Some(span_ir),
                        notes: vec![],
                        help: Some(format!(
                            "Remove this import - just use @derive({}) directly in a JSDoc comment",
                            local_name
                        )),
                    });
                }
            }
        }
    }

    warnings
}
