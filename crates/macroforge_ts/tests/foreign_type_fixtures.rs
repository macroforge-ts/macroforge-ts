//! Foreign-type fixtures: the expanded config each config produces.
//!
//! Each fixture under `tests/fixtures/foreign_types/<name>/` holds a
//! `macroforge.config.ts` and an `input.ts`. The fixture is copied into a
//! project that declares `#macroforge/config`, its config is expanded into
//! `.macroforge/config/` exactly as a sync does, and the snapshot records the
//! expanded `handlers.ts` and `handlers.js` beside the imports that expanding
//! `input.ts` adds. The `foreign_types_full` conformance test snapshots the
//! whole expansion of the same inputs.
//!
//! ```text
//! cargo test --test foreign_type_fixtures
//! cargo insta review
//! ```

use std::path::{Path, PathBuf};

use macroforge_ts::{
    ExpandOptions,
    api::CoreEngine,
    host::{
        clear_foreign_types,
        config::{
            MacroforgeConfigLoader, clear_config_cache,
            hoist::{EXPANDED_CONFIG_DIR, HANDLERS_STEM, sync_expanded_config},
        },
        import_registry::clear_registry,
    },
};

const MANIFEST: &str = r##"{
  "name": "fixture",
  "imports": {
    "#macroforge/config": {
      "types": "./.macroforge/config/handlers.ts",
      "default": "./.macroforge/config/handlers.js"
    }
  }
}
"##;

/// A fresh project holding the fixture's config and a manifest that maps
/// `#macroforge/config`.
fn project(fixture_name: &str, config: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "mf-foreign-type-fixture-{}-{fixture_name}",
        std::process::id()
    ));
    if root.exists() {
        std::fs::remove_dir_all(&root).expect("clear the fixture project");
    }
    std::fs::create_dir_all(&root).expect("create the fixture project");
    std::fs::write(root.join("package.json"), MANIFEST).expect("write the manifest");
    std::fs::write(root.join("macroforge.config.ts"), config).expect("write the config");
    root
}

fn read_expanded(root: &Path, extension: &str) -> String {
    let path = root
        .join(EXPANDED_CONFIG_DIR)
        .join(format!("{HANDLERS_STEM}.{extension}"));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn section(out: &mut String, title: &str, body: &str) {
    out.push_str(&format!("## {title}\n\n{}\n\n", body.trim()));
}

#[test]
fn foreign_type_fixtures() {
    insta::glob!(
        "fixtures/foreign_types/*/macroforge.config.ts",
        |config_path| {
            let fixture_dir = config_path.parent().expect("config has parent");
            let fixture_name = fixture_dir
                .file_name()
                .and_then(|name| name.to_str())
                .expect("fixture name");
            let config = std::fs::read_to_string(config_path).expect("read config");
            let input = std::fs::read_to_string(fixture_dir.join("input.ts")).expect("read input");

            let root = project(fixture_name, &config);
            let project_config = root.join("macroforge.config.ts");
            sync_expanded_config(&root).unwrap_or_else(|error| {
                panic!("expanding the {fixture_name} config failed: {error}")
            });
            let handlers_ts = read_expanded(&root, "ts");
            let handlers_js = read_expanded(&root, "js");

            let config_key = project_config.to_string_lossy().into_owned();
            clear_config_cache();
            clear_foreign_types();
            clear_registry();
            MacroforgeConfigLoader::load_and_cache(&config, &config_key).expect("config parses");
            let options = ExpandOptions {
                keep_decorators: None,
                external_decorator_modules: None,
                config_path: Some(config_key),
                type_registry_json: None,
                declarative_registry_json: None,
                type_registry_id: None,
                declarative_registry_id: None,
                build_mode: None,
                emit_metadata: None,
            };
            let result = CoreEngine::expand_sync(input, "input.ts".to_string(), Some(options))
                .unwrap_or_else(|error| panic!("expand_sync failed for {fixture_name}: {error}"));
            clear_config_cache();
            clear_foreign_types();
            clear_registry();
            std::fs::remove_dir_all(&root).expect("remove the fixture project");

            let imports: Vec<&str> = result
                .code
                .lines()
                .filter(|line| line.starts_with("import "))
                .collect();
            let diagnostics: Vec<String> = result
                .diagnostics
                .iter()
                .map(|diagnostic| format!("{}: {}", diagnostic.level, diagnostic.message))
                .collect();

            let mut snapshot = String::new();
            section(&mut snapshot, "handlers.ts", &handlers_ts);
            section(&mut snapshot, "handlers.js", &handlers_js);
            section(
                &mut snapshot,
                "Imports of the expanded input",
                &imports.join("\n"),
            );
            let diagnostics = if diagnostics.is_empty() {
                "(none)".to_string()
            } else {
                diagnostics.join("\n")
            };
            section(&mut snapshot, "Diagnostics", &diagnostics);

            insta::with_settings!({
                snapshot_path => fixture_dir.join("snapshots"),
                prepend_module_to_snapshot => false,
                description => fixture_name,
            }, {
                insta::assert_snapshot!(fixture_name, snapshot);
            });
        }
    );
}
