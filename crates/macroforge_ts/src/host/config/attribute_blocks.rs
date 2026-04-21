//! Backend-agnostic parsing of the attribute-macro config blocks
//! (`cfg`, `deprecated`, `mustUse`, `nonExhaustive`).
//!
//! The SWC and OXC parsers each know how to convert their own AST literal
//! expressions into [`serde_json::Value`] — a tiny per-backend function.
//! From there, every config key lookup and type coercion is identical, so
//! we centralise it here. New keys only need to be added in one place.

use macroforge_ts_syn::config::{
    CfgFlags, DeprecatedConfig, MustUseConfig, MustUseMode, NonExhaustiveConfig,
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

/// Parse `deprecated: { runtimeWarn, failOnUse }`.
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
