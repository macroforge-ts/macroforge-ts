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
fn a_module_exports_a_declaration_under_other_names() {
    let source = r#"
        export default class User { name: string = ""; }
        interface Late { note: string; }
        class Held {}
        export type { Late as Shown };
        export { Held as Kept, Held };
        export default Held;
        export { Elsewhere as Renamed } from './elsewhere';
    "#;

    let allocator = Allocator::default();
    let mut aliases = crate::ts_syn::collect_export_aliases(&parse(&allocator, source, "test.ts"));
    aliases.sort();
    let named = |declared: &str, exported: &str| (declared.to_string(), exported.to_string());
    assert_eq!(
        aliases,
        vec![
            named("Held", "Kept"),
            named("Held", "default"),
            named("Late", "Shown"),
            named("User", "default"),
        ]
    );
}

#[test]
fn a_module_says_what_it_exports_again() {
    let source = r#"
        export * from './user';
        export { default as Chosen, User as Member } from './user';
        export type { Late } from './late';
        export * as space from './space';
        export { own };
        import Picked, { Brought, Brought as Local } from './brought';
        import * as whole from './whole';
        export { Brought, Local as Sent, whole };
        export default Picked;
    "#;

    let allocator = Allocator::default();
    let again: Vec<(String, String, String)> =
        crate::ts_syn::collect_re_exports(&parse(&allocator, source, "test.ts"))
            .into_iter()
            .map(|again| (again.module_specifier, again.imported, again.exported))
            .collect();
    let named = |module: &str, imported: &str, exported: &str| {
        (
            module.to_string(),
            imported.to_string(),
            exported.to_string(),
        )
    };
    assert_eq!(
        again,
        vec![
            named("./user", "*", "*"),
            named("./user", "default", "Chosen"),
            named("./user", "User", "Member"),
            named("./late", "Late", "Late"),
            named("./brought", "Brought", "Brought"),
            named("./brought", "Brought", "Sent"),
            named("./brought", "default", "default"),
        ]
    );
}

/// A type is found through the barrels that export it again, under the name
/// each gives it, and a barrel that exports all of a module brings its
/// generated functions where one that names the type does not.
#[test]
fn an_import_is_followed_through_the_modules_that_export_it_again() {
    use crate::ts_syn::abi::ir::type_registry::FileImportEntry;

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or_default();
    let root = std::env::temp_dir().join(format!("macroforge_barrels_{nanos}"));
    std::fs::create_dir_all(root.join("models")).expect("create fixture dir");
    let write = |name: &str, source: &str| {
        std::fs::write(root.join(name), source).expect("write source");
    };
    write(
        "models/user.ts",
        "export interface User { id: string; }\nexport default class Model { id = ''; }\n",
    );
    write(
        "models/index.ts",
        "export * from './user';\nexport { default as Chosen, User as Member } from './user';\n",
    );
    write(
        "all.ts",
        "export * from './models';\nexport * from './round';\n",
    );
    write(
        "round.ts",
        "export * from './all';\nexport interface Round { id: string; }\n",
    );
    write(
        "passed.ts",
        "import Model, { User as Known } from './models/user';\nexport { Known as Handed };\nexport default Model;\n",
    );
    // A name the project declares once elsewhere, which no import below leads to.
    write("other.ts", "export interface Member { id: string; }\n");

    let output = ProjectScanner::with_root(root.clone())
        .scan()
        .expect("scan");
    let page = root.join("page.ts").to_string_lossy().to_string();
    let import = |local: &str, exported: Option<&str>, module: &str| FileImportEntry {
        local_name: local.to_string(),
        module_specifier: module.to_string(),
        original_name: exported.map(str::to_string),
        is_type_only: true,
    };
    let resolve = |name: &str, imports: &[FileImportEntry]| {
        output
            .registry
            .resolve_in_file(name, &page, imports)
            .map(|entry| entry.name.clone())
    };

    // `export *` keeps the name, and a named re-export gives another.
    assert_eq!(
        resolve("User", &[import("User", None, "./models")]),
        Some("User".to_string())
    );
    assert_eq!(
        resolve("Member", &[import("Member", None, "./models")]),
        Some("User".to_string())
    );
    assert_eq!(
        resolve("Chosen", &[import("Chosen", None, "./models")]),
        Some("Model".to_string())
    );
    // Through a second barrel, and renamed once more by the import.
    assert_eq!(
        resolve("Seen", &[import("Seen", Some("Member"), "./all")]),
        Some("User".to_string())
    );
    assert_eq!(
        resolve("Chosen", &[import("Chosen", None, "./all")]),
        Some("Model".to_string())
    );
    // A module that imports a type and then exports it.
    assert_eq!(
        resolve("Handed", &[import("Handed", None, "./passed")]),
        Some("User".to_string())
    );
    assert_eq!(
        resolve("Mine", &[import("Mine", Some("default"), "./passed")]),
        Some("Model".to_string())
    );
    // Two modules that export each other end, with what each declares.
    assert_eq!(
        resolve("Round", &[import("Round", None, "./all")]),
        Some("Round".to_string())
    );

    let user = output.registry.get("User").expect("User is scanned");
    let again = |file: &str, name: &str| {
        user.re_exported
            .iter()
            .find(|again| again.file_path.ends_with(file) && again.name == name)
            .map(|again| again.whole)
    };
    assert_eq!(again("models/index.ts", "User"), Some(true));
    assert_eq!(again("models/index.ts", "Member"), Some(false));
    assert_eq!(again("all.ts", "User"), Some(true));
    assert_eq!(again("all.ts", "Member"), Some(false));

    std::fs::remove_dir_all(&root).expect("remove fixture");
}

