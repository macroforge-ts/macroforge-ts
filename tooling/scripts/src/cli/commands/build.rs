//! Clean build packages command
//!
//! Performs complete clean rebuilds of selected packages.

use crate::cli::BuildArgs;
use crate::core::config::Config;
use crate::utils::format;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Type alias for build step function
type BuildStepFn = Box<dyn Fn(&Config) -> Result<()>>;

/// A build step with a label and execution details
struct BuildStep {
    label: String,
    cmd: Option<Vec<String>>,
    cwd: Option<std::path::PathBuf>,
    func: Option<BuildStepFn>,
}

pub fn run(args: BuildArgs) -> Result<()> {
    let config = Config::load()?;
    let repos = config.filter_repos(&args.repos);
    let repo_names: Vec<_> = repos.iter().map(|r| r.name.as_str()).collect();

    format::header("Clean Build");
    println!("Repos: {}", repo_names.join(", "));

    let mut steps: Vec<BuildStep> = Vec::new();

    // Always clean root node_modules if building everything
    if args.repos == "all" {
        steps.push(BuildStep {
            label: "Remove root node_modules".to_string(),
            cmd: Some(vec!["rm".to_string(), "-rf".to_string(), "node_modules".to_string()]),
            cwd: Some(config.root.clone()),
            func: None,
        });
    }

    // Core rust crate
    if repo_names.contains(&"core") {
        steps.push(BuildStep {
            label: "Cleanbuild core (macroforge)".to_string(),
            cmd: Some(vec!["deno".to_string(), "task".to_string(), "cleanbuild".to_string()]),
            cwd: Some(config.root.join("crates/macroforge_ts")),
            func: None,
        });
    }

    // TypeScript packages
    let ts_packages = ["shared", "vite-plugin", "typescript-plugin", "svelte-language-server", "svelte-preprocessor", "mcp-server"];
    for pkg in ts_packages {
        if repo_names.contains(&pkg) {
            let pkg_dir = config.root.join(format!("packages/{}", pkg));
            steps.push(BuildStep {
                label: format!("Cleanbuild {}", pkg),
                cmd: Some(vec![
                    "sh".to_string(),
                    "-c".to_string(),
                    "rm -rf node_modules dist && deno install --node-modules-dir && deno task build".to_string(),
                ]),
                cwd: Some(pkg_dir),
                func: None,
            });
        }
    }

    // Tooling/playground
    if repo_names.contains(&"tooling") {
        let playgrounds = ["macro", "svelte", "vanilla"];
        for pg in playgrounds {
            let pg_dir = config.root.join(format!("tooling/playground/{}", pg));
            if pg_dir.exists() {
                steps.push(BuildStep {
                    label: format!("Cleanbuild playground/{}", pg),
                    cmd: Some(vec!["deno".to_string(), "task".to_string(), "cleanbuild".to_string()]),
                    cwd: Some(pg_dir),
                    func: None,
                });
            }
        }
    }

    // Website
    if repo_names.contains(&"website") {
        let website_dir = config.root.join("website");

        steps.push(BuildStep {
            label: "Git pull website from origin".to_string(),
            cmd: Some(vec!["git".to_string(), "pull".to_string(), "origin".to_string()]),
            cwd: Some(website_dir.clone()),
            func: None,
        });

        steps.push(BuildStep {
            label: "Cleanbuild website".to_string(),
            cmd: Some(vec![
                "sh".to_string(),
                "-c".to_string(),
                "rm -rf node_modules .svelte-kit && deno install --node-modules-dir".to_string(),
            ]),
            cwd: Some(website_dir.clone()),
            func: None,
        });

        let website_dir_clone = website_dir.clone();
        steps.push(BuildStep {
            label: "Configure website for local build".to_string(),
            cmd: None,
            cwd: None,
            func: Some(Box::new(move |_config| {
                add_external_config(&website_dir_clone)
            })),
        });

        steps.push(BuildStep {
            label: "Build website".to_string(),
            cmd: Some(vec!["deno".to_string(), "task".to_string(), "build".to_string()]),
            cwd: Some(website_dir.clone()),
            func: None,
        });

        let website_dir_clone = website_dir.clone();
        steps.push(BuildStep {
            label: "Restore website config for production".to_string(),
            cmd: None,
            cwd: None,
            func: Some(Box::new(move |_config| {
                remove_external_config(&website_dir_clone)
            })),
        });
    }

    if steps.is_empty() {
        format::warning("No build steps to run");
        return Ok(());
    }

    // Execute steps
    for (i, step) in steps.iter().enumerate() {
        format::step(i + 1, steps.len(), &step.label);

        if let Some(func) = &step.func {
            if let Err(e) = func(&config) {
                format::error(&format!("Step failed: {}", e));
                return Err(e);
            }
        } else if let Some(cmd) = &step.cmd {
            let cwd = step.cwd.as_ref().unwrap_or(&config.root);

            if !cwd.exists() {
                format::warning(&format!("Directory does not exist: {}", cwd.display()));
                continue;
            }

            let result = if cmd[0] == "sh" {
                Command::new("sh")
                    .args(&cmd[1..])
                    .current_dir(cwd)
                    .output()
            } else {
                Command::new(&cmd[0])
                    .args(&cmd[1..])
                    .current_dir(cwd)
                    .output()
            };

            match result {
                Ok(output) if output.status.success() => {
                    println!("  {} {}", "✓".green(), "done".dimmed());
                }
                Ok(output) => {
                    println!("  {} {}", "✗".red(), "failed".red());
                    if !output.stderr.is_empty() {
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                    anyhow::bail!("Build step failed: {}", step.label);
                }
                Err(e) => {
                    println!("  {} {}: {}", "✗".red(), "error".red(), e);
                    anyhow::bail!("Build step error: {}", e);
                }
            }
        }
    }

    println!();
    format::success("All builds finished successfully");
    Ok(())
}

/// Add ssr.external for macroforge to vite.config.ts and svelte.config.js
fn add_external_config(website_dir: &Path) -> Result<()> {
    // Update vite.config.ts
    let vite_config_path = website_dir.join("vite.config.ts");
    if vite_config_path.exists() {
        let mut content = fs::read_to_string(&vite_config_path)?;
        if !content.contains("external: ['macroforge']") {
            content = content.replace(
                "plugins: [tailwindcss(), sveltekit()]",
                "plugins: [tailwindcss(), sveltekit()],\n\tssr: {\n\t\t// Temporary: needed for local file: dependency build\n\t\texternal: ['macroforge']\n\t}"
            );
            fs::write(&vite_config_path, content)?;
            println!("  {} Added ssr.external to vite.config.ts", "→".blue());
        }
    }

    // Update svelte.config.js
    let svelte_config_path = website_dir.join("svelte.config.js");
    if svelte_config_path.exists() {
        let mut content = fs::read_to_string(&svelte_config_path)?;
        if !content.contains("external: ['macroforge']") {
            content = content.replace(
                "adapter: adapter({\n\t\t\tout: 'build'\n\t\t})",
                "adapter: adapter({\n\t\t\tout: 'build',\n\t\t\texternal: ['macroforge']\n\t\t})"
            );
            fs::write(&svelte_config_path, content)?;
            println!("  {} Added external to svelte.config.js", "→".blue());
        }
    }

    Ok(())
}

/// Remove ssr.external config from vite.config.ts and svelte.config.js
fn remove_external_config(website_dir: &Path) -> Result<()> {
    // Update vite.config.ts
    let vite_config_path = website_dir.join("vite.config.ts");
    if vite_config_path.exists() {
        let content = fs::read_to_string(&vite_config_path)?;
        let updated = regex::Regex::new(
            r",\n\tssr: \{\n\t\t// Temporary: needed for local file: dependency build\n\t\texternal: \['macroforge'\]\n\t\}"
        )?.replace(&content, "");
        fs::write(&vite_config_path, updated.as_ref())?;
        println!("  {} Removed ssr.external from vite.config.ts", "→".blue());
    }

    // Update svelte.config.js
    let svelte_config_path = website_dir.join("svelte.config.js");
    if svelte_config_path.exists() {
        let content = fs::read_to_string(&svelte_config_path)?;
        let updated = content.replace(
            "adapter: adapter({\n\t\t\tout: 'build',\n\t\t\texternal: ['macroforge']\n\t\t})",
            "adapter: adapter({\n\t\t\tout: 'build'\n\t\t})"
        );
        fs::write(&svelte_config_path, updated)?;
        println!("  {} Removed external from svelte.config.js", "→".blue());
    }

    Ok(())
}
