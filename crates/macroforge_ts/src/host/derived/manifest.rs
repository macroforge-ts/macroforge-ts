use super::descriptors::{DecoratorMetadata, DerivedMacroRegistration};
use super::registry::decorator_metadata;
use crate::ts_syn::abi::MacroKind;
use serde::Serialize;

/// Manifest entry describing a single macro.
///
/// Used in [`MacroManifest`] for tooling and documentation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroManifestEntry {
    /// The macro name (e.g., "Debug", "Clone").
    pub name: &'static str,
    /// The macro kind (Derive, Attribute, Function).
    pub kind: MacroKind,
    /// Human-readable description.
    pub description: &'static str,
    /// The package providing this macro.
    pub package: &'static str,
}

/// Complete manifest of all available macros and decorators.
///
/// This struct is returned by [`get_manifest()`] and contains everything
/// tooling needs to understand what macros are available.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroManifest {
    /// Manifest format version (currently always 1).
    pub version: u32,
    /// All registered macros.
    pub macros: Vec<MacroManifestEntry>,
    /// All registered decorators.
    pub decorators: Vec<DecoratorMetadata>,
}

/// Returns the complete manifest of all registered macros and decorators.
///
/// The manifest includes all macros discovered via the `inventory` crate
/// at link time, along with their field-level decorators. This is the
/// primary introspection API for tooling.
///
/// # Example
///
/// ```rust,no_run
/// use macroforge_ts::host::derived::get_manifest;
///
/// let manifest = get_manifest();
/// println!("Macroforge v{} manifest:", manifest.version);
/// for m in &manifest.macros {
///     println!("  @derive({}) [{}] - {}", m.name, m.package, m.description);
/// }
/// for d in &manifest.decorators {
///     println!("  @{}({}) - {}", d.module, d.export, d.docs);
/// }
/// ```
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
