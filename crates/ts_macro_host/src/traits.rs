//! Core macro traits

use ts_macro_abi::{MacroKind, MacroResult};
use ts_syn::TsStream;

/// Core trait that all TypeScript macros must implement
pub trait Macroforge: Send + Sync {
    /// Returns the name of this macro
    fn name(&self) -> &str;

    /// Returns the kind of this macro
    fn kind(&self) -> MacroKind;

    /// Execute the macro with the given input stream
    fn run(&self, input: TsStream) -> MacroResult;

    /// Returns a description of what this macro does
    fn description(&self) -> &str {
        "A TypeScript macro"
    }

    /// Returns the ABI version this macro was compiled against
    fn abi_version(&self) -> u32 {
        1
    }
}

/// Trait for macro packages that can provide multiple macros
pub trait MacroPackage: Send + Sync {
    /// Returns the package name
    fn package_name(&self) -> &str;

    /// Returns all macros provided by this package
    fn macros(&self) -> Vec<Box<dyn Macroforge>>;

    /// Returns the package version
    fn version(&self) -> &str {
        "0.1.0"
    }
}
