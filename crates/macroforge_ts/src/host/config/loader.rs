use super::super::error::{MacroError, Result};
use super::{CONFIG_CACHE, CONFIG_FILES, CachedConfig, MacroforgeConfig, has_project_manifest};
use std::collections::HashMap;
use std::path::Path;

use crate::host::file_stamp::FileStamp;

use macroforge_ts_syn::config::{
    ForeignHandler, ForeignTypeAlias, ForeignTypeConfig, HandlerSite, ImportInfo,
};
use oxc::span::GetSpan;

/// Loader/parser for MacroforgeConfig files.
pub struct MacroforgeConfigLoader;

/// A config file as last parsed, with the stamps it and its bases had then.
struct ParsedConfigFile {
    stamp: FileStamp,
    dependencies: Vec<(std::path::PathBuf, FileStamp)>,
    config: MacroforgeConfig,
}

/// The current stamp of each of `paths`, which were just read.
fn stamp_all(paths: Vec<std::path::PathBuf>) -> Result<Vec<(std::path::PathBuf, FileStamp)>> {
    paths
        .into_iter()
        .map(|path| {
            let stamp = FileStamp::of(&path)?;
            Ok((path, stamp))
        })
        .collect()
}

impl MacroforgeConfigLoader {
    /// Parse a macroforge.config.js/ts file, following any base config it
    /// extends, spreads or re-exports. Also returns the files besides
    /// `filepath` it was built from, so a cache can tell when a base changes.
    pub(crate) fn from_config_file_with_dependencies(
        content: &str,
        filepath: &str,
    ) -> Result<(MacroforgeConfig, Vec<std::path::PathBuf>)> {
        super::resolve::resolve_config(content, filepath)
            .map(|resolved| (resolved.config, resolved.dependencies))
    }

    /// Load configuration from cache or parse from file content.
    pub fn load_and_cache(content: &str, filepath: &str) -> Result<MacroforgeConfig> {
        use std::hash::{Hash, Hasher};

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        content.hash(&mut hasher);
        let content_hash = hasher.finish();
        if let Some(cached) = CONFIG_CACHE.get(filepath)
            && cached.content_hash == content_hash
            && cached.dependencies_unchanged()
        {
            return Ok(MacroforgeConfig::clone(&cached.config));
        }

        let (config, dependencies) = Self::from_config_file_with_dependencies(content, filepath)?;
        CONFIG_CACHE.insert(
            filepath.to_string(),
            CachedConfig {
                content_hash,
                config: std::sync::Arc::new(config.clone()),
                dependencies: stamp_all(dependencies)?,
            },
        );

        Ok(config)
    }

    pub fn find_with_root() -> Result<Option<(MacroforgeConfig, std::path::PathBuf)>> {
        let current_dir = std::env::current_dir()?;
        Self::find_config_in_ancestors(&current_dir)
    }

    /// The config governing `start_path`, with the directory holding it. The
    /// root is absolute even for a relative `start_path`, since callers walk
    /// its ancestors for `node_modules`.
    pub fn find_with_root_from_path(
        start_path: &Path,
    ) -> Result<Option<(MacroforgeConfig, std::path::PathBuf)>> {
        let start_path = std::path::absolute(start_path)?;
        let start_dir = if start_path.is_file() {
            start_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| start_path.clone())
        } else {
            start_path
        };
        Self::find_config_in_ancestors(&start_dir)
    }

    /// The config file at `config_path`, parsed once for as long as it and
    /// every base config it builds on are unchanged. A file that cannot be
    /// stamped is parsed on every call; reading it reports why it is
    /// unreadable, if it is.
    fn load_config_file(config_path: &Path) -> Result<MacroforgeConfig> {
        static PARSED: std::sync::LazyLock<
            std::sync::Mutex<HashMap<std::path::PathBuf, ParsedConfigFile>>,
        > = std::sync::LazyLock::new(std::sync::Mutex::default);

        let stamp = FileStamp::of(config_path).ok();
        if let Some(stamp) = stamp {
            // Entries are inserted whole, so a poisoned map is still consistent.
            let parsed = PARSED
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(cached) = parsed.get(config_path)
                && cached.stamp == stamp
                && cached.dependencies.iter().all(|(path, dependency)| {
                    FileStamp::of(path).is_ok_and(|current| current == *dependency)
                })
            {
                return Ok(cached.config.clone());
            }
        }
        let content = std::fs::read_to_string(config_path)?;
        let (config, dependencies) = Self::from_config_file_with_dependencies(
            &content,
            config_path.to_string_lossy().as_ref(),
        )?;
        if let Some(stamp) = stamp {
            let dependencies = stamp_all(dependencies)?;
            PARSED
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .insert(
                    config_path.to_path_buf(),
                    ParsedConfigFile {
                        stamp,
                        dependencies,
                        config: config.clone(),
                    },
                );
        }
        Ok(config)
    }

    fn find_config_in_ancestors(
        start_dir: &Path,
    ) -> Result<Option<(MacroforgeConfig, std::path::PathBuf)>> {
        let mut current = start_dir.to_path_buf();

        loop {
            for config_name in CONFIG_FILES {
                let config_path = current.join(config_name);
                if config_path.exists() {
                    let config = Self::load_config_file(&config_path)?;
                    return Ok(Some((config, current.clone())));
                }
            }

            if has_project_manifest(&current) {
                break;
            }

            if !current.pop() {
                break;
            }
        }

        Ok(None)
    }
}

