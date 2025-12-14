//! Low-level TypeScript parsing utilities.
//!
//! This module provides direct access to SWC's TypeScript parser with
//! sensible defaults for the macro system. For most use cases, prefer
//! the higher-level [`TsStream`](crate::TsStream) API or the lowering
//! functions in [`lower`](crate::lower).
//!
//! ## Primary Function
//!
//! - [`parse_ts_module`] - Parse TypeScript source into an SWC [`Module`]
//!
//! ## Example
//!
//! ```rust,ignore
//! use macroforge_ts_syn::parse_ts_module;
//!
//! let source = r#"
//!     class User {
//!         name: string;
//!         age: number;
//!     }
//! "#;
//!
//! let module = parse_ts_module(source, "input.ts")?;
//!
//! // Now you can use the module with lowering functions
//! let classes = lower_classes(&module, source)?;
//! ```
//!
//! ## File Type Detection
//!
//! The parser automatically enables TSX support based on the file name:
//! - `*.tsx` - TSX (TypeScript with JSX) mode enabled
//! - `*.ts` or other - Standard TypeScript mode
//!
//! ## Feature Flag
//!
//! This module requires the `swc` feature (enabled by default).

#[cfg(feature = "swc")]
use swc_core::common::{FileName, SourceMap, sync::Lrc};
#[cfg(feature = "swc")]
use swc_core::ecma::ast::{EsVersion, Module};
#[cfg(feature = "swc")]
use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

use crate::TsSynError;

/// Parses TypeScript source code into an SWC [`Module`] AST.
///
/// This is the primary parsing entry point for the macro system. It configures
/// SWC's parser with appropriate settings for TypeScript:
/// - TypeScript syntax enabled
/// - Decorators enabled (for `@derive` and other decorators)
/// - TSX support auto-detected from file extension
/// - Early errors disabled (more permissive parsing)
///
/// # Arguments
///
/// - `source` - The TypeScript source code to parse
/// - `file_name` - The file name (used for error messages and TSX detection)
///
/// # Returns
///
/// - `Ok(Module)` - The parsed AST on success
/// - `Err(TsSynError::Parse)` - If the source contains syntax errors
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::parse_ts_module;
///
/// // Parse TypeScript
/// let module = parse_ts_module("const x: number = 5;", "input.ts")?;
///
/// // Parse TSX (JSX in TypeScript)
/// let module = parse_ts_module(
///     "const el = <div>Hello</div>;",
///     "component.tsx"
/// )?;
/// ```
///
/// # Integration with Lowering
///
/// The returned `Module` can be passed to the lowering functions:
///
/// ```rust,ignore
/// let module = parse_ts_module(source, "input.ts")?;
/// let classes = lower_classes(&module, source)?;
/// let interfaces = lower_interfaces(&module, source)?;
/// let enums = lower_enums(&module, source)?;
/// let type_aliases = lower_type_aliases(&module, source)?;
/// ```
#[cfg(feature = "swc")]
pub fn parse_ts_module(source: &str, file_name: &str) -> Result<Module, TsSynError> {
    let cm: Lrc<SourceMap> = Lrc::new(Default::default());
    let fm = cm.new_source_file(
        FileName::Custom(file_name.into()).into(),
        source.to_string(),
    );

    let syntax = Syntax::Typescript(TsSyntax {
        tsx: file_name.ends_with(".tsx"),
        decorators: true,
        no_early_errors: true,
        ..Default::default()
    });

    let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);

    let mut parser = Parser::new_from(lexer);
    parser
        .parse_module()
        .map_err(|e| TsSynError::Parse(format!("{:?}", e)))
}

#[cfg(not(feature = "swc"))]
pub fn parse_ts_module(_source: &str, _file_name: &str) -> Result<(), TsSynError> {
    Err(TsSynError::Parse("swc feature disabled".into()))
}
