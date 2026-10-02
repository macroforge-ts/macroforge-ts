use anyhow::{Context, Result, anyhow, bail};

use oxc::allocator::Allocator;
use oxc::parser::Parser;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, LazyLock, Mutex};

use crate::api_types::{
    ExpandOptions, ExpandResult, GeneratedRegionResult, MacroDiagnostic, MappingSegmentResult,
    SourceMappingResult,
};
use crate::host::CONFIG_CACHE;
use crate::host::MacroExpander;
use crate::host::declarative::ProjectDeclarativeRegistry;
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;

// ============================================================================
// MacroExpander Cache
// ============================================================================

/// Cached config discovery result to avoid filesystem walks on every expand call.
/// On native, `MacroExpander::new()` calls `MacroConfig::find_with_root()` which
/// walks up directories looking for config files. Caching the result here means
/// the filesystem is only touched once.
#[cfg(not(target_arch = "wasm32"))]
static DISCOVERED_CONFIG: LazyLock<
    std::sync::Mutex<Option<(crate::host::config::MacroConfig, std::path::PathBuf)>>,
> = LazyLock::new(|| std::sync::Mutex::new(None));

/// A MacroExpander from the process's pool, using cached config discovery.
fn create_expander() -> Result<MacroExpander> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::host::MacroConfig;
        let mut guard = DISCOVERED_CONFIG
            .lock()
            .map_err(|e| anyhow!("Lock poisoned: {e}"))?;
        let (config, root) = if let Some((c, r)) = guard.as_ref() {
            (c.clone(), r.clone())
        } else {
            let discovered = MacroConfig::find_with_root()
                .map_err(|e| anyhow!("Config discovery failed: {e}"))?
                .unwrap_or_else(|| {
                    (
                        MacroConfig::default(),
                        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
                    )
                });
            *guard = Some(discovered.clone());
            discovered
        };
        MacroExpander::pooled(config, root)
            .map_err(|err| anyhow!("Failed to initialize macro host: {err:?}"))
    }
    #[cfg(target_arch = "wasm32")]
    {
        // The wasm build has no filesystem to discover a config on.
        MacroExpander::pooled(
            crate::host::MacroConfig::default(),
            std::path::PathBuf::from("."),
        )
        .map_err(|err| anyhow!("Failed to initialize macro host: {err:?}"))
    }
}

// ============================================================================
// Resident Registries
// ============================================================================

/// A registry the bindings hold for the process, so an expansion names it by
/// id rather than sending its JSON across the boundary on every call.
#[derive(Clone)]
enum ResidentRegistry {
    Types(TypeRegistry),
    Declarative(Arc<ProjectDeclarativeRegistry>),
}

/// How many registries passed as JSON stay parsed. A process works against
/// a registry or two at a time; the bound keeps one that sees many versions,
/// such as a dev server through a session of edits, from growing without end.
const JSON_REGISTRY_SLOTS: usize = 4;

/// The registries set by id, and the most recent ones passed as JSON, keyed
/// by a hash of that JSON.
#[derive(Default)]
struct RegistryTable {
    next_id: u32,
    by_id: HashMap<u32, ResidentRegistry>,
    by_json: Vec<(u64, ResidentRegistry)>,
}

static REGISTRIES: LazyLock<Mutex<RegistryTable>> = LazyLock::new(Mutex::default);

fn lock_registries() -> Result<std::sync::MutexGuard<'static, RegistryTable>> {
    REGISTRIES
        .lock()
        .map_err(|err| anyhow!("registry table lock poisoned: {err}"))
}

fn json_key(json: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    json.hash(&mut hasher);
    hasher.finish()
}

fn parse_type_registry(json: &str) -> Result<TypeRegistry> {
    serde_json::from_str(json).context("the type registry JSON is invalid")
}

fn parse_declarative_registry(json: &str) -> Result<Arc<ProjectDeclarativeRegistry>> {
    ProjectDeclarativeRegistry::from_json(json)
        .map(Arc::new)
        .map_err(|err| anyhow!("the declarative registry JSON is invalid: {err}"))
}

