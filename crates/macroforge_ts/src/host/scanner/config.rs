use std::collections::HashSet;
use std::path::PathBuf;

/// Configuration for the project scanner.
pub struct ScanConfig {
    /// Project root directory.
    pub root_dir: PathBuf,

    /// File extensions to scan (default: `[".ts", ".tsx"]`).
    pub extensions: Vec<String>,

    /// Directories to skip (default: common non-source dirs).
    pub skip_dirs: HashSet<String>,

    /// Whether to only collect exported types (default: `false` - collect all).
    pub exported_only: bool,

    /// Maximum number of files to scan (safety limit, default: 10,000).
    pub max_files: usize,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            root_dir: PathBuf::from("."),
            extensions: vec![".ts".into(), ".tsx".into()],
            skip_dirs: [
                "node_modules",
                ".git",
                "dist",
                "build",
                ".macroforge",
                "coverage",
                ".next",
                ".nuxt",
                ".svelte-kit",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            exported_only: false,
            max_files: 10_000,
        }
    }
}
