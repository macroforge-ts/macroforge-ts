use std::collections::HashSet;

use convert_case::{Case, Casing};

use crate::host::expand::derive_scope::{prototype_member_message, unnamed_member_message};
use crate::ts_syn::abi::ir::class::{ClassIR, FieldIR, MemberKey, Visibility};
use crate::ts_syn::abi::ir::interface::InterfaceIR;
use crate::ts_syn::abi::ir::type_alias::{TypeMember, TypeMemberKind};
use crate::ts_syn::abi::ir::type_registry::{
    FileImportEntry, ResolvedTypeRef, TypeDefinitionIR, TypeRegistry, TypeRegistryEntry,
};
use crate::ts_syn::abi::{DecoratorIR, InterfaceFieldIR};
use crate::ts_syn::{DataClass, DataInterface, DataTypeAlias, DeriveInput, MacroforgeError};

/// Whether one of `decorators` is an `@derive` naming `derive_name`.
fn derives(decorators: &[DecoratorIR], derive_name: &str) -> bool {
    decorators.iter().any(|decorator| {
        decorator.name.eq_ignore_ascii_case("derive")
            && decorator
                .args_src
                .split(',')
                .any(|argument| argument.trim().eq_ignore_ascii_case(derive_name))
    })
}

/// Check if a type in the registry has `@derive(MacroName)` applied.
/// Works for classes, interfaces, enums, and type aliases.
/// Returns false if the type is not in the registry or has no such derive.
///
/// When a name is ambiguous (same type name in multiple files), checks ALL
/// qualified entries: returns true if ANY entry with that name has the derive.
pub fn type_has_derive(registry: &TypeRegistry, type_name: &str, derive_name: &str) -> bool {
    let has_derive = |entry: &TypeRegistryEntry| {
        let decorators = match &entry.definition {
            TypeDefinitionIR::Class(c) => &c.decorators,
            TypeDefinitionIR::Interface(i) => &i.decorators,
            TypeDefinitionIR::Enum(e) => &e.decorators,
            TypeDefinitionIR::TypeAlias(t) => &t.decorators,
        };
        derives(decorators, derive_name)
    };

    // Check primary entry first (fast path for unambiguous names)
    if let Some(entry) = registry.get(type_name)
        && has_derive(entry)
    {
        return true;
    }

    // If ambiguous, check all qualified entries with this name
    if registry.ambiguous_names.contains(type_name) {
        return registry.get_all(type_name).any(has_derive);
    }

    false
}

/// Check if a resolved field type (or its inner element type for collections)
/// has a specific derive. Handles arrays, generics, and direct types.
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

/// Generate the standalone function name for a given type and derive macro.
/// E.g., `("User", "Clone")` → `"userClone"`, `("User", "HashCode")` → `"userHashCode"`.
pub fn standalone_fn_name(type_name: &str, suffix: &str) -> String {
    format!("{}{}", type_name.to_case(Case::Camel), suffix)
}

/// Extract fields from a type definition in the registry.
///
/// Returns `None` if the definition cannot provide fields: an enum, or a
/// type with a member under a computed key or an index signature.
/// For type aliases with intersection bodies, recursively flattens.
pub fn fields_from_definition(
    definition: &TypeDefinitionIR,
    type_registry: &TypeRegistry,
) -> Option<Vec<InterfaceFieldIR>> {
    fields_from_definition_visiting(definition, type_registry, &mut Vec::new())
}