/// Every import `program` declares, by local name.
pub(super) fn collect_imports(program: &oxc::ast::ast::Program<'_>) -> HashMap<String, ImportInfo> {
    use oxc::ast::ast::{ImportDeclarationSpecifier, Statement};

    let mut imports = HashMap::new();
    for stmt in &program.body {
        let Statement::ImportDeclaration(import) = stmt else {
            continue;
        };
        let Some(specifiers) = &import.specifiers else {
            continue;
        };
        let source = import.source.value.to_string();
        for specifier in specifiers {
            let (local, name) = match specifier {
                ImportDeclarationSpecifier::ImportSpecifier(named) => (
                    named.local.name.to_string(),
                    named.imported.name().to_string(),
                ),
                ImportDeclarationSpecifier::ImportDefaultSpecifier(default) => {
                    (default.local.name.to_string(), "default".to_string())
                }
                ImportDeclarationSpecifier::ImportNamespaceSpecifier(namespace) => {
                    (namespace.local.name.to_string(), "*".to_string())
                }
            };
            imports.insert(
                local,
                ImportInfo {
                    name,
                    source: source.clone(),
                },
            );
        }
    }
    imports
}

/// Sets on `layer` the config field `key` names, from `value`. Keys the
/// config does not know are ignored.
pub(super) fn parse_block_property(
    layer: &mut super::resolve::ConfigLayer,
    key: &str,
    value: &oxc::ast::ast::Expression<'_>,
) -> Result<()> {
    match key {
        "keepDecorators" => {
            layer.keep_decorators = Some(get_bool_value(value).unwrap_or(false));
        }
        "generateConvenienceConst" => {
            layer.generate_convenience_const = Some(get_bool_value(value).unwrap_or(true));
        }
        "cfg" => {
            if let Some(map) = object_to_json_map(value) {
                layer.cfg = Some(super::attribute_blocks::parse_cfg_flags(&map));
            }
        }
        "deprecated" => {
            if let Some(map) = object_to_json_map(value) {
                layer.deprecated = Some(super::attribute_blocks::parse_deprecated_config(&map));
            }
        }
        "mustUse" => {
            if let Some(map) = object_to_json_map(value) {
                layer.must_use = Some(super::attribute_blocks::parse_must_use_config(&map));
            }
        }
        "nonExhaustive" => {
            if let Some(map) = object_to_json_map(value) {
                layer.non_exhaustive =
                    Some(super::attribute_blocks::parse_non_exhaustive_config(&map));
            }
        }
        "buildtime" => {
            if let Some(map) = object_to_json_map(value) {
                layer.buildtime = Some(super::attribute_blocks::parse_buildtime_config(&map)?);
            }
        }
        _ => {}
    }
    Ok(())
}

/// Convert an OXC object-expression node into a `serde_json::Map` so the
/// shared attribute-block parsers in [`super::attribute_blocks`] can handle
/// it. Returns `None` when the expression isn't an object literal.
fn object_to_json_map(
    expr: &oxc::ast::ast::Expression<'_>,
) -> Option<serde_json::Map<String, serde_json::Value>> {
    match expr_to_json(expr)? {
        serde_json::Value::Object(map) => Some(map),
        _ => None,
    }
}

/// Convert a literal-ish OXC expression into a `serde_json::Value`.
fn expr_to_json(expr: &oxc::ast::ast::Expression<'_>) -> Option<serde_json::Value> {
    use oxc::ast::ast::Expression;
    match expr {
        Expression::StringLiteral(s) => Some(serde_json::Value::String(s.value.to_string())),
        Expression::BooleanLiteral(b) => Some(serde_json::Value::Bool(b.value)),
        Expression::NullLiteral(_) => Some(serde_json::Value::Null),
        Expression::NumericLiteral(n) => {
            serde_json::Number::from_f64(n.value).map(serde_json::Value::Number)
        }
        Expression::ArrayExpression(arr) => {
            let items = arr
                .elements
                .iter()
                .filter_map(|e| match e {
                    oxc::ast::ast::ArrayExpressionElement::SpreadElement(_) => None,
                    oxc::ast::ast::ArrayExpressionElement::Elision(_) => None,
                    other => other.as_expression().and_then(expr_to_json),
                })
                .collect();
            Some(serde_json::Value::Array(items))
        }
        Expression::ObjectExpression(obj) => {
            let mut map = serde_json::Map::new();
            for prop in &obj.properties {
                let oxc::ast::ast::ObjectPropertyKind::ObjectProperty(prop) = prop else {
                    continue;
                };
                if prop.kind != oxc::ast::ast::PropertyKind::Init {
                    continue;
                }
                let key = get_prop_key(&prop.key, "");
                if let Some(val) = expr_to_json(&prop.value) {
                    map.insert(key, val);
                }
            }
            Some(serde_json::Value::Object(map))
        }
        _ => None,
    }
}

