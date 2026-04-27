//! Foreign type configuration and matching.

use crate::host::ForeignTypeConfig;
use crate::host::import_registry::{with_registry, with_registry_mut};

/// Get a clone of the current foreign types, including built-in global types.
/// User-configured types are listed first (higher priority), followed by built-ins.
pub fn get_foreign_types() -> Vec<ForeignTypeConfig> {
    let mut types = crate::host::import_registry::with_foreign_types(|ft| ft.to_vec());
    types.extend(get_builtin_foreign_types());
    types
}

/// Built-in foreign type registrations for global JS/TS types.
/// These have empty `from` lists so they skip import validation.
fn get_builtin_foreign_types() -> Vec<ForeignTypeConfig> {
    let ft = |name: &str, ser: &str, deser: &str| ForeignTypeConfig {
        name: name.to_string(),
        namespace: None,
        from: vec![],
        serialize_expr: Some(ser.to_string()),
        serialize_import: None,
        deserialize_expr: Some(deser.to_string()),
        deserialize_import: None,
        default_expr: None,
        default_import: None,
        has_shape_expr: None,
        has_shape_import: None,
        aliases: vec![],
        expression_namespaces: vec![],
    };

    let typed_array = |name: &str| {
        ft(
            name,
            "(v) => Array.from(v)",
            &format!("(v) => new {name}(v as number[])"),
        )
    };

    let big_typed_array = |name: &str| {
        ft(
            name,
            "(v) => Array.from(v, (n) => String(n))",
            &format!("(v) => new {name}((v as string[]).map((s) => BigInt(s)))"),
        )
    };

    vec![
        ft("bigint", "(v) => String(v)", "(v) => BigInt(v as string)"),
        ft("URL", "(v) => v.toString()", "(v) => new URL(v as string)"),
        ft(
            "URLSearchParams",
            "(v) => v.toString()",
            "(v) => new URLSearchParams(v as string)",
        ),
        ft(
            "RegExp",
            "(v) => ({ source: v.source, flags: v.flags })",
            "(v) => new RegExp((v as any).source, (v as any).flags)",
        ),
        ft(
            "Error",
            "(v) => ({ name: v.name, message: v.message, stack: v.stack })",
            "(v) => Object.assign(new Error((v as any).message), { name: (v as any).name })",
        ),
        typed_array("Int8Array"),
        typed_array("Uint8Array"),
        typed_array("Uint8ClampedArray"),
        typed_array("Int16Array"),
        typed_array("Uint16Array"),
        typed_array("Int32Array"),
        typed_array("Uint32Array"),
        typed_array("Float32Array"),
        typed_array("Float64Array"),
        big_typed_array("BigInt64Array"),
        big_typed_array("BigUint64Array"),
        ft(
            "ArrayBuffer",
            "(v) => Array.from(new Uint8Array(v))",
            "(v) => new Uint8Array(v as number[]).buffer",
        ),
    ]
}

/// Rewrite namespace references in an expression to use the generated aliases.
///
/// For namespaces that need to be imported (registered via `register_required_namespace`),
/// this function replaces the namespace identifier with its alias.
///
/// For example, if `DateTime` is registered with alias `__mf_DateTime`, then:
/// - `(v) => DateTime.formatIso(v)` becomes `(v) => __mf_DateTime.formatIso(v)`
/// - `DateTime.unsafeNow()` becomes `__mf_DateTime.unsafeNow()`
///
/// # Arguments
/// * `expr` - The expression string to rewrite
///
/// # Returns
/// The rewritten expression string with namespace aliases applied
pub fn rewrite_expression_namespaces(expr: &str) -> String {
    with_registry(|r| {
        let mut result = expr.to_string();
        let mut found_any = false;

        for import in r.generated_imports() {
            if let Some(ref original) = import.original_name
                && !import.is_type_only
            {
                let pattern = format!("{}.", original);
                let replacement = format!("{}.", import.local_name);
                if result.contains(&pattern) {
                    result = result.replace(&pattern, &replacement);
                    found_any = true;
                }
            }
        }

        if !found_any {
            return expr.to_string();
        }

        result
    })
}

