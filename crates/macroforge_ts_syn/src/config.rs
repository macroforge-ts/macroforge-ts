//! # Macroforge Configuration Types
//!
//! Encodable configuration types shared between the host process and external macro
//! processes. These live in `macroforge_ts_syn` so they can be used in [`MacroContextIR`]
//! for cross-process transfer.
//!
//! The config parsing logic (reading `macroforge.config.ts`) remains in
//! `macroforge_ts::host::config`.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

/// Information about an imported function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInfo {
    /// The imported name (or "default" for default imports).
    pub name: String,
    /// The module specifier.
    pub source: String,
}

/// An alias for a foreign type that allows matching different name-package pairs.
///
/// This is useful when a type can be imported from different paths or with different names.
///
/// ## Example
///
/// ```javascript
/// foreignTypes: {
///   "DateTime.DateTime": {
///     from: ["effect"],
///     aliases: [
///       { name: "DateTime", from: "effect/DateTime" }
///     ],
///     encode: (v) => DateTime.formatIso(v),
///     // ...
///   }
/// }
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForeignTypeAlias {
    /// The type name to match (e.g., "DateTime" or "DateTime.DateTime").
    pub name: String,
    /// The import source to match (e.g., "effect/DateTime").
    pub from: String,
}

/// The specifier generated code imports foreign-type handlers from: the
/// expanded config, which exports each handler by name.
pub const FOREIGN_HANDLERS_MODULE: &str = "#macroforge/config";

/// One of the functions a foreign type declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ForeignHandler {
    /// `encode`: the value as its wire form.
    Encode,
    /// `decode`: the value from its wire form.
    Decode,
    /// `default`: a fresh value.
    Default,
    /// `hasShape`: whether a raw value could be this type.
    HasShape,
}

impl ForeignHandler {
    /// Every handler, in the order the config declares them.
    pub const ALL: [Self; 4] = [Self::Encode, Self::Decode, Self::Default, Self::HasShape];

    /// The config key a handler is declared under.
    pub fn key(self) -> &'static str {
        match self {
            Self::Encode => "encode",
            Self::Decode => "decode",
            Self::Default => "default",
            Self::HasShape => "hasShape",
        }
    }

    /// The handler a config key declares.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|handler| handler.key() == key)
    }

    fn suffix(self) -> &'static str {
        match self {
            Self::Encode => "Encode",
            Self::Decode => "Decode",
            Self::Default => "Default",
            Self::HasShape => "HasShape",
        }
    }

    /// The name the expanded config exports `type_name`'s handler under:
    /// `__foreign__` + the camel-cased type name + the handler, so
    /// `DateTime.DateTime`'s encode is `__foreign__dateTimeDateTimeEncode`.
    pub fn export_name(self, type_name: &str) -> String {
        format!("__foreign__{}{}", camel_type_name(type_name), self.suffix())
    }
}

/// A dotted type name as one camel-cased identifier: the first segment's
/// leading capitals lowered (`URL` to `url`, `HTMLElement` to `htmlElement`),
/// each later segment kept as written.
fn camel_type_name(name: &str) -> String {
    let mut camel = String::with_capacity(name.len());
    for (index, segment) in name.split('.').enumerate() {
        if index > 0 {
            camel.push_str(segment);
            continue;
        }
        let chars: Vec<char> = segment.chars().collect();
        let capitals = chars.iter().take_while(|c| c.is_uppercase()).count();
        // Of a run of capitals followed by more letters, the last starts the
        // next word and stays capital.
        let lowered = if capitals == chars.len() || capitals <= 1 {
            capitals
        } else {
            capitals - 1
        };
        for (position, c) in chars.into_iter().enumerate() {
            if position < lowered.max(1) {
                camel.extend(c.to_lowercase());
            } else {
                camel.push(c);
            }
        }
    }
    camel
}

/// Where a foreign type's handler is written in a config module, so the
/// expanded config can hoist it into an export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandlerSite {
    /// Which handler this is.
    pub handler: ForeignHandler,
    /// The config module that declares it.
    pub module: std::path::PathBuf,
    /// Byte offset where the handler's value starts in that module.
    pub start: u32,
    /// Byte offset where the handler's value ends in that module.
    pub end: u32,
    /// The handler when written as a bare identifier, which is re-exported
    /// rather than hoisted.
    pub identifier: Option<String>,
}

