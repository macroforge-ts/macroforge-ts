use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use macroforge_ts_syn::config::{ForeignHandler, HandlerSite};

use super::{ExpandedFile, expand_config, hoist_module};
use crate::host::config::resolve::resolve_config;

static NEXT: AtomicUsize = AtomicUsize::new(0);

/// A fresh project directory holding `files`, as `(relative path, source)`.
fn project(files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "mf-hoist-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    for (path, source) in files {
        let path = dir.join(path);
        std::fs::create_dir_all(path.parent().expect("a file has a parent")).expect("create dirs");
        std::fs::write(&path, source).expect("write");
    }
    dir
}

/// The expanded config of the project's `macroforge.config.ts`.
fn expand(dir: &Path) -> Result<Vec<ExpandedFile>, String> {
    let root = dir.join("macroforge.config.ts");
    let source = std::fs::read_to_string(&root).expect("read config");
    let resolved =
        resolve_config(&source, root.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
    expand_config(&root, &resolved.config.foreign_types, |path| {
        std::fs::read_to_string(path)
    })
    .map_err(|e| e.to_string())
}

fn file<'a>(files: &'a [ExpandedFile], stem: &str) -> &'a ExpandedFile {
    files
        .iter()
        .find(|file| file.stem == Path::new(stem))
        .unwrap_or_else(|| panic!("no expanded file {stem} in {files:#?}"))
}

fn handlers(dir: &Path) -> ExpandedFile {
    let files = expand(dir).expect("expand");
    file(&files, "handlers").clone()
}

const RECORD_ID: &str = r#"import { fromRecordId, toRecordId } from "@app/record-id";
import { RecordId } from "surrealdb";

export default {
    foreignTypes: {
        RecordId: {
            from: ["surrealdb"],
            encode: (value: RecordId) => fromRecordId(value),
            decode: (raw: unknown) => {
                if (raw instanceof RecordId) return raw;
                return toRecordId(raw);
            },
        },
    },
};
"#;

#[test]
fn an_arrow_handler_is_hoisted_as_written() {
    let dir = project(&[("macroforge.config.ts", RECORD_ID)]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains(
            "export const __foreign__recordIdEncode = (value: RecordId) => fromRecordId(value);"
        ),
        "{ts}"
    );
    assert!(ts.contains("encode: __foreign__recordIdEncode,"), "{ts}");
}

#[test]
fn a_block_bodied_handler_keeps_its_body() {
    let dir = project(&[("macroforge.config.ts", RECORD_ID)]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains("export const __foreign__recordIdDecode = (raw: unknown) => {"),
        "{ts}"
    );
    assert!(
        ts.contains("if (raw instanceof RecordId) return raw;"),
        "{ts}"
    );
    assert!(ts.contains("decode: __foreign__recordIdDecode,"), "{ts}");
}

#[test]
fn handlers_are_exported_in_source_order_before_their_statement() {
    let dir = project(&[("macroforge.config.ts", RECORD_ID)]);
    let ts = handlers(&dir).typescript;
    let encode = ts
        .find("export const __foreign__recordIdEncode")
        .expect("encode");
    let decode = ts
        .find("export const __foreign__recordIdDecode")
        .expect("decode");
    let config = ts.find("export default").expect("config");
    assert!(encode < decode && decode < config, "{ts}");
}

