//! Foreign-type fixture tests with snapshots.
//!
//! Each fixture under `tests/fixtures/foreign_types/<name>/` is a triple:
//!
//! - `macroforge.config.ts` — config to load (gives us `config_imports`,
//!   `expression_namespaces`, and the foreign-type registry).
//! - `input.ts` — source file to expand against that config.
//! - `snapshots/` — insta-managed expected output.
//!
//! The snapshot captures what users actually see in their generated cache:
//! the emitted imports (top of the file), the rewritten body, and any
//! diagnostics. That covers the bug class we keep tripping on — a missing
//! `__mf_<ns>` alias OR a synthetic import for a JS global — without us
//! having to write per-symbol assertions.
//!
//! ## Running / updating
//!
//! ```text
//! cargo test --test foreign_type_fixtures
//! cargo insta review     # accept new/changed snapshots
//! ```
//!
//! ## Adding a fixture
//!
//! Drop a new directory under `tests/fixtures/foreign_types/` containing
//! `macroforge.config.ts` and `input.ts`. The harness picks it up
//! automatically; running `cargo insta review` produces the initial snapshot.

use macroforge_ts::{
    ExpandOptions,
    api::CoreEngine,
    host::{
        clear_foreign_types,
        config::{MacroforgeConfigLoader, clear_config_cache},
        import_registry::clear_registry,
    },
};

/// Narrow the noisy raw output to just the bug-class signal:
///
/// - **Config-declared namespaces** — the names the config file pulls in via
///   `import { … } from "…"`. These are the *only* identifiers the engine is
///   allowed to register as `__mf_*` aliases. Anything else is implicitly
///   global.
/// - **Generated imports** — the `__mf_*` lines the engine emitted. Each
///   `__mf_X` here MUST correspond to an entry in the config-declared list.
///   If a JS global like `Math` ever shows up here, the test breaks loudly.
/// - **Body references** — every `<Identifier>.` (and every `__mf_*`) that
///   appears in the rewritten body, partitioned into "config-declared" and
///   "treated as global". A config-declared namespace appearing in the body
///   *without* an `__mf_` prefix is a regression of the missing-import bug.
/// - **Default function** — the IIFE'd default body, narrow enough that
///   unrelated codegen churn doesn't flap these snapshots.
fn format_snapshot(
    config: &str,
    input: &str,
    result: &macroforge_ts::api_types::ExpandResult,
) -> String {
    let code = &result.code;
    let config_imports = config_imports(config);

    let mut generated_imports: Vec<&str> = Vec::new();
    for line in code.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("import ") && trimmed.contains("__mf_") {
            generated_imports.push(trimmed);
        }
    }

    let mut mf_aliases: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut body_namespaces: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let bytes = code.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_alphabetic() || b == b'_' || b == b'$' {
            // Walk a JS identifier.
            let start = i;
            while i < bytes.len() {
                let c = bytes[i];
                if c.is_ascii_alphanumeric() || c == b'_' || c == b'$' {
                    i += 1;
                } else {
                    break;
                }
            }
            // We want identifiers that are followed by `.` — i.e. the root of
            // a member expression like `Math.floor` or `__mf_Option.match`.
            let next_is_dot = bytes.get(i).copied() == Some(b'.');
            // Filter out cases where the identifier is itself a *property* of
            // a preceding `.` (e.g. the `floor` in `Math.floor`).
            let prev_is_dot = start > 0 && bytes[start - 1] == b'.';
            if next_is_dot && !prev_is_dot {
                let ident = &code[start..i];
                if let Some(stripped) = ident.strip_prefix("__mf_") {
                    mf_aliases.insert(format!("__mf_{stripped}"));
                } else if !is_keyword(ident) && looks_like_namespace(ident) {
                    body_namespaces.insert(ident.to_string());
                }
            }
        } else {
            i += 1;
        }
    }

    let (config_declared_in_body, treated_as_global): (Vec<&String>, Vec<&String>) =
        body_namespaces
            .iter()
            .partition(|ns| config_imports.contains(ns.as_str()));

    let default_body = extract_default_function(code);

    let mut out = String::new();

    out.push_str("## Config\n\n");
    out.push_str(config.trim());

    out.push_str("\n\n## Input\n\n");
    out.push_str(input.trim());

    out.push_str(
        "\n\n## Config-declared namespaces (any name not in this list is treated as a global)\n\n",
    );
    if config_imports.is_empty() {
        out.push_str("(none)");
    } else {
        let mut sorted: Vec<&str> = config_imports.iter().copied().collect();
        sorted.sort();
        for ns in sorted {
            out.push_str(ns);
            out.push('\n');
        }
        while out.ends_with('\n') {
            out.pop();
        }
    }

    out.push_str("\n\n## Generated imports (only `__mf_*` lines)\n\n");
    if generated_imports.is_empty() {
        out.push_str("(none)");
    } else {
        for line in &generated_imports {
            out.push_str(line);
            out.push('\n');
        }
        while out.ends_with('\n') {
            out.pop();
        }
    }

    out.push_str("\n\n## `__mf_*` aliases referenced in body\n\n");
    if mf_aliases.is_empty() {
        out.push_str("(none)");
    } else {
        for alias in &mf_aliases {
            out.push_str(alias);
            out.push('\n');
        }
        while out.ends_with('\n') {
            out.pop();
        }
    }

    out.push_str("\n\n## Body references — config-declared (must be aliased)\n\n");
    if config_declared_in_body.is_empty() {
        out.push_str("(none)");
    } else {
        for ns in &config_declared_in_body {
            out.push_str(ns);
            out.push('\n');
        }
        while out.ends_with('\n') {
            out.pop();
        }
    }

    out.push_str("\n\n## Body references — treated as global (must NOT be aliased)\n\n");
    if treated_as_global.is_empty() {
        out.push_str("(none)");
    } else {
        for ns in &treated_as_global {
            out.push_str(ns);
            out.push('\n');
        }
        while out.ends_with('\n') {
            out.pop();
        }
    }

    out.push_str("\n\n## Default function\n\n");
    if let Some(body) = default_body {
        out.push_str(body.trim());
    } else {
        out.push_str("(no defaultValue function emitted)");
    }

    out.push_str("\n\n## Diagnostics\n\n");
    if result.diagnostics.is_empty() {
        out.push_str("(none)");
    } else {
        for d in &result.diagnostics {
            out.push_str(&format!("{}: {}\n", d.level, d.message));
        }
        while out.ends_with('\n') {
            out.pop();
        }
    }

    out.push('\n');
    out
}