/// Keeps `registry`, parsed from JSON, for the process and returns its id.
fn register(registry: ResidentRegistry) -> Result<u32> {
    let mut table = lock_registries()?;
    table.next_id = table
        .next_id
        .checked_add(1)
        .ok_or_else(|| anyhow!("registry ids are exhausted"))?;
    let id = table.next_id;
    table.by_id.insert(id, registry);
    Ok(id)
}

/// Parses `json` as a type registry and keeps it for the process.
pub(crate) fn set_type_registry(json: &str) -> Result<u32> {
    register(ResidentRegistry::Types(parse_type_registry(json)?))
}

/// Parses `json` as a declarative registry and keeps it for the process.
pub(crate) fn set_declarative_registry(json: &str) -> Result<u32> {
    register(ResidentRegistry::Declarative(parse_declarative_registry(
        json,
    )?))
}

/// Forgets the registry kept under `id`. Releasing an id twice, or one never
/// issued, is an error, since it means the caller lost track of its ids.
pub(crate) fn release_registry(id: u32) -> Result<()> {
    lock_registries()?
        .by_id
        .remove(&id)
        .map(|_| ())
        .ok_or_else(|| anyhow!("no registry is kept under id {id}"))
}

fn registry_by_id(id: u32) -> Result<ResidentRegistry> {
    lock_registries()?
        .by_id
        .get(&id)
        .cloned()
        .ok_or_else(|| anyhow!("no registry is kept under id {id}; set it before expanding"))
}

/// The registry `json` parses to, reusing the parse of an identical JSON
/// seen recently.
fn registry_from_json(
    json: &str,
    parse: impl FnOnce(&str) -> Result<ResidentRegistry>,
) -> Result<ResidentRegistry> {
    let key = json_key(json);
    {
        let mut table = lock_registries()?;
        if let Some(position) = table.by_json.iter().position(|(cached, _)| *cached == key) {
            let entry = table.by_json.remove(position);
            let registry = entry.1.clone();
            table.by_json.push(entry);
            return Ok(registry);
        }
    }
    let registry = parse(json)?;
    let mut table = lock_registries()?;
    if table.by_json.len() == JSON_REGISTRY_SLOTS {
        table.by_json.remove(0);
    }
    table.by_json.push((key, registry.clone()));
    Ok(registry)
}

/// The type registry the options name, by id or by JSON.
fn requested_type_registry(opts: &ExpandOptions) -> Result<Option<TypeRegistry>> {
    let registry = match (opts.type_registry_id, &opts.type_registry_json) {
        (Some(_), Some(_)) => {
            bail!("pass one of typeRegistryId and typeRegistryJson, not both")
        }
        (Some(id), None) => registry_by_id(id)?,
        (None, Some(json)) => registry_from_json(json, |json| {
            parse_type_registry(json).map(ResidentRegistry::Types)
        })?,
        (None, None) => return Ok(None),
    };
    match registry {
        ResidentRegistry::Types(registry) => Ok(Some(registry)),
        ResidentRegistry::Declarative(_) => {
            bail!("typeRegistryId names a declarative registry, not a type registry")
        }
    }
}

/// The declarative registry the options name, by id or by JSON.
fn requested_declarative_registry(
    opts: &ExpandOptions,
) -> Result<Option<Arc<ProjectDeclarativeRegistry>>> {
    let registry = match (
        opts.declarative_registry_id,
        &opts.declarative_registry_json,
    ) {
        (Some(_), Some(_)) => {
            bail!("pass one of declarativeRegistryId and declarativeRegistryJson, not both")
        }
        (Some(id), None) => registry_by_id(id)?,
        (None, Some(json)) => registry_from_json(json, |json| {
            parse_declarative_registry(json).map(ResidentRegistry::Declarative)
        })?,
        (None, None) => return Ok(None),
    };
    match registry {
        ResidentRegistry::Declarative(registry) => Ok(Some(registry)),
        ResidentRegistry::Types(_) => {
            bail!("declarativeRegistryId names a type registry, not a declarative registry")
        }
    }
}