#[test]
fn imports_stay_with_the_copy() {
    let dir = project(&[("macroforge.config.ts", RECORD_ID)]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains(r#"import { fromRecordId, toRecordId } from "@app/record-id";"#),
        "{ts}"
    );
    assert!(
        ts.contains(r#"import { RecordId } from "surrealdb";"#),
        "{ts}"
    );
}

#[test]
fn a_function_expression_and_a_generic_arrow_are_hoisted() {
    let dir = project(&[(
        "macroforge.config.ts",
        r#"import { Option } from "effect";
export default {
    foreignTypes: {
        "Option.Option": {
            from: ["effect"],
            encode: <Item>(value: Option.Option<Item>) => Option.getOrNull(value),
            decode: function (raw: unknown) { return Option.fromNullable(raw); },
        },
    },
};
"#,
    )]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains("export const __foreign__optionOptionEncode = <Item>(value: Option.Option<Item>) => Option.getOrNull(value);"),
        "{ts}"
    );
    assert!(
        ts.contains("export const __foreign__optionOptionDecode = function (raw: unknown) { return Option.fromNullable(raw); };"),
        "{ts}"
    );
}

#[test]
fn a_config_local_helper_stays_in_scope() {
    let dir = project(&[(
        "macroforge.config.ts",
        r#"import { Duration } from "effect";
function isSerdeDuration(raw: unknown): raw is { secs: number; nanos: number } {
    return typeof raw === "object" && raw !== null && "secs" in raw;
}
export default {
    foreignTypes: {
        "Duration.Duration": {
            from: ["effect"],
            hasShape: (value: unknown) => Duration.isDuration(value) || isSerdeDuration(value),
        },
    },
};
"#,
    )]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains("function isSerdeDuration(raw: unknown)"),
        "{ts}"
    );
    assert!(
        ts.contains("export const __foreign__durationDurationHasShape = (value: unknown) => Duration.isDuration(value) || isSerdeDuration(value);"),
        "{ts}"
    );
}

#[test]
fn an_imported_identifier_handler_is_re_exported() {
    let dir = project(&[(
        "macroforge.config.ts",
        r#"import { fromRecordId } from "@app/record-id";
export default { foreignTypes: { RecordId: { from: ["surrealdb"], encode: fromRecordId } } };
"#,
    )]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains("export { fromRecordId as __foreign__recordIdEncode };"),
        "{ts}"
    );
    assert!(ts.contains("encode: fromRecordId"), "{ts}");
    assert!(
        !ts.contains("export const __foreign__recordIdEncode"),
        "{ts}"
    );
}

#[test]
fn a_locally_declared_identifier_handler_is_re_exported() {
    let dir = project(&[(
        "macroforge.config.ts",
        r#"function encodeRecordId(value: unknown) { return String(value); }
export default { foreignTypes: { RecordId: { from: ["surrealdb"], encode: encodeRecordId } } };
"#,
    )]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains("export { encodeRecordId as __foreign__recordIdEncode };"),
        "{ts}"
    );
}

#[test]
fn a_global_identifier_handler_is_hoisted() {
    let dir = project(&[(
        "macroforge.config.ts",
        r#"export default { foreignTypes: { Tag: { from: ["tags"], encode: String } } };
"#,
    )]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains("export const __foreign__tagEncode = String;"),
        "{ts}"
    );
}

#[test]
fn a_parameter_shadowing_an_import_is_left_alone() {
    let dir = project(&[(
        "macroforge.config.ts",
        r#"import { fromRecordId } from "@app/record-id";
export default { foreignTypes: { RecordId: { from: ["surrealdb"], encode: (fromRecordId: unknown) => fromRecordId } } };
"#,
    )]);
    let ts = handlers(&dir).typescript;
    assert!(
        ts.contains(
            "export const __foreign__recordIdEncode = (fromRecordId: unknown) => fromRecordId;"
        ),
        "{ts}"
    );
}

#[test]
fn export_names_camel_case_the_type_name() {
    assert_eq!(
        ForeignHandler::Encode.export_name("DateTime.DateTime"),
        "__foreign__dateTimeDateTimeEncode"
    );
    assert_eq!(
        ForeignHandler::Decode.export_name("DateTime.TimeZone.Named"),
        "__foreign__dateTimeTimeZoneNamedDecode"
    );
    assert_eq!(
        ForeignHandler::Default.export_name("URL"),
        "__foreign__urlDefault"
    );
    assert_eq!(
        ForeignHandler::HasShape.export_name("HTMLElement"),
        "__foreign__htmlElementHasShape"
    );
    assert_eq!(
        ForeignHandler::Encode.export_name("RecordId"),
        "__foreign__recordIdEncode"
    );
}

#[test]
fn colliding_export_names_are_refused() {
    let dir = project(&[(
        "macroforge.config.ts",
        r#"export default { foreignTypes: {
    "Foo.Bar": { from: ["a"], encode: (v: unknown) => v },
    fooBar: { from: ["b"], encode: (v: unknown) => v },
} };
"#,
    )]);
    let error = expand(&dir).expect_err("a collision");
    assert!(error.contains("__foreign__fooBarEncode"), "{error}");
}