fn fields_from_definition_visiting(
    definition: &TypeDefinitionIR,
    type_registry: &TypeRegistry,
    visiting: &mut Vec<String>,
) -> Option<Vec<InterfaceFieldIR>> {
    let fields = match definition {
        TypeDefinitionIR::Interface(interface) => Some(interface.fields.clone()),
        TypeDefinitionIR::Class(class) => Some(
            class
                .fields
                .iter()
                .map(|field| InterfaceFieldIR {
                    name: field.name.clone(),
                    key: field.key,
                    span: field.span,
                    ts_type: field.ts_type.clone(),
                    optional: field.optional,
                    readonly: field.readonly,
                    decorators: field.decorators.clone(),
                })
                .collect(),
        ),
        TypeDefinitionIR::TypeAlias(alias) => {
            if let Some(obj_fields) = alias.body.as_object() {
                Some(obj_fields.to_vec())
            } else if let Some(members) = alias.body.as_intersection() {
                flatten_intersection_fields_visiting(members, type_registry, visiting)
            } else {
                None
            }
        }
        TypeDefinitionIR::Enum(_) => None,
    };
    // A member without a name leaves the type's fields unknown.
    fields.filter(|fields| fields.iter().all(|field| field.key != MemberKey::Unnamed))
}

/// Flatten an intersection type's members into a single field list.
///
/// Returns `None` if any `TypeRef` member cannot be resolved (incomplete
/// knowledge: the named type is missing from the registry), or if the
/// references form a cycle, which the expander reports at the derive.
/// Returns `Some(fields)` with deduplicated fields (first-seen-wins) on
/// success. Literal and brand members are skipped (they contribute no fields).
pub fn flatten_intersection_fields(
    members: &[TypeMember],
    type_registry: &TypeRegistry,
) -> Option<Vec<InterfaceFieldIR>> {
    flatten_intersection_fields_visiting(members, type_registry, &mut Vec::new())
}

fn flatten_intersection_fields_visiting(
    members: &[TypeMember],
    type_registry: &TypeRegistry,
    visiting: &mut Vec<String>,
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
                if visiting.contains(type_name) {
                    return None;
                }
                let entry = type_registry.get(type_name)?;
                visiting.push(type_name.clone());
                let member_fields =
                    fields_from_definition_visiting(&entry.definition, type_registry, visiting);
                visiting.pop();
                for field in member_fields? {
                    if seen.insert(field.name.clone()) {
                        fields.push(field);
                    }
                }
            }
            TypeMemberKind::Literal(_) | TypeMemberKind::Brand(_) => {
                // Literals and symbol brands contribute no fields to intersections
            }
            TypeMemberKind::Intersection(sub_members) => {
                // Recursively flatten nested intersections
                let sub_fields =
                    flatten_intersection_fields_visiting(sub_members, type_registry, visiting)?;
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
    type_registry: &TypeRegistry,
) -> Option<Vec<InterfaceFieldIR>> {
    if let Some(fields) = type_alias.as_object() {
        Some(fields.to_vec())
    } else if let Some(members) = type_alias.as_intersection() {
        flatten_intersection_fields(members, type_registry)
    } else {
        None
    }
}

/// Where the names in a declaration resolve: the registry, and the file the
/// declaration is in with its imports.
#[derive(Clone, Copy)]
pub struct Declared<'a> {
    pub registry: &'a TypeRegistry,
    pub file_path: &'a str,
    pub file_imports: &'a [FileImportEntry],
}

impl<'a> Declared<'a> {
    fn resolve(&self, name: &str) -> Option<&'a TypeRegistryEntry> {
        self.registry
            .resolve_in_file(name, self.file_path, self.file_imports)
    }

    /// The same registry, read from the file that declares `entry`.
    fn at(&self, entry: &'a TypeRegistryEntry) -> Self {
        Self {
            registry: self.registry,
            file_path: &entry.file_path,
            file_imports: &entry.file_imports,
        }
    }
}

