use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{InitOutcome, add_handlers_import, handlers_import_declared};

static NEXT: AtomicUsize = AtomicUsize::new(0);

fn project(files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mf-manifest-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).expect("create dir");
    for (name, text) in files {
        std::fs::write(dir.join(name), text).expect("write");
    }
    dir
}

#[test]
fn a_package_json_entry_is_found() {
    let dir = project(&[(
        "package.json",
        r##"{ "name": "app", "imports": { "#macroforge/config": { "default": "./.macroforge/config/handlers.js" } } }"##,
    )]);
    assert!(handlers_import_declared(&dir).expect("read"));
}

#[test]
fn a_deno_jsonc_entry_with_comments_is_found() {
    let dir = project(&[(
        "deno.jsonc",
        "{\n  // handlers for foreign types\n  \"imports\": { \"#macroforge/config\": \"./.macroforge/config/handlers.ts\" }, /* trailing */\n}\n",
    )]);
    assert!(handlers_import_declared(&dir).expect("read"));
}

#[test]
fn a_manifest_without_the_entry_does_not_declare_it() {
    let dir = project(&[(
        "package.json",
        r##"{ "name": "app", "imports": { "#lib": "./src/lib/index.js" } }"##,
    )]);
    assert!(!handlers_import_declared(&dir).expect("read"));
}

#[test]
fn init_adds_the_entry_keeping_order_and_indentation() {
    let original = "{\n\t\"name\": \"app\",\n\t\"type\": \"module\",\n\t\"imports\": {\n\t\t\"#lib\": \"./src/lib/index.js\"\n\t}\n}\n";
    let dir = project(&[("package.json", original)]);
    assert_eq!(
        add_handlers_import(&dir).expect("init"),
        InitOutcome::Added(dir.join("package.json"))
    );
    let written = std::fs::read_to_string(dir.join("package.json")).expect("read");
    assert!(
        written.starts_with("{\n\t\"name\": \"app\",\n\t\"type\": \"module\","),
        "{written}"
    );
    assert!(
        written.contains("\t\t\"#lib\": \"./src/lib/index.js\","),
        "{written}"
    );
    assert!(written.contains("\"#macroforge/config\": {"), "{written}");
    assert!(
        written.contains("\"types\": \"./.macroforge/config/handlers.ts\""),
        "{written}"
    );
    assert!(written.ends_with("}\n"), "{written}");
    assert!(handlers_import_declared(&dir).expect("read"));
}

#[test]
fn init_creates_imports_when_there_is_none() {
    let dir = project(&[("package.json", "{\n  \"name\": \"app\"\n}\n")]);
    add_handlers_import(&dir).expect("init");
    assert!(handlers_import_declared(&dir).expect("read"));
}

#[test]
fn init_leaves_a_declared_entry_alone() {
    let original = r##"{"imports":{"#macroforge/config":"./custom.js"}}"##;
    let dir = project(&[("package.json", original)]);
    assert_eq!(
        add_handlers_import(&dir).expect("init"),
        InitOutcome::AlreadyDeclared(dir.join("package.json"))
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("package.json")).expect("read"),
        original
    );
}

#[test]
fn init_writes_a_deno_json_import_map_entry() {
    let dir = project(&[("deno.json", "{\n  \"imports\": {}\n}\n")]);
    add_handlers_import(&dir).expect("init");
    let written = std::fs::read_to_string(dir.join("deno.json")).expect("read");
    assert!(
        written.contains("\"#macroforge/config\": \"./.macroforge/config/handlers.ts\""),
        "{written}"
    );
}

#[test]
fn init_will_not_rewrite_a_deno_jsonc() {
    let dir = project(&[("deno.jsonc", "{\n  // keep me\n  \"imports\": {}\n}\n")]);
    let error = add_handlers_import(&dir)
        .expect_err("a jsonc is not rewritten")
        .to_string();
    assert!(error.contains("#macroforge/config"), "{error}");
}

#[test]
fn init_without_a_manifest_is_an_error() {
    let dir = project(&[]);
    assert!(add_handlers_import(&dir).is_err());
}
