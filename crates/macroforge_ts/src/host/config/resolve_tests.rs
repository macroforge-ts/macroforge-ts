use std::fs;
use std::path::Path;

use super::MacroforgeConfigLoader;
use crate::host::MacroError;

/// Writes `files` under a fresh directory and returns it.
fn project(files: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("a temporary directory");
    for (path, content) in files {
        let full = dir.path().join(path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).expect("the fixture directory");
        }
        fs::write(&full, content).expect("the fixture file");
    }
    dir
}

fn load(dir: &Path, name: &str) -> Result<super::MacroforgeConfig, MacroError> {
    let path = dir.join(name);
    let content = fs::read_to_string(&path).expect("the config file");
    MacroforgeConfigLoader::from_config_file_with_dependencies(
        &content,
        path.to_string_lossy().as_ref(),
    )
    .map(|(config, _)| config)
}

fn foreign_type_names(config: &super::MacroforgeConfig) -> Vec<&str> {
    config
        .foreign_types
        .iter()
        .map(|foreign_type| foreign_type.name.as_str())
        .collect()
}

const BASE: &str = r#"
    import { DateTime } from "effect";
    export default {
        keepDecorators: true,
        foreignTypes: {
            "DateTime.DateTime": { from: ["effect"], encode: (v) => DateTime.formatIso(v) },
            "Duration.Duration": { from: ["effect"] },
        },
    };
"#;

#[test]
fn a_config_that_exports_an_imported_base_is_that_base() {
    let dir = project(&[
        ("shared/base.config.ts", BASE),
        (
            "macroforge.config.ts",
            "import base from './shared/base.config';\nexport default base;\n",
        ),
    ]);
    let config = load(dir.path(), "macroforge.config.ts").expect("the config resolves");
    assert!(config.keep_decorators);
    assert_eq!(
        foreign_type_names(&config),
        ["DateTime.DateTime", "Duration.Duration"]
    );
    let sites = &config
        .foreign_types
        .iter()
        .find(|foreign_type| foreign_type.name == "DateTime.DateTime")
        .expect("the base's foreign type")
        .handler_sites;
    assert!(
        !sites.is_empty()
            && sites
                .iter()
                .all(|site| site.module.ends_with("shared/base.config.ts")),
        "the base's handlers are hoisted from the base, where their imports are: {sites:?}"
    );
}

#[test]
fn a_spread_base_is_overridden_by_the_fields_after_it() {
    let dir = project(&[
        ("base.config.ts", BASE),
        (
            "macroforge.config.ts",
            r#"import base from "./base.config.ts";
            export default { ...base, keepDecorators: false };"#,
        ),
    ]);
    let config = load(dir.path(), "macroforge.config.ts").expect("the config resolves");
    assert!(!config.keep_decorators);
    assert_eq!(config.foreign_types.len(), 2);
}

#[test]
fn a_spread_replaces_foreign_types_wholesale_as_in_javascript() {
    let dir = project(&[
        ("base.config.ts", BASE),
        (
            "macroforge.config.ts",
            r#"import base from "./base.config.ts";
            export default { ...base, foreignTypes: { "Option.Option": { from: ["effect"] } } };"#,
        ),
    ]);
    let config = load(dir.path(), "macroforge.config.ts").expect("the config resolves");
    assert_eq!(foreign_type_names(&config), ["Option.Option"]);
}

#[test]
fn extends_merges_foreign_types_by_name() {
    let dir = project(&[
        ("base.config.ts", BASE),
        (
            "macroforge.config.ts",
            r#"export default {
                extends: "./base.config.ts",
                foreignTypes: {
                    "Duration.Duration": { from: ["effect/Duration"] },
                    "Option.Option": { from: ["effect"] },
                },
            };"#,
        ),
    ]);
    let config = load(dir.path(), "macroforge.config.ts").expect("the config resolves");
    assert!(config.keep_decorators, "unset fields come from the base");
    assert_eq!(
        foreign_type_names(&config),
        ["DateTime.DateTime", "Duration.Duration", "Option.Option"]
    );
    let duration = &config.foreign_types[1];
    assert_eq!(
        duration.from,
        ["effect/Duration"],
        "the child's entry replaces the base's"
    );
}

#[test]
fn extends_takes_several_bases_later_ones_winning() {
    let dir = project(&[
        (
            "a.config.ts",
            "export default { keepDecorators: true, generateConvenienceConst: false };",
        ),
        ("b.config.ts", "export default { keepDecorators: false };"),
        (
            "macroforge.config.ts",
            r#"export default { extends: ["./a.config.ts", "./b.config.ts"] };"#,
        ),
    ]);
    let config = load(dir.path(), "macroforge.config.ts").expect("the config resolves");
    assert!(!config.keep_decorators);
    assert!(!config.generate_convenience_const);
}

#[test]
fn foreign_types_spread_another_configs_foreign_types() {
    let dir = project(&[
        ("base.config.ts", BASE),
        (
            "macroforge.config.ts",
            r#"import base from "./base.config.ts";
            export default {
                foreignTypes: { ...base.foreignTypes, "Option.Option": { from: ["effect"] } },
            };"#,
        ),
    ]);
    let config = load(dir.path(), "macroforge.config.ts").expect("the config resolves");
    assert!(
        !config.keep_decorators,
        "only the foreign types were spread"
    );
    assert_eq!(
        foreign_type_names(&config),
        ["DateTime.DateTime", "Duration.Duration", "Option.Option"]
    );
}