/// Best-effort extract of the local names from `import { … } from "…"` lines
/// at the top of the config file. Covers named (`{ A, B as C }`), default
/// (`import D from "…"`), and namespace (`import * as N from "…"`) forms.
/// Doesn't try to handle everything — the config files are small and
/// well-formed so a regex-style scan is enough.
fn config_imports(config: &str) -> std::collections::HashSet<&str> {
    let mut out = std::collections::HashSet::new();
    for line in config.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("import ") {
            continue;
        }
        let after_import = &trimmed[6..].trim_start();
        let body = match after_import.find(" from ") {
            Some(idx) => &after_import[..idx],
            None => continue,
        };
        let body = body.trim();
        if let Some(rest) = body.strip_prefix('{') {
            let inner = rest.trim_end_matches('}');
            for part in inner.split(',') {
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }
                // `Foo as Bar` — the local name is `Bar`.
                let local = part.rsplit(" as ").next().unwrap_or(part).trim();
                let local = local.trim_start_matches("type ").trim();
                if !local.is_empty() {
                    out.insert(local);
                }
            }
        } else if let Some(rest) = body.strip_prefix("* as ") {
            let local = rest.trim();
            if !local.is_empty() {
                out.insert(local);
            }
        } else {
            // Bare default import: `import Foo from "…"`.
            let local = body.split(',').next().unwrap_or("").trim();
            if !local.is_empty() {
                out.insert(local);
            }
        }
    }
    out
}

/// Reserve a few JS keywords that can appear before `.` in the body
/// (`new.target`, `this.foo`, etc.) so they don't show up as fake
/// "namespaces".
fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "this"
            | "new"
            | "super"
            | "import"
            | "export"
            | "return"
            | "typeof"
            | "instanceof"
            | "in"
            | "of"
            | "void"
            | "delete"
    )
}

/// True for identifiers that *could* plausibly be a namespace — anything
/// PascalCase, plus a couple of well-known lowercase globals. This filters
/// out local-variable property access (`obj.id`, `ctx.register`, …) so the
/// "treated as global" snapshot section stays focused on the bug class.
fn looks_like_namespace(s: &str) -> bool {
    if s.starts_with(|c: char| c.is_ascii_uppercase()) {
        return true;
    }
    matches!(s, "console" | "globalThis" | "process")
}

/// Pull out the `export function <type>DefaultValue(): <T> { ... }` block.
/// Brace-matched extraction so nested object literals don't truncate it.
fn extract_default_function(code: &str) -> Option<&str> {
    let marker = "DefaultValue(): ";
    let needle_pos = code.find(marker)?;
    let fn_start = code[..needle_pos]
        .rfind("export function")
        .or_else(|| code[..needle_pos].rfind("function"))?;
    let brace_start = needle_pos + code[needle_pos..].find('{')?;

    let bytes = code.as_bytes();
    let mut depth: i32 = 0;
    let mut end = brace_start;
    for (i, &b) in bytes.iter().enumerate().skip(brace_start) {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    end = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    Some(&code[fn_start..end])
}

#[test]
fn foreign_type_fixtures() {
    insta::glob!(
        "fixtures/foreign_types/*/macroforge.config.ts",
        |config_path| {
            let fixture_dir = config_path.parent().expect("config has parent");
            let input_path = fixture_dir.join("input.ts");
            let config = std::fs::read_to_string(config_path).expect("read config");
            let input = std::fs::read_to_string(&input_path).expect("read input");
            let fixture_name = fixture_dir
                .file_name()
                .and_then(|n| n.to_str())
                .expect("fixture name");

            // Use a unique config_path string per fixture so CONFIG_CACHE
            // doesn't serve a stale entry across runs.
            let cache_key =
                format!("/test/foreign_type_fixtures/{fixture_name}/macroforge.config.ts");

            clear_config_cache();
            clear_foreign_types();
            clear_registry();
            MacroforgeConfigLoader::load_and_cache(&config, &cache_key).expect("config parses");

            let options = ExpandOptions {
                keep_decorators: None,
                external_decorator_modules: None,
                config_path: Some(cache_key),
                type_registry_json: None,
                declarative_registry_json: None,
                build_mode: None,
            };
            let result =
                CoreEngine::expand_sync(input.clone(), "input.ts".to_string(), Some(options))
                    .unwrap_or_else(|e| panic!("expand_sync failed for {fixture_name}: {e}"));

            clear_config_cache();
            clear_foreign_types();
            clear_registry();

            let snapshot = format_snapshot(&config, &input, &result);

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
