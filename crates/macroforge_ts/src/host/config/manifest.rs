//! The project manifest entry that maps `#macroforge/config`.
//!
//! Generated code imports foreign-type handlers from `#macroforge/config`, a
//! Node subpath import the project declares in its manifest's `imports`, as
//! SvelteKit 3 declares `#lib`. TypeScript, bundlers and runtimes resolve it
//! natively, so the sync only checks that it is declared, and
//! `macroforge init` adds it.

use std::path::{Path, PathBuf};

use macroforge_ts_syn::config::FOREIGN_HANDLERS_MODULE;
use oxc::allocator::Allocator;
use oxc::ast::ast::{Expression, ObjectPropertyKind, PropertyKey};

use super::super::error::{MacroError, Result};

/// The manifests that can declare the entry, in the order they are read.
const MANIFESTS: &[&str] = &["package.json", "deno.json", "deno.jsonc"];

/// The entry, as a `package.json` declares it.
pub fn package_json_entry() -> serde_json::Value {
    serde_json::json!({
        "types": "./.macroforge/config/handlers.ts",
        "default": "./.macroforge/config/handlers.js",
    })
}

/// The entry, as a Deno import map declares it.
pub fn deno_entry() -> serde_json::Value {
    serde_json::Value::String("./.macroforge/config/handlers.ts".to_string())
}

/// Whether a manifest in `root_dir` maps `#macroforge/config`.
pub fn handlers_import_declared(root_dir: &Path) -> Result<bool> {
    for name in MANIFESTS {
        let path = root_dir.join(name);
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        if declares_entry(&text).map_err(|message| invalid(&path, message))? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// The error for a project whose config declares foreign types but whose
/// manifest does not map `#macroforge/config`.
pub fn missing_entry_error(root_dir: &Path) -> MacroError {
    let entry = format!(
        "\"imports\": {{\n  \"{FOREIGN_HANDLERS_MODULE}\": {{\n    \"types\": \"./.macroforge/config/handlers.ts\",\n    \"default\": \"./.macroforge/config/handlers.js\"\n  }}\n}}"
    );
    MacroError::InvalidConfig(format!(
        "{}: the config declares foreign types, whose handlers generated code imports from \
         `{FOREIGN_HANDLERS_MODULE}`, but no manifest maps it. Run `macroforge init`, or add \
         this to package.json:\n{entry}",
        root_dir.display()
    ))
}

/// What `macroforge init` did to a project's manifest.
#[derive(Debug, PartialEq, Eq)]
pub enum InitOutcome {
    /// The manifest already mapped `#macroforge/config`.
    AlreadyDeclared(PathBuf),
    /// The entry was added to this manifest.
    Added(PathBuf),
}

/// Adds the `#macroforge/config` entry to the project's `package.json`, or
/// to its `deno.json` when it has no `package.json`. A `deno.jsonc` is not
/// rewritten, since that would drop its comments; the error says what to add.
pub fn add_handlers_import(root_dir: &Path) -> Result<InitOutcome> {
    for name in MANIFESTS {
        let path = root_dir.join(name);
        if !path.exists() {
            continue;
        }
        let text =
            std::fs::read_to_string(&path).map_err(|error| invalid(&path, error.to_string()))?;
        if declares_entry(&text).map_err(|message| invalid(&path, message))? {
            return Ok(InitOutcome::AlreadyDeclared(path));
        }
        if *name == "deno.jsonc" {
            return Err(invalid(
                &path,
                format!(
                    "add `\"{FOREIGN_HANDLERS_MODULE}\": \"./.macroforge/config/handlers.ts\"` to its \
                     `imports`; it is not rewritten automatically, which would drop its comments"
                ),
            ));
        }
        let entry = if *name == "package.json" {
            package_json_entry()
        } else {
            deno_entry()
        };
        let rewritten = with_entry(&text, entry).map_err(|message| invalid(&path, message))?;
        std::fs::write(&path, rewritten).map_err(|error| invalid(&path, error.to_string()))?;
        return Ok(InitOutcome::Added(path));
    }
    Err(MacroError::InvalidConfig(format!(
        "{}: no package.json or deno.json to declare `{FOREIGN_HANDLERS_MODULE}` in",
        root_dir.display()
    )))
}

/// Whether manifest text maps `#macroforge/config` in its `imports`. Read as a
/// JavaScript object expression, so a `deno.jsonc`'s comments are allowed.
fn declares_entry(text: &str) -> std::result::Result<bool, String> {
    let allocator = Allocator::default();
    let wrapped = format!("({text})");
    let expression = crate::ts_syn::parse_expr(&allocator, &wrapped)
        .map_err(|error| format!("could not read the manifest: {error}"))?;
    let Some(manifest) = object_of(&expression) else {
        return Err("the manifest is not a JSON object".to_string());
    };
    let Some(imports) = property(manifest, "imports").and_then(object_of) else {
        return Ok(false);
    };
    Ok(property(imports, FOREIGN_HANDLERS_MODULE).is_some())
}

fn object_of<'a, 'b>(
    expression: &'b Expression<'a>,
) -> Option<&'b oxc::ast::ast::ObjectExpression<'a>> {
    match expression {
        Expression::ObjectExpression(object) => Some(object),
        Expression::ParenthesizedExpression(paren) => object_of(&paren.expression),
        _ => None,
    }
}

fn property<'a, 'b>(
    object: &'b oxc::ast::ast::ObjectExpression<'a>,
    key: &str,
) -> Option<&'b Expression<'a>> {
    object
        .properties
        .iter()
        .find_map(|property| match property {
            ObjectPropertyKind::ObjectProperty(property) => {
                let name = match &property.key {
                    PropertyKey::StringLiteral(literal) => literal.value.as_str(),
                    PropertyKey::StaticIdentifier(identifier) => identifier.name.as_str(),
                    _ => return None,
                };
                (name == key).then_some(&property.value)
            }
            ObjectPropertyKind::SpreadProperty(_) => None,
        })
}

/// `text` with `imports["#macroforge/config"]` set to `entry`, keeping its
/// key order and its indentation.
fn with_entry(text: &str, entry: serde_json::Value) -> std::result::Result<String, String> {
    let mut manifest: serde_json::Value =
        serde_json::from_str(text).map_err(|error| format!("is not valid JSON: {error}"))?;
    let object = manifest.as_object_mut().ok_or("is not a JSON object")?;
    let imports = object
        .entry("imports")
        .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
    let imports = imports
        .as_object_mut()
        .ok_or("has an `imports` that is not an object")?;
    imports.insert(FOREIGN_HANDLERS_MODULE.to_string(), entry);

    let indent = indentation(text);
    let mut out = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
    let mut serializer = serde_json::Serializer::with_formatter(&mut out, formatter);
    serde::Serialize::serialize(&manifest, &mut serializer).map_err(|error| error.to_string())?;
    let mut rewritten = String::from_utf8(out).map_err(|error| error.to_string())?;
    if text.ends_with('\n') {
        rewritten.push('\n');
    }
    Ok(rewritten)
}

/// The indentation of the first indented line, or two spaces.
fn indentation(text: &str) -> String {
    text.lines()
        .find_map(|line| {
            let indent: String = line
                .chars()
                .take_while(|c| *c == ' ' || *c == '\t')
                .collect();
            (!indent.is_empty() && indent.len() < line.len()).then_some(indent)
        })
        .unwrap_or_else(|| "  ".to_string())
}

fn invalid(path: &Path, message: String) -> MacroError {
    MacroError::InvalidConfig(format!("{}: {message}", path.display()))
}

#[cfg(test)]
#[path = "manifest_tests.rs"]
mod tests;
