//! Registries kept for the process by id: an expansion against one names it
//! instead of sending its JSON, and must expand exactly as the JSON would.

use crate::api::CoreEngine;
use crate::api_types::ExpandOptions;
use crate::host::scanner::{ProjectScanner, ScanConfig};

fn options() -> ExpandOptions {
    ExpandOptions {
        keep_decorators: None,
        external_decorator_modules: None,
        config_path: None,
        type_registry_json: None,
        declarative_registry_json: None,
        type_registry_id: None,
        declarative_registry_id: None,
        build_mode: None,
        emit_metadata: None,
    }
}

const ADDRESS: &str =
    "/** @derive(Default) */\nexport interface Address {\n    street: string;\n}\n";
const USER: &str = "import type { Address } from './address';\n\n/** @derive(Default) */\nexport interface User {\n    home: Address;\n}\n";

/// A scanned two-file project whose `User` default depends on `Address`
/// through the registry.
fn registry_json() -> (tempfile::TempDir, String) {
    let dir = tempfile::tempdir().expect("a temporary directory");
    std::fs::write(dir.path().join("address.ts"), ADDRESS).expect("address.ts");
    std::fs::write(dir.path().join("user.ts"), USER).expect("user.ts");
    let scan = ProjectScanner::new(ScanConfig {
        root_dir: dir.path().to_path_buf(),
        ..ScanConfig::default()
    })
    .scan()
    .expect("the project scans");
    let json = serde_json::to_string(&scan.registry).expect("the registry encodes");
    (dir, json)
}

fn expand(dir: &tempfile::TempDir, options: ExpandOptions) -> Result<String, String> {
    let path = dir.path().join("user.ts").display().to_string();
    CoreEngine::expand_sync(USER.to_string(), path, Some(options)).map(|result| result.code)
}

#[test]
fn an_expansion_by_id_matches_one_by_json() {
    let (dir, json) = registry_json();
    let by_json = expand(
        &dir,
        ExpandOptions {
            type_registry_json: Some(json.clone()),
            ..options()
        },
    )
    .expect("expansion by JSON");
    let id = CoreEngine::set_type_registry(&json).expect("the registry is kept");
    let by_id = expand(
        &dir,
        ExpandOptions {
            type_registry_id: Some(id),
            ..options()
        },
    )
    .expect("expansion by id");
    assert_eq!(by_id, by_json);
    CoreEngine::release_registry(id).expect("the registry is released");
}

#[test]
fn naming_a_registry_both_ways_is_an_error() {
    let (dir, json) = registry_json();
    let id = CoreEngine::set_type_registry(&json).expect("the registry is kept");
    let error = expand(
        &dir,
        ExpandOptions {
            type_registry_id: Some(id),
            type_registry_json: Some(json),
            ..options()
        },
    )
    .expect_err("both forms at once are rejected");
    assert!(error.contains("not both"), "{error}");
    CoreEngine::release_registry(id).expect("the registry is released");
}

#[test]
fn a_released_or_mistyped_id_is_an_error() {
    let (dir, json) = registry_json();
    let declarative = CoreEngine::set_declarative_registry("{\"by_file\":{}}")
        .expect("the declarative registry is kept");
    let mistyped = expand(
        &dir,
        ExpandOptions {
            type_registry_id: Some(declarative),
            ..options()
        },
    )
    .expect_err("a declarative registry is not a type registry");
    assert!(mistyped.contains("declarative registry"), "{mistyped}");
    CoreEngine::release_registry(declarative).expect("the registry is released");

    let id = CoreEngine::set_type_registry(&json).expect("the registry is kept");
    CoreEngine::release_registry(id).expect("the registry is released");
    let released = expand(
        &dir,
        ExpandOptions {
            type_registry_id: Some(id),
            ..options()
        },
    )
    .expect_err("a released id names nothing");
    assert!(released.contains("no registry is kept"), "{released}");
    assert!(
        CoreEngine::release_registry(id).is_err(),
        "releasing twice is an error"
    );
}
