#![cfg(feature = "swc")]
use super::super::error::Result;
use super::namespaces::extract_expression_namespaces;
use macroforge_ts_syn::config::{
    ForeignTypeAlias, ForeignTypeConfig, ImportInfo, MacroforgeConfig,
};
use std::collections::HashMap;

use swc_core::{
    common::{SourceMap, sync::Lrc},
    ecma::{
        ast::*,
        codegen::{Config, Emitter, Node, text_writer::JsWriter},
    },
};

/// Helper to convert a Wtf8Atom (string value) to a String.
pub(crate) fn atom_to_string(atom: &swc_core::ecma::utils::swc_atoms::Wtf8Atom) -> String {
    String::from_utf8_lossy(atom.as_bytes()).to_string()
}

/// Extract import statements into a lookup map.
pub(crate) fn extract_imports(module: &Module) -> HashMap<String, ImportInfo> {
    let mut imports = HashMap::new();

    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
            let source = atom_to_string(&import.src.value);

            for specifier in &import.specifiers {
                match specifier {
                    ImportSpecifier::Named(named) => {
                        let local = named.local.sym.to_string();
                        let imported = named
                            .imported
                            .as_ref()
                            .map(|i| match i {
                                ModuleExportName::Ident(id) => id.sym.to_string(),
                                ModuleExportName::Str(s) => atom_to_string(&s.value),
                            })
                            .unwrap_or_else(|| local.clone());
                        imports.insert(
                            local,
                            ImportInfo {
                                name: imported,
                                source: source.clone(),
                            },
                        );
                    }
                    ImportSpecifier::Default(default) => {
                        imports.insert(
                            default.local.sym.to_string(),
                            ImportInfo {
                                name: "default".to_string(),
                                source: source.clone(),
                            },
                        );
                    }
                    ImportSpecifier::Namespace(ns) => {
                        imports.insert(
                            ns.local.sym.to_string(),
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
}

/// Extract the default export and parse it as configuration.
pub(crate) fn extract_default_export(
    module: &Module,
    imports: &HashMap<String, ImportInfo>,
    cm: &Lrc<SourceMap>,
) -> Result<MacroforgeConfig> {
    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(export)) = item {
            match &*export.expr {
                // export default { ... }
                Expr::Object(obj) => {
                    return parse_config_object(obj, imports, cm);
                }
                // export default defineConfig({ ... }) or similar
                Expr::Call(call) => {
                    if let Some(first_arg) = call.args.first()
                        && let Expr::Object(obj) = &*first_arg.expr
                    {
                        return parse_config_object(obj, imports, cm);
                    }
                }
                _ => {}
            }
        }
    }

    // No default export found - return defaults
    Ok(MacroforgeConfig::default())
}

/// Parse the config object to extract configuration values.
fn parse_config_object(
    obj: &ObjectLit,
    imports: &HashMap<String, ImportInfo>,
    cm: &Lrc<SourceMap>,
) -> Result<MacroforgeConfig> {
    let mut config = MacroforgeConfig::default();

    for prop in &obj.props {
        if let PropOrSpread::Prop(prop) = prop
            && let Prop::KeyValue(kv) = &**prop
        {
            let key = get_prop_key(&kv.key);

            match key.as_str() {
                "keepDecorators" => {
                    config.keep_decorators = get_bool_value(&kv.value).unwrap_or(false);
                }
                "generateConvenienceConst" => {
                    config.generate_convenience_const = get_bool_value(&kv.value).unwrap_or(true);
                }
                "foreignTypes" => {
                    if let Expr::Object(ft_obj) = &*kv.value {
                        config.foreign_types = parse_foreign_types(ft_obj, imports, cm)?;
                    }
                }
                _ => {}
            }
        }
    }

    // Store the config file's imports for use when generating namespace imports
    config.config_imports = imports.clone();

    Ok(config)
}

/// Parse the foreignTypes object.
fn parse_foreign_types(
    obj: &ObjectLit,
    imports: &HashMap<String, ImportInfo>,
    cm: &Lrc<SourceMap>,
) -> Result<Vec<ForeignTypeConfig>> {
    let mut foreign_types = vec![];

    for prop in &obj.props {
        if let PropOrSpread::Prop(prop) = prop
            && let Prop::KeyValue(kv) = &**prop
        {
            let type_name = get_prop_key(&kv.key);
            if let Expr::Object(type_obj) = &*kv.value {
                let ft = parse_single_foreign_type(&type_name, type_obj, imports, cm)?;
                foreign_types.push(ft);
            }
        }
    }

    Ok(foreign_types)
}

/// Parse a single foreign type configuration.
fn parse_single_foreign_type(
    name: &str,
    obj: &ObjectLit,
    imports: &HashMap<String, ImportInfo>,
    cm: &Lrc<SourceMap>,
) -> Result<ForeignTypeConfig> {
    let mut ft = ForeignTypeConfig {
        name: name.to_string(),
        ..Default::default()
    };

    for prop in &obj.props {
        if let PropOrSpread::Prop(prop) = prop
            && let Prop::KeyValue(kv) = &**prop
        {
            let key = get_prop_key(&kv.key);

            match key.as_str() {
                "from" => {
                    ft.from = extract_string_or_array(&kv.value);
                }
                "serialize" => {
                    let (expr, import) = extract_function_expr(&kv.value, imports, cm);
                    ft.serialize_expr = expr;
                    ft.serialize_import = import;
                }
                "deserialize" => {
                    let (expr, import) = extract_function_expr(&kv.value, imports, cm);
                    ft.deserialize_expr = expr;
                    ft.deserialize_import = import;
                }
                "default" => {
                    let (expr, import) = extract_function_expr(&kv.value, imports, cm);
                    ft.default_expr = expr;
                    ft.default_import = import;
                }
                "hasShape" => {
                    let (expr, import) = extract_function_expr(&kv.value, imports, cm);
                    ft.has_shape_expr = expr;
                    ft.has_shape_import = import;
                }
                "aliases" => {
                    ft.aliases = parse_aliases_array(&kv.value);
                }
                _ => {}
            }
        }
    }

    // Extract namespace references from all expressions
    let mut all_namespaces = std::collections::HashSet::new();
    if let Some(ref expr) = ft.serialize_expr {
        for ns in extract_expression_namespaces(expr) {
            all_namespaces.insert(ns);
        }
    }
    if let Some(ref expr) = ft.deserialize_expr {
        for ns in extract_expression_namespaces(expr) {
            all_namespaces.insert(ns);
        }
    }
    if let Some(ref expr) = ft.default_expr {
        for ns in extract_expression_namespaces(expr) {
            all_namespaces.insert(ns);
        }
    }
    if let Some(ref expr) = ft.has_shape_expr {
        for ns in extract_expression_namespaces(expr) {
            all_namespaces.insert(ns);
        }
    }
    ft.expression_namespaces = all_namespaces.into_iter().collect();

    Ok(ft)
}

/// Parse an array of aliases: [{ name: "DateTime", from: "effect/DateTime" }, ...]
fn parse_aliases_array(expr: &Expr) -> Vec<ForeignTypeAlias> {
    let mut aliases = Vec::new();

    if let Expr::Array(arr) = expr {
        for elem in arr.elems.iter().flatten() {
            if let Expr::Object(obj) = &*elem.expr
                && let Some(alias) = parse_single_alias(obj)
            {
                aliases.push(alias);
            }
        }
    }

    aliases
}

/// Parse a single alias object: { name: "DateTime", from: "effect/DateTime" }
fn parse_single_alias(obj: &ObjectLit) -> Option<ForeignTypeAlias> {
    let mut name = None;
    let mut from = None;

    for prop in &obj.props {
        if let PropOrSpread::Prop(prop) = prop
            && let Prop::KeyValue(kv) = &**prop
        {
            let key = get_prop_key(&kv.key);

            match key.as_str() {
                "name" => {
                    if let Expr::Lit(Lit::Str(s)) = &*kv.value {
                        name = Some(atom_to_string(&s.value));
                    }
                }
                "from" => {
                    if let Expr::Lit(Lit::Str(s)) = &*kv.value {
                        from = Some(atom_to_string(&s.value));
                    }
                }
                _ => {}
            }
        }
    }

    // Both name and from are required
    match (name, from) {
        (Some(name), Some(from)) => Some(ForeignTypeAlias { name, from }),
        _ => None,
    }
}

/// Get property key as string.
pub(crate) fn get_prop_key(key: &PropName) -> String {
    match key {
        PropName::Ident(id) => id.sym.to_string(),
        PropName::Str(s) => atom_to_string(&s.value),
        PropName::Num(n) => n.value.to_string(),
        PropName::BigInt(b) => b.value.to_string(),
        PropName::Computed(c) => {
            if let Expr::Lit(Lit::Str(s)) = &*c.expr {
                atom_to_string(&s.value)
            } else {
                "[computed]".to_string()
            }
        }
    }
}

/// Get boolean value from expression.
pub(crate) fn get_bool_value(expr: &Expr) -> Option<bool> {
    match expr {
        Expr::Lit(Lit::Bool(b)) => Some(b.value),
        _ => None,
    }
}

/// Extract string or array of strings from expression.
pub(crate) fn extract_string_or_array(expr: &Expr) -> Vec<String> {
    match expr {
        Expr::Lit(Lit::Str(s)) => vec![atom_to_string(&s.value)],
        Expr::Array(arr) => arr
            .elems
            .iter()
            .filter_map(|elem| {
                elem.as_ref().and_then(|e| {
                    if let Expr::Lit(Lit::Str(s)) = &*e.expr {
                        Some(atom_to_string(&s.value))
                    } else {
                        None
                    }
                })
            })
            .collect(),
        _ => vec![],
    }
}

/// Extract function expression - either inline arrow or reference to imported/declared function.
pub(crate) fn extract_function_expr(
    expr: &Expr,
    imports: &HashMap<String, ImportInfo>,
    cm: &Lrc<SourceMap>,
) -> (Option<String>, Option<ImportInfo>) {
    match expr {
        // Inline arrow function: (v, ctx) => v.toJSON()
        Expr::Arrow(_) => {
            let source = codegen_expr(expr, cm);
            (Some(source), None)
        }
        // Function expression: function(v, ctx) { return v.toJSON(); }
        Expr::Fn(_) => {
            let source = codegen_expr(expr, cm);
            (Some(source), None)
        }
        // Reference to a variable: serializeDateTime
        Expr::Ident(ident) => {
            let name = ident.sym.to_string();
            if let Some(import_info) = imports.get(&name) {
                // It's an imported function
                (Some(name.clone()), Some(import_info.clone()))
            } else {
                // It's a locally declared function - just use the name
                (Some(name), None)
            }
        }
        // Member expression: DateTime.fromJSON
        Expr::Member(_) => {
            let source = codegen_expr(expr, cm);
            (Some(source), None)
        }
        _ => (None, None),
    }
}

/// Convert expression AST back to source code using SWC's codegen.
pub(crate) fn codegen_expr(expr: &Expr, cm: &Lrc<SourceMap>) -> String {
    let mut buf = Vec::new();

    {
        let writer = JsWriter::new(cm.clone(), "\n", &mut buf, None);
        let mut emitter = Emitter {
            cfg: Config::default(),
            cm: cm.clone(),
            comments: None,
            wr: writer,
        };

        // Use the Node trait's emit_with method
        if expr.emit_with(&mut emitter).is_err() {
            return String::new();
        }
    }

    String::from_utf8(buf).unwrap_or_default()
}
