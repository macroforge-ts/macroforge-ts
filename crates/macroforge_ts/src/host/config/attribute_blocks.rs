//! Backend-agnostic parsing of the attribute-macro config blocks
//! (`cfg`, `deprecated`, `mustUse`, `nonExhaustive`).
//!
//! The SWC and OXC parsers each know how to convert their own AST literal
//! expressions into [`serde_json::Value`] — a tiny per-backend function.
//! From there, every config key lookup and type coercion is identical, so
//! we centralise it here. New keys only need to be added in one place.

use macroforge_ts_syn::config::{
    BuildtimeConfig, CfgFlags, DeprecatedConfig, MustUseConfig, MustUseMode, NonExhaustiveConfig,
};
use serde_json::Value;

/// Parse `cfg: { features, target, debugAssertions, custom }`.
pub(crate) fn parse_cfg_flags(obj: &serde_json::Map<String, Value>) -> CfgFlags {
    let mut flags = CfgFlags::default();
    if let Some(features) = obj.get("features") {
        flags.features = extract_string_array(features);
    }
    if let Some(target) = obj.get("target").and_then(Value::as_str) {
        flags.target = Some(target.to_string());
    }
    if let Some(debug) = obj.get("debugAssertions").and_then(Value::as_bool) {
        flags.debug_assertions = debug;
    }
    if let Some(custom) = obj.get("custom").and_then(Value::as_object) {
        for (k, v) in custom {
            flags.custom.insert(k.clone(), v.clone());
        }
    }
    flags
}

/// Parse `deprecated: { runtimeWarn, failOnUse }`. Note that `runtimeWarn`
/// is parsed but currently has no effect — the attribute pass does not yet
/// inject the runtime `console.warn`.
pub(crate) fn parse_deprecated_config(obj: &serde_json::Map<String, Value>) -> DeprecatedConfig {
    let mut config = DeprecatedConfig::default();
    if let Some(b) = obj.get("runtimeWarn").and_then(Value::as_bool) {
        config.runtime_warn = b;
    }
    if let Some(b) = obj.get("failOnUse").and_then(Value::as_bool) {
        config.fail_on_use = b;
    }
    config
}

/// Parse `mustUse: { mode }`. Unknown modes fall back to the default
/// (currently only `"lint"` is recognised).
pub(crate) fn parse_must_use_config(obj: &serde_json::Map<String, Value>) -> MustUseConfig {
    let mut config = MustUseConfig::default();
    if obj.get("mode").and_then(Value::as_str) == Some("lint") {
        config.mode = MustUseMode::Lint;
    }
    config
}

/// Parse `nonExhaustive: { brand }`.
pub(crate) fn parse_non_exhaustive_config(
    obj: &serde_json::Map<String, Value>,
) -> NonExhaustiveConfig {
    let mut config = NonExhaustiveConfig::default();
    if let Some(brand) = obj.get("brand").and_then(Value::as_str) {
        config.brand = brand.to_string();
    }
    config
}

/// Accept either a single string or an array of strings (the same shape
/// `extract_string_or_array` produces from AST nodes) and flatten to a
/// `Vec<String>`. Non-string members are skipped — parser-side validation
/// can't express "string only" in JSON.
/// Parse the `buildtime` block:
/// `{ timeout, maxHeap, filesystem: { read, write }, env, network, flags }`.
///
/// Unset keys keep their defaults, so a partial block only overrides what it
/// names. `timeout` is milliseconds; `maxHeap` is MiB.
pub(crate) fn parse_buildtime_config(obj: &serde_json::Map<String, Value>) -> BuildtimeConfig {
    // Config numbers arrive as JSON floats (both parsers build them with
    // `Number::from_f64`), so `as_u64` alone would silently miss every value.
    fn as_unsigned(value: &Value) -> Option<u64> {
        value
            .as_u64()
            .or_else(|| value.as_f64().filter(|f| *f >= 0.0).map(|f| f as u64))
    }

    let mut config = BuildtimeConfig::default();

    // Capability keys are canonically nested under `capabilities` — that is
    // the path every sandbox diagnostic points users at. The flat form
    // (`buildtime.timeout`, …) is accepted too so a short config doesn't
    // need the extra level.
    let caps = obj.get("capabilities").and_then(Value::as_object);
    let lookup = |key: &str| caps.and_then(|c| c.get(key)).or_else(|| obj.get(key));

    if let Some(ms) = lookup("timeout").and_then(as_unsigned) {
        config.timeout_ms = ms;
    }
    if let Some(mb) = lookup("maxHeap").and_then(as_unsigned) {
        config.max_heap_mb = mb as usize;
    }
    if let Some(fs) = lookup("filesystem").and_then(Value::as_object) {
        if let Some(read) = fs.get("read") {
            config.fs_read = extract_string_array(read);
        }
        if let Some(write) = fs.get("write") {
            config.fs_write = extract_string_array(write);
        }
    }
    if let Some(env) = lookup("env") {
        config.env_allow = extract_string_array(env);
    }
    if let Some(network) = lookup("network").and_then(Value::as_bool) {
        config.network = network;
    }
    if let Some(flags) = obj.get("flags").and_then(Value::as_object) {
        for (k, v) in flags {
            // Flags are surfaced to JS as strings; accept scalars and
            // stringify them rather than silently dropping non-strings.
            let text = match v {
                Value::String(s) => s.clone(),
                Value::Bool(b) => b.to_string(),
                Value::Number(n) => n.to_string(),
                _ => continue,
            };
            config.flags.insert(k.clone(), text);
        }
    }
    config
}

fn extract_string_array(value: &Value) -> Vec<String> {
    match value {
        Value::String(s) => vec![s.clone()],
        Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    }
}