#[test]
fn relative_imports_are_rerooted_and_packages_kept() {
    let dir = project(&[
        (
            "codecs/record-id.ts",
            "export const fromRecordId = (v: unknown) => v;\n",
        ),
        (
            "macroforge.config.ts",
            r#"import { fromRecordId } from "./codecs/record-id.ts";
import { Option } from "effect";
export default { foreignTypes: { RecordId: { from: ["surrealdb"], encode: (v: unknown) => fromRecordId(v) } } };
"#,
        ),
    ]);
    let ts = handlers(&dir).typescript;
    assert!(ts.contains(r#"from "../../codecs/record-id.ts""#), "{ts}");
    assert!(ts.contains(r#"from "effect""#), "{ts}");
}

#[test]
fn a_base_config_handler_is_hoisted_in_its_copy_and_re_exported() {
    let dir = project(&[
        (
            "configs/base.config.ts",
            r#"import { fromRecordId } from "@app/record-id";
export default { foreignTypes: { RecordId: { from: ["surrealdb"], encode: (v: unknown) => fromRecordId(v) } } };
"#,
        ),
        (
            "macroforge.config.ts",
            r#"export default { extends: "./configs/base.config.ts" };
"#,
        ),
    ]);
    let files = expand(&dir).expect("expand");
    let base = file(&files, "modules/configs/base.config");
    assert!(
        base.typescript
            .contains("export const __foreign__recordIdEncode = (v: unknown) => fromRecordId(v);"),
        "{}",
        base.typescript
    );
    let root = file(&files, "handlers");
    assert!(
        root.typescript.contains(
            r#"export { __foreign__recordIdEncode } from "./modules/configs/base.config.js";"#
        ),
        "{}",
        root.typescript
    );
}

#[test]
fn an_override_hoists_only_the_winning_handler() {
    let dir = project(&[
        (
            "base.config.ts",
            r#"export default { foreignTypes: { RecordId: { from: ["surrealdb"], encode: (v: unknown) => "base" } } };
"#,
        ),
        (
            "macroforge.config.ts",
            r#"export default {
    extends: "./base.config.ts",
    foreignTypes: { RecordId: { from: ["surrealdb"], encode: (v: unknown) => "root" } },
};
"#,
        ),
    ]);
    let files = expand(&dir).expect("expand");
    let root = file(&files, "handlers");
    assert!(
        root.typescript
            .contains(r#"export const __foreign__recordIdEncode = (v: unknown) => "root";"#),
        "{}",
        root.typescript
    );
    assert!(
        !root
            .typescript
            .contains("from \"./modules/base.config.js\""),
        "{}",
        root.typescript
    );
}

#[test]
fn a_top_level_side_effect_is_refused() {
    let dir = project(&[(
        "macroforge.config.ts",
        r#"console.log("loaded");
export default { foreignTypes: { Tag: { from: ["t"], encode: (v: unknown) => v } } };
"#,
    )]);
    let error = expand(&dir).expect_err("a side effect");
    assert!(error.contains("console.log"), "{error}");
}

#[test]
fn import_meta_and_dynamic_imports_are_refused() {
    for hazard in ["import.meta.url", r#"import("./x")"#] {
        let dir = project(&[(
            "macroforge.config.ts",
            &format!(
                "export default {{ foreignTypes: {{ Tag: {{ from: [\"t\"], encode: (v: unknown) => {hazard} }} }} }};\n"
            ),
        )]);
        let error = expand(&dir).expect_err("a relocation hazard");
        assert!(
            error.contains("expanded into `.macroforge/config`"),
            "{hazard}: {error}"
        );
    }
}

#[test]
fn a_handler_inside_a_function_is_refused() {
    let source = "function make() { return { encode: (v: unknown) => v }; }\nexport default {};\n";
    let start = source.find("(v: unknown) => v").expect("handler") as u32;
    let site = HandlerSite {
        handler: ForeignHandler::Encode,
        module: PathBuf::from("/project/macroforge.config.ts"),
        start,
        end: start + "(v: unknown) => v".len() as u32,
        identifier: None,
    };
    let error = hoist_module(
        source,
        Path::new("/project/macroforge.config.ts"),
        &[(&site, "__foreign__tagEncode".to_string())],
        Path::new("/project/.macroforge/config"),
        Path::new("/project"),
    )
    .expect_err("a nested handler");
    assert!(error.to_string().contains("inside a function"), "{error}");
}

#[test]
fn the_config_call_is_marked_pure_in_both_outputs() {
    let dir = project(&[(
        "macroforge.config.ts",
        r#"import { defineConfig } from "macroforge";
export default defineConfig({ foreignTypes: { Tag: { from: ["t"], encode: (v: unknown) => v } } });
"#,
    )]);
    let expanded = handlers(&dir);
    assert!(
        expanded
            .typescript
            .contains("export default /*#__PURE__*/ defineConfig("),
        "{}",
        expanded.typescript
    );
    assert!(
        expanded.javascript.contains("/*#__PURE__*/"),
        "{}",
        expanded.javascript
    );
}

#[test]
fn the_javascript_has_no_types() {
    let dir = project(&[("macroforge.config.ts", RECORD_ID)]);
    let js = handlers(&dir).javascript;
    assert!(
        js.contains("__foreign__recordIdEncode = (value) => fromRecordId(value)"),
        "{js}"
    );
    assert!(!js.contains(": RecordId"), "{js}");
    assert!(!js.contains(": unknown"), "{js}");
}

#[test]
fn a_config_without_foreign_types_still_has_a_handlers_module() {
    let dir = project(&[("macroforge.config.ts", "export default {};\n")]);
    let files = expand(&dir).expect("expand");
    assert_eq!(files.len(), 1);
    assert!(
        file(&files, "handlers")
            .typescript
            .contains("Generated by macroforge from macroforge.config.ts")
    );
}

#[test]
fn a_config_that_does_not_parse_is_reported() {
    let dir = project(&[("macroforge.config.ts", "export default {\n")]);
    assert!(expand(&dir).is_err());
}

#[path = "hoist_sync_tests.rs"]
mod sync;

#[path = "hoist_layout_tests.rs"]
mod layout;
