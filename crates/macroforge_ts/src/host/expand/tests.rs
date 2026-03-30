#[cfg(test)]
mod external_macro_loader_tests {
    use super::super::external_loader::ExternalMacroLoader;
    use crate::ts_syn::abi::{ClassIR, MacroContextIR, SpanIR};
    use std::{fs, path::Path};
    use tempfile::tempdir;

    fn write(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    fn test_class() -> ClassIR {
        ClassIR {
            name: "Temp".into(),
            span: SpanIR::new(0, 10),
            body_span: SpanIR::new(1, 9),
            is_abstract: false,
            type_params: vec![],
            heritage: vec![],
            decorators: vec![],
            decorators_ast: vec![],
            fields: vec![],
            methods: vec![],
            members: vec![],
        }
    }

    #[test]
    fn loads_esm_workspace_macro_via_dynamic_import() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        write(
            &root.join("package.json"),
            r#"{"name":"root","type":"module","workspaces":["packages/*"]}"#,
        );
        write(
            &root.join("packages/macro/package.json"),
            r#"{"name":"@ext/macro","type":"module","main":"index.js"}"#,
        );
        write(
            &root.join("packages/macro/index.js"),
            r#"
export function __macroforgeRunDebug(ctxJson) {
  return JSON.stringify({
    runtime_patches: [],
    type_patches: [],
    diagnostics: [],
    tokens: null,
    debug: null
  });
}
"#,
        );

        let loader = ExternalMacroLoader::new(root.to_path_buf());
        let ctx = MacroContextIR::new_derive_class(
            "Debug".into(),
            "@ext/macro".into(),
            SpanIR::new(0, 1),
            SpanIR::new(0, 1),
            "file.ts".into(),
            test_class(),
            "class Temp {}".into(),
        );

        let result = loader
            .run_macro(&ctx)
            .expect("should load ESM macro via dynamic import");

        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn prefers_nearest_node_modules_relative_to_file() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        // Create a nested project with its own node_modules containing the macro
        let nested_root = root.join("apps/app");
        let node_modules_macro = nested_root.join("node_modules/@ext/macro");
        fs::create_dir_all(&node_modules_macro).unwrap();

        // Node.js ESM resolution requires package.json with "type":"module"
        write(
            &nested_root.join("package.json"),
            r#"{"name":"app","type":"module"}"#,
        );

        write(
            &node_modules_macro.join("package.json"),
            r#"{"name":"@ext/macro","type":"module","main":"index.js"}"#,
        );
        write(
            &node_modules_macro.join("index.js"),
            r#"
export function __macroforgeRunDebug(ctxJson) {
  return JSON.stringify({
    runtime_patches: [],
    type_patches: [],
    diagnostics: [],
    tokens: null,
    debug: null
  });
}
"#,
        );

        let file_path = nested_root.join("src/file.ts");
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();

        let loader = ExternalMacroLoader::new(root.to_path_buf());
        let ctx = MacroContextIR::new_derive_class(
            "Debug".into(),
            "@ext/macro".into(),
            SpanIR::new(0, 1),
            SpanIR::new(0, 1),
            file_path.to_string_lossy().to_string(),
            test_class(),
            "class Temp {}".into(),
        );

        let result = loader
            .run_macro(&ctx)
            .expect("should load macro from nearest node_modules");

        assert!(result.diagnostics.is_empty());
    }
}

#[cfg(test)]
mod builtin_import_warning_tests {
    use super::super::imports::check_builtin_import_warnings;
    use crate::ts_syn::abi::DiagnosticLevel;
    use crate::ts_syn::parse_ts_module;

