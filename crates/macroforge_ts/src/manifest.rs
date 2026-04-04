use crate::host::MacroExpander;
use crate::host::derived;
pub use crate::host::derived::manifest::{
    DecoratorManifestEntry, MacroManifest, MacroManifestEntry,
};
#[cfg(feature = "node")]
use napi_derive::napi;

// ============================================================================
// Manifest / Debug API
// ============================================================================

/// Returns the complete manifest of all registered macros and decorators.
///
/// This is a debug/introspection API that allows tooling to discover
/// what macros are available at runtime.
///
/// # Returns
///
/// A [`MacroManifest`] containing all registered macros and decorators.
///
/// # Example (JavaScript)
///
/// ```javascript
/// const manifest = __macroforgeGetManifest();
/// console.log("Available macros:", manifest.macros.map(m => m.name));
/// // ["Debug", "Clone", "PartialEq", "Hash", "Serialize", "Deserialize", ...]
/// ```
#[cfg_attr(feature = "node", napi(js_name = "__macroforgeGetManifest"))]
pub fn get_macro_manifest() -> MacroManifest {
    derived::get_manifest()
}

/// Checks if any macros are registered in this package.
///
/// Useful for build tools to determine if macro expansion is needed.
///
/// # Returns
///
/// `true` if at least one macro is registered, `false` otherwise.
#[cfg_attr(feature = "node", napi(js_name = "__macroforgeIsMacroPackage"))]
pub fn is_macro_package() -> bool {
    !derived::macro_names().is_empty()
}

/// Returns the names of all registered macros.
///
/// # Returns
///
/// A vector of macro names (e.g., `["Debug", "Clone", "Serialize"]`).
#[cfg_attr(feature = "node", napi(js_name = "__macroforgeGetMacroNames"))]
pub fn get_macro_names() -> Vec<String> {
    derived::macro_names()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Returns all registered macro module names (debug API).
///
/// Modules group related macros together (e.g., "builtin", "serde").
///
/// # Returns
///
/// A vector of module names.
#[cfg_attr(feature = "node", napi(js_name = "__macroforgeDebugGetModules"))]
pub fn debug_get_modules() -> Vec<String> {
    crate::host::derived::modules()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Looks up a macro by module and name (debug API).
///
/// Useful for testing macro registration and debugging lookup issues.
///
/// # Arguments
///
/// * `module` - The module name (e.g., "builtin")
/// * `name` - The macro name (e.g., "Debug")
///
/// # Returns
///
/// A string describing whether the macro was found or not.
#[cfg_attr(feature = "node", napi(js_name = "__macroforgeDebugLookup"))]
pub fn debug_lookup(module: String, name: String) -> String {
    match MacroExpander::new() {
        Ok(host) => match host.dispatcher.registry().lookup(&module, &name) {
            Ok(_) => format!("Found: ({}, {})", module, name),
            Err(_) => format!("Not found: ({}, {})", module, name),
        },
        Err(e) => format!("Host init failed: {}", e),
    }
}

/// Returns debug information about all registered macro descriptors (debug API).
///
/// This provides low-level access to the inventory-based macro registration
/// system for debugging purposes.
///
/// # Returns
///
/// A vector of strings describing each registered macro descriptor.
#[cfg_attr(feature = "node", napi(js_name = "__macroforgeDebugDescriptors"))]
pub fn debug_descriptors() -> Vec<String> {
    inventory::iter::<crate::host::derived::DerivedMacroRegistration>()
        .map(|entry| {
            format!(
                "name={}, module={}, package={}",
                entry.descriptor.name, entry.descriptor.module, entry.descriptor.package
            )
        })
        .collect()
}