/// Register required namespace imports for a matched foreign type.
///
/// For each namespace `ns` referenced inside `ft`'s expression bodies,
/// register `import { ns as __mf_ns } from "<module>"` so
/// [`rewrite_expression_namespaces`] can substitute `ns.` → `__mf_ns.` in
/// the inlined body.
///
/// `ns` is only registered when we have a definite module to import it
/// from:
/// 1. The macroforge.config.ts top-level imports (`config_imports`), or
/// 2. A configured foreign type whose surface name / namespace root is
///    `ns` (e.g. `DateTime.Utc`'s default body calls `Option.match` and
///    `Option` is itself a foreign-type entry — its `from[0]` tells us
///    where to import it).
///
/// **Globals are never imported.** Identifiers like `Array`, `console`,
/// `Math`, `Object`, `BigInt`, `JSON`, `Date`, `Error`, etc. show up in
/// `expression_namespaces` exactly the same as user-imported namespaces,
/// but they live in the JS runtime — emitting `import { console as
/// __mf_console } from "<ft.from>"` would produce a broken cache. The
/// rule is: if neither `config_imports` nor the foreign-type registry
/// names `ns`, leave it unrewritten — the runtime resolves it as a global
/// for free.
///
/// `ft.from[0]` is **not** used as a fallback module for `ns`. The
/// matched foreign type's `from` describes where `ft` itself lives, not
/// where arbitrary other identifiers in its expression body live.
pub(super) fn register_foreign_type_namespaces(ft: &ForeignTypeConfig, _import_module: &str) {
    let foreign_types = crate::host::import_registry::with_foreign_types(|fts| fts.to_vec());
    with_registry_mut(|r| {
        for ns in &ft.expression_namespaces {
            // Already a non-type-only value import in the target source —
            // the inlined `ns.foo()` resolves directly, no alias needed.
            if r.source_map().contains_key(ns) && !r.is_type_only(ns) {
                continue;
            }

            // Only register when we *know* the module. Anything else
            // (Array, console, Math, JSON, …) is a JS global; leave it
            // alone.
            let module = if let Some(m) = r.config_imports.get(ns).cloned() {
                m
            } else if let Some(m) = foreign_type_module(&foreign_types, ns) {
                m
            } else {
                continue;
            };

            let alias = format!("__mf_{}", ns);
            r.request_namespace_import(ns, &module, &alias);
        }

        // For dotted names like "DateTime.Utc", import the namespace root ("DateTime")
        // since the leaf ("Utc") isn't a standalone export — it's accessed via the namespace.
        let import_name = ft.get_namespace().unwrap_or_else(|| ft.get_type_name());
        if !r.is_available(&ft.name) && !r.is_available(import_name) && !ft.from.is_empty() {
            r.request_type_import(import_name, &ft.from[0]);
        }
    });
}

/// Look up a configured foreign type whose surface name (or namespace root)
/// matches `ns`. Used to recover the module specifier when the referenced
/// namespace isn't named in macroforge.config.ts's top-level imports but is
/// itself a configured foreign type (e.g. `Option`).
fn foreign_type_module(foreign_types: &[ForeignTypeConfig], ns: &str) -> Option<String> {
    for candidate in foreign_types {
        let matches = candidate.name == ns
            || candidate.get_namespace().is_some_and(|root| root == ns)
            || candidate.get_type_name() == ns;
        if matches && let Some(m) = candidate.from.first() {
            return Some(m.clone());
        }
    }
    None
}

/// Result of matching a type against foreign type configurations.
#[derive(Debug)]
pub struct ForeignTypeMatch<'a> {
    /// The matched foreign type config, if any.
    pub config: Option<&'a ForeignTypeConfig>,
    /// Warning message for informational hints.
    pub warning: Option<String>,
    /// Error message for import source mismatches (should fail the build).
    pub error: Option<String>,
}

impl<'a> ForeignTypeMatch<'a> {
    /// Create a successful match.
    pub fn matched(config: &'a ForeignTypeConfig) -> Self {
        Self {
            config: Some(config),
            warning: None,
            error: None,
        }
    }

    /// Create an import mismatch error (type matches but import source doesn't).
    /// This should cause a build failure.
    pub fn import_mismatch(_config: &'a ForeignTypeConfig, error: String) -> Self {
        Self {
            config: None,
            warning: None,
            error: Some(error),
        }
    }

    /// Create a near-match (no match, but with a helpful warning).
    /// The config parameter is for API consistency but not stored since this is a non-match.
    pub fn near_match(_config: &'a ForeignTypeConfig, warning: String) -> Self {
        Self {
            config: None,
            warning: Some(warning),
            error: None,
        }
    }

    /// Create an empty result (no match, no warning, no error).
    pub fn none() -> Self {
        Self {
            config: None,
            warning: None,
            error: None,
        }
    }

    /// Returns true if there was a successful match.
    pub fn is_match(&self) -> bool {
        self.config.is_some()
    }

    /// Returns true if there was an error (import source mismatch).
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
}
