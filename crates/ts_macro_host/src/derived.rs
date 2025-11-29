use crate::{MacroRegistry, Result, TsMacro};
use serde::Serialize;
use std::{collections::BTreeSet, sync::Arc};
use ts_macro_abi::MacroKind;

pub struct DerivedMacroDescriptor {
    pub package: &'static str,
    pub module: &'static str,
    pub runtime: &'static [&'static str],
    pub name: &'static str,
    pub kind: MacroKind,
    pub description: &'static str,
    pub constructor: fn() -> Arc<dyn TsMacro>,
    pub decorators: &'static [DecoratorDescriptor],
}

pub struct DecoratorDescriptor {
    pub module: &'static str,
    pub export: &'static str,
    pub kind: DecoratorKind,
    pub docs: &'static str,
}

#[derive(Clone, Copy, Serialize)]
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

#[derive(Serialize)]
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
