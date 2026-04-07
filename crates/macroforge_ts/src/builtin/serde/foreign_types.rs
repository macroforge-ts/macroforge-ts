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
/// Checks each namespace referenced in the foreign type's expressions and determines
/// if it needs to be imported (i.e., if it's not already available as a value import).
///
/// The import source is determined by looking at the config file's imports first
/// (e.g., if the config has `import { DateTime } from "effect"`, we use "effect").
/// This ensures we import from the same place the config uses for its expressions.
///
/// # Arguments
/// * `ft` - The matched foreign type configuration
/// * `import_module` - The module the type is imported from (fallback if not in config)
pub(super) fn register_foreign_type_namespaces(ft: &ForeignTypeConfig, import_module: &str) {
    with_registry_mut(|r| {
        for ns in &ft.expression_namespaces {
            let has_import = r.is_available(ns);
            let has_config_import = r.config_imports.contains_key(ns);

            if !has_import && !has_config_import {
                continue;
            }

            let is_type_only = r.is_type_only(ns);

            if is_type_only || (!r.source_map().contains_key(ns) && has_config_import) {
                let module = r.config_imports.get(ns).cloned().unwrap_or_else(|| {
                    ft.from
                        .first()
                        .cloned()
                        .unwrap_or_else(|| import_module.to_string())
                });

                let alias = format!("__mf_{}", ns);
                r.request_namespace_import(ns, &module, &alias);
            }
        }

        let type_name = ft.get_type_name();
        if !r.is_available(&ft.name) && !r.is_available(type_name) && !ft.from.is_empty() {
            r.request_type_import(type_name, &ft.from[0]);
        }
    });
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
