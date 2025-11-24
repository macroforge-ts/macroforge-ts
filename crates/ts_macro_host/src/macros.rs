#[macro_export]
macro_rules! register_macro_package {
    ($module:expr, $registrar:path) => {
        inventory::submit! {
            $crate::package_registry::MacroPackageRegistration {
                module: $module,
                registrar: $registrar,
            }
        }
    };
}