    #[test]
    fn warns_on_importing_debug_from_macroforge() {
        let source = r#"import { Debug } from "macroforge";

/** @derive(Debug) */
class User {
    name: string;
}"#;

        let module = parse_ts_module(source).unwrap();
        let warnings = check_builtin_import_warnings(&module, source);

        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].level, DiagnosticLevel::Warning);
        assert!(warnings[0].message.contains("Debug"));
        assert!(warnings[0].message.contains("built-in macro"));
        assert!(
            warnings[0]
                .help
                .as_ref()
                .unwrap()
                .contains("@derive(Debug)")
        );
    }

    #[test]
    fn warns_on_importing_serialize_from_macroforge_core() {
        let source = r#"import { Serialize, Deserialize } from "@macroforge/core";

/** @derive(Serialize, Deserialize) */
class User {
    name: string;
}"#;

        let module = parse_ts_module(source).unwrap();
        let warnings = check_builtin_import_warnings(&module, source);

        assert_eq!(warnings.len(), 2);
        assert!(warnings.iter().any(|w| w.message.contains("Serialize")));
        assert!(warnings.iter().any(|w| w.message.contains("Deserialize")));
    }

    #[test]
    fn warns_on_importing_clone_from_macro_derive() {
        let source = r#"import { Clone, Default, Hash } from "@macro/derive";

/** @derive(Clone, Default, Hash) */
class Config {
    value: number;
}"#;

        let module = parse_ts_module(source).unwrap();
        let warnings = check_builtin_import_warnings(&module, source);

        assert_eq!(warnings.len(), 3);
        assert!(warnings.iter().any(|w| w.message.contains("Clone")));
        assert!(warnings.iter().any(|w| w.message.contains("Default")));
        assert!(warnings.iter().any(|w| w.message.contains("Hash")));
    }

    #[test]
    fn no_warning_for_non_macro_imports() {
        let source = r#"import { Debug } from "my-custom-lib";
import { Clone } from "./local-utils";

class User {
    name: string;
}"#;

        let module = parse_ts_module(source).unwrap();
        let warnings = check_builtin_import_warnings(&module, source);

        // No warnings because imports are not from macro-related modules
        assert!(warnings.is_empty());
    }

    #[test]
    fn no_warning_for_custom_macro_imports() {
        let source = r#"import { MyCustomMacro } from "macroforge";

/** @derive(MyCustomMacro) */
class User {
    name: string;
}"#;

        let module = parse_ts_module(source).unwrap();
        let warnings = check_builtin_import_warnings(&module, source);

        // No warnings because MyCustomMacro is not a built-in
        assert!(warnings.is_empty());
    }

    #[test]
    fn warns_with_correct_span() {
        let source = r#"import { Debug } from "macroforge";"#;

        let module = parse_ts_module(source).unwrap();
        let warnings = check_builtin_import_warnings(&module, source);

        assert_eq!(warnings.len(), 1);
        let span = warnings[0].span.unwrap();
        // Span should point to "Debug" in the import statement
        let highlighted = &source[span.start as usize..span.end as usize];
        assert_eq!(highlighted, "Debug");
    }

    #[test]
    fn warns_all_ord_variants() {
        let source = r#"import { Ord, PartialOrd, PartialEq } from "macroforge";

/** @derive(Ord, PartialOrd, PartialEq) */
class Comparable {
    value: number;
}"#;

        let module = parse_ts_module(source).unwrap();
        let warnings = check_builtin_import_warnings(&module, source);

        assert_eq!(warnings.len(), 3);
        assert!(warnings.iter().any(|w| w.message.contains("Ord")));
        assert!(warnings.iter().any(|w| w.message.contains("PartialOrd")));
        assert!(warnings.iter().any(|w| w.message.contains("PartialEq")));
    }
}

#[cfg(test)]
mod external_type_function_import_tests {
    use super::super::imports::external_type_function_import_patches;
    use crate::ts_syn::abi::Patch;
    use std::collections::HashMap;

