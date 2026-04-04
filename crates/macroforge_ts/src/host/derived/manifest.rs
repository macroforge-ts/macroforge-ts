use super::descriptors::DerivedMacroRegistration;
use super::registry::decorator_metadata;
use serde::{Deserialize, Serialize};

#[cfg(feature = "node")]
use napi_derive::napi;

/// Manifest entry describing a single macro.
#[cfg_attr(feature = "node", napi(object))]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroManifestEntry {
    /// The macro name (e.g., "Debug", "Clone").
    pub name: String,
    /// The macro kind (derive, attribute, function).
    pub kind: String,
    /// Human-readable description.
    pub description: String,
    /// The package providing this macro.
    pub package: String,
}

/// Entry for a registered decorator in the manifest.
#[cfg_attr(feature = "node", napi(object))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecoratorManifestEntry {
    /// The module this decorator belongs to (e.g., "serde").
    pub module: String,
    /// The exported name of the decorator (e.g., "skip", "rename").
    pub export: String,
    /// The decorator kind: "class", "property", "method", "accessor", "parameter".
    pub kind: String,
    /// Documentation string for the decorator.
    pub docs: String,
}

/// Complete manifest of all available macros and decorators.
#[cfg_attr(feature = "node", napi(object))]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MacroManifest {
    /// Manifest format version (currently always 1).
    pub version: u32,
    /// All registered macros.
    pub macros: Vec<MacroManifestEntry>,
    /// All registered decorators.
    pub decorators: Vec<DecoratorManifestEntry>,
}

/// Returns the complete manifest of all registered macros and decorators.
pub fn get_manifest() -> MacroManifest {
    let macros: Vec<MacroManifestEntry> = inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .map(|entry| MacroManifestEntry {
            name: entry.descriptor.name.to_string(),
            kind: format!("{:?}", entry.descriptor.kind).to_lowercase(),
            description: entry.descriptor.description.to_string(),
            package: entry.descriptor.package.to_string(),
        })
        .collect();

    let decorators: Vec<DecoratorManifestEntry> = decorator_metadata()
        .into_iter()
        .map(|d| DecoratorManifestEntry {
            module: d.module.to_string(),
            export: d.export.to_string(),
            kind: format!("{:?}", d.kind).to_lowercase(),
            docs: d.docs.to_string(),
        })
        .collect();

    MacroManifest {
        version: 1,
        macros,
        decorators,
    }
}
