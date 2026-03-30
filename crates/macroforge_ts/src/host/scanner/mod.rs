//! Project-wide TypeScript scanner for building the type registry.
//!
//! This module walks the project directory, parses all `.ts`/`.tsx` files with SWC,
//! lowers declarations to IR, and collects them into a [`TypeRegistry`].
//!
//! The scanner runs BEFORE macro expansion as a pre-pass to give macros
//! project-wide type awareness (inspired by Zig's compile-time type introspection).
//!
//! ## Usage
//!
//! ```rust,no_run
//! use macroforge_ts::host::scanner::{ProjectScanner, ScanConfig};
//!
//! let config = ScanConfig {
//!     root_dir: std::path::PathBuf::from("/path/to/project"),
//!     ..Default::default()
//! };
//! let scanner = ProjectScanner::new(config);
//! let output = scanner.scan().expect("scan failed");
//! println!("Found {} types", output.registry.len());
//! ```

mod collectors;
mod config;
mod core;
#[cfg(test)]
mod tests;

pub use collectors::{collect_exported_names, collect_file_imports};
pub use config::ScanConfig;
pub use core::{ProjectScanner, ScanOutput};
