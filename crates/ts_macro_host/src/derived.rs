use crate::{
    MacroError, MacroManifest, MacroRegistry, Result, TsMacro, registry::MacroManifestEntry,
};
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

    let package = descriptors[0].package;
    if descriptors
        .iter()
        .any(|descriptor| descriptor.package != package)
    {
        return Err(MacroError::InvalidConfig(format!(
            "derived macros for module '{module}' have conflicting package names"
        )));
    }

    let mut runtime: BTreeSet<String> = BTreeSet::new();
    let mut manifest_entries = Vec::with_capacity(descriptors.len());
    for descriptor in &descriptors {
        for entry in descriptor.runtime {
            runtime.insert(entry.to_string());
        }
        manifest_entries.push(MacroManifestEntry {
            name: descriptor.name.to_string(),
            kind: macro_kind_name(descriptor.kind).to_string(),
        });
    }

    let manifest = MacroManifest {
        module: Some(module.to_string()),
        native: None,
        abi_version: 1,
        macros: manifest_entries,
        runtime: runtime.into_iter().collect(),
    };

    registry.register_manifest(package, manifest)?;

    for descriptor in descriptors {
        registry.register(module, descriptor.name, (descriptor.constructor)())?;
    }

    Ok(true)
}

fn macro_kind_name(kind: MacroKind) -> &'static str {
    match kind {
        MacroKind::Derive => "derive",
        MacroKind::Attribute => "attribute",
        MacroKind::Call => "call",
    }
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