    fn make_import_sources(entries: &[(&str, &str)]) -> HashMap<String, String> {
        entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn generates_builtin_suffix_imports() {
        let tokens = "const val = companyNameDefaultValue();";
        let import_sources = make_import_sources(&[("CompanyName", "./account.svelte")]);

        let patches = external_type_function_import_patches(tokens, &import_sources, &[], &[]);

        assert_eq!(patches.len(), 1);
        match &patches[0] {
            Patch::InsertRaw { code, .. } => {
                assert!(code.contains("companyNameDefaultValue"));
                assert!(code.contains("./account.svelte"));
            }
            _ => panic!("Expected InsertRaw patch"),
        }
    }

    #[test]
    fn generates_extra_suffix_imports() {
        let tokens = "const fields = companyNameGetFields();";
        let import_sources = make_import_sources(&[("CompanyName", "./account.svelte")]);
        let extra = vec!["GetFields".to_string()];

        let patches = external_type_function_import_patches(tokens, &import_sources, &extra, &[]);

        assert_eq!(patches.len(), 1);
        match &patches[0] {
            Patch::InsertRaw { code, .. } => {
                assert!(code.contains("companyNameGetFields"));
                assert!(code.contains("./account.svelte"));
            }
            _ => panic!("Expected InsertRaw patch"),
        }
    }

    #[test]
    fn no_import_when_suffix_not_registered() {
        let tokens = "const fields = companyNameGetFields();";
        let import_sources = make_import_sources(&[("CompanyName", "./account.svelte")]);

        let patches = external_type_function_import_patches(tokens, &import_sources, &[], &[]);

        assert!(patches.is_empty());
    }

    #[test]
    fn skips_already_imported_identifiers() {
        let tokens = "const val = companyNameDefaultValue();";
        let import_sources = make_import_sources(&[
            ("CompanyName", "./account.svelte"),
            ("companyNameDefaultValue", "./account.svelte"),
        ]);

        let patches = external_type_function_import_patches(tokens, &import_sources, &[], &[]);

        assert!(patches.is_empty());
    }

    #[test]
    fn skips_non_relative_module_specifiers() {
        let tokens = "const val = companyNameDefaultValue();";
        let import_sources = make_import_sources(&[("CompanyName", "some-package")]);

        let patches = external_type_function_import_patches(tokens, &import_sources, &[], &[]);

        assert!(patches.is_empty());
    }

    #[test]
    fn multiple_extra_suffixes_all_resolve() {
        let tokens = r#"
            const a = companyNameGetFields();
            const b = companyNameCustomSuffix();
        "#;
        let import_sources = make_import_sources(&[("CompanyName", "./account.svelte")]);
        let extra = vec!["GetFields".to_string(), "CustomSuffix".to_string()];

        let patches = external_type_function_import_patches(tokens, &import_sources, &extra, &[]);

        assert_eq!(patches.len(), 2);
        let codes: Vec<String> = patches
            .iter()
            .map(|p| match p {
                Patch::InsertRaw { code, .. } => code.clone(),
                _ => panic!("Expected InsertRaw patch"),
            })
            .collect();
        assert!(codes.iter().any(|c| c.contains("companyNameGetFields")));
        assert!(codes.iter().any(|c| c.contains("companyNameCustomSuffix")));
    }

    #[test]
    fn extra_suffix_only_matches_when_referenced_in_tokens() {
        let tokens = "const val = companyNameDefaultValue();";
        let import_sources = make_import_sources(&[("CompanyName", "./account.svelte")]);
        let extra = vec!["GetFields".to_string()];

        let patches = external_type_function_import_patches(tokens, &import_sources, &extra, &[]);

        assert_eq!(patches.len(), 1);
        match &patches[0] {
            Patch::InsertRaw { code, .. } => {
                assert!(code.contains("companyNameDefaultValue"));
                assert!(!code.contains("companyNameGetFields"));
            }
            _ => panic!("Expected InsertRaw patch"),
        }
    }

    #[test]
    fn generates_pascal_case_type_imports() {
        // Tokens reference PascalCase type names like ColorsErrors, ColorsTainted
        let tokens = r#"
            let errors: ColorsErrors = {};
            let tainted: ColorsTainted = {};
        "#;
        let import_sources = make_import_sources(&[("Colors", "./shared.svelte")]);
        let type_suffixes = vec!["Errors".to_string(), "Tainted".to_string()];

        let patches =
            external_type_function_import_patches(tokens, &import_sources, &[], &type_suffixes);

        let codes: Vec<String> = patches
            .iter()
            .map(|p| match p {
                Patch::InsertRaw { code, .. } => code.clone(),
                _ => panic!("Expected InsertRaw patch"),
            })
            .collect();
        assert!(
            codes
                .iter()
                .any(|c| c.contains("import type { ColorsErrors }"))
        );
        assert!(
            codes
                .iter()
                .any(|c| c.contains("import type { ColorsTainted }"))
        );
    }

    #[test]
    fn pascal_case_type_imports_skip_already_imported() {
        let tokens = "let errors: ColorsErrors = {};";
        let import_sources = make_import_sources(&[
            ("Colors", "./shared.svelte"),
            ("ColorsErrors", "./shared.svelte"),
        ]);
        let type_suffixes = vec!["Errors".to_string()];

        let patches =
            external_type_function_import_patches(tokens, &import_sources, &[], &type_suffixes);

        assert!(!patches.iter().any(|p| match p {
            Patch::InsertRaw { code, .. } => code.contains("ColorsErrors"),
            _ => false,
        }));
    }

    #[test]
    fn mixed_camel_and_pascal_imports() {
        // camelCase function from extra_suffixes, PascalCase type from extra_type_suffixes
        let tokens = r#"
            const ctrl = colorsGetControllers(data, errors, tainted);
            let e: ColorsErrors = {};
        "#;
        let import_sources = make_import_sources(&[("Colors", "./shared.svelte")]);
        let extra = vec!["GetControllers".to_string()];
        let type_suffixes = vec!["Errors".to_string()];

        let patches =
            external_type_function_import_patches(tokens, &import_sources, &extra, &type_suffixes);

        let codes: Vec<String> = patches
            .iter()
            .map(|p| match p {
                Patch::InsertRaw { code, .. } => code.clone(),
                _ => panic!("Expected InsertRaw patch"),
            })
            .collect();
        // camelCase function -> value import
        assert!(
            codes
                .iter()
                .any(|c| c.contains("import { colorsGetControllers }"))
        );
        // PascalCase type -> type import
        assert!(
            codes
                .iter()
                .any(|c| c.contains("import type { ColorsErrors }"))
        );
    }
}
