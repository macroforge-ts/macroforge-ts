use super::*;
use std::collections::HashMap;

#[test]
fn test_parse_simple_config() {
    let content = r#"
            export default {
                keepDecorators: true,
                generateConvenienceConst: false
            }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();
    assert!(config.keep_decorators);
    assert!(!config.generate_convenience_const);
}

#[test]
fn test_parse_config_with_foreign_types() {
    let content = r#"
            export default {
                foreignTypes: {
                    DateTime: {
                        from: ["effect"],
                        serialize: (v, ctx) => v.toJSON(),
                        deserialize: (raw, ctx) => DateTime.fromJSON(raw)
                    }
                }
            }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();
    assert_eq!(config.foreign_types.len(), 1);

    let dt = &config.foreign_types[0];
    assert_eq!(dt.name, "DateTime");
    assert_eq!(dt.from, vec!["effect"]);
    assert!(dt.serialize_expr.is_some());
    assert!(dt.deserialize_expr.is_some());
}

#[test]
fn test_parse_config_with_multiple_sources() {
    let content = r#"
            export default {
                foreignTypes: {
                    DateTime: {
                        from: ["effect", "@effect/schema"]
                    }
                }
            }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();
    let dt = &config.foreign_types[0];
    assert_eq!(dt.from, vec!["effect", "@effect/schema"]);
}

#[test]
fn test_parse_typescript_config() {
    let content = r#"
            import { DateTime } from "effect";

            export default {
                foreignTypes: {
                    DateTime: {
                        from: ["effect"],
                        serialize: (v: DateTime, ctx: unknown) => v.toJSON(),
                    }
                }
            }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.ts").unwrap();
    assert_eq!(config.foreign_types.len(), 1);
}

#[test]
fn test_default_values() {
    let content = "export default {}";
    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();

    assert!(!config.keep_decorators);
    assert!(config.generate_convenience_const);
    assert!(config.foreign_types.is_empty());
}

#[test]
fn test_legacy_macro_config_conversion() {
    let mf_config = MacroforgeConfig {
        keep_decorators: true,
        generate_convenience_const: false,
        foreign_types: vec![],
        config_imports: HashMap::new(),
        ..Default::default()
    };

    let legacy: MacroConfig = mf_config.into();
    assert!(legacy.keep_decorators);
    assert!(!legacy.generate_convenience_const);
}

#[test]
fn test_parse_buildtime_block() {
    let content = r#"
            export default {
                buildtime: {
                    timeout: 1500,
                    maxHeap: 64,
                    filesystem: { read: ["src/**"], write: ["out/**"] },
                    env: ["HOME", "CI"],
                    network: true,
                    flags: { RELEASE: "1", DEBUG: false }
                }
            }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();
    let bt = &config.buildtime;
    assert_eq!(bt.timeout_ms, 1500);
    assert_eq!(bt.max_heap_mb, 64);
    assert_eq!(bt.fs_read, vec!["src/**".to_string()]);
    assert_eq!(bt.fs_write, vec!["out/**".to_string()]);
    assert_eq!(bt.env_allow, vec!["HOME".to_string(), "CI".to_string()]);
    assert!(bt.network);
    assert_eq!(bt.flags.get("RELEASE").map(String::as_str), Some("1"));
    // Non-string scalars are stringified rather than dropped.
    assert_eq!(bt.flags.get("DEBUG").map(String::as_str), Some("false"));
}

#[test]
fn test_buildtime_block_defaults_when_absent() {
    let content = r#"
            export default { keepDecorators: true }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();
    let bt = &config.buildtime;
    assert_eq!(bt.timeout_ms, 5_000);
    assert_eq!(bt.max_heap_mb, 256);
    assert_eq!(bt.fs_read, vec!["**".to_string()]);
    assert!(bt.fs_write.is_empty());
    assert!(bt.env_allow.is_empty());
    assert!(!bt.network);
    assert!(bt.flags.is_empty());
}

#[test]
fn test_parse_buildtime_capabilities_nesting() {
    // The nested form is what every sandbox diagnostic tells users to edit.
    let content = r#"
            export default {
                buildtime: {
                    capabilities: {
                        timeout: 250,
                        filesystem: { read: ["assets/**"] },
                        env: ["HOME"]
                    },
                    flags: { MODE: "fast" }
                }
            }
        "#;

    let config = MacroforgeConfigLoader::from_config_file(content, "macroforge.config.js").unwrap();
    let bt = &config.buildtime;
    assert_eq!(bt.timeout_ms, 250);
    assert_eq!(bt.fs_read, vec!["assets/**".to_string()]);
    assert_eq!(bt.env_allow, vec!["HOME".to_string()]);
    assert_eq!(bt.flags.get("MODE").map(String::as_str), Some("fast"));
    // Unspecified capability keeps its default.
    assert_eq!(bt.max_heap_mb, 256);
}

#[test]
fn edited_config_is_reparsed_under_the_same_path() {
    let path = "/tmp/edited/macroforge.config.js";
    CONFIG_CACHE.remove(path);
    let first =
        MacroforgeConfigLoader::load_and_cache("export default { keepDecorators: true }", path)
            .unwrap();
    assert!(first.keep_decorators);
    let edited =
        MacroforgeConfigLoader::load_and_cache("export default { keepDecorators: false }", path)
            .unwrap();
    assert!(
        !edited.keep_decorators,
        "a changed file must not be served from the cache"
    );
    assert_eq!(
        MacroforgeConfigLoader::get_cached(path).map(|config| config.keep_decorators),
        Some(false)
    );
    CONFIG_CACHE.remove(path);
}
