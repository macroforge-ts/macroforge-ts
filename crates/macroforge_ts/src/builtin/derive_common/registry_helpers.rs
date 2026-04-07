use std::collections::HashSet;

use convert_case::{Case, Casing};

use crate::ts_syn::DataTypeAlias;
use crate::ts_syn::abi::InterfaceFieldIR;
use crate::ts_syn::abi::ir::type_alias::{TypeMember, TypeMemberKind};
use crate::ts_syn::abi::ir::type_registry::{
    ResolvedTypeRef, TypeDefinitionIR, TypeRegistry, TypeRegistryEntry,
};

/// Check if a type in the registry has `@derive(MacroName)` applied.
/// Works for classes, interfaces, enums, and type aliases.
/// Returns false if the type is not in the registry or has no such derive.
///
/// When a name is ambiguous (same type name in multiple files), checks ALL
/// qualified entries — returns true if ANY entry with that name has the derive.
pub fn type_has_derive(registry: &TypeRegistry, type_name: &str, derive_name: &str) -> bool {
    let has_derive = |entry: &TypeRegistryEntry| {
        let decorators = match &entry.definition {
            TypeDefinitionIR::Class(c) => &c.decorators,
            TypeDefinitionIR::Interface(i) => &i.decorators,
            TypeDefinitionIR::Enum(e) => &e.decorators,
            TypeDefinitionIR::TypeAlias(t) => &t.decorators,
        };
        decorators.iter().any(|d| {
            d.name.eq_ignore_ascii_case("derive")
                && d.args_src
                    .split(',')
                    .any(|arg| arg.trim().eq_ignore_ascii_case(derive_name))
        })
    };

    // Check primary entry first (fast path for unambiguous names)
    if let Some(entry) = registry.get(type_name)
        && has_derive(entry)
    {
        return true;
    }

    // If ambiguous, check all qualified entries with this name
    if registry.ambiguous_names.iter().any(|n| n == type_name) {
        return registry
            .qualified_types
            .values()
            .any(|entry| entry.name == type_name && has_derive(entry));
    }

    false
}

/// Check if a resolved field type (or its inner element type for collections)
/// has a specific derive. Handles arrays, generics, and direct types.
#[allow(dead_code)]
pub fn resolved_type_has_derive(
    registry: &TypeRegistry,
    resolved: &ResolvedTypeRef,
    derive_name: &str,
) -> bool {
    type_has_derive(registry, &resolved.base_type_name, derive_name)
}

/// Get the inner element [`ResolvedTypeRef`] for a collection type.
/// For `User[]` or `Array<User>`, returns the `User` ref.
/// For `Map<K, V>`, returns the `V` ref (value type).
/// For `Set<T>`, returns the `T` ref.
pub fn collection_element_type(resolved: &ResolvedTypeRef) -> Option<&ResolvedTypeRef> {
    if !resolved.is_collection || resolved.type_args.is_empty() {
        return None;
    }
    match resolved.base_type_name.as_str() {
        "Map" if resolved.type_args.len() >= 2 => Some(&resolved.type_args[1]),
        _ => Some(&resolved.type_args[0]), // Array, Set, etc.
    }
}

/// Get the Map key type for `Map<K, V>` collections.
#[allow(dead_code)]
pub fn map_key_type(resolved: &ResolvedTypeRef) -> Option<&ResolvedTypeRef> {
    if resolved.base_type_name == "Map" && resolved.type_args.len() >= 2 {
        Some(&resolved.type_args[0])
    } else {
        None
    }
}

/// Generate the standalone function name for a given type and derive macro.
/// E.g., `("User", "Clone")` → `"userClone"`, `("User", "HashCode")` → `"userHashCode"`.
pub fn standalone_fn_name(type_name: &str, suffix: &str) -> String {
    format!("{}{}", type_name.to_case(Case::Camel), suffix)
}

/// Extract fields from a type definition in the registry.
///
/// Returns `None` if the definition cannot provide fields (e.g. enums).
/// For type aliases with intersection bodies, recursively flattens.
pub fn fields_from_definition(
    definition: &TypeDefinitionIR,
    type_registry: Option<&TypeRegistry>,
) -> Option<Vec<InterfaceFieldIR>> {
    match definition {
        TypeDefinitionIR::Interface(i) => Some(i.fields.clone()),
        TypeDefinitionIR::Class(c) => Some(
            c.fields
                .iter()
                .map(|f| InterfaceFieldIR {
                    name: f.name.clone(),
                    span: f.span,
                    ts_type: f.ts_type.clone(),
                    optional: f.optional,
                    readonly: f.readonly,
                    decorators: f.decorators.clone(),
                })
                .collect(),
        ),
        TypeDefinitionIR::TypeAlias(t) => {
            if let Some(obj_fields) = t.body.as_object() {
                Some(obj_fields.to_vec())
            } else if let Some(members) = t.body.as_intersection() {
                flatten_intersection_fields(members, type_registry)
            } else {
                None
            }
        }
        TypeDefinitionIR::Enum(_) => None,
    }
}

/// Flatten an intersection type's members into a single field list.
///
/// Returns `None` if any `TypeRef` member cannot be resolved (incomplete knowledge).
/// Returns `Some(fields)` with deduplicated fields (first-seen-wins) on success.
/// Literal members are skipped (they contribute no fields).
pub fn flatten_intersection_fields(
    members: &[TypeMember],
    type_registry: Option<&TypeRegistry>,
) -> Option<Vec<InterfaceFieldIR>> {
    let mut fields = Vec::new();
    let mut seen = HashSet::new();

    for member in members {
        match &member.kind {
            TypeMemberKind::Object { fields: obj_fields } => {
                for field in obj_fields {
                    if seen.insert(field.name.clone()) {
                        fields.push(field.clone());
                    }
                }
            }
            TypeMemberKind::TypeRef(type_name) => {
                let registry = type_registry?;
                let entry = registry.get(type_name)?;
                let member_fields = fields_from_definition(&entry.definition, type_registry)?;
                for field in member_fields {
                    if seen.insert(field.name.clone()) {
                        fields.push(field);
                    }
                }
            }
            TypeMemberKind::Literal(_) => {
                // Literals contribute no fields to intersections
            }
            TypeMemberKind::Intersection(sub_members) => {
                // Recursively flatten nested intersections
                let sub_fields = flatten_intersection_fields(sub_members, type_registry)?;
                for field in sub_fields {
                    if seen.insert(field.name.clone()) {
                        fields.push(field);
                    }
                }
            }
        }
    }

    Some(fields)
}

/// Get effective object-like fields from a type alias.
///
/// Works for both `Object` types (direct fields) and `Intersection` types (flattened fields).
/// Returns `None` for unions, tuples, simple aliases, or unresolvable intersections.
pub fn get_effective_fields(
    type_alias: &DataTypeAlias,
    type_registry: Option<&TypeRegistry>,
) -> Option<Vec<InterfaceFieldIR>> {
    if let Some(fields) = type_alias.as_object() {
        Some(fields.to_vec())
    } else if let Some(members) = type_alias.as_intersection() {
        flatten_intersection_fields(members, type_registry)
    } else {
        None
    }
}
