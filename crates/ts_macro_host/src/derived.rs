use crate::{MacroRegistry, Result, Macroforge};
use serde::Serialize;
use std::{collections::BTreeSet, sync::Arc};
use ts_macro_abi::MacroKind;

/// Special marker module path that indicates dynamic resolution
/// When a macro is registered with this module, the host will accept
/// any import path and resolve the macro by name alone
pub const DYNAMIC_MODULE_MARKER: &str = "__DYNAMIC_MODULE__";

pub struct DerivedMacroDescriptor {
    pub package: &'static str,
    pub module: &'static str,
    pub runtime: &'static [&'static str],
    pub name: &'static str,
    pub kind: MacroKind,
    pub description: &'static str,
    pub constructor: fn() -> Arc<dyn Macroforge>,
    pub decorators: &'static [DecoratorDescriptor],
}

pub struct DecoratorDescriptor {
    pub module: &'static str,
    pub export: &'static str,
    pub kind: DecoratorKind,
    pub docs: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DecoratorKind {
    Class,
    Property,
    Method,
    Accessor,
    Parameter,
}

impl DecoratorKind {
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

pub struct DerivedMacroRegistration {
    pub descriptor: &'static DerivedMacroDescriptor,
}

inventory::collect!(DerivedMacroRegistration);

pub fn modules() -> BTreeSet<&'static str> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .map(|entry| entry.descriptor.module)
        .collect()
}

pub fn register_module(module: &str, registry: &MacroRegistry) -> Result<bool> {
    let descriptors: Vec<&DerivedMacroDescriptor> = inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .filter(|entry| entry.descriptor.module == module)
        .map(|entry| entry.descriptor)
        .collect();

    if descriptors.is_empty() {
        return Ok(false);
    }

    // Collect runtime entries from all packages
    let mut runtime: BTreeSet<String> = BTreeSet::new();
    for descriptor in &descriptors {
        for entry in descriptor.runtime {
            runtime.insert(entry.to_string());
        }
    }

    // Register all macros - the registry will catch duplicate names
    for descriptor in descriptors {
        registry.register(module, descriptor.name, (descriptor.constructor)())?;
    }

    Ok(true)
}

#[derive(Debug, Clone, Serialize)]
pub struct DecoratorMetadata {
    pub module: &'static str,
    pub export: &'static str,
    pub kind: DecoratorKind,
    pub docs: &'static str,
}

pub fn decorator_metadata() -> Vec<DecoratorMetadata> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .flat_map(|entry| entry.descriptor.decorators)
        .map(|decorator| DecoratorMetadata {
            module: decorator.module,
            export: decorator.export,
            kind: decorator.kind,
            docs: decorator.docs,
        })
        .collect()
}

/// Manifest entry for a single macro
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroManifestEntry {
    pub name: &'static str,
    pub kind: MacroKind,
    pub description: &'static str,
    pub package: &'static str,
}

/// Complete manifest for a macro package
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroManifest {
    /// Manifest format version
    pub version: u32,
    /// List of macros provided by this package
    pub macros: Vec<MacroManifestEntry>,
    /// List of decorator exports (for TypeScript type stubs)
    pub decorators: Vec<DecoratorMetadata>,
}

/// Get the manifest for all macros registered via inventory in this binary
/// This is used by NAPI exports to allow the TS plugin to discover macros
pub fn get_manifest() -> MacroManifest {
    let macros: Vec<MacroManifestEntry> = inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .map(|entry| MacroManifestEntry {
            name: entry.descriptor.name,
            kind: entry.descriptor.kind,
            description: entry.descriptor.description,
            package: entry.descriptor.package,
        })
        .collect();

    let decorators = decorator_metadata();

    MacroManifest {
        version: 1,
        macros,
        decorators,
    }
}

/// Get all macro names registered in this binary
pub fn macro_names() -> Vec<&'static str> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .map(|entry| entry.descriptor.name)
        .collect()
}

/// Look up a macro by name only (ignoring module path)
/// This is used for dynamic module resolution
pub fn lookup_by_name(name: &str) -> Option<&'static DerivedMacroDescriptor> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .find(|entry| entry.descriptor.name == name)
        .map(|entry| entry.descriptor)
}

/// Register all macros with dynamic module support
/// If a macro has DYNAMIC_MODULE_MARKER as its module, it will be registered
/// under the provided actual_module path instead
pub fn register_all_with_module(actual_module: &str, registry: &MacroRegistry) -> Result<usize> {
    let mut count = 0;

    for entry in inventory::iter::<DerivedMacroRegistration> {
        let descriptor = entry.descriptor;

        // Determine the module to use for registration
        let module = if descriptor.module == DYNAMIC_MODULE_MARKER {
            actual_module
        } else {
            descriptor.module
        };

        // Register the macro
        registry.register(module, descriptor.name, (descriptor.constructor)())?;
        count += 1;
    }

    Ok(count)
}