/// Configuration for a single foreign type.
///
/// Foreign types allow global registration of handlers for external types
/// (like Effect's `DateTime`) so they work like primitives without per-field annotations.
///
/// ## Key Format
///
/// The key in `foreignTypes` should be the fully qualified type name as used in code:
/// - Simple type name: `"DateTime"` - matches `DateTime` in code
/// - Fully qualified: `"DateTime.DateTime"` - matches `DateTime.DateTime` (namespace.type pattern)
///
/// ## Import Source Validation
///
/// Foreign types are only matched when the type is imported from a source listed in
/// `from` or one of the `aliases`. Types with the same name from different packages
/// are ignored (fall back to generic handling).
///
/// ## Example
///
/// ```javascript
/// foreignTypes: {
///   // For Effect's DateTime where you import { DateTime } and use DateTime.DateTime
///   "DateTime.DateTime": {
///     from: ["effect"],
///     aliases: [
///       { name: "DateTime", from: "effect/DateTime" },
///       { name: "MyDateTime", from: "my-effect-wrapper" }
///     ],
///     encode: (v) => DateTime.formatIso(v),
///     decode: (raw) => DateTime.unsafeFromDate(new Date(raw)),
///     default: () => DateTime.unsafeNow()
///   }
/// }
/// ```
///
/// This configuration matches:
/// - `import { DateTime } from 'effect'` with type `DateTime.DateTime`
/// - `import { DateTime } from 'effect/DateTime'` with type `DateTime`
/// - `import { MyDateTime } from 'my-effect-wrapper'` with type `MyDateTime`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForeignTypeConfig {
    /// The full type key as specified in config (e.g., "DateTime" or "DateTime.DateTime").
    /// This is the key from the foreignTypes object.
    pub name: String,

    /// Import sources where this type can come from (e.g., ["effect", "effect/DateTime"]).
    /// Used to validate that the type is imported from the correct module.
    pub from: Vec<String>,

    /// Encoding function expression (e.g., "(v, ctx) => v.toJSON()").
    pub encode_expr: Option<String>,

    /// Decoding function expression.
    pub decode_expr: Option<String>,

    /// Default value function expression (e.g., "() => DateTime.now()").
    pub default_expr: Option<String>,

    /// Shape-check predicate expression for union variant matching.
    /// Used when this foreign type appears as a variant in a union type alias.
    /// The expression should be a function `(value: unknown) => boolean`.
    /// Example: `(v: unknown) => typeof v === "string"` for types decoded from strings.
    pub has_shape_expr: Option<String>,

    /// Aliases for this foreign type, allowing different name-package pairs to use the same config.
    #[serde(default)]
    pub aliases: Vec<ForeignTypeAlias>,

    /// Whether macroforge itself declares this type rather than a config.
    /// A builtin's handlers read only JavaScript globals and are written
    /// into generated code; a config's are imported from the expanded config.
    #[serde(default)]
    pub builtin: bool,

    /// Where each of this type's handlers is written in the config, for
    /// expanding the config. Host-side only.
    #[serde(skip)]
    pub handler_sites: Vec<HandlerSite>,
}

impl ForeignTypeConfig {
    /// The namespace of a dotted name: everything before its last dot, so
    /// `"Deep.A.B.Type"` gives `"Deep.A.B"`. `None` for an undotted name.
    pub fn get_namespace(&self) -> Option<&str> {
        self.name.rsplit_once('.').map(|(namespace, _)| namespace)
    }

    /// Returns the simple type name (last segment after dots).
    /// For "DateTime.DateTime", returns "DateTime".
    /// For "DateTime", returns "DateTime".
    pub fn get_type_name(&self) -> &str {
        self.name.rsplit('.').next().unwrap_or(&self.name)
    }

