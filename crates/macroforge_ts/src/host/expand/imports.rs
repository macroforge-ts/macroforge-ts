use std::collections::HashMap;

use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, SpanIR};

use super::helpers::Identifiers;

/// Built-in derive macros, available everywhere without an import.
const BUILTIN_MACRO_NAMES: &[&str] = &[
    "Debug",
    "Clone",
    "Default",
    "Hash",
    "Eq",
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

/// Requests the import of each function of another module's type that
/// `tokens` calls. The generated code names such a function after the type
/// as the file names it; `exported_names` gives the name its module exports
/// it under where that differs (`import { User as Member }`), which is the
/// name its functions carry there. `project_types` names the imported types
/// the project declares: theirs are imported however the module is written
/// (`@/types/user`), where a package's type has none to import.
pub(super) fn external_type_function_import_patches(
    tokens: &str,
    import_sources: &HashMap<String, String>,
    exported_names: &HashMap<String, String>,
    project_types: &std::collections::HashSet<String>,
    extra_suffixes: &[String],
    extra_type_suffixes: &[String],
) -> Vec<Patch> {
    use crate::builtin::derive::common::function_prefix;

    let identifiers = Identifiers::of(tokens);

    // Track needed imports: (ident, module_src) -> (the name it is exported
    // under, is_type_only)
    let mut needed: std::collections::BTreeMap<(String, String), (String, bool)> =
        Default::default();

    for (type_name, module_src) in import_sources {
        // A relative or `$` specifier names a module of the project. Any
        // other may name a package, unless the project is known to declare
        // the type it brings in.
        if !module_src.starts_with('.')
            && !module_src.starts_with('$')
            && !project_types.contains(type_name)
        {
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
        let camel = function_prefix(base_type);
        let exported_type = exported_names
            .get(base_type)
            .map_or(base_type, String::as_str);
        let exported_camel = function_prefix(exported_type);

        // Built-in suffixes from core macros (Default, Encode, Decode, etc.)
        // These are always camelCase value references (function calls).
        const SUFFIXES: [&str; 18] = [
            "EncodeWithContext",
            "EncodePartialWithContext",
            "DecodeWithContext",
            "DecodePartialWithContext",
            "DefaultValue",
            "Encode",
            "Decode",
            "FromString",
            "ValidateField",
            "ValidateFields",
            "HasShape",
            "Is",
            "ToString",
            "Clone",
            "Equals",
            "HashCode",
            "PartialCompare",
            "Compare",
        ];
        // Each candidate as the generated code names it, as its module
        // exports it, and whether it is a type.
        let mut candidates: Vec<(String, String, bool)> = SUFFIXES
            .iter()
            .map(|suffix| {
                (
                    format!("{camel}{suffix}"),
                    format!("{exported_camel}{suffix}"),
                    false,
                )
            })
            .collect();

        // Append camelCase suffixes registered by external macros via add_cross_module_suffix()
        for suffix in extra_suffixes {
            candidates.push((
                format!("{camel}{suffix}"),
                format!("{exported_camel}{suffix}"),
                false,
            ));
        }

        // Append PascalCase type suffixes registered via add_cross_module_type_suffix()
        // These resolve {TypeName}{Suffix} references and generate `import type` statements.
        for suffix in extra_type_suffixes {
            candidates.push((
                format!("{base_type}{suffix}"),
                format!("{exported_type}{suffix}"),
                true,
            ));
        }

        for (ident, exported, is_type) in candidates {
            if import_sources.contains_key(&ident) {
                continue;
            }
            if identifiers.contains(&ident) {
                needed.insert((ident, module_src.clone()), (exported, is_type));
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
        for ((ident, module_src), (exported, is_type)) in &needed {
            let exported = (exported != ident).then_some(exported.as_str());
            r.request_import(ident, exported, module_src, *is_type);
        }
    });

    Vec::new()
}
