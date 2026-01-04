//! Generate README.md files from API docs
//!
//! Creates README files for packages from extracted documentation.

use crate::core::config::Config;
use crate::utils::format;
use anyhow::Result;
use std::fs;

/// Packages to generate READMEs for
const PACKAGES: &[(&str, &str, &str)] = &[
    ("core", "crates/macroforge_ts", "macroforge"),
    ("shared", "packages/shared", "@macroforge/shared"),
    (
        "vite-plugin",
        "packages/vite-plugin",
        "@macroforge/vite-plugin",
    ),
    (
        "typescript-plugin",
        "packages/typescript-plugin",
        "@macroforge/typescript-plugin",
    ),
    (
        "svelte-language-server",
        "packages/svelte-language-server",
        "@macroforge/svelte-language-server",
    ),
    (
        "svelte-preprocessor",
        "packages/svelte-preprocessor",
        "@macroforge/svelte-preprocessor",
    ),
    (
        "mcp-server",
        "packages/mcp-server",
        "@macroforge/mcp-server",
    ),
];

pub fn run() -> Result<()> {
    let config = Config::load()?;

    format::header("Generating README Files");

    let mut generated = 0;

    for (pkg_id, pkg_path, npm_name) in PACKAGES {
        let pkg_dir = config.root.join(pkg_path);
        if !pkg_dir.exists() {
            format::warning(&format!("Package not found: {}", pkg_path));
            continue;
        }

        print!("Generating README for {}... ", pkg_id);

        let readme = generate_readme(&config.root, pkg_id, npm_name, pkg_path)?;
        let readme_path = pkg_dir.join("README.md");
        fs::write(&readme_path, readme)?;

        println!("done");
        generated += 1;
    }

    println!();
    format::success(&format!("Generated {} README files", generated));

    Ok(())
}

fn generate_readme(
    root: &std::path::Path,
    _pkg_id: &str,
    npm_name: &str,
    pkg_path: &str,
) -> Result<String> {
    let pkg_json_path = root.join(pkg_path).join("package.json");

    // Parse package.json for description
    let description = if pkg_json_path.exists() {
        let content = fs::read_to_string(&pkg_json_path)?;
        let pkg: serde_json::Value = serde_json::from_str(&content)?;
        pkg.get("description")
            .and_then(|d| d.as_str())
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    };

    let readme = format!(
        r#"# {npm_name}

{description}

## Installation

```bash
npm install {npm_name}
```

## Usage

```typescript
import {{ /* exports */ }} from '{npm_name}';
```

## Documentation

See the [Macroforge documentation](https://macroforge.dev) for more information.

## License

MIT

---

_This README was auto-generated. Do not edit manually._
"#
    );

    Ok(readme)
}