fn apply_options(macro_host: &mut MacroExpander, options: &Option<ExpandOptions>) -> Result<()> {
    let Some(opts) = options else {
        return Ok(());
    };
    if let Some(keep) = opts.keep_decorators {
        macro_host.set_keep_decorators(keep);
    }
    if let Some(modules) = &opts.external_decorator_modules {
        macro_host.set_external_decorator_modules(modules.clone());
    }
    if let Some(registry) = requested_type_registry(opts)? {
        macro_host.set_type_registry(registry);
    }
    if let Some(registry) = requested_declarative_registry(opts)? {
        macro_host.set_declarative_registry_shared(Some(registry));
    }
    macro_host.set_build_mode(crate::host::declarative::BuildMode::from_option(
        opts.build_mode.as_deref(),
    ));
    if let Some(path) = opts.config_path.as_ref() {
        let config = CONFIG_CACHE
            .get(path)
            .map(|cached| Arc::clone(&cached.config))
            .ok_or_else(|| {
                anyhow!("config {path} was passed to expand before loadConfig parsed it")
            })?;
        crate::host::import_registry::set_foreign_types_from(&config);
        crate::host::import_registry::with_registry_mut(|registry| {
            registry.config_imports = config
                .config_imports
                .iter()
                .map(|(name, info)| (name.clone(), info.source.clone()))
                .collect();
        });
        macro_host.set_project_config(config);
    }
    Ok(())
}

/// The processed classes as JSON, when there are any.
fn serialize_metadata(
    classes: &[crate::ts_syn::abi::ir::ClassIR],
) -> std::result::Result<Option<String>, serde_json::Error> {
    if classes.is_empty() {
        Ok(None)
    } else {
        serde_json::to_string(classes).map(Some)
    }
}

/// Converts an expansion into the result the bindings return, with its
/// classes as metadata when `emit_metadata` is set.
fn expansion_result(
    expansion: crate::host::expand::MacroExpansion,
    emit_metadata: bool,
) -> Result<ExpandResult> {
    let metadata = if emit_metadata {
        serialize_metadata(&expansion.classes)
            .context("failed to serialize the expansion's class metadata")?
    } else {
        None
    };
    let diagnostics = expansion
        .diagnostics
        .into_iter()
        .map(|diagnostic| MacroDiagnostic {
            level: format!("{:?}", diagnostic.level).to_lowercase(),
            message: diagnostic.message,
            start: diagnostic.span.map(|span| span.start),
            end: diagnostic.span.map(|span| span.end),
        })
        .collect();

    let source_mapping = expansion.source_mapping.map(|mapping| SourceMappingResult {
        segments: mapping
            .segments
            .into_iter()
            .map(|segment| MappingSegmentResult {
                original_start: segment.original_start,
                original_end: segment.original_end,
                expanded_start: segment.expanded_start,
                expanded_end: segment.expanded_end,
            })
            .collect(),
        generated_regions: mapping
            .generated_regions
            .into_iter()
            .map(|region| GeneratedRegionResult {
                start: region.start,
                end: region.end,
                source_macro: region.source_macro,
            })
            .collect(),
    });

    let mut result = ExpandResult {
        metadata,
        code: expansion.code,
        types: expansion.type_output,
        diagnostics,
        source_mapping,
        buildtime_dependencies: expansion
            .buildtime_dependencies
            .iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect(),
    };
    inject_log_comments(&mut result);
    Ok(result)
}

// ============================================================================
// Public Interface
// ============================================================================

pub(crate) fn expand_inner(
    code: &str,
    filepath: &str,
    options: Option<ExpandOptions>,
) -> Result<ExpandResult> {
    if !has_macro_annotations(code, filepath) {
        return Ok(ExpandResult::unchanged(code));
    }

    let mut macro_host = create_expander()?;
    apply_options(&mut macro_host, &options)?;
    let expansion = macro_host
        .expand_source(code, filepath)
        .map_err(anyhow::Error::from)?;
    let emit_metadata = options
        .as_ref()
        .and_then(|options| options.emit_metadata)
        .unwrap_or(true);
    expansion_result(expansion, emit_metadata)
}

// ============================================================================
// Log Level Support
// ============================================================================

/// Log levels for MF_LOG env var, ordered by verbosity.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum LogLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

/// `MF_LOG`'s level, read once rather than on every expansion.
static LOG_LEVEL: LazyLock<LogLevel> = LazyLock::new(parse_log_level);

