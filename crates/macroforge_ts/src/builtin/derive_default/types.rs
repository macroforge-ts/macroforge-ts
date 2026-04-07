use crate::builtin::derive_common::{is_primitive_type, type_has_derive};
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;

/// Contains field information needed for default value generation.
///
/// Each non-optional field that needs a default value is represented by this struct,
/// capturing both the field name and the expression to use as its default value.
pub(super) struct DefaultField {
    /// The field name as it appears in the source TypeScript class.
    /// Used to generate assignment statements like `instance.name = value`.
    pub name: String,

    /// The JavaScript expression for the default value.
    /// This can be a literal (`0`, `""`, `[]`), a constructor call (`new Date()`),
    /// or a recursive `defaultValue()` call for custom types.
    pub value: String,
}

/// Emit compile-time warnings for fields whose types are in the type registry
/// but do not derive `Default`. The generated code will still call `typeNameDefaultValue()`,
/// which may fail at runtime if the type doesn't actually provide that function.
pub(super) fn validate_default_fields(
    fields: &[(String, String)], // (field_name, ts_type)
    parent_name: &str,
    registry: Option<&TypeRegistry>,
) {
    let registry = match registry {
        Some(r) => r,
        None => return,
    };

    for (field_name, ts_type) in fields {
        let t = ts_type.trim();
        // Only check non-primitive, non-collection types
        if is_primitive_type(t)
            || t.ends_with("[]")
            || t.starts_with("Array<")
            || t.starts_with("Map<")
            || t.starts_with("Set<")
            || t == "Date"
            || t.contains('|')
        {
            continue;
        }

        // Check if the type is in the registry (use get or get_all for ambiguous names)
        let type_known = registry.get(t).is_some() || registry.get_all(t).next().is_some();
        if type_known && !type_has_derive(registry, t, "Default") {
            eprintln!(
                "[macroforge] warning: field `{field_name}` in `{parent_name}` has type `{t}` \
                 which does not derive Default — generated code may fail at runtime"
            );
        }
    }
}