/// One `foreignTypes` entry of the config at `location`. A key it cannot read
/// is an error, so an option that would be ignored never goes unnoticed.
pub(super) fn parse_single_foreign_type(
    location: &str,
    name: &str,
    obj: &oxc::ast::ast::ObjectExpression<'_>,
    source: &str,
    module: &Path,
) -> Result<ForeignTypeConfig> {
    let mut ft = ForeignTypeConfig {
        name: name.to_string(),
        ..Default::default()
    };

    for prop in &obj.properties {
        let oxc::ast::ast::ObjectPropertyKind::ObjectProperty(prop) = prop else {
            return Err(MacroError::InvalidConfig(format!(
                "{location}: the foreign type `{name}` cannot take a spread; write its keys out"
            )));
        };
        let key = get_prop_key(&prop.key, source);
        if prop.kind != oxc::ast::ast::PropertyKind::Init {
            return Err(MacroError::InvalidConfig(format!(
                "{location}: `{key}` of the foreign type `{name}` must be a plain property, \
                 not a getter or setter"
            )));
        }

        if let Some(handler) = ForeignHandler::from_key(&key) {
            let span = prop.value.span();
            ft.handler_sites.push(HandlerSite {
                handler,
                module: module.to_path_buf(),
                start: span.start,
                end: span.end,
                identifier: match &prop.value {
                    oxc::ast::ast::Expression::Identifier(ident) => Some(ident.name.to_string()),
                    _ => None,
                },
            });
        }
        match key.as_str() {
            "from" => {
                ft.from = extract_string_or_array(&prop.value);
            }
            "encode" => {
                ft.encode_expr = Some(source_slice(source, prop.value.span()));
            }
            "decode" => {
                ft.decode_expr = Some(source_slice(source, prop.value.span()));
            }
            "default" => {
                ft.default_expr = Some(source_slice(source, prop.value.span()));
            }
            "hasShape" => {
                ft.has_shape_expr = Some(source_slice(source, prop.value.span()));
            }
            "aliases" => {
                ft.aliases = parse_aliases_array(&prop.value, source);
            }
            unknown => {
                return Err(MacroError::InvalidConfig(format!(
                    "{location}: the foreign type `{name}` has an unknown key `{unknown}`; \
                     expected `from`, `aliases`, `encode`, `decode`, `default` or `hasShape`"
                )));
            }
        }
    }

    Ok(ft)
}

fn parse_aliases_array(
    expr: &oxc::ast::ast::Expression<'_>,
    source: &str,
) -> Vec<ForeignTypeAlias> {
    let oxc::ast::ast::Expression::ArrayExpression(array) = expr else {
        return Vec::new();
    };

    let mut aliases = Vec::new();
    for element in &array.elements {
        let oxc::ast::ast::ArrayExpressionElement::ObjectExpression(obj) = element else {
            continue;
        };
        let mut alias = ForeignTypeAlias::default();
        for prop in &obj.properties {
            let oxc::ast::ast::ObjectPropertyKind::ObjectProperty(prop) = prop else {
                continue;
            };
            if prop.kind != oxc::ast::ast::PropertyKind::Init {
                continue;
            }
            match get_prop_key(&prop.key, source).as_str() {
                "name" => alias.name = get_string_value(&prop.value).unwrap_or_default(),
                "from" => alias.from = get_string_value(&prop.value).unwrap_or_default(),
                _ => {}
            }
        }
        if !alias.name.is_empty() && !alias.from.is_empty() {
            aliases.push(alias);
        }
    }
    aliases
}

fn extract_string_or_array(expr: &oxc::ast::ast::Expression<'_>) -> Vec<String> {
    match expr {
        oxc::ast::ast::Expression::StringLiteral(string) => vec![string.value.to_string()],
        oxc::ast::ast::Expression::ArrayExpression(array) => array
            .elements
            .iter()
            .filter_map(|element| match element {
                oxc::ast::ast::ArrayExpressionElement::StringLiteral(string) => {
                    Some(string.value.to_string())
                }
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn get_bool_value(expr: &oxc::ast::ast::Expression<'_>) -> Option<bool> {
    match expr {
        oxc::ast::ast::Expression::BooleanLiteral(boolean) => Some(boolean.value),
        _ => None,
    }
}

fn get_string_value(expr: &oxc::ast::ast::Expression<'_>) -> Option<String> {
    match expr {
        oxc::ast::ast::Expression::StringLiteral(string) => Some(string.value.to_string()),
        _ => None,
    }
}

pub(super) fn get_prop_key(key: &oxc::ast::ast::PropertyKey<'_>, source: &str) -> String {
    match key {
        oxc::ast::ast::PropertyKey::StaticIdentifier(ident) => ident.name.to_string(),
        oxc::ast::ast::PropertyKey::StringLiteral(string) => string.value.to_string(),
        _ => source_slice(source, key.span()),
    }
}

pub(super) fn source_slice(source: &str, span: oxc::span::Span) -> String {
    source
        .get(span.start as usize..span.end as usize)
        .unwrap_or("")
        .to_string()
}