#[test]
fn define_config_local_bindings_and_named_exports_resolve() {
    let dir = project(&[
        (
            "shared.ts",
            "export const shared = { keepDecorators: true };\nconst hidden = {};\n",
        ),
        (
            "macroforge.config.ts",
            r#"import { shared } from "./shared.js";
            const config = defineConfig({ ...shared, generateConvenienceConst: false } satisfies object);
            export default config;"#,
        ),
    ]);
    let config = load(dir.path(), "macroforge.config.ts").expect("the config resolves");
    assert!(config.keep_decorators, "`./shared.js` finds `shared.ts`");
    assert!(!config.generate_convenience_const);
}

#[test]
fn a_base_config_resolves_from_a_package() {
    let dir = project(&[
        (
            "node_modules/@acme/macroforge-config/package.json",
            r#"{ "name": "@acme/macroforge-config", "exports": { ".": { "import": "./src/index.ts" } } }"#,
        ),
        ("node_modules/@acme/macroforge-config/src/index.ts", BASE),
        (
            "app/macroforge.config.ts",
            r#"export default { extends: "@acme/macroforge-config" };"#,
        ),
    ]);
    let config =
        load(&dir.path().join("app"), "macroforge.config.ts").expect("the config resolves");
    assert_eq!(config.foreign_types.len(), 2);
}

#[test]
fn a_config_that_cannot_be_followed_is_an_error() {
    let cases = [
        ("export default makeConfig();", "has 0 arguments"),
        (
            "export default base;",
            "neither declared in the file nor imported",
        ),
        (
            "export default { extends: 42 };",
            "`extends` must name config modules",
        ),
        ("export default { ...(await load()) };", "cannot follow"),
        (
            "import * as shared from './shared';\nexport default shared;",
            "namespace import",
        ),
    ];
    for (source, expected) in cases {
        let dir = project(&[("macroforge.config.ts", source)]);
        let error = load(dir.path(), "macroforge.config.ts")
            .expect_err("an unfollowable config is rejected");
        assert!(
            error.to_string().contains(expected),
            "`{source}` should fail with `{expected}`, failed with: {error}"
        );
    }
}

#[test]
fn a_missing_base_is_an_error() {
    let dir = project(&[(
        "macroforge.config.ts",
        r#"export default { extends: "./missing.config.ts" };"#,
    )]);
    let error = load(dir.path(), "macroforge.config.ts").expect_err("the base is missing");
    assert!(
        error
            .to_string()
            .contains("cannot resolve the config module `./missing.config.ts`")
    );
}

#[test]
fn an_import_cycle_is_an_error() {
    let dir = project(&[
        (
            "a.config.ts",
            r#"export default { extends: "./b.config.ts" };"#,
        ),
        (
            "b.config.ts",
            r#"export default { extends: "./a.config.ts" };"#,
        ),
        (
            "macroforge.config.ts",
            r#"export default { extends: "./a.config.ts" };"#,
        ),
    ]);
    let error = load(dir.path(), "macroforge.config.ts").expect_err("the bases form a cycle");
    assert!(error.to_string().contains("form a cycle"), "{error}");
}

#[test]
fn editing_a_base_reaches_a_config_found_by_discovery() {
    let dir = project(&[
        ("package.json", "{}"),
        ("base.config.ts", "export default { keepDecorators: true };"),
        (
            "macroforge.config.ts",
            r#"export default { extends: "./base.config.ts" };"#,
        ),
    ]);
    let first = MacroforgeConfigLoader::find_with_root_from_path(dir.path())
        .expect("discovery succeeds")
        .expect("the config is found")
        .0;
    assert!(first.keep_decorators);

    // A different length, so the stamp changes even within one mtime tick.
    fs::write(
        dir.path().join("base.config.ts"),
        "export default { keepDecorators: false };",
    )
    .expect("the edited base");
    let edited = MacroforgeConfigLoader::find_with_root_from_path(dir.path())
        .expect("discovery succeeds")
        .expect("the config is found")
        .0;
    assert!(
        !edited.keep_decorators,
        "the cached config must follow its base"
    );
}

#[test]
fn a_relative_file_path_finds_an_absolute_root() {
    // Relative to the test's working directory, as a CLI argument would be.
    let dir = tempfile::tempdir_in(".").expect("a directory under the working directory");
    for (path, content) in [
        ("package.json", "{}"),
        ("macroforge.config.ts", "export default {};"),
        ("src/lib/model.ts", "export interface Model {}"),
    ] {
        let full = dir.path().join(path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).expect("the fixture directory");
        }
        fs::write(&full, content).expect("the fixture file");
    }
    let name = dir.path().file_name().expect("the directory's name");
    let relative = Path::new(name).join("src/lib/model.ts");

    let (_, root) = MacroforgeConfigLoader::find_with_root_from_path(&relative)
        .expect("discovery succeeds")
        .expect("the config is found");

    assert!(root.is_absolute(), "{}", root.display());
    assert_eq!(
        root.canonicalize().expect("the root exists"),
        dir.path().canonicalize().expect("the fixture exists")
    );
}