fn parse_log_level() -> LogLevel {
    match std::env::var("MF_LOG").ok().as_deref() {
        Some("error") => LogLevel::Error,
        Some("warn") => LogLevel::Warn,
        Some("info") => LogLevel::Info,
        Some("debug") => LogLevel::Debug,
        Some("trace") => LogLevel::Trace,
        Some("1" | "true") => LogLevel::Info,
        _ => LogLevel::Off,
    }
}

/// Inject trace/debug diagnostics as comments into expanded code.
/// Diagnostics with spans are inserted above the relevant line;
/// those without spans go into a block comment at the top.
fn inject_log_comments(result: &mut ExpandResult) {
    let level = *LOG_LEVEL;
    if level == LogLevel::Off {
        return;
    }

    let trace_diags: Vec<_> = result
        .diagnostics
        .iter()
        .filter(|d| {
            d.message.starts_with("[trace]") && level >= LogLevel::Trace
                || d.level == "error" && level >= LogLevel::Error
                || d.level == "warning" && level >= LogLevel::Warn
                || d.level == "info" && level >= LogLevel::Info
        })
        .collect();

    if trace_diags.is_empty() {
        return;
    }

    // Separate positioned vs unpositioned
    let mut positioned: Vec<(u32, &str)> = Vec::new();
    let mut top_lines: Vec<String> = Vec::new();

    for d in &trace_diags {
        if let Some(start) = d.start {
            positioned.push((start, &d.message));
        } else {
            top_lines.push(format!("// {}", d.message));
        }
    }

    // Build the top block
    let mut header = String::new();
    if !top_lines.is_empty() {
        header.push_str("/*\n * MF_LOG output\n");
        for line in &top_lines {
            header.push_str(" * ");
            header.push_str(line.trim_start_matches("// "));
            header.push('\n');
        }
        header.push_str(" */\n");
    }

    // Insert positioned comments (process in reverse order to preserve offsets)
    let mut code = result.code.clone();
    positioned.sort_by_key(|p| std::cmp::Reverse(p.0));
    for (offset, msg) in &positioned {
        let offset = *offset as usize;
        if offset <= code.len() {
            // Find the start of the line containing this offset
            let line_start = code[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let indent = &code[line_start..offset]
                .chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>();
            let comment = format!("{}// {}\n", indent, msg);
            code.insert_str(line_start, &comment);
        }
    }

    if !header.is_empty() {
        code.insert_str(0, &header);
    }

    result.code = code;
}

// ============================================================================
// Inner Logic (Optimized)
// ============================================================================

/// Whether `source` may contain anything the engine expands: a `@buildtime`
/// marker, a declarative macro definition or `import macro` comment, a
/// `$name` call in code, or a `@derive(` / attribute-macro tag at the start of
/// a JSDoc line. Prose mentions, fenced doc examples, strings and other
/// comments don't count.
///
/// This is the one gate every integration uses to skip files cheaply.
/// `file_name` is the source's path: in a Svelte module the compiler's runes
/// (`$state`, `$derived`, ...) are reserved and never macro calls.
pub fn has_macro_annotations(source: &str, file_name: &str) -> bool {
    source.contains("@buildtime")
        || source.contains(crate::package::RULES)
        || crate::ts_syn::jsdoc::may_have_macro_import(source)
        || has_dollar_call(source, file_name)
        || has_attribute_tag(source)
}

/// The names the Svelte compiler reserves as runes.
const SVELTE_RUNES: [&str; 7] = [
    "$state",
    "$derived",
    "$effect",
    "$props",
    "$bindable",
    "$inspect",
    "$host",
];

/// Whether `file_name` is a module the Svelte compiler reads, where runes are
/// reserved: a component or a `.svelte.ts` / `.svelte.js` module.
fn reserves_runes(file_name: &str) -> bool {
    [".svelte", ".svelte.ts", ".svelte.js"]
        .iter()
        .any(|suffix| file_name.ends_with(suffix))
}

/// The macros `source` imports through `/** import macro { A } from "pkg" */`
/// comments, as macro name to module. Directives come from the parser's
/// comments, so the same text in a string or a doc example does not count. A
/// file mid-edit still yields the directives the parser reached.
pub fn macro_imports(source: &str, file_name: &str) -> std::collections::HashMap<String, String> {
    if !crate::ts_syn::jsdoc::may_have_macro_import(source) {
        return std::collections::HashMap::new();
    }
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, crate::source_type::for_path(file_name)).parse();
    crate::ts_syn::import_registry::macro_imports_in_comments(&parsed.program.comments, source)
}

