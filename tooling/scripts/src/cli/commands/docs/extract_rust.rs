//! Extract Rust documentation to JSON
//!
//! Parses //! (module docs) and /// (item docs) comments from Rust crates
//! and outputs structured JSON for website/README generation.

use crate::core::config::Config;
use crate::parsers::rust_docs::{self, ItemDoc};
use crate::utils::format;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Crate documentation output
#[derive(Debug, Serialize, Deserialize)]
pub struct CrateDoc {
    pub name: String,
    pub kind: String,
    pub version: String,
    pub description: String,
    pub overview: String,
    pub items: Vec<ItemDoc>,
}

/// Crate configurations
const CRATES: &[(&str, &str)] = &[
    ("macroforge_ts", "src/lib.rs"),
    ("macroforge_ts_syn", "src/lib.rs"),
    ("macroforge_ts_quote", "src/lib.rs"),
    ("macroforge_ts_macros", "src/lib.rs"),
];

/// Builtin macro configurations
const BUILTIN_MACROS: &[(&str, &str, &str)] = &[
    ("debug", "src/builtin/derive_debug.rs", "Debug"),
    ("clone", "src/builtin/derive_clone.rs", "Clone"),
    ("default", "src/builtin/derive_default.rs", "Default"),
    ("hash", "src/builtin/derive_hash.rs", "Hash"),
    ("ord", "src/builtin/derive_ord.rs", "Ord"),
    (
        "partial_eq",
        "src/builtin/derive_partial_eq.rs",
        "PartialEq",
    ),
    (
        "partial_ord",
        "src/builtin/derive_partial_ord.rs",
        "PartialOrd",
    ),
    (
        "serialize",
        "src/builtin/serde/derive_serialize.rs",
        "Serialize",
    ),
    (
        "deserialize",
        "src/builtin/serde/derive_deserialize.rs",
        "Deserialize",
    ),
];

pub fn run(output_dir: &Path) -> Result<()> {
    let config = Config::load()?;
    let crates_root = config.root.join("crates");
    let output_path = config.root.join(output_dir);

    fs::create_dir_all(&output_path)?;

    format::header("Extracting Rust Documentation");
    println!("Output: {}", output_path.display());

    let mut all_docs = Vec::new();
    let mut total_items = 0;

    // Process crates
    for (crate_name, entry_file) in CRATES {
        let crate_path = crates_root.join(crate_name);
        let entry_path = crate_path.join(entry_file);

        if !entry_path.exists() {
            format::warning(&format!("Entry file not found: {}", entry_path.display()));
            continue;
        }

        print!("Processing {}... ", crate_name);

        let docs = match extract_crate_docs(&crate_path, entry_file) {
            Ok(d) => d,
            Err(e) => {
                println!("failed: {}", e);
                continue;
            }
        };

        let item_count = docs.items.len();
        total_items += item_count;

        let out_path = output_path.join(format!("{}.json", crate_name));
        let json = serde_json::to_string_pretty(&docs)?;
        fs::write(&out_path, json)?;

        println!("{} items", item_count);
        all_docs.push(docs);
    }

    // Process builtin macros
    println!("\nProcessing builtin macros...");
    let mut builtin_docs: HashMap<String, serde_json::Value> = HashMap::new();

    for (name, file, display_name) in BUILTIN_MACROS {
        let file_path = crates_root.join("macroforge_ts").join(file);
        if !file_path.exists() {
            format::warning(&format!("Macro file not found: {}", file_path.display()));
            continue;
        }

        if let Ok(source) = fs::read_to_string(&file_path) {
            let module_docs = rust_docs::extract_module_docs(&source);
            builtin_docs.insert(
                name.to_string(),
                serde_json::json!({
                    "name": display_name,
                    "slug": name.replace('_', "-"),
                    "description": module_docs,
                }),
            );
        }
    }

    let builtin_path = output_path.join("builtin-macros.json");
    fs::write(&builtin_path, serde_json::to_string_pretty(&builtin_docs)?)?;
    println!("  {} macros documented", builtin_docs.len());

    // Write index
    let index = serde_json::json!({
        "generated": chrono::Utc::now().to_rfc3339(),
        "crates": all_docs.iter().map(|d| serde_json::json!({
            "name": d.name,
            "version": d.version,
            "itemCount": d.items.len(),
        })).collect::<Vec<_>>(),
        "builtinMacros": builtin_docs.keys().collect::<Vec<_>>(),
    });

    let index_path = output_path.join("index.json");
    fs::write(&index_path, serde_json::to_string_pretty(&index)?)?;

    println!();
    format::success(&format!(
        "Extracted {} items from {} crates, {} macros",
        total_items,
        all_docs.len(),
        builtin_docs.len()
    ));

    Ok(())
}

fn extract_crate_docs(crate_path: &Path, entry_file: &str) -> Result<CrateDoc> {
    let cargo_path = crate_path.join("Cargo.toml");
    let entry_path = crate_path.join(entry_file);

    // Parse Cargo.toml for version
    let cargo_content = fs::read_to_string(&cargo_path).unwrap_or_default();
    let cargo: toml::Value =
        toml::from_str(&cargo_content).unwrap_or(toml::Value::Table(Default::default()));

    let name = cargo
        .get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("")
        .to_string();

    let version = cargo
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0")
        .to_string();

    let description = cargo
        .get("package")
        .and_then(|p| p.get("description"))
        .and_then(|d| d.as_str())
        .unwrap_or("")
        .to_string();

    // Parse source file
    let source = fs::read_to_string(&entry_path)?;
    let overview = rust_docs::extract_module_docs(&source);
    let items = rust_docs::extract_item_docs(&source);

    Ok(CrateDoc {
        name,
        kind: "rust_crate".to_string(),
        version,
        description,
        overview,
        items,
    })
}