/// Every field an instance of `class` holds: those of its superclasses,
/// outermost first, then its own.
///
/// The superclasses are read from the registry. With none, as when a file is
/// expanded on its own without a scan of its project, no type beyond the one
/// at hand is known, and the class's own fields are all there is to give.
///
/// The error names what keeps a derive from one of them: a `#private` field,
/// which only its class's constructor can set, and a derive runs none; a
/// superclass the registry does not hold, or one with type parameters, whose
/// fields cannot be read off; a `private` field of a superclass, which the
/// subclass cannot touch.
pub fn class_instance_fields(
    class: &ClassIR,
    declared: Declared<'_>,
) -> Result<Vec<FieldIR>, String> {
    let mut fields = match &class.superclass {
        Some(superclass) => superclass_fields(
            superclass,
            &class.name,
            declared,
            &mut vec![class.name.clone()],
        )?,
        None => Vec::new(),
    };
    for field in &class.fields {
        if field.key == MemberKey::Private {
            return Err(format!(
                "field '{}' of class '{}' can only be set by the class's own constructor, which a derive does not run; declare it `private {}` for a derive to read and build it",
                field.name,
                class.name,
                field.name.trim_start_matches('#')
            ));
        }
        fields.push(match rune_state_type(field) {
            Some(ts_type) => FieldIR {
                ts_type,
                untyped: false,
                ..field.clone()
            },
            None => field.clone(),
        });
    }
    if let Some(unnamed) = fields.iter().find(|field| field.key == MemberKey::Unnamed) {
        return Err(unnamed_member_message(&unnamed.name, &class.name));
    }
    if fields.iter().any(|field| field.name == "__proto__") {
        return Err(prototype_member_message(&class.name));
    }
    Ok(fields)
}

fn superclass_fields(
    name: &str,
    subclass: &str,
    declared: Declared<'_>,
    visiting: &mut Vec<String>,
) -> Result<Vec<FieldIR>, String> {
    let Some(entry) = declared.resolve(name) else {
        // Asked only once a name fails to resolve: the answer depends on
        // the whole registry, which a lookup by name does not.
        if declared.registry.is_empty() {
            return Ok(Vec::new());
        }
        return Err(format!(
            "class '{subclass}' extends '{name}', which this project does not declare, so the fields it gives an instance are unknown"
        ));
    };
    let TypeDefinitionIR::Class(base) = &entry.definition else {
        return Err(format!(
            "class '{subclass}' extends '{name}', which is not a class"
        ));
    };
    if !base.type_params.is_empty() {
        return Err(format!(
            "class '{subclass}' extends the generic class '{name}', whose fields depend on type arguments a derive on '{subclass}' cannot read off"
        ));
    }
    if visiting.contains(&base.name) {
        return Err(format!("class '{name}' extends itself"));
    }
    visiting.push(base.name.clone());
    let mut fields = match &base.superclass {
        Some(superclass) => {
            superclass_fields(superclass, &base.name, declared.at(entry), visiting)?
        }
        None => Vec::new(),
    };
    for field in &base.fields {
        if field.key == MemberKey::Private || field.visibility == Visibility::Private {
            return Err(format!(
                "field '{}' of class '{name}' is private to it, so a derive on its subclass '{subclass}' cannot reach it; declare it `protected`",
                field.name
            ));
        }
        fields.push(field.clone());
    }
    Ok(fields)
}

/// Every field a value of `interface` has: those of the types it extends,
/// in the order it names them, then its own. A field it declares again takes
/// the place of the one it inherits. Like a class's superclasses, the
/// extended types are read from the registry, and left out when it is empty.
///
/// The error names a type it extends whose fields cannot be read off: one
/// the registry does not hold, a generic one, or one that is no object type.
pub fn interface_instance_fields(
    interface: &InterfaceIR,
    declared: Declared<'_>,
) -> Result<Vec<InterfaceFieldIR>, String> {
    let mut fields = Vec::new();
    extend_with_inherited(
        &mut fields,
        interface,
        declared,
        &mut vec![interface.name.clone()],
    )?;
    if let Some(unnamed) = fields.iter().find(|field| field.key == MemberKey::Unnamed) {
        return Err(unnamed_member_message(&unnamed.name, &interface.name));
    }
    if fields.iter().any(|field| field.name == "__proto__") {
        return Err(prototype_member_message(&interface.name));
    }
    Ok(fields)
}