/// The tags the attribute and derive passes act on, as a JSDoc line starts.
const ATTRIBUTE_TAGS: [&str; 5] = [
    "@derive(",
    "@cfg(",
    "@deprecated",
    "@mustUse",
    "@nonExhaustive",
];

/// Whether a `/** … */` block in `source` has a line that starts with one of
/// [`ATTRIBUTE_TAGS`], outside a fenced example. Lowering and attribute
/// discovery read tags only from JSDoc blocks.
fn has_attribute_tag(source: &str) -> bool {
    if !ATTRIBUTE_TAGS
        .iter()
        .any(|tag| source.contains(tag.trim_end_matches('(')))
    {
        return false;
    }
    let mut rest = source;
    while let Some(open) = rest.find("/**") {
        let body_start = open + 3;
        let close = rest[open + 2..]
            .find("*/")
            .map_or(rest.len(), |offset| open + 2 + offset);
        let body = rest.get(body_start..close).unwrap_or_default();
        let mut in_example = false;
        for line in body.lines() {
            let trimmed = line
                .trim()
                .trim_start_matches('*')
                .trim_end_matches('*')
                .trim();
            if trimmed.starts_with("```") {
                in_example = !in_example;
            } else if !in_example && ATTRIBUTE_TAGS.iter().any(|tag| trimmed.starts_with(tag)) {
                return true;
            }
        }
        rest = rest.get(close + 2..).unwrap_or_default();
    }
    false
}

/// Keywords after which a `/` starts a regular expression rather than
/// dividing.
const REGEX_AFTER_KEYWORDS: [&[u8]; 14] = [
    b"return",
    b"typeof",
    b"case",
    b"do",
    b"else",
    b"in",
    b"of",
    b"new",
    b"delete",
    b"void",
    b"throw",
    b"yield",
    b"await",
    b"instanceof",
];

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}

/// Whether `source`, the text of `file_name`, has a `$name` identifier in
/// code, the shape of a call macro: not inside a string, template text,
/// regular expression or comment, not part of a longer identifier, not a `$$`
/// name, and not a rune in a Svelte module.
///
/// The scan is a light lexer. Where it cannot tell (a `/` that might start a
/// regular expression), it errs toward reading code, and a quoted string or
/// regular expression ends at its line, so a misread hides at most one line.
pub(crate) fn has_dollar_call(source: &str, file_name: &str) -> bool {
    let bytes = source.as_bytes();
    let runes_reserved = reserves_runes(file_name);
    if !bytes.contains(&b'$') {
        return false;
    }
    // Brace depths at which each open `${` of a template literal started.
    let mut templates: Vec<usize> = Vec::new();
    let mut depth = 0usize;
    // The last byte of code that was not whitespace, for regex detection.
    let mut last_code = 0u8;
    let mut last_code_at = 0usize;
    let mut at = 0usize;
    while at < bytes.len() {
        let byte = bytes[at];
        let next = bytes.get(at + 1).copied().unwrap_or(0);
        match byte {
            b'/' if next == b'/' => {
                at = skip_to_line_end(bytes, at);
                continue;
            }
            b'/' if next == b'*' => {
                at = source[at + 2..]
                    .find("*/")
                    .map_or(bytes.len(), |offset| at + 2 + offset + 2);
                continue;
            }
            b'/' if starts_regex(bytes, last_code, last_code_at) => {
                at = skip_regex(bytes, at + 1);
                last_code = b'/';
                last_code_at = at.saturating_sub(1);
                continue;
            }
            b'\'' | b'"' => {
                at = skip_quoted(bytes, at + 1, byte);
                last_code = byte;
                last_code_at = at.saturating_sub(1);
                continue;
            }
            b'`' => {
                match skip_template_text(bytes, at + 1) {
                    TemplateEnd::Closed(after) => at = after,
                    TemplateEnd::Expression(after) => {
                        templates.push(depth);
                        depth += 1;
                        at = after;
                    }
                }
                last_code = b'`';
                last_code_at = at.saturating_sub(1);
                continue;
            }
            b'{' => depth += 1,
            b'}' => {
                depth = depth.saturating_sub(1);
                if templates.last() == Some(&depth) {
                    templates.pop();
                    match skip_template_text(bytes, at + 1) {
                        TemplateEnd::Closed(after) => at = after,
                        TemplateEnd::Expression(after) => {
                            templates.push(depth);
                            depth += 1;
                            at = after;
                        }
                    }
                    last_code = b'`';
                    last_code_at = at.saturating_sub(1);
                    continue;
                }
            }
            b'$' => {
                let follows_identifier = at > 0 && is_identifier_byte(bytes[at - 1]);
                if !follows_identifier && next.is_ascii_alphabetic() {
                    let end = bytes[at + 1..]
                        .iter()
                        .position(|&byte| !is_identifier_byte(byte))
                        .map_or(bytes.len(), |length| at + 1 + length);
                    if !(runes_reserved && SVELTE_RUNES.contains(&&source[at..end])) {
                        return true;
                    }
                }
            }
            _ => {}
        }
        if !byte.is_ascii_whitespace() {
            last_code = byte;
            last_code_at = at;
        }
        at += 1;
    }
    false
}

