mod builtin_import_warning_tests {
    use super::super::imports::check_builtin_import_warnings;
    use crate::host::MacroExpander;
    use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel};
    use oxc::allocator::Allocator;
    use oxc::parser::Parser;
    use oxc::span::SourceType;

    fn warnings_for(source: &str) -> Vec<Diagnostic> {
        let allocator = Allocator::default();
        let parsed = Parser::new(&allocator, source, SourceType::ts()).parse();
        assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
        check_builtin_import_warnings(&parsed.program)
    }

    #[test]
    fn warns_on_importing_debug_from_macroforge() {
        let source = r#"import { Debug } from "@macroforge/core";

/** @derive(Debug) */
class User {
    name: string;
}"#;

        let warnings = warnings_for(source);

        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].level, DiagnosticLevel::Warning);
        assert!(warnings[0].message.contains("Debug"));
        assert!(warnings[0].message.contains("built-in macro"));
        assert!(
            warnings[0]
                .help
                .as_deref()
                .is_some_and(|help| help.contains("@derive(Debug)"))
        );
    }

    #[test]
    fn warns_on_importing_encode_from_macroforge_core() {
        let source = r#"import { Encode, Decode } from "@macroforge/core";

/** @derive(Encode, Decode) */
class User {
    name: string;
}"#;

        let warnings = warnings_for(source);

        assert_eq!(warnings.len(), 2);
        assert!(
            warnings
                .iter()
                .any(|warning| warning.message.contains("Encode"))
        );
        assert!(
            warnings
                .iter()
                .any(|warning| warning.message.contains("Decode"))
        );
    }

    #[test]
    fn warns_on_importing_clone_from_macro_derive() {
        let source = r#"import { Clone, Default, Hash } from "@macro/derive";

/** @derive(Clone, Default, Hash) */
class Config {
    value: number;
}"#;

        let warnings = warnings_for(source);

        assert_eq!(warnings.len(), 3);
        assert!(
            warnings
                .iter()
                .any(|warning| warning.message.contains("Clone"))
        );
        assert!(
            warnings
                .iter()
                .any(|warning| warning.message.contains("Default"))
        );
        assert!(
            warnings
                .iter()
                .any(|warning| warning.message.contains("Hash"))
        );
    }

    #[test]
    fn no_warning_for_non_macro_imports() {
        let source = r#"import { Debug } from "my-custom-lib";
import { Clone } from "./local-utils";

class User {
    name: string;
}"#;

        assert!(warnings_for(source).is_empty());
    }

    #[test]
    fn no_warning_for_custom_macro_imports() {
        let source = r#"import { MyCustomMacro } from "@macroforge/core";

/** @derive(MyCustomMacro) */
class User {
    name: string;
}"#;

        assert!(warnings_for(source).is_empty());
    }

    #[test]
    fn warns_with_correct_span() {
        let source = r#"import { Debug } from "@macroforge/core";"#;

        let warnings = warnings_for(source);

        assert_eq!(warnings.len(), 1);
        let span = warnings[0].span.expect("warning should carry a span");
        assert_eq!(&source[span.start as usize..span.end as usize], "Debug");
    }

    #[test]
    fn warns_all_ord_variants() {
        let source = r#"import { Ord, PartialOrd, PartialEq } from "@macroforge/core";

/** @derive(Ord, PartialOrd, PartialEq) */
class Comparable {
    value: number;
}"#;

        let warnings = warnings_for(source);

        assert_eq!(warnings.len(), 3);
        assert!(
            warnings
                .iter()
                .any(|warning| warning.message.contains("Ord"))
        );
        assert!(
            warnings
                .iter()
                .any(|warning| warning.message.contains("PartialOrd"))
        );
        assert!(
            warnings
                .iter()
                .any(|warning| warning.message.contains("PartialEq"))
        );
    }

    #[test]
    fn expansion_reports_builtin_import_warning() {
        let source = r#"import { Debug } from "@macroforge/core";

/** @derive(Debug) */
class User {
    name: string;
}"#;

        let expansion = MacroExpander::new()
            .expect("expander should build")
            .expand_source(source, "user.ts")
            .expect("source should expand");

        assert!(
            expansion.diagnostics.iter().any(|diagnostic| {
                diagnostic.level == DiagnosticLevel::Warning
                    && diagnostic.message.contains("'Debug' is a built-in macro")
            }),
            "{:?}",
            expansion.diagnostics
        );
    }
}