/// Barrels that reach one module by many ways are read once a round: forty
/// layers of two, each exporting both of the layer below, are a million
/// million ways down, and are linked as fast as forty files.
#[test]
fn barrels_that_reach_a_module_by_many_ways_are_linked_once() {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or_default();
    let root = std::env::temp_dir().join(format!("macroforge_diamond_{nanos}"));
    std::fs::create_dir_all(&root).expect("create fixture dir");
    let layers = 40;
    std::fs::write(
        root.join("left0.ts"),
        "export interface Deep { id: string; }\n",
    )
    .expect("write source");
    std::fs::write(
        root.join("right0.ts"),
        "export interface Wide { id: string; }\n",
    )
    .expect("write source");
    for layer in 1..=layers {
        let below = layer - 1;
        let source = format!("export * from './left{below}';\nexport * from './right{below}';\n");
        for side in ["left", "right"] {
            std::fs::write(root.join(format!("{side}{layer}.ts")), &source).expect("write source");
        }
    }

    let output = ProjectScanner::with_root(root.clone())
        .scan()
        .expect("scan");
    let deep = output.registry.get("Deep").expect("Deep is scanned");
    assert_eq!(deep.re_exported.len(), layers * 2);
    assert!(
        deep.re_exported
            .iter()
            .all(|again| again.whole && again.name == "Deep")
    );

    std::fs::remove_dir_all(&root).expect("remove fixture");
}

#[test]
fn test_scanner_svelte_ts_interface_with_derive() {
    use crate::ts_syn::abi::ir::type_registry::{
        TypeDefinitionIR, TypeRegistry, TypeRegistryEntry,
    };

    // A .svelte.ts file with a JSDoc @derive on an interface.
    let source = r#"/** @derive(Default, Encode, Decode, Gigaform) */
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
            exported_as: Vec::new(),
            re_exported: Vec::new(),
            definition: TypeDefinitionIR::Interface(iface.clone()),
            file_imports: vec![],
        },
        "",
    );

    assert!(
        crate::builtin::derive::common::type_has_derive(&registry, "PhoneNumber", "Gigaform"),
        "type_has_derive should return true for Gigaform"
    );
    assert!(
        crate::builtin::derive::common::type_has_derive(&registry, "PhoneNumber", "Default"),
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
