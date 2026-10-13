//! Foreign type configuration and matching.

use std::rc::Rc;

use crate::host::ForeignTypeConfig;
use crate::host::import_registry::with_registry_mut;

/// The full foreign type list, and the configured types it was built from.
struct WithBuiltins {
    configured: Rc<[ForeignTypeConfig]>,
    all: Rc<[ForeignTypeConfig]>,
}

thread_local! {
    static WITH_BUILTINS: std::cell::RefCell<Option<WithBuiltins>> =
        const { std::cell::RefCell::new(None) };
}

/// The current foreign types, including built-in global types: user-configured
/// types first (higher priority), followed by built-ins. Built once per
/// configuration and shared, not copied per call.
pub fn get_foreign_types() -> Rc<[ForeignTypeConfig]> {
    let configured = crate::host::import_registry::configured_foreign_types();
    WITH_BUILTINS.with(|cell| {
        let mut cell = cell.borrow_mut();
        if let Some(cached) = cell.as_ref()
            && Rc::ptr_eq(&cached.configured, &configured)
        {
            return Rc::clone(&cached.all);
        }
        let all: Rc<[ForeignTypeConfig]> = configured
            .iter()
            .chain(BUILTIN_FOREIGN_TYPES.iter())
            .cloned()
            .collect();
        *cell = Some(WithBuiltins {
            configured,
            all: Rc::clone(&all),
        });
        all
    })
}

/// Built-in foreign type registrations for global JS/TS types.
/// These have empty `from` lists so they skip import validation.
static BUILTIN_FOREIGN_TYPES: std::sync::LazyLock<Vec<ForeignTypeConfig>> =
    std::sync::LazyLock::new(builtin_foreign_types);

fn builtin_foreign_types() -> Vec<ForeignTypeConfig> {
    let ft = |name: &str, ser: &str, deser: &str| ForeignTypeConfig {
        name: name.to_string(),
        from: vec![],
        encode_expr: Some(ser.to_string()),
        decode_expr: Some(deser.to_string()),
        default_expr: None,
        has_shape_expr: None,
        aliases: vec![],
        builtin: true,
        handler_sites: Vec::new(),
    };

    // Each decoder takes the encoding its encoder writes and refuses any
    // other value: the constructors it ends in would coerce one instead, as
    // `BigInt(true)` is `1n` and `new Int8Array(5)` is five zeroes.
    let typed_array = |name: &str| {
        ft(
            name,
            "(v) => Array.from(v)",
            &format!(
                r#"(v) => {{
                if (!Array.isArray(v) || !v.every((n: unknown): n is number => typeof n === "number" && Number.isFinite(n))) throw new TypeError("expected an array of numbers");
                return new {name}(v);
            }}"#
            ),
        )
    };

    let big_typed_array = |name: &str| {
        ft(
            name,
            "(v) => Array.from(v, (n) => String(n))",
            &format!(
                r#"(v) => {{
                if (!Array.isArray(v) || !v.every((s: unknown): s is string => typeof s === "string" && /^-?\d+$/.test(s))) throw new TypeError("expected an array of whole numbers as text");
                return new {name}(v.map((s) => BigInt(s)));
            }}"#
            ),
        )
    };

    vec![
        ft(
            "bigint",
            "(v) => String(v)",
            r#"(v) => {
                const raw: unknown = v;
                if (typeof raw === "bigint") return raw;
                if (typeof raw === "number" && Number.isInteger(raw)) return BigInt(raw);
                if (typeof raw !== "string" || !/^-?\d+$/.test(raw)) throw new TypeError("expected a whole number as text");
                return BigInt(raw);
            }"#,
        ),
        ft(
            "URL",
            "(v) => v.toString()",
            r#"(v) => {
                const raw: unknown = v;
                if (typeof raw !== "string") throw new TypeError("expected the text of a URL");
                return new URL(raw);
            }"#,
        ),
        ft(
            "URLSearchParams",
            "(v) => v.toString()",
            r#"(v) => {
                const raw: unknown = v;
                if (typeof raw !== "string") throw new TypeError("expected the text of a query string");
                return new URLSearchParams(raw);
            }"#,
        ),
        ft(
            "RegExp",
            "(v) => ({ source: v.source, flags: v.flags })",
            r#"(v) => {
                const raw: unknown = v;
                if (typeof raw !== "object" || raw === null || !("source" in raw) || typeof raw.source !== "string") throw new TypeError("expected the source and flags of a RegExp");
                const flags = "flags" in raw && typeof raw.flags === "string" ? raw.flags : "";
                return new RegExp(raw.source, flags);
            }"#,
        ),
        ft(
            "Error",
            "(v) => ({ name: v.name, message: v.message, stack: v.stack })",
            r#"(v) => {
                const raw: unknown = v;
                if (typeof raw !== "object" || raw === null || !("message" in raw) || typeof raw.message !== "string") throw new TypeError("expected the name and message of an Error");
                const error = new Error(raw.message);
                if ("name" in raw && typeof raw.name === "string") error.name = raw.name;
                return error;
            }"#,
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
            r#"(v) => {
                if (!Array.isArray(v) || !v.every((n: unknown): n is number => typeof n === "number" && Number.isFinite(n))) throw new TypeError("expected an array of numbers");
                return new Uint8Array(v).buffer;
            }"#,
        ),
    ]
}

/// Requests the import of a matched foreign type itself, which generated
/// annotations name. A dotted name imports its namespace root
/// (`DateTime.Utc` imports `DateTime`), since the leaf is not an export.
pub(super) fn register_foreign_type_import(ft: &ForeignTypeConfig) {
    with_registry_mut(|r| {
        let import_name = ft.get_namespace().unwrap_or_else(|| ft.get_type_name());
        if !r.is_available(&ft.name) && !r.is_available(import_name) && !ft.from.is_empty() {
            r.request_type_import(import_name, &ft.from[0]);
        }
    });
}

/// Result of matching a type against foreign type configurations. A type
/// imported from a source other than the configured one is not a match: it
/// falls through to generic handling, and TypeScript reports any problem.
#[derive(Debug)]
pub struct ForeignTypeMatch<'a> {
    /// The matched foreign type config, if any.
    pub config: Option<&'a ForeignTypeConfig>,
    /// Warning message for informational hints.
    pub warning: Option<String>,
}

impl<'a> ForeignTypeMatch<'a> {
    /// Create a successful match.
    pub fn matched(config: &'a ForeignTypeConfig) -> Self {
        Self {
            config: Some(config),
            warning: None,
        }
    }

    /// Create a near-match: no match, but a hint about the one it resembles.
    pub fn near_match(warning: String) -> Self {
        Self {
            config: None,
            warning: Some(warning),
        }
    }

    /// Create an empty result (no match, no warning).
    pub fn none() -> Self {
        Self {
            config: None,
            warning: None,
        }
    }

    /// Returns true if there was a successful match.
    pub fn is_match(&self) -> bool {
        self.config.is_some()
    }
}