    /// What generated code calls to run `handler`, or `None` when the type
    /// declares no such handler. A config's handler is called by its export
    /// from the expanded config, whose import from [`FOREIGN_HANDLERS_MODULE`]
    /// this requests; a builtin, which no config declares and which reads only
    /// JavaScript globals, is its own expression.
    pub fn handler_callee(&self, handler: ForeignHandler) -> Option<String> {
        let expression = match handler {
            ForeignHandler::Encode => self.encode_expr.as_deref(),
            ForeignHandler::Decode => self.decode_expr.as_deref(),
            ForeignHandler::Default => self.default_expr.as_deref(),
            ForeignHandler::HasShape => self.has_shape_expr.as_deref(),
        }?;
        if self.builtin {
            return Some(format!("({expression})"));
        }
        let name = handler.export_name(&self.name);
        crate::import_registry::with_registry_mut(|registry| {
            registry.request_import(&name, None, FOREIGN_HANDLERS_MODULE, false);
        });
        Some(name)
    }
}

/// Build flags consumed by the `@cfg` attribute macro.
///
/// The predicate in `/** @cfg({ feature: 'ssr' }) */` evaluates against these
/// flags: keys with single-value configs (e.g. `target`) match exactly; keys
/// whose config value is an array (e.g. `features`) match when the annotation
/// value is a member; multiple keys in one annotation combine with implicit AND.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CfgFlags {
    /// Active feature flags. `@cfg({ feature: 'ssr' })` passes when `'ssr'` is in this list.
    #[serde(default)]
    pub features: Vec<String>,

    /// Build target (e.g. `"web"`, `"node"`, `"deno"`). `@cfg({ target: 'web' })` matches exactly.
    #[serde(default)]
    pub target: Option<String>,

    /// Whether this build treats `@cfg({ debugAssertions: true })` as truthy.
    #[serde(default)]
    pub debug_assertions: bool,

    /// Arbitrary string-keyed predicate values. Accepts any JSON scalar so
    /// annotations like `@cfg({ tenant: 'acme' })` or `@cfg({ version: 2 })`
    /// can match exactly.
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

/// Behavior knobs for the `@deprecated` attribute macro.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeprecatedConfig {
    /// Inject a one-shot `console.warn(...)` into the deprecated declaration's
    /// body so runtime use is visible alongside tsc's static `@deprecated` tag.
    ///
    /// Parsed but currently not acted on: the attribute pass only rewrites
    /// the JSDoc; runtime warn injection is not yet implemented, so this
    /// flag has no effect today (despite defaulting to `true`).
    #[serde(default = "crate::config::default_true")]
    pub runtime_warn: bool,

    /// Treat any use of a `@deprecated` symbol as a macro-expansion error.
    /// Off by default; turn on when chasing deprecations out of a codebase.
    #[serde(default)]
    pub fail_on_use: bool,
}

impl Default for DeprecatedConfig {
    fn default() -> Self {
        Self {
            runtime_warn: true,
            fail_on_use: false,
        }
    }
}

/// Enforcement strategy for `@mustUse`. Only one mode today; keeping this as
/// an enum reserves room for a future `Wrap` variant without a breaking change.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MustUseMode {
    /// Emit a macroforge diagnostic at the discarded-call site. No runtime cost.
    #[default]
    Lint,
}

/// Behavior knobs for the `@mustUse` attribute macro.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MustUseConfig {
    #[serde(default)]
    pub mode: MustUseMode,
}

/// Sandbox configuration for `@buildtime` evaluation.
///
/// Mirrors the `buildtime` block of `macroforge.config.*`. Build-time code
/// can read files and see the listed env vars and flags, and nothing more:
/// it cannot write files or reach the network.
///
/// ```js
/// buildtime: {
///   timeout: 5000,                  // ms
///   maxHeap: 256,                   // MiB (advisory)
///   filesystem: { read: ["**"] },
///   env: ["HOME"],
///   flags: { RELEASE: "1" }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildtimeConfig {
    /// Wall-clock evaluation budget in milliseconds.
    #[serde(default = "default_buildtime_timeout_ms")]
    pub timeout_ms: u64,

    /// JS heap ceiling in MiB. Advisory: the Boa backend exposes no
    /// memory-limit hook, so this is carried but not enforced.
    #[serde(default = "default_buildtime_max_heap_mb")]
    pub max_heap_mb: usize,

    /// Glob patterns readable via `buildtime.fs`. Defaults to everything.
    #[serde(default = "default_buildtime_fs_read")]
    pub fs_read: Vec<String>,

    /// Environment variable names exposed as `buildtime.env.NAME`.
    #[serde(default)]
    pub env_allow: Vec<String>,

    /// Build flags exposed as `buildtime.flags.has(name)` / `.get(name)`.
    #[serde(default)]
    pub flags: BTreeMap<String, String>,
}

