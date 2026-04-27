use super::super::error::Result;
use super::{CONFIG_CACHE, CONFIG_FILES, MacroforgeConfig};
#[cfg(all(not(feature = "swc"), feature = "oxc"))]
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
use macroforge_ts_syn::config::{ForeignTypeAlias, ForeignTypeConfig, ImportInfo};
#[cfg(all(not(feature = "swc"), feature = "oxc"))]
use oxc::span::GetSpan;

/// Loader/parser for MacroforgeConfig files.
pub struct MacroforgeConfigLoader;

impl MacroforgeConfigLoader {
    /// Parse a macroforge.config.js/ts file and extract configuration.
    pub fn from_config_file(content: &str, filepath: &str) -> Result<MacroforgeConfig> {
        #[cfg(feature = "swc")]
        {
            use super::parser::{extract_default_export, extract_imports};
            use swc_core::{
                common::{FileName, SourceMap, sync::Lrc},
                ecma::parser::{EsSyntax, Lexer, Parser, StringInput, Syntax, TsSyntax},
            };

            let is_typescript = filepath.ends_with(".ts") || filepath.ends_with(".mts");

            let cm: Lrc<SourceMap> = Default::default();
            let fm = cm.new_source_file(
                FileName::Custom(filepath.to_string()).into(),
                content.to_string(),
            );

            let syntax = if is_typescript {
                Syntax::Typescript(TsSyntax {
                    tsx: false,
                    decorators: true,
                    ..Default::default()
                })
            } else {
                Syntax::Es(EsSyntax {
                    decorators: true,
                    ..Default::default()
                })
            };

            let lexer = Lexer::new(
                syntax,
                swc_core::ecma::ast::EsVersion::latest(),
                StringInput::from(&*fm),
                None,
            );
            let mut parser = Parser::new_from(lexer);

            let module = parser.parse_module().map_err(|e| {
                super::super::MacroError::InvalidConfig(format!("Parse error: {:?}", e))
            })?;

            let imports = extract_imports(&module);
            extract_default_export(&module, &imports, &cm)
        }

        #[cfg(all(not(feature = "swc"), feature = "oxc"))]
        {
            use oxc::ast::ast::{
                Argument, ExportDefaultDeclarationKind, ImportDeclarationSpecifier, Statement,
            };
            use oxc::parser::Parser;
            use oxc::span::SourceType;

            let source_type = if filepath.ends_with(".ts") || filepath.ends_with(".mts") {
                SourceType::ts()
            } else {
                SourceType::unambiguous()
            };

            let allocator = oxc::allocator::Allocator::default();
            let parsed = Parser::new(&allocator, content, source_type).parse();
            if !parsed.errors.is_empty() {
                return Err(super::super::MacroError::InvalidConfig(format!(
                    "Parse error: {}",
                    parsed
                        .errors
                        .into_iter()
                        .map(|diagnostic| diagnostic.to_string())
                        .collect::<Vec<_>>()
                        .join("; ")
                )));
            }

            let imports = {
                let mut imports = HashMap::new();
                for stmt in &parsed.program.body {
                    if let Statement::ImportDeclaration(import) = stmt
                        && let Some(specifiers) = &import.specifiers
                    {
                        let source = import.source.value.to_string();
                        for specifier in specifiers {
                            match specifier {
                                ImportDeclarationSpecifier::ImportSpecifier(named) => {
                                    imports.insert(
                                        named.local.name.to_string(),
                                        ImportInfo {
                                            name: named.imported.name().to_string(),
                                            source: source.clone(),
                                        },
                                    );
                                }
                                ImportDeclarationSpecifier::ImportDefaultSpecifier(default) => {
                                    imports.insert(
                                        default.local.name.to_string(),
                                        ImportInfo {
                                            name: "default".to_string(),
                                            source: source.clone(),
                                        },
                                    );
                                }
                                ImportDeclarationSpecifier::ImportNamespaceSpecifier(ns) => {
                                    imports.insert(
                                        ns.local.name.to_string(),
                                        ImportInfo {
                                            name: "*".to_string(),
                                            source: source.clone(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
                imports
            };

            for stmt in &parsed.program.body {
                if let Statement::ExportDefaultDeclaration(export) = stmt {
                    let config = match &export.declaration {
                        ExportDefaultDeclarationKind::ObjectExpression(obj) => {
                            parse_config_object_oxc(obj, &imports, content)?
                        }
                        ExportDefaultDeclarationKind::CallExpression(call) => {
                            let first = call.arguments.first();
                            match first {
                                Some(Argument::ObjectExpression(obj)) => {
                                    parse_config_object_oxc(obj, &imports, content)?
                                }
                                _ => MacroforgeConfig::default(),
                            }
                        }
                        _ => MacroforgeConfig::default(),
                    };
                    return Ok(config);
                }
            }

            Ok(MacroforgeConfig::default())
        }
    }

    /// Load configuration from cache or parse from file content.
    pub fn load_and_cache(content: &str, filepath: &str) -> Result<MacroforgeConfig> {
        if let Some(cached) = CONFIG_CACHE.get(filepath) {
            return Ok(cached.clone());
        }

        let config = Self::from_config_file(content, filepath)?;
        CONFIG_CACHE.insert(filepath.to_string(), config.clone());

        Ok(config)
    }

    pub fn get_cached(filepath: &str) -> Option<MacroforgeConfig> {
        CONFIG_CACHE.get(filepath).map(|c| c.clone())
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

    pub fn find_from_path(start_path: &Path) -> Result<Option<MacroforgeConfig>> {
        Ok(Self::find_with_root_from_path(start_path)?.map(|(cfg, _)| cfg))
    }

    fn find_config_in_ancestors(
        start_dir: &Path,
    ) -> Result<Option<(MacroforgeConfig, std::path::PathBuf)>> {
        let mut current = start_dir.to_path_buf();

        loop {
            for config_name in CONFIG_FILES {
                let config_path = current.join(config_name);
                if config_path.exists() {
                    let content = std::fs::read_to_string(&config_path)?;
                    let config =
                        Self::from_config_file(&content, config_path.to_string_lossy().as_ref())?;
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

    pub fn find_and_load() -> Result<Option<MacroforgeConfig>> {
        Ok(Self::find_with_root()?.map(|(cfg, _)| cfg))
    }
}

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn parse_config_object_oxc(
    obj: &oxc::ast::ast::ObjectExpression<'_>,
    imports: &HashMap<String, ImportInfo>,
    source: &str,
) -> Result<MacroforgeConfig> {
    let mut config = MacroforgeConfig::default();

    for prop in &obj.properties {
        let oxc::ast::ast::ObjectPropertyKind::ObjectProperty(prop) = prop else {
            continue;
        };
        if prop.kind != oxc::ast::ast::PropertyKind::Init {
            continue;
        }

        let key = get_prop_key_oxc(&prop.key, source);
        match key.as_str() {
            "keepDecorators" => {
                config.keep_decorators = get_bool_value_oxc(&prop.value).unwrap_or(false);
            }
            "generateConvenienceConst" => {
                config.generate_convenience_const = get_bool_value_oxc(&prop.value).unwrap_or(true);
            }
            "foreignTypes" => {
                if let oxc::ast::ast::Expression::ObjectExpression(ft_obj) = &prop.value {
                    config.foreign_types = parse_foreign_types_oxc(ft_obj, imports, source)?;
                }
            }
            "cfg" => {
                if let Some(map) = object_to_json_map_oxc(&prop.value) {
                    config.cfg = super::attribute_blocks::parse_cfg_flags(&map);
                }
            }
            "deprecated" => {
                if let Some(map) = object_to_json_map_oxc(&prop.value) {
                    config.deprecated = super::attribute_blocks::parse_deprecated_config(&map);
                }
            }
            "mustUse" => {
                if let Some(map) = object_to_json_map_oxc(&prop.value) {
                    config.must_use = super::attribute_blocks::parse_must_use_config(&map);
                }
            }
            "nonExhaustive" => {
                if let Some(map) = object_to_json_map_oxc(&prop.value) {
                    config.non_exhaustive =
                        super::attribute_blocks::parse_non_exhaustive_config(&map);
                }
            }
            _ => {}
        }
    }

    config.config_imports = imports.clone();
    Ok(config)
}

/// Convert an OXC object-expression node into a `serde_json::Map` so the
/// shared attribute-block parsers in [`super::attribute_blocks`] can handle
/// it. Returns `None` when the expression isn't an object literal.
#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn object_to_json_map_oxc(
    expr: &oxc::ast::ast::Expression<'_>,
) -> Option<serde_json::Map<String, serde_json::Value>> {
    match expr_to_json_oxc(expr)? {
        serde_json::Value::Object(map) => Some(map),
        _ => None,
    }
}

/// Convert a literal-ish OXC expression into a `serde_json::Value`. Mirrors
/// the SWC-side `parser.rs::expr_to_json` — the two exist only because the
/// AST types differ; the downstream shape is identical.
#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn expr_to_json_oxc(expr: &oxc::ast::ast::Expression<'_>) -> Option<serde_json::Value> {
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
                    other => other.as_expression().and_then(expr_to_json_oxc),
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
                let key = get_prop_key_oxc(&prop.key, "");
                if let Some(val) = expr_to_json_oxc(&prop.value) {
                    map.insert(key, val);
                }
            }
            Some(serde_json::Value::Object(map))
        }
        _ => None,
    }
}

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn parse_foreign_types_oxc(
    obj: &oxc::ast::ast::ObjectExpression<'_>,
    imports: &HashMap<String, ImportInfo>,
    source: &str,
) -> Result<Vec<ForeignTypeConfig>> {
    let mut foreign_types = Vec::new();

    for prop in &obj.properties {
        let oxc::ast::ast::ObjectPropertyKind::ObjectProperty(prop) = prop else {
            continue;
        };
        if prop.kind != oxc::ast::ast::PropertyKind::Init {
            continue;
        }

        let type_name = get_prop_key_oxc(&prop.key, source);
        if let oxc::ast::ast::Expression::ObjectExpression(type_obj) = &prop.value {
            foreign_types.push(parse_single_foreign_type_oxc(
                &type_name, type_obj, imports, source,
            )?);
        }
    }

    Ok(foreign_types)
}

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn parse_single_foreign_type_oxc(
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

        let key = get_prop_key_oxc(&prop.key, source);
        match key.as_str() {
            "from" => {
                ft.from = extract_string_or_array_oxc(&prop.value);
            }
            "serialize" => {
                let (expr, import) = extract_function_expr_oxc(&prop.value, imports, source);
                ft.serialize_expr = expr;
                ft.serialize_import = import;
            }
            "deserialize" => {
                let (expr, import) = extract_function_expr_oxc(&prop.value, imports, source);
                ft.deserialize_expr = expr;
                ft.deserialize_import = import;
            }
            "default" => {
                let (expr, import) = extract_function_expr_oxc(&prop.value, imports, source);
                ft.default_expr = expr;
                ft.default_import = import;
            }
            "hasShape" => {
                let (expr, import) = extract_function_expr_oxc(&prop.value, imports, source);
                ft.has_shape_expr = expr;
                ft.has_shape_import = import;
            }
            "aliases" => {
                ft.aliases = parse_aliases_array_oxc(&prop.value, source);
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
        namespaces.extend(extract_expression_namespaces_oxc(expr));
    }
    ft.expression_namespaces = namespaces.into_iter().collect();

    Ok(ft)
}

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn parse_aliases_array_oxc(
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
            match get_prop_key_oxc(&prop.key, source).as_str() {
                "name" => alias.name = get_string_value_oxc(&prop.value).unwrap_or_default(),
                "from" => alias.from = get_string_value_oxc(&prop.value).unwrap_or_default(),
                _ => {}
            }
        }
        if !alias.name.is_empty() && !alias.from.is_empty() {
            aliases.push(alias);
        }
    }
    aliases
}

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn extract_function_expr_oxc(
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

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn extract_string_or_array_oxc(expr: &oxc::ast::ast::Expression<'_>) -> Vec<String> {
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

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn get_bool_value_oxc(expr: &oxc::ast::ast::Expression<'_>) -> Option<bool> {
    match expr {
        oxc::ast::ast::Expression::BooleanLiteral(boolean) => Some(boolean.value),
        _ => None,
    }
}

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn get_string_value_oxc(expr: &oxc::ast::ast::Expression<'_>) -> Option<String> {
    match expr {
        oxc::ast::ast::Expression::StringLiteral(string) => Some(string.value.to_string()),
        _ => None,
    }
}

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn get_prop_key_oxc(key: &oxc::ast::ast::PropertyKey<'_>, source: &str) -> String {
    match key {
        oxc::ast::ast::PropertyKey::StaticIdentifier(ident) => ident.name.to_string(),
        oxc::ast::ast::PropertyKey::StringLiteral(string) => string.value.to_string(),
        _ => source_slice(source, key.span()),
    }
}

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn extract_expression_namespaces_oxc(expr_str: &str) -> Vec<String> {
    use crate::ts_syn::parse_oxc_expr;
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
            Expression::ArrowFunctionExpression(arrow) => {
                for stmt in &arrow.body.statements {
                    collect_statement(stmt, namespaces);
                }
            }
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

    let Ok(expr) = parse_oxc_expr(expr_str) else {
        return Vec::new();
    };

    let mut namespaces = HashSet::new();
    collect_expr(&expr, &mut namespaces);
    namespaces.into_iter().collect()
}

#[cfg(all(not(feature = "swc"), feature = "oxc"))]
fn source_slice(source: &str, span: oxc::span::Span) -> String {
    source
        .get(span.start as usize..span.end as usize)
        .unwrap_or("")
        .to_string()
}
