use crate::host::Macroforge;
use crate::ts_syn::abi::MacroKind;
use serde::Serialize;
use std::sync::Arc;

/// Special marker module path that indicates dynamic resolution.
///
/// When a macro is registered with this module path, the host will accept
/// any import path and resolve the macro by name alone, enabling flexible
/// import patterns without requiring exact module path matches.
pub const DYNAMIC_MODULE_MARKER: &str = "__DYNAMIC_MODULE__";

/// Static descriptor for a derive macro, containing all metadata needed for registration.
///
/// This struct holds compile-time information about a macro that was registered
/// using the `#[derive_macro]` attribute. The `inventory` crate collects all
/// descriptors for automatic registration.
///
/// # Fields
///
/// * `package` - The npm package name (e.g., "macroforge-ts")
/// * `module` - The module path for registration (e.g., "@macro/derive")
/// * `runtime` - Runtime dependencies this macro requires
/// * `name` - The macro name used in `@derive(Name)`
/// * `kind` - The macro kind (Derive, Attribute, Function)
/// * `description` - Human-readable description
/// * `constructor` - Factory function to create the macro instance
/// * `decorators` - Field/class decorators this macro provides
pub struct DerivedMacroDescriptor {
    /// The npm package providing this macro.
    pub package: &'static str,
    /// Module path for registry lookup (e.g., "@macro/derive", or `DYNAMIC_MODULE_MARKER`).
    pub module: &'static str,
    /// Runtime dependencies (currently unused, reserved for future use).
    pub runtime: &'static [&'static str],
    /// Macro name as used in `@derive(Name)`.
    pub name: &'static str,
    /// The kind of macro (Derive, Attribute, or Function).
    pub kind: MacroKind,
    /// Human-readable description for documentation and IDE hints.
    pub description: &'static str,
    /// Factory function that creates a new instance of the macro.
    pub constructor: fn() -> Arc<dyn Macroforge>,
    /// Field-level decorators provided by this macro.
    pub decorators: &'static [DecoratorDescriptor],
}

/// Descriptor for a field or class decorator provided by a macro.
///
/// Decorators are additional annotations that modify how a macro processes
/// a field or class. For example, `@serde(skip)` tells the Serialize macro
/// to skip a particular field.
pub struct DecoratorDescriptor {
    /// Module this decorator belongs to (e.g., "serde").
    pub module: &'static str,
    /// The exported decorator name (e.g., "skip", "rename").
    pub export: &'static str,
    /// What the decorator can be applied to (class, property, method, etc.).
    pub kind: DecoratorKind,
    /// Documentation string for the decorator.
    pub docs: &'static str,
}

/// The target a decorator can be applied to.
///
/// Used for TypeScript type generation and validation.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DecoratorKind {
    /// Applied to class declarations.
    Class,
    /// Applied to class properties/fields.
    Property,
    /// Applied to class methods.
    Method,
    /// Applied to getters/setters.
    Accessor,
    /// Applied to method/constructor parameters.
    Parameter,
}

impl DecoratorKind {
    /// Returns the TypeScript decorator type for this kind.
    ///
    /// Used when generating `.d.ts` type declarations.
    pub fn ts_type(&self) -> &'static str {
        match self {
            DecoratorKind::Class => "ClassDecorator",
            DecoratorKind::Property => "PropertyDecorator",
            DecoratorKind::Method => "MethodDecorator",
            DecoratorKind::Accessor => "MethodDecorator",
            DecoratorKind::Parameter => "ParameterDecorator",
        }
    }
}

/// A registration entry for the `inventory` crate.
///
/// This is the type that gets collected at link time. Each macro implementation
/// creates one of these via the `#[derive_macro]` attribute.
pub struct DerivedMacroRegistration {
    /// Reference to the static descriptor for this macro.
    pub descriptor: &'static DerivedMacroDescriptor,
}

/// Metadata about a decorator for serialization and tooling.
#[derive(Debug, Clone, Serialize)]
pub struct DecoratorMetadata {
    /// Module this decorator belongs to.
    pub module: &'static str,
    /// The exported decorator name.
    pub export: &'static str,
    /// What the decorator can be applied to.
    pub kind: DecoratorKind,
    /// Documentation string.
    pub docs: &'static str,
}
