use oxc::allocator::Allocator;
use oxc::ast::ast::Program;
use oxc::parser::Parser;
use oxc::span::SourceType;

use super::*;
use crate::ts_syn::{collect_exported_names, collect_file_imports, lower_interfaces};

#[test]
fn test_scan_config_defaults() {
    let config = ScanConfig::default();
    assert_eq!(config.extensions, vec![".ts", ".tsx"]);
    assert!(config.skip_dirs.contains("node_modules"));
    assert!(config.skip_dirs.contains("dist"));
    assert!(!config.exported_only);
    assert_eq!(config.max_files, 10_000);
}

#[test]
fn skipped_directories_are_skipped_below_the_root_only() {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or_default();
    // A root that itself sits under a `build` directory still scans.
    let root = std::env::temp_dir().join(format!("macroforge_skip_{nanos}/build/project"));
    for dir in ["src", "node_modules/dep", "dist"] {
        std::fs::create_dir_all(root.join(dir)).expect("create fixture dir");
    }
    std::fs::write(
        root.join("src/user.ts"),
        "export interface User { id: string; }\n",
    )
    .expect("write source");
    std::fs::write(
        root.join("node_modules/dep/index.ts"),
        "export interface Dep { id: string; }\n",
    )
    .expect("write dependency");
    std::fs::write(
        root.join("dist/out.ts"),
        "export interface Out { id: string; }\n",
    )
    .expect("write output");

    let output = ProjectScanner::with_root(root.clone())
        .scan()
        .expect("scan");
    assert_eq!(output.files_scanned, 1);
    assert!(output.registry.get("User").is_some());
    assert!(output.registry.get("Dep").is_none());
    assert!(output.registry.get("Out").is_none());

    let top = root
        .ancestors()
        .nth(2)
        .expect("the fixture root has a temp parent");
    std::fs::remove_dir_all(top).expect("remove fixture");
}

fn parse<'a>(allocator: &'a Allocator, source: &'a str, file_name: &str) -> Program<'a> {
    let source_type = SourceType::from_path(file_name).expect("test file names are TypeScript");
    let parsed = Parser::new(allocator, source, source_type).parse();
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    parsed.program
}

#[test]
fn test_collect_exported_names() {
    let source = r#"
        export class User { name: string = ""; }
        export interface Config { key: string; }
        class Internal {}
        export enum Status { Active, Inactive }
        export type ID = string;
    "#;

    let allocator = Allocator::default();
    let names = collect_exported_names(&parse(&allocator, source, "test.ts"));
    assert!(names.contains("User"));
    assert!(names.contains("Config"));
    assert!(names.contains("Status"));
    assert!(names.contains("ID"));
    assert!(!names.contains("Internal"));
}

#[test]
fn test_scanner_svelte_ts_interface_with_derive() {
    use crate::ts_syn::abi::ir::type_registry::{
        TypeDefinitionIR, TypeRegistry, TypeRegistryEntry,
    };

    // A .svelte.ts file with a JSDoc @derive on an interface.
    let source = r#"/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface PhoneNumber {
    label: string;
    number: string;
}"#;

    let allocator = Allocator::default();
    let program = parse(&allocator, source, "phone-number.svelte.ts");
    let interfaces = lower_interfaces(&program, source, None).expect("interfaces should lower");
    assert_eq!(interfaces.len(), 1, "Should find one interface");

    let iface = &interfaces[0];
    assert_eq!(iface.name, "PhoneNumber");

    let derive = iface
        .decorators
        .iter()
        .find(|decorator| decorator.name == "Derive")
        .unwrap_or_else(|| {
            panic!(
                "Interface should have @derive decorator, got: {:?}",
                iface.decorators
            )
        });
    assert!(derive.args_src.contains("Gigaform"));

    let mut registry = TypeRegistry::new();
    registry.insert(
        TypeRegistryEntry {
            name: iface.name.clone(),
            file_path: "phone-number.svelte.ts".to_string(),
            is_exported: true,
            definition: TypeDefinitionIR::Interface(iface.clone()),
            file_imports: vec![],
        },
        "",
    );

    assert!(
        crate::builtin::derive_common::type_has_derive(&registry, "PhoneNumber", "Gigaform"),
        "type_has_derive should return true for Gigaform"
    );
    assert!(
        crate::builtin::derive_common::type_has_derive(&registry, "PhoneNumber", "Default"),
        "type_has_derive should return true for Default"
    );
}

#[test]
fn test_collect_file_imports() {
    let source = r#"
        import { User, type Config } from "./models";
        import type { Status } from "./status";
        import DefaultExport from "./default";
    "#;

    let allocator = Allocator::default();
    let imports = collect_file_imports(&parse(&allocator, source, "test.ts"));

    assert_eq!(imports.len(), 4);
    let find = |name: &str| {
        imports
            .iter()
            .find(|import| import.local_name == name)
            .unwrap_or_else(|| panic!("{name} should be imported"))
    };

    let user_import = find("User");
    assert_eq!(user_import.module_specifier, "./models");
    assert!(!user_import.is_type_only);
    assert!(find("Config").is_type_only);
    assert!(find("Status").is_type_only);
    assert_eq!(
        find("DefaultExport").original_name.as_deref(),
        Some("default")
    );
}
