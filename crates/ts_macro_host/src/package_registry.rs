//! Global registry for macro package registrars.

use crate::{MacroRegistry, Result};

pub struct MacroPackageRegistration {
    pub module: &'static str,
    pub registrar: fn(&MacroRegistry) -> Result<()>,
}

inventory::collect!(MacroPackageRegistration);

pub fn registrars() -> Vec<&'static MacroPackageRegistration> {
    inventory::iter::<MacroPackageRegistration>
        .into_iter()
        .collect()
}
