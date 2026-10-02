use super::super::error::Result;
use super::{CONFIG_CACHE, CONFIG_FILES, CachedConfig, MacroforgeConfig};
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::host::file_stamp::FileStamp;

use macroforge_ts_syn::config::{ForeignTypeAlias, ForeignTypeConfig, ImportInfo};
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

    pub fn find_with_root_from_path(
        start_path: &Path,
    ) -> Result<Option<(MacroforgeConfig, std::path::PathBuf)>> {
        let start_dir = if start_path.is_file() {
            start_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| start_path.to_path_buf())
        } else {
            start_path.to_path_buf()
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

            if current.join("package.json").exists() {
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

pub(super) fn parse_single_foreign_type(
    name: &str,
    obj: &oxc::ast::ast::ObjectExpression<'_>,
    imports: &HashMap<String, ImportInfo>,
    source: &str,
) -> Result<ForeignTypeConfig> {
    let mut ft = ForeignTypeConfig {
        name: name.to_string(),
        ..Default::default()
    };

    for prop in &obj.properties {
        let oxc::ast::ast::ObjectPropertyKind::ObjectProperty(prop) = prop else {
            continue;
        };
        if prop.kind != oxc::ast::ast::PropertyKind::Init {
            continue;
        }

        let key = get_prop_key(&prop.key, source);
        match key.as_str() {
            "from" => {
                ft.from = extract_string_or_array(&prop.value);
            }
            "serialize" => {
                let (expr, import) = extract_function_expr(&prop.value, imports, source);
                ft.serialize_expr = expr;
                ft.serialize_import = import;
            }
            "deserialize" => {
                let (expr, import) = extract_function_expr(&prop.value, imports, source);
                ft.deserialize_expr = expr;
                ft.deserialize_import = import;
            }
            "default" => {
                let (expr, import) = extract_function_expr(&prop.value, imports, source);
                ft.default_expr = expr;
                ft.default_import = import;
            }
            "hasShape" => {
                let (expr, import) = extract_function_expr(&prop.value, imports, source);
                ft.has_shape_expr = expr;
                ft.has_shape_import = import;
            }
            "aliases" => {
                ft.aliases = parse_aliases_array(&prop.value, source);
            }
            _ => {}
        }
    }

    let mut namespaces = HashSet::new();
    for expr in [
        ft.serialize_expr.as_deref(),
        ft.deserialize_expr.as_deref(),
        ft.default_expr.as_deref(),
        ft.has_shape_expr.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        namespaces.extend(extract_expression_namespaces(expr));
    }
    ft.expression_namespaces = namespaces.into_iter().collect();

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

fn extract_function_expr(
    expr: &oxc::ast::ast::Expression<'_>,
    imports: &HashMap<String, ImportInfo>,
    source: &str,
) -> (Option<String>, Option<ImportInfo>) {
    match expr {
        oxc::ast::ast::Expression::Identifier(ident) => {
            let name = ident.name.to_string();
            (Some(name.clone()), imports.get(&name).cloned())
        }
        _ => (Some(source_slice(source, expr.span())), None),
    }
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

fn extract_expression_namespaces(expr_str: &str) -> Vec<String> {
    use crate::ts_syn::parse_expr;
    use oxc::ast::ast::{Argument, Expression, ObjectPropertyKind, Statement};

    fn member_root(expr: &Expression<'_>) -> Option<String> {
        match expr {
            Expression::Identifier(ident) => Some(ident.name.to_string()),
            Expression::StaticMemberExpression(member) => member_root(&member.object),
            Expression::ComputedMemberExpression(member) => member_root(&member.object),
            _ => None,
        }
    }

    fn collect_statement(stmt: &Statement<'_>, namespaces: &mut HashSet<String>) {
        match stmt {
            Statement::ExpressionStatement(expr) => collect_expr(&expr.expression, namespaces),
            Statement::ReturnStatement(ret) => {
                if let Some(argument) = &ret.argument {
                    collect_expr(argument, namespaces);
                }
            }
            Statement::IfStatement(stmt) => {
                collect_expr(&stmt.test, namespaces);
                collect_statement(&stmt.consequent, namespaces);
                if let Some(alternate) = &stmt.alternate {
                    collect_statement(alternate, namespaces);
                }
            }
            Statement::BlockStatement(block) => {
                for stmt in &block.body {
                    collect_statement(stmt, namespaces);
                }
            }
            _ => {}
        }
    }

    fn collect_argument(arg: &Argument<'_>, namespaces: &mut HashSet<String>) {
        match arg {
            Argument::SpreadElement(spread) => collect_expr(&spread.argument, namespaces),
            other => {
                if let Some(expr) = other.as_expression() {
                    collect_expr(expr, namespaces);
                }
            }
        }
    }

    fn collect_expr(expr: &Expression<'_>, namespaces: &mut HashSet<String>) {
        match expr {
            Expression::StaticMemberExpression(member) => {
                if let Some(root) = member_root(&member.object) {
                    namespaces.insert(root);
                }
                collect_expr(&member.object, namespaces);
            }
            Expression::ComputedMemberExpression(member) => {
                if let Some(root) = member_root(&member.object) {
                    namespaces.insert(root);
                }
                collect_expr(&member.object, namespaces);
                collect_expr(&member.expression, namespaces);
            }
            Expression::CallExpression(call) => {
                collect_expr(&call.callee, namespaces);
                for arg in &call.arguments {
                    collect_argument(arg, namespaces);
                }
            }
            Expression::ArrowFunctionExpression(arrow) => match arrow.get_function_body() {
                Some(body) => {
                    for stmt in &body.statements {
                        collect_statement(stmt, namespaces);
                    }
                }
                None => {
                    if let Some(expr) = arrow.get_expression() {
                        collect_expr(expr, namespaces);
                    }
                }
            },
            Expression::FunctionExpression(function) => {
                if let Some(body) = &function.body {
                    for stmt in &body.statements {
                        collect_statement(stmt, namespaces);
                    }
                }
            }
            Expression::ParenthesizedExpression(paren) => {
                collect_expr(&paren.expression, namespaces)
            }
            Expression::BinaryExpression(binary) => {
                collect_expr(&binary.left, namespaces);
                collect_expr(&binary.right, namespaces);
            }
            Expression::ConditionalExpression(cond) => {
                collect_expr(&cond.test, namespaces);
                collect_expr(&cond.consequent, namespaces);
                collect_expr(&cond.alternate, namespaces);
            }
            Expression::NewExpression(new_expr) => {
                collect_expr(&new_expr.callee, namespaces);
                for arg in &new_expr.arguments {
                    collect_argument(arg, namespaces);
                }
            }
            Expression::ArrayExpression(array) => {
                for element in &array.elements {
                    match element {
                        oxc::ast::ast::ArrayExpressionElement::SpreadElement(spread) => {
                            collect_expr(&spread.argument, namespaces);
                        }
                        oxc::ast::ast::ArrayExpressionElement::Elision(_) => {}
                        _ => {}
                    }
                }
            }
            Expression::ObjectExpression(object) => {
                for prop in &object.properties {
                    if let ObjectPropertyKind::ObjectProperty(prop) = prop {
                        collect_expr(&prop.value, namespaces);
                    }
                }
            }
            Expression::TemplateLiteral(template) => {
                for expr in &template.expressions {
                    collect_expr(expr, namespaces);
                }
            }
            Expression::LogicalExpression(logical) => {
                collect_expr(&logical.left, namespaces);
                collect_expr(&logical.right, namespaces);
            }
            _ => {}
        }
    }

    let allocator = oxc::allocator::Allocator::default();
    let Ok(expr) = parse_expr(&allocator, expr_str) else {
        return Vec::new();
    };

    let mut namespaces = HashSet::new();
    collect_expr(&expr, &mut namespaces);
    namespaces.into_iter().collect()
}

pub(super) fn source_slice(source: &str, span: oxc::span::Span) -> String {
    source
        .get(span.start as usize..span.end as usize)
        .unwrap_or("")
        .to_string()
}