fn extend_with_inherited(
    fields: &mut Vec<InterfaceFieldIR>,
    interface: &InterfaceIR,
    declared: Declared<'_>,
    visiting: &mut Vec<String>,
) -> Result<(), String> {
    let declare = |fields: &mut Vec<InterfaceFieldIR>, field: &InterfaceFieldIR| match fields
        .iter_mut()
        .find(|known| known.name == field.name)
    {
        Some(known) => *known = field.clone(),
        None => fields.push(field.clone()),
    };
    for name in &interface.heritage {
        let Some(entry) = declared.resolve(name) else {
            if declared.registry.is_empty() {
                continue;
            }
            return Err(format!(
                "interface '{}' extends '{name}', which this project does not declare, so the fields it gives a value are unknown",
                interface.name
            ));
        };
        if visiting.contains(name) {
            return Err(format!("interface '{name}' extends itself"));
        }
        let generic = match &entry.definition {
            TypeDefinitionIR::Interface(base) => !base.type_params.is_empty(),
            TypeDefinitionIR::TypeAlias(base) => !base.type_params.is_empty(),
            TypeDefinitionIR::Class(base) => !base.type_params.is_empty(),
            TypeDefinitionIR::Enum(_) => false,
        };
        if generic {
            return Err(format!(
                "interface '{}' extends the generic type '{name}', whose fields depend on type arguments a derive cannot read off",
                interface.name
            ));
        }
        match &entry.definition {
            TypeDefinitionIR::Interface(base) => {
                visiting.push(name.clone());
                extend_with_inherited(fields, base, declared.at(entry), visiting)?;
                visiting.pop();
            }
            definition => {
                let Some(inherited) = fields_from_definition(definition, declared.registry) else {
                    return Err(format!(
                        "interface '{}' extends '{name}', which is not an object type",
                        interface.name
                    ));
                };
                for field in &inherited {
                    declare(fields, field);
                }
            }
        }
    }
    for field in &interface.fields {
        declare(fields, field);
    }
    // A method is a member that holds a function, as a field typed as one is.
    for method in &interface.methods {
        declare(fields, &method.as_field());
    }
    Ok(())
}

/// What a derive does with a class's instances.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ClassUse {
    /// It reads the fields of instances it is given.
    Reads,
    /// It also makes instances, without running the class's constructor.
    Builds,
}

/// The Svelte rune a field's initializer sets up, if it is one. Such a field
/// is state the class's constructor wires into the instance.
fn rune_of(initializer: &str) -> Option<&'static str> {
    ["$state", "$derived", "$props", "$bindable"]
        .into_iter()
        .find(|rune| {
            initializer
                .trim_start()
                .strip_prefix(rune)
                .is_some_and(|rest| rest.starts_with(['(', '.', '<']))
        })
}

/// The type of a field left untyped whose initializer is `$state`, read off
/// the rune: its type argument, or what the value it is given makes plain.
fn rune_state_type(field: &FieldIR) -> Option<String> {
    use oxc::allocator::Allocator;
    use oxc::ast::ast::Expression;
    use oxc::span::GetSpan;

    if !field.untyped {
        return None;
    }
    let initializer = field.initializer.as_deref()?;
    let text = |span: oxc::span::Span| &initializer[span.start as usize..span.end as usize];
    let allocator = Allocator::default();
    let parser = oxc::parser::Parser::new(&allocator, initializer, oxc::span::SourceType::ts());
    // The initializer was read out of a module that parsed.
    let parsed = parser.parse_expression().ok()?;
    let Expression::CallExpression(call) = parsed.without_parentheses() else {
        return None;
    };
    // `$state` and `$state.raw` hold the value they are given.
    if !matches!(text(call.callee.span()), "$state" | "$state.raw") {
        return None;
    }
    let given = call.arguments.first();
    match &call.type_arguments {
        Some(arguments) => {
            let [written] = arguments.params.as_slice() else {
                return None;
            };
            let held = text(written.span());
            // With nothing given, the state starts out `undefined`.
            Some(if given.is_some() {
                held.to_string()
            } else {
                format!("{held} | undefined")
            })
        }
        None => crate::ts_syn::initializer_type(given?.as_expression()?, false, initializer),
    }
}

/// The static members the classes a class extends hand down to it.
#[derive(Default)]
pub struct InheritedStatics {
    /// One of them derives the macro asking, so has each static it generates.
    derived: bool,
    /// The statics they declare themselves.
    declared: Vec<String>,
}