mod external_type_function_import_tests {
    use super::super::imports::external_type_function_import_patches;
    use crate::host::import_registry::{clear_registry, with_registry};
    use std::collections::HashMap;

    fn make_import_sources(entries: &[(&str, &str)]) -> HashMap<String, String> {
        entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    /// Snapshot the registry's generated imports as `(local_name, module, is_type_only)`
    /// tuples so tests can assert against the registered set instead of inspecting
    /// patch text. The function under test now routes everything through the
    /// registry: its return value is intentionally always empty.
    fn snapshot_generated() -> Vec<(String, String, bool)> {
        with_registry(|r| {
            r.generated_imports()
                .map(|g| {
                    (
                        g.local_name.clone(),
                        g.source_module.clone(),
                        g.is_type_only,
                    )
                })
                .collect()
        })
    }

    #[test]
    fn generates_builtin_suffix_imports() {
        clear_registry();
        let tokens = "const val = companyNameDefaultValue();";
        let import_sources = make_import_sources(&[("CompanyName", "./account.svelte")]);

        let patches = external_type_function_import_patches(tokens, &import_sources, &[], &[]);
        assert!(patches.is_empty(), "registry path returns no patches");

        let generated = snapshot_generated();
        assert_eq!(generated.len(), 1);
        let (name, module, is_type) = &generated[0];
        assert_eq!(name, "companyNameDefaultValue");
        assert_eq!(module, "./account.svelte");
        assert!(!is_type);
    }

    #[test]
    fn generates_extra_suffix_imports() {
        clear_registry();
        let tokens = "const fields = companyNameGetFields();";
        let import_sources = make_import_sources(&[("CompanyName", "./account.svelte")]);
        let extra = vec!["GetFields".to_string()];

        let patches = external_type_function_import_patches(tokens, &import_sources, &extra, &[]);
        assert!(patches.is_empty());

        let generated = snapshot_generated();
        assert_eq!(generated.len(), 1);
        assert_eq!(generated[0].0, "companyNameGetFields");
        assert_eq!(generated[0].1, "./account.svelte");
    }

    #[test]
    fn no_import_when_suffix_not_registered() {
        clear_registry();
        let tokens = "const fields = companyNameGetFields();";
        let import_sources = make_import_sources(&[("CompanyName", "./account.svelte")]);

        let patches = external_type_function_import_patches(tokens, &import_sources, &[], &[]);
        assert!(patches.is_empty());
        assert!(snapshot_generated().is_empty());
    }

    #[test]
    fn skips_already_imported_identifiers() {
        clear_registry();
        let tokens = "const val = companyNameDefaultValue();";
        let import_sources = make_import_sources(&[
            ("CompanyName", "./account.svelte"),
            ("companyNameDefaultValue", "./account.svelte"),
        ]);

        let patches = external_type_function_import_patches(tokens, &import_sources, &[], &[]);
        assert!(patches.is_empty());
        // The identifier is already present in source imports, so the
        // registry should not be asked to generate it.
        let generated = snapshot_generated();
        assert!(
            generated
                .iter()
                .all(|(n, _, _)| n != "companyNameDefaultValue"),
            "should not register an identifier already imported in source",
        );
    }

    #[test]
    fn skips_non_relative_module_specifiers() {
        clear_registry();
        let tokens = "const val = companyNameDefaultValue();";
        let import_sources = make_import_sources(&[("CompanyName", "some-package")]);

        let patches = external_type_function_import_patches(tokens, &import_sources, &[], &[]);
        assert!(patches.is_empty());
        assert!(snapshot_generated().is_empty());
    }

    #[test]
    fn multiple_extra_suffixes_all_resolve() {
        clear_registry();
        let tokens = r#"
            const a = companyNameGetFields();
            const b = companyNameCustomSuffix();
        "#;
        let import_sources = make_import_sources(&[("CompanyName", "./account.svelte")]);
        let extra = vec!["GetFields".to_string(), "CustomSuffix".to_string()];

        let patches = external_type_function_import_patches(tokens, &import_sources, &extra, &[]);
        assert!(patches.is_empty());

        let names: Vec<String> = snapshot_generated()
            .into_iter()
            .map(|(n, _, _)| n)
            .collect();
        assert!(names.iter().any(|n| n == "companyNameGetFields"));
        assert!(names.iter().any(|n| n == "companyNameCustomSuffix"));
    }

    #[test]
    fn extra_suffix_only_matches_when_referenced_in_tokens() {
        clear_registry();
        let tokens = "const val = companyNameDefaultValue();";
        let import_sources = make_import_sources(&[("CompanyName", "./account.svelte")]);
        let extra = vec!["GetFields".to_string()];

        let patches = external_type_function_import_patches(tokens, &import_sources, &extra, &[]);
        assert!(patches.is_empty());

        let names: Vec<String> = snapshot_generated()
            .into_iter()
            .map(|(n, _, _)| n)
            .collect();
        assert!(names.iter().any(|n| n == "companyNameDefaultValue"));
        assert!(!names.iter().any(|n| n == "companyNameGetFields"));
    }

    #[test]
    fn generates_pascal_case_type_imports() {
        clear_registry();
        let tokens = r#"
            let errors: ColorsErrors = {};
            let tainted: ColorsTainted = {};
        "#;
        let import_sources = make_import_sources(&[("Colors", "./shared.svelte")]);
        let type_suffixes = vec!["Errors".to_string(), "Tainted".to_string()];

        let patches =
            external_type_function_import_patches(tokens, &import_sources, &[], &type_suffixes);
        assert!(patches.is_empty());

        let generated = snapshot_generated();
        assert!(
            generated
                .iter()
                .any(|(n, m, t)| n == "ColorsErrors" && m == "./shared.svelte" && *t)
        );
        assert!(
            generated
                .iter()
                .any(|(n, m, t)| n == "ColorsTainted" && m == "./shared.svelte" && *t)
        );
    }

    #[test]
    fn pascal_case_type_imports_skip_already_imported() {
        clear_registry();
        let tokens = "let errors: ColorsErrors = {};";
        let import_sources = make_import_sources(&[
            ("Colors", "./shared.svelte"),
            ("ColorsErrors", "./shared.svelte"),
        ]);
        let type_suffixes = vec!["Errors".to_string()];

        let patches =
            external_type_function_import_patches(tokens, &import_sources, &[], &type_suffixes);
        assert!(patches.is_empty());

        let generated = snapshot_generated();
        assert!(
            !generated.iter().any(|(n, _, _)| n == "ColorsErrors"),
            "should not register an identifier already imported in source",
        );
    }

    #[test]
    fn mixed_camel_and_pascal_imports() {
        clear_registry();
        let tokens = r#"
            const ctrl = colorsGetControllers(data, errors, tainted);
            let e: ColorsErrors = {};
        "#;
        let import_sources = make_import_sources(&[("Colors", "./shared.svelte")]);
        let extra = vec!["GetControllers".to_string()];
        let type_suffixes = vec!["Errors".to_string()];

        let patches =
            external_type_function_import_patches(tokens, &import_sources, &extra, &type_suffixes);
        assert!(patches.is_empty());

        let generated = snapshot_generated();
        assert!(
            generated
                .iter()
                .any(|(n, _, t)| n == "colorsGetControllers" && !t),
            "camelCase function should be a value import",
        );
        assert!(
            generated.iter().any(|(n, _, t)| n == "ColorsErrors" && *t),
            "PascalCase identifier should be a type-only import",
        );
    }
}

mod existing_declaration_tests {
    use super::super::helpers::has_existing_namespace_or_const;

    #[test]
    fn a_longer_name_first_does_not_hide_the_declaration() {
        let source = "export const FooBar = 1;\nexport const Foo = { a: 1 };\n";
        assert!(has_existing_namespace_or_const(source, "Foo"));
    }

    #[test]
    fn namespaces_and_typed_consts_count() {
        assert!(has_existing_namespace_or_const("namespace Foo {}", "Foo"));
        assert!(has_existing_namespace_or_const(
            "const Foo: Bar = x;",
            "Foo"
        ));
        assert!(has_existing_namespace_or_const(
            "export const\n  Foo = 1;",
            "Foo"
        ));
    }

    #[test]
    fn other_uses_of_the_name_do_not() {
        assert!(!has_existing_namespace_or_const("interface Foo {}", "Foo"));
        assert!(!has_existing_namespace_or_const("let x: Foo = y;", "Foo"));
        assert!(!has_existing_namespace_or_const("myconst Foo = 1;", "Foo"));
        assert!(!has_existing_namespace_or_const("const FooBar = 1;", "Foo"));
    }
}
