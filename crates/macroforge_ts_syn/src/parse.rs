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
//! ```rust
//! let source = r#"
//!     class User {
//!         name: string;
//!         age: number;
//!     }
//! "#;
//!
//! # #[cfg(feature = "swc")] {
//! use macroforge_ts_syn::lower_classes;
//! use macroforge_ts_syn::parse::parse_ts_module;
//!
//! let module = parse_ts_module(source, "input.ts")?;
//! let _classes = lower_classes(&module, source, None)?;
//! # }
//! # #[cfg(feature = "oxc")] {
//! use macroforge_ts_syn::{lower_classes_oxc, parse_oxc_program};
//!
//! let program = parse_oxc_program(source)?;
//! let _classes = lower_classes_oxc(&program, source, None)?;
//! # }
//! # Ok::<(), macroforge_ts_syn::TsSynError>(())
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
/// ```rust
/// # #[cfg(feature = "swc")] {
/// use macroforge_ts_syn::parse::parse_ts_module;
///
/// let _module = parse_ts_module("const x: number = 5;", "input.ts")?;
/// let _module = parse_ts_module("const el = <div>Hello</div>;", "component.tsx")?;
/// # }
/// # #[cfg(feature = "oxc")] {
/// use macroforge_ts_syn::parse_oxc_program;
///
/// let _program = parse_oxc_program("const x: number = 5;")?;
/// let _program = parse_oxc_program("const el = <div>Hello</div>;")?;
/// # }
/// # Ok::<(), macroforge_ts_syn::TsSynError>(())
/// ```
///
/// # Integration with Lowering
///
/// The returned `Module` can be passed to the lowering functions:
///
/// ```rust
/// let source = "class User {}";
///
/// # #[cfg(feature = "swc")] {
/// use macroforge_ts_syn::parse::parse_ts_module;
/// use macroforge_ts_syn::{lower_classes, lower_enums, lower_interfaces, lower_type_aliases};
///
/// let module = parse_ts_module(source, "input.ts")?;
/// let _classes = lower_classes(&module, source, None)?;
/// let _interfaces = lower_interfaces(&module, source, None)?;
/// let _enums = lower_enums(&module, source, None)?;
/// let _type_aliases = lower_type_aliases(&module, source, None)?;
/// # }
/// # #[cfg(feature = "oxc")] {
/// use macroforge_ts_syn::{
///     lower_classes_oxc, lower_enums_oxc, lower_interfaces_oxc, lower_type_aliases_oxc,
///     parse_oxc_program,
/// };
///
/// let program = parse_oxc_program(source)?;
/// let _classes = lower_classes_oxc(&program, source, None)?;
/// let _interfaces = lower_interfaces_oxc(&program, source, None)?;
/// let _enums = lower_enums_oxc(&program, source, None)?;
/// let _type_aliases = lower_type_aliases_oxc(&program, source, None)?;
/// # }
/// # Ok::<(), macroforge_ts_syn::TsSynError>(())
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
