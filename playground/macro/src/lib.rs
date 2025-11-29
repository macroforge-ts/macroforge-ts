pub mod native;

// Re-export the manifest API for this macro package
// This allows the TS plugin to discover macros from @playground/macro

use napi_derive::napi;
use ts_macro_host::derived;

/// Entry for a single macro in the manifest
#[napi(object)]
pub struct MacroManifestEntry {
    pub name: String,
    pub kind: String,
    pub description: String,
    pub package: String,
}

/// Decorator metadata for TypeScript type stubs
#[napi(object)]
pub struct DecoratorManifestEntry {
    pub module: String,
    pub export: String,
    pub kind: String,
    pub docs: String,
}

/// Complete manifest for this macro package
#[napi(object)]
pub struct MacroManifest {
    pub version: u32,
    pub macros: Vec<MacroManifestEntry>,
    pub decorators: Vec<DecoratorManifestEntry>,
}

/// Get the manifest of all macros available in this package
#[napi(js_name = "__tsMacrosGetManifest")]
pub fn get_macro_manifest() -> MacroManifest {
    let manifest = derived::get_manifest();

    MacroManifest {
        version: manifest.version,
        macros: manifest
            .macros
            .into_iter()
            .map(|m| MacroManifestEntry {
                name: m.name.to_string(),
                kind: format!("{:?}", m.kind).to_lowercase(),
                description: m.description.to_string(),
                package: m.package.to_string(),
            })
            .collect(),
        decorators: manifest
            .decorators
            .into_iter()
            .map(|d| DecoratorManifestEntry {
                module: d.module.to_string(),
                export: d.export.to_string(),
                kind: format!("{:?}", d.kind).to_lowercase(),
                docs: d.docs.to_string(),
            })
            .collect(),
    }
}

/// Check if this package exports macros
#[napi(js_name = "__tsMacrosIsMacroPackage")]
pub fn is_macro_package() -> bool {
    !derived::macro_names().is_empty()
}

/// Get the names of all macros in this package
#[napi(js_name = "__tsMacrosGetMacroNames")]
pub fn get_macro_names() -> Vec<String> {
    derived::macro_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}
