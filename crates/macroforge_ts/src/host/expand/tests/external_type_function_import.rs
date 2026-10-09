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
