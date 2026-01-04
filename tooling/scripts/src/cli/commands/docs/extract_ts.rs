//! Extract TypeScript documentation to JSON
//!
//! Parses TypeScript source files and extracts JSDoc comments and type information.

use crate::core::config::Config;
use crate::utils::format;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// TypeScript package documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDoc {
    pub name: String,
    pub version: String,
    pub description: String,
    pub exports: Vec<ExportDoc>,
}

/// Export documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportDoc {
    pub name: String,
    pub kind: String,
    #[serde(rename = "type")]
    pub export_type: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<ParamDoc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamDoc {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

/// TypeScript packages to document
const TS_PACKAGES: &[(&str, &str)] = &[
    ("core", "crates/macroforge_ts"),
    ("shared", "packages/shared"),
    ("vite-plugin", "packages/vite-plugin"),
    ("typescript-plugin", "packages/typescript-plugin"),
    ("svelte-language-server", "packages/svelte-language-server"),
    ("svelte-preprocessor", "packages/svelte-preprocessor"),
    ("mcp-server", "packages/mcp-server"),
];

pub fn run(output_dir: &Path) -> Result<()> {
    let config = Config::load()?;
    let output_path = config.root.join(output_dir);

    fs::create_dir_all(&output_path)?;

    format::header("Extracting TypeScript Documentation");
    println!("Output: {}", output_path.display());

    let mut all_docs = Vec::new();
    let mut total_exports = 0;

    for (pkg_name, pkg_path) in TS_PACKAGES {
        let pkg_dir = config.root.join(pkg_path);
        if !pkg_dir.exists() {
            format::warning(&format!("Package not found: {}", pkg_dir.display()));
            continue;
        }

        print!("Processing {}... ", pkg_name);

        let docs = match extract_package_docs(&pkg_dir, pkg_name) {
            Ok(d) => d,
            Err(e) => {
                println!("failed: {}", e);
                continue;
            }
        };

        let export_count = docs.exports.len();
        total_exports += export_count;

        let out_path = output_path.join(format!("{}.json", pkg_name));
        let json = serde_json::to_string_pretty(&docs)?;
        fs::write(&out_path, json)?;

        println!("{} exports", export_count);
        all_docs.push(docs);
    }

    // Write index
    let index = serde_json::json!({
        "generated": chrono::Utc::now().to_rfc3339(),
        "packages": all_docs.iter().map(|d| serde_json::json!({
            "name": d.name,
            "version": d.version,
            "exportCount": d.exports.len(),
        })).collect::<Vec<_>>(),
    });

    let index_path = output_path.join("index.json");
    fs::write(&index_path, serde_json::to_string_pretty(&index)?)?;

    println!();
    format::success(&format!(
        "Extracted {} exports from {} packages",
        total_exports,
        all_docs.len()
    ));

    Ok(())
}

fn extract_package_docs(pkg_dir: &Path, pkg_name: &str) -> Result<PackageDoc> {
    let pkg_json_path = pkg_dir.join("package.json");

    // Parse package.json
    let pkg_json_content = fs::read_to_string(&pkg_json_path).unwrap_or_default();
    let pkg_json: serde_json::Value = serde_json::from_str(&pkg_json_content).unwrap_or_default();

    let name = pkg_json
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or(pkg_name)
        .to_string();

    let version = pkg_json
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0")
        .to_string();

    let description = pkg_json
        .get("description")
        .and_then(|d| d.as_str())
        .unwrap_or("")
        .to_string();

    // Find entry point
    let main_file = pkg_json
        .get("main")
        .or_else(|| pkg_json.get("module"))
        .and_then(|m| m.as_str())
        .unwrap_or("src/index.ts");

    // Look for source file
    let source_paths = [
        pkg_dir.join("src/index.ts"),
        pkg_dir.join("src/lib.ts"),
        pkg_dir.join(main_file.replace(".js", ".ts")),
    ];

    let mut exports = Vec::new();

    for source_path in source_paths {
        if source_path.exists() {
            if let Ok(source) = fs::read_to_string(&source_path) {
                exports.extend(extract_exports_from_source(&source));
            }
            break;
        }
    }

    Ok(PackageDoc {
        name,
        version,
        description,
        exports,
    })
}

/// Extract exports from TypeScript source (basic regex-based extraction)
fn extract_exports_from_source(source: &str) -> Vec<ExportDoc> {
    let mut exports = Vec::new();

    // Match export function declarations
    let fn_re =
        regex::Regex::new(r"(?m)^(?:/\*\*[\s\S]*?\*/\s*)?export\s+(?:async\s+)?function\s+(\w+)")
            .unwrap();

    for cap in fn_re.captures_iter(source) {
        let name = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
        let full_match = cap.get(0).map(|m| m.as_str()).unwrap_or("");
        let description = extract_jsdoc_description(full_match);

        exports.push(ExportDoc {
            name,
            kind: "function".to_string(),
            export_type: None,
            description,
            params: None,
            returns: None,
        });
    }

    // Match export class declarations
    let class_re =
        regex::Regex::new(r"(?m)^(?:/\*\*[\s\S]*?\*/\s*)?export\s+class\s+(\w+)").unwrap();

    for cap in class_re.captures_iter(source) {
        let name = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
        let full_match = cap.get(0).map(|m| m.as_str()).unwrap_or("");
        let description = extract_jsdoc_description(full_match);

        exports.push(ExportDoc {
            name,
            kind: "class".to_string(),
            export_type: None,
            description,
            params: None,
            returns: None,
        });
    }

    // Match export interface declarations
    let interface_re =
        regex::Regex::new(r"(?m)^(?:/\*\*[\s\S]*?\*/\s*)?export\s+interface\s+(\w+)").unwrap();

    for cap in interface_re.captures_iter(source) {
        let name = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
        let full_match = cap.get(0).map(|m| m.as_str()).unwrap_or("");
        let description = extract_jsdoc_description(full_match);

        exports.push(ExportDoc {
            name,
            kind: "interface".to_string(),
            export_type: None,
            description,
            params: None,
            returns: None,
        });
    }

    // Match export type declarations
    let type_re = regex::Regex::new(r"(?m)^(?:/\*\*[\s\S]*?\*/\s*)?export\s+type\s+(\w+)").unwrap();

    for cap in type_re.captures_iter(source) {
        let name = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
        let full_match = cap.get(0).map(|m| m.as_str()).unwrap_or("");
        let description = extract_jsdoc_description(full_match);

        exports.push(ExportDoc {
            name,
            kind: "type".to_string(),
            export_type: None,
            description,
            params: None,
            returns: None,
        });
    }

    // Match export const declarations
    let const_re =
        regex::Regex::new(r"(?m)^(?:/\*\*[\s\S]*?\*/\s*)?export\s+const\s+(\w+)").unwrap();

    for cap in const_re.captures_iter(source) {
        let name = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
        let full_match = cap.get(0).map(|m| m.as_str()).unwrap_or("");
        let description = extract_jsdoc_description(full_match);

        exports.push(ExportDoc {
            name,
            kind: "const".to_string(),
            export_type: None,
            description,
            params: None,
            returns: None,
        });
    }

    exports
}

/// Extract description from JSDoc comment
fn extract_jsdoc_description(source: &str) -> String {
    let jsdoc_re = regex::Regex::new(r"/\*\*\s*([\s\S]*?)\s*\*/").unwrap();

    if let Some(cap) = jsdoc_re.captures(source) {
        let comment = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        // Clean up the comment
        let lines: Vec<&str> = comment
            .lines()
            .map(|l| l.trim().trim_start_matches('*').trim())
            .filter(|l| !l.starts_with('@'))
            .collect();
        return lines.join(" ").trim().to_string();
    }

    String::new()
}
