use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::ship_expanded_config;

static NEXT: AtomicUsize = AtomicUsize::new(0);

fn tree(files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mf-package-config-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    for (path, text) in files {
        let path = dir.join(path);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("dirs");
        std::fs::write(path, text).expect("write");
    }
    dir
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).expect("read")
}

#[test]
fn imports_are_pointed_at_the_shipped_copy() {
    let root = tree(&[
        (
            ".macroforge/config/handlers.js",
            "export const __foreign__tagEncode = (v) => v;\n",
        ),
        (
            ".macroforge/config/handlers.ts",
            "export const __foreign__tagEncode = (v: unknown) => v;\n",
        ),
        (
            "dist/index.js",
            "import { __foreign__tagEncode } from \"#macroforge/config\";\n",
        ),
        (
            "dist/types/user.svelte.js",
            "import { __foreign__tagEncode } from '#macroforge/config';\n",
        ),
    ]);
    ship_expanded_config(&root, &root.join("dist")).expect("ship");
    assert!(root.join("dist/__macroforge/config/handlers.js").is_file());
    assert!(root.join("dist/__macroforge/config/handlers.ts").is_file());
    assert_eq!(
        read(&root.join("dist/index.js")),
        "import { __foreign__tagEncode } from \"./__macroforge/config/handlers.js\";\n"
    );
    assert_eq!(
        read(&root.join("dist/types/user.svelte.js")),
        "import { __foreign__tagEncode } from '../__macroforge/config/handlers.js';\n"
    );
}

#[test]
fn base_config_copies_ship_with_the_handlers() {
    let root = tree(&[
        (
            ".macroforge/config/handlers.js",
            "export { __foreign__aEncode } from \"./modules/base.config.js\";\n",
        ),
        (
            ".macroforge/config/modules/base.config.js",
            "export const __foreign__aEncode = (v) => v;\n",
        ),
        (
            "dist/index.js",
            "import { __foreign__aEncode } from \"#macroforge/config\";\n",
        ),
    ]);
    ship_expanded_config(&root, &root.join("dist")).expect("ship");
    assert!(
        root.join("dist/__macroforge/config/modules/base.config.js")
            .is_file()
    );
}

#[test]
fn an_output_without_handler_imports_is_left_alone() {
    let root = tree(&[("dist/index.js", "export const a = 1;\n")]);
    ship_expanded_config(&root, &root.join("dist")).expect("ship");
    assert!(!root.join("dist/__macroforge").exists());
}

#[test]
fn an_import_with_nothing_to_ship_is_an_error() {
    let root = tree(&[(
        "dist/index.js",
        "import { x } from \"#macroforge/config\";\n",
    )]);
    let error = ship_expanded_config(&root, &root.join("dist")).expect_err("nothing to ship");
    assert!(error.to_string().contains("no expanded config"), "{error}");
}
