use std::path::Path;

use super::*;

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
fn test_is_in_skip_dir() {
    let scanner = ProjectScanner::new(ScanConfig::default());
    assert!(scanner.is_in_skip_dir(Path::new("/project/node_modules/foo/bar.ts")));
    assert!(scanner.is_in_skip_dir(Path::new("/project/dist/main.ts")));
    assert!(!scanner.is_in_skip_dir(Path::new("/project/src/models/user.ts")));
}

#[test]
fn test_collect_exported_names() {
    use swc_core::common::{GLOBALS, Globals};

    let source = r#"
        export class User { name: string = ""; }
        export interface Config { key: string; }
        class Internal {}
        export enum Status { Active, Inactive }
        export type ID = string;
    "#;

    let globals = Globals::default();
    GLOBALS.set(&globals, || {
        let module =
            crate::ts_syn::parse::parse_ts_module(source, "test.ts").expect("parse failed");
        let names = collect_exported_names(&module);
        assert!(names.contains("User"));
        assert!(names.contains("Config"));
        assert!(names.contains("Status"));
        assert!(names.contains("ID"));
        assert!(!names.contains("Internal"));
    });
}

#[test]
fn test_scanner_svelte_ts_interface_with_derive() {
    use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};
    use swc_core::common::{GLOBALS, Globals};

    // Simulate scanning a .svelte.ts file with a JSDoc @derive on an interface
    let source = r#"/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface PhoneNumber {
    label: string;
    number: string;
}"#;

    let globals = Globals::default();
    GLOBALS.set(&globals, || {
        let module = crate::ts_syn::parse::parse_ts_module(source, "phone-number.svelte.ts")
            .expect("parse failed");

        let interfaces =
            crate::ts_syn::lower_interfaces(&module, source, None).unwrap_or_default();
        assert_eq!(interfaces.len(), 1, "Should find one interface");

        let iface = &interfaces[0];
        assert_eq!(iface.name, "PhoneNumber");

        // Should have the Derive decorator
        let derive = iface.decorators.iter().find(|d| d.name == "Derive");
        assert!(
            derive.is_some(),
            "Interface should have @derive decorator, got: {:?}",
            iface.decorators
        );
        assert!(derive.unwrap().args_src.contains("Gigaform"));

        // Insert into registry and verify type_has_derive works
        let mut registry = TypeRegistry::new();
        let entry = TypeRegistryEntry {
            name: iface.name.clone(),
            file_path: "phone-number.svelte.ts".to_string(),
            is_exported: true,
            definition: TypeDefinitionIR::Interface(iface.clone()),
            file_imports: vec![],
        };
        registry.insert(entry, "");

        assert!(
            crate::builtin::derive_common::type_has_derive(
                &registry,
                "PhoneNumber",
                "Gigaform"
            ),
            "type_has_derive should return true for Gigaform"
        );
        assert!(
            crate::builtin::derive_common::type_has_derive(&registry, "PhoneNumber", "Default"),
            "type_has_derive should return true for Default"
        );
    });
}

#[test]
fn test_collect_file_imports() {
    use swc_core::common::{GLOBALS, Globals};

    let source = r#"
        import { User, type Config } from "./models";
        import type { Status } from "./status";
        import DefaultExport from "./default";
    "#;

    let globals = Globals::default();
    GLOBALS.set(&globals, || {
        let module =
            crate::ts_syn::parse::parse_ts_module(source, "test.ts").expect("parse failed");
        let imports = collect_file_imports(&module);

        assert_eq!(imports.len(), 4);

        let user_import = imports.iter().find(|i| i.local_name == "User").unwrap();
        assert_eq!(user_import.module_specifier, "./models");
        assert!(!user_import.is_type_only);

        let config_import = imports.iter().find(|i| i.local_name == "Config").unwrap();
        assert!(config_import.is_type_only);

        let status_import = imports.iter().find(|i| i.local_name == "Status").unwrap();
        assert!(status_import.is_type_only);

        let default_import = imports
            .iter()
            .find(|i| i.local_name == "DefaultExport")
            .unwrap();
        assert_eq!(default_import.original_name.as_deref(), Some("default"));
    });
}
