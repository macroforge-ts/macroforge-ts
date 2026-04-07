//! Build markdown documentation book
//!
//! Concatenates markdown files into a single book format.

use crate::core::config::Config;
use crate::utils::format;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Markdown file to include in the book
struct BookSection {
    title: String,
    path: String,
    level: usize,
}

pub fn run(output_path: &Path) -> Result<()> {
    let config = Config::load()?;
    let docs_root = config.root.join("docs");
    let output = config.root.join(output_path);

    format::header("Building Documentation Book");

    // Define book structure
    let sections = vec![
        BookSection {
            title: "Introduction".to_string(),
            path: "README.md".to_string(),
            level: 1,
        },
        BookSection {
            title: "Getting Started".to_string(),
            path: "getting-started.md".to_string(),
            level: 1,
        },
        BookSection {
            title: "Configuration".to_string(),
            path: "configuration.md".to_string(),
            level: 1,
        },
        BookSection {
            title: "API Reference".to_string(),
            path: "api/README.md".to_string(),
            level: 1,
        },
    ];

    let mut book_content = String::new();
    let mut found_count = 0;

    // Add title
    book_content.push_str("# Macroforge Documentation\n\n");
    book_content.push_str(&format!(
        "_Generated: {}_\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));
    book_content.push_str("---\n\n");

    for section in &sections {
        let file_path = docs_root.join(&section.path);

        if file_path.exists() {
            print!("Including {}... ", section.path);

            if let Ok(content) = fs::read_to_string(&file_path) {
                let heading = "#".repeat(section.level);
                book_content.push_str(&format!("{} {}\n\n", heading, section.title));
                book_content.push_str(&process_markdown(&content, section.level));
                book_content.push_str("\n\n---\n\n");
                println!("done");
                found_count += 1;
            } else {
                println!("failed to read");
            }
        } else {
            format::warning(&format!("Section not found: {}", section.path));
        }
    }

    // Ensure output directory exists
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&output, book_content)?;

    println!();
    format::success(&format!(
        "Built book with {} sections -> {}",
        found_count,
        output.display()
    ));

    Ok(())
}

/// Process markdown content, adjusting heading levels
fn process_markdown(content: &str, base_level: usize) -> String {
    let lines: Vec<String> = content
        .lines()
        .map(|line| {
            // Adjust heading levels
            if line.starts_with('#') {
                let level = line.chars().take_while(|&c| c == '#').count();
                let new_level = level + base_level;
                let heading = "#".repeat(new_level);
                let text = line.trim_start_matches('#').trim();
                format!("{} {}", heading, text)
            } else {
                line.to_string()
            }
        })
        .collect();

    lines.join("\n")
}
