//! The sync that writes the expanded config into a project.

use super::super::{EXPANDED_CONFIG_DIR, sync_expanded_config};
use super::{RECORD_ID, project};

const PACKAGE_JSON: &str = r##"{ "imports": { "#macroforge/config": { "types": "./.macroforge/config/handlers.ts", "default": "./.macroforge/config/handlers.js" } } }"##;

#[test]
fn sync_requires_the_manifest_entry_when_the_config_declares_handlers() {
    let dir = project(&[("macroforge.config.ts", RECORD_ID), ("package.json", "{}")]);
    let error = sync_expanded_config(&dir)
        .expect_err("no entry")
        .to_string();
    assert!(
        error.contains("macroforge init") && error.contains("#macroforge/config"),
        "{error}"
    );
}

#[test]
fn sync_writes_both_outputs_and_skips_an_unchanged_expansion() {
    let dir = project(&[
        ("macroforge.config.ts", RECORD_ID),
        ("package.json", PACKAGE_JSON),
    ]);
    sync_expanded_config(&dir).expect("sync");
    let ts = dir.join(EXPANDED_CONFIG_DIR).join("handlers.ts");
    let js = dir.join(EXPANDED_CONFIG_DIR).join("handlers.js");
    assert!(
        std::fs::read_to_string(&ts)
            .expect("ts")
            .contains("__foreign__recordIdEncode")
    );
    assert!(
        std::fs::read_to_string(&js)
            .expect("js")
            .contains("__foreign__recordIdEncode")
    );

    let written = std::fs::metadata(&ts)
        .and_then(|meta| meta.modified())
        .expect("mtime");
    std::thread::sleep(std::time::Duration::from_millis(20));
    sync_expanded_config(&dir).expect("sync again");
    let rewritten = std::fs::metadata(&ts)
        .and_then(|meta| meta.modified())
        .expect("mtime");
    assert_eq!(
        written, rewritten,
        "an unchanged expansion is not rewritten"
    );
}

#[test]
fn sync_replaces_a_stale_expansion() {
    let dir = project(&[
        ("macroforge.config.ts", RECORD_ID),
        ("package.json", PACKAGE_JSON),
    ]);
    let stale = dir.join(EXPANDED_CONFIG_DIR).join("modules/gone.ts");
    std::fs::create_dir_all(stale.parent().expect("parent")).expect("dirs");
    std::fs::write(&stale, "stale").expect("write");
    sync_expanded_config(&dir).expect("sync");
    assert!(!stale.exists(), "a module no longer expanded is removed");
}

#[test]
fn sync_without_a_config_removes_the_expansion() {
    let dir = project(&[("src/a.ts", "export const a = 1;\n")]);
    let old = dir.join(EXPANDED_CONFIG_DIR).join("handlers.ts");
    std::fs::create_dir_all(old.parent().expect("parent")).expect("dirs");
    std::fs::write(&old, "old").expect("write");
    sync_expanded_config(&dir).expect("sync");
    assert!(!dir.join(EXPANDED_CONFIG_DIR).exists());
}