/// Whether a `/` after `last_code` (ending at `last_code_at`) starts a
/// regular expression.
fn starts_regex(bytes: &[u8], last_code: u8, last_code_at: usize) -> bool {
    match last_code {
        0 => true,
        b'(' | b',' | b'=' | b':' | b'[' | b'!' | b'&' | b'|' | b'?' | b'{' | b'}' | b';'
        | b'+' | b'-' | b'*' | b'%' | b'<' | b'>' | b'~' | b'^' => true,
        byte if is_identifier_byte(byte) => {
            let start = bytes[..=last_code_at]
                .iter()
                .rposition(|&byte| !is_identifier_byte(byte))
                .map_or(0, |before| before + 1);
            REGEX_AFTER_KEYWORDS.contains(&&bytes[start..=last_code_at])
        }
        _ => false,
    }
}

fn skip_to_line_end(bytes: &[u8], from: usize) -> usize {
    bytes[from..]
        .iter()
        .position(|&byte| byte == b'\n')
        .map_or(bytes.len(), |offset| from + offset)
}

/// Past the quoted string whose body starts at `from`, or to the end of its
/// line when it is unterminated there.
fn skip_quoted(bytes: &[u8], from: usize, quote: u8) -> usize {
    let mut at = from;
    while at < bytes.len() {
        match bytes[at] {
            b'\\' => at += 2,
            b'\n' => return at,
            byte if byte == quote => return at + 1,
            _ => at += 1,
        }
    }
    bytes.len()
}

/// Past the regular expression whose body starts at `from`, or to the end of
/// its line when it is unterminated there.
fn skip_regex(bytes: &[u8], from: usize) -> usize {
    let mut at = from;
    let mut in_class = false;
    while at < bytes.len() {
        match bytes[at] {
            b'\\' => at += 2,
            b'\n' => return at,
            b'[' => {
                in_class = true;
                at += 1;
            }
            b']' => {
                in_class = false;
                at += 1;
            }
            b'/' if !in_class => return at + 1,
            _ => at += 1,
        }
    }
    bytes.len()
}

/// Where a run of template literal text ends.
enum TemplateEnd {
    /// The closing backtick; the position just past it.
    Closed(usize),
    /// A `${`; the position just past it.
    Expression(usize),
}

fn skip_template_text(bytes: &[u8], from: usize) -> TemplateEnd {
    let mut at = from;
    while at < bytes.len() {
        match bytes[at] {
            b'\\' => at += 2,
            b'`' => return TemplateEnd::Closed(at + 1),
            b'$' if bytes.get(at + 1) == Some(&b'{') => return TemplateEnd::Expression(at + 2),
            _ => at += 1,
        }
    }
    TemplateEnd::Closed(bytes.len())
}
