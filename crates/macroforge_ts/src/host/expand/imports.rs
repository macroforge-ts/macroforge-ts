use std::collections::HashMap;

use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, SpanIR};

use super::helpers::Identifiers;

/// Built-in derive macros, available everywhere without an import.
const BUILTIN_MACRO_NAMES: &[&str] = &[
    "Debug",
    "Clone",
    "Default",
    "Hash",
    "Ord",
    "PartialEq",
    "PartialOrd",
    "Encode",
    "Decode",
];

/// Built-in type-position call macros, available everywhere without an import.
const BUILTIN_CALL_MACRO_NAMES: &[&str] = &["$Newtype"];

/// Warns on each built-in macro imported from a macro module: built-ins need
/// no import, so the import is dead and suggests otherwise.
pub(super) fn check_builtin_import_warnings(
    program: &oxc::ast::ast::Program<'_>,
) -> Vec<Diagnostic> {
    use oxc::ast::ast::{ImportDeclarationSpecifier, Statement};

    let mut warnings = Vec::new();
    for statement in &program.body {
        let Statement::ImportDeclaration(import) = statement else {
            continue;
        };
        let module_source = import.source.value.as_str();
        if !module_source.contains(crate::package::PACKAGE) && !module_source.contains("macro") {
            continue;
        }
        let Some(specifiers) = &import.specifiers else {
            continue;
        };
        for specifier in specifiers {
            let (local_name, span) = match specifier {
                ImportDeclarationSpecifier::ImportSpecifier(named) => {
                    (named.local.name.as_str(), named.span)
                }
                ImportDeclarationSpecifier::ImportDefaultSpecifier(default) => {
                    (default.local.name.as_str(), default.span)
                }
                ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => continue,
            };
            let help = if BUILTIN_MACRO_NAMES.contains(&local_name) {
                format!(
                    "Remove this import - just use @derive({local_name}) directly in a JSDoc comment"
                )
            } else if BUILTIN_CALL_MACRO_NAMES.contains(&local_name) {
                format!("Remove this import - just write {local_name}<T> in type position")
            } else {
                continue;
            };
            warnings.push(Diagnostic {
                level: DiagnosticLevel::Warning,
                message: format!(
                    "'{local_name}' is a built-in macro and doesn't need to be imported"
                ),
                span: Some(SpanIR::new(span.start + 1, span.end + 1)),
                notes: vec![],
                help: Some(help),
            });
        }
    }
    warnings
}

pub(super) fn external_type_function_import_patches(
    tokens: &str,
    import_sources: &HashMap<String, String>,
    extra_suffixes: &[String],
    extra_type_suffixes: &[String],
) -> Vec<Patch> {
    use convert_case::{Case, Casing};

    let identifiers = Identifiers::of(tokens);

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
        // and would produce broken function names like `recordLink<employee>Encode`.
        let base_type = if let Some(idx) = type_name.find('<') {
            &type_name[..idx]
        } else {
            type_name.as_str()
        };
        let camel = base_type.to_case(Case::Camel);

        // Built-in suffixes from core macros (Default, Encode, Decode, etc.)
        // These are always camelCase value references (function calls).
        let mut candidates: Vec<(String, bool)> = vec![
            (format!("{camel}EncodeWithContext"), false),
            (format!("{camel}DecodeWithContext"), false),
            (format!("{camel}DefaultValue"), false),
            (format!("{camel}Encode"), false),
            (format!("{camel}Decode"), false),
            (format!("{camel}ValidateField"), false),
            (format!("{camel}ValidateFields"), false),
            (format!("{camel}HasShape"), false),
            (format!("{camel}Is"), false),
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
            if identifiers.contains(&ident) {
                needed.insert((ident, module_src.clone()), is_type);
            }
        }
    }

    // Register each needed import in the global registry. The registry is a
    // HashMap keyed by `local_name`, so re-registration across multiple
    // derive-macro dispatches against the same target file is naturally
    // idempotent. The registry's `emit_generated_imports()` call at the end
    // of expansion prepends them to the runtime output exactly once. We
    // return zero patches because emitting a `Patch::InsertRaw` here in
    // addition to the registry path would land the same identifier in the
    // file twice and break downstream parsers on re-declaration.
    crate::host::import_registry::with_registry_mut(|r| {
        for ((ident, module_src), is_type) in &needed {
            if *is_type {
                r.request_type_import(ident, module_src);
            } else {
                r.request_import(ident, None, module_src, false);
            }
        }
    });

    Vec::new()
}