impl InheritedStatics {
    /// Whether the static `name` is handed down, so that a class's own
    /// overrides it. A `#private` one is never handed down.
    pub fn holds(&self, name: &str) -> bool {
        !name.starts_with('#') && (self.derived || self.declared.iter().any(|held| held == name))
    }
}

/// The static members the classes `class` extends hand down to it, as
/// `derive` on `class` meets them. A superclass the registry does not hold
/// hands down none that can be known, but for the `toString` every class has
/// as a function, which a class with a superclass overrides.
pub fn inherited_statics(input: &DeriveInput, class: &DataClass, derive: &str) -> InheritedStatics {
    let file_imports = input.context.import_registry.file_import_entries();
    let mut declared = Declared {
        registry: &input.context.type_registry,
        file_path: input.context.file_name.as_str(),
        file_imports: &file_imports,
    };
    let mut inherited = InheritedStatics::default();
    if class.inner.superclass.is_some() {
        inherited.declared.push("toString".to_string());
    }
    let mut visited = vec![class.inner.name.as_str()];
    let mut superclass = class.inner.superclass.as_deref();
    while let Some(entry) = superclass.and_then(|name| declared.resolve(name)) {
        let TypeDefinitionIR::Class(base) = &entry.definition else {
            break;
        };
        if visited.contains(&base.name.as_str()) {
            break;
        }
        visited.push(base.name.as_str());
        inherited.derived |= derives(&base.decorators, derive);
        inherited.declared.extend(
            base.methods
                .iter()
                .filter(|method| method.is_static)
                .map(|method| method.name.clone()),
        );
        declared = declared.at(entry);
        superclass = base.superclass.as_deref();
    }
    inherited
}

/// The fields a derive on `class` works over, inherited ones included, or
/// the error saying which one `derive` cannot reach, or cannot set up in an
/// instance it builds.
pub fn derive_class_fields(
    input: &DeriveInput,
    class: &DataClass,
    derive: &str,
    class_use: ClassUse,
) -> Result<Vec<FieldIR>, MacroforgeError> {
    let file_imports = input.context.import_registry.file_import_entries();
    let declared = Declared {
        registry: &input.context.type_registry,
        file_path: input.context.file_name.as_str(),
        file_imports: &file_imports,
    };
    let failure = |message: String| {
        MacroforgeError::new(
            input.decorator_span(),
            format!("@derive({derive}): {message}"),
        )
    };
    let fields = class_instance_fields(&class.inner, declared).map_err(failure)?;
    if class_use == ClassUse::Builds {
        let rune_field = fields.iter().find_map(|field| {
            let rune = rune_of(field.initializer.as_deref()?)?;
            Some((field, rune))
        });
        if let Some((field, rune)) = rune_field {
            return Err(failure(format!(
                "field '{}' of class '{}' is set up by `{rune}`, which only the class's constructor can do, and a derive builds an instance without running it",
                field.name, class.inner.name
            )));
        }
        if let Some(field) = fields.iter().find(|field| field.accessor) {
            return Err(failure(format!(
                "field '{}' of class '{}' is an `accessor`, whose value is kept where only the class's constructor can set it up, and a derive builds an instance without running it",
                field.name, class.inner.name
            )));
        }
    }
    Ok(fields)
}

/// The fields a derive on `interface` works over, inherited ones included,
/// or the error saying which type it extends `derive` cannot read.
pub fn derive_interface_fields(
    input: &DeriveInput,
    interface: &DataInterface,
    derive: &str,
) -> Result<Vec<InterfaceFieldIR>, MacroforgeError> {
    let file_imports = input.context.import_registry.file_import_entries();
    let declared = Declared {
        registry: &input.context.type_registry,
        file_path: input.context.file_name.as_str(),
        file_imports: &file_imports,
    };
    interface_instance_fields(&interface.inner, declared).map_err(|message| {
        MacroforgeError::new(
            input.decorator_span(),
            format!("@derive({derive}): {message}"),
        )
    })
}