impl Default for BuildtimeConfig {
    fn default() -> Self {
        Self {
            timeout_ms: default_buildtime_timeout_ms(),
            max_heap_mb: default_buildtime_max_heap_mb(),
            fs_read: default_buildtime_fs_read(),
            env_allow: Vec::new(),
            flags: BTreeMap::new(),
        }
    }
}

/// Default `@buildtime` evaluation timeout (5s), in milliseconds.
pub fn default_buildtime_timeout_ms() -> u64 {
    5_000
}

/// Default `@buildtime` heap ceiling (256 MiB).
pub fn default_buildtime_max_heap_mb() -> usize {
    256
}

/// Default readable globs for `@buildtime` (`**`).
pub fn default_buildtime_fs_read() -> Vec<String> {
    vec!["**".to_string()]
}

/// Behavior knobs for the `@nonExhaustive` attribute macro.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonExhaustiveConfig {
    /// Brand property name used in the intersection. Keep this stable across a
    /// project so downstream consumers can pattern-match on it.
    #[serde(default = "default_non_exhaustive_brand")]
    pub brand: String,
}

impl Default for NonExhaustiveConfig {
    fn default() -> Self {
        Self {
            brand: default_non_exhaustive_brand(),
        }
    }
}

fn default_non_exhaustive_brand() -> String {
    "__nonExhaustive".to_string()
}

pub(crate) fn default_true() -> bool {
    true
}

/// Configuration for the macro host system.
///
/// This struct represents the contents of a `macroforge.config.js` file.
/// It controls macro loading, execution, and foreign type handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroforgeConfig {
    /// Whether to preserve `@derive` decorators in the output code.
    ///
    /// When `false` (default), decorators are stripped after expansion.
    /// When `true`, decorators remain in the output (useful for debugging).
    #[serde(default)]
    pub keep_decorators: bool,

    /// Whether to generate a convenience const for non-class types.
    ///
    /// When `true` (default), generates an `export const TypeName = { ... } as const;`
    /// that groups all generated functions for a type into a single namespace-like object.
    #[serde(default = "default_generate_convenience_const")]
    pub generate_convenience_const: bool,

    /// Foreign type configurations.
    ///
    /// Maps type names to their handlers for encoding, decoding, and defaults.
    #[serde(default)]
    pub foreign_types: Vec<ForeignTypeConfig>,

    /// Build flags consumed by `@cfg`. Missing key is equivalent to an empty block.
    #[serde(default)]
    pub cfg: CfgFlags,

    /// Knobs for `@deprecated`. Missing key uses per-field defaults.
    #[serde(default)]
    pub deprecated: DeprecatedConfig,

    /// Knobs for `@mustUse`. Missing key = lint-mode diagnostic.
    #[serde(default)]
    pub must_use: MustUseConfig,

    /// Knobs for `@nonExhaustive`. Missing key = default brand name.
    #[serde(default)]
    pub non_exhaustive: NonExhaustiveConfig,

    /// Sandbox settings for `@buildtime`. Missing key uses the defaults.
    #[serde(default)]
    pub buildtime: BuildtimeConfig,
}

/// Returns the default for generate_convenience_const (true).
pub fn default_generate_convenience_const() -> bool {
    true
}

impl Default for MacroforgeConfig {
    fn default() -> Self {
        Self {
            keep_decorators: false,
            generate_convenience_const: true,
            foreign_types: Vec::new(),
            cfg: CfgFlags::default(),
            deprecated: DeprecatedConfig::default(),
            must_use: MustUseConfig::default(),
            non_exhaustive: NonExhaustiveConfig::default(),
            buildtime: BuildtimeConfig::default(),
        }
    }
}
