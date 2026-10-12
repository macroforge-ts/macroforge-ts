//! What the module around a built-in derive has to leave alone, and what its
//! declaration has to spell out, for the derive to read it.

use crate::ts_syn::abi::ir::class::MemberKey;
use crate::ts_syn::abi::ir::type_alias::{TypeBody, TypeMember, TypeMemberKind};
use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, InterfaceFieldIR, SpanIR};
use crate::ts_syn::import_registry::SourceImportEntry;

use super::DERIVE_MODULE_PATH;
use super::derive_targets::{DeriveTarget, DeriveTargetIR, diagnostic_span_for_derive};

/// The global types a derive reads a field's type as by its name, and writes
/// in the code it generates. A module that declares a type of one of these
/// names changes what both mean.
const TYPE_GLOBALS: &[&str] = &[
    "Array",
    "Date",
    "Extract",
    "Map",
    "NonNullable",
    "Omit",
    "Partial",
    "Pick",
    "Readonly",
    "ReadonlyArray",
    "ReadonlyMap",
    "ReadonlySet",
    "Record",
    "Required",
    "Set",
];

/// The global values the generated code builds and tests values through. A
/// module that declares a value of one of these names takes its place.
const VALUE_GLOBALS: &[&str] = &[
    "Array", "BigInt", "Boolean", "Date", "Error", "JSON", "Map", "Math", "Number", "Object",
    "Set", "String",
];

/// What a declaration gives its name to.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Declares {
    /// A type alone: an interface, a type alias, a type-only import.
    Type,
    /// A value alone: a function.
    Value,
    /// Both: a class, an enum, an import.
    TypeAndValue,
}

/// A name a module declares or imports at its top level, with the span of
/// its declaration when it has one of its own.
pub(super) struct ModuleName<'a> {
    pub(super) name: &'a str,
    pub(super) declares: Declares,
    pub(super) declared_at: Option<SpanIR>,
}

impl ModuleName<'_> {
    /// Whether the name takes a global's place where the derives use it.
    fn shadows_a_global(&self) -> bool {
        let as_type = matches!(self.declares, Declares::Type | Declares::TypeAndValue)
            && TYPE_GLOBALS.contains(&self.name);
        let as_value = matches!(self.declares, Declares::Value | Declares::TypeAndValue)
            && VALUE_GLOBALS.contains(&self.name);
        as_type || as_value
    }
}

fn is_builtin(target: &DeriveTarget) -> bool {
    target
        .macro_names
        .iter()
        .any(|(_, module)| module == DERIVE_MODULE_PATH)
}

/// An error for each of `names` that takes a global's name in a module with
/// a built-in derive. An import has no span here, so it is reported at the
/// first such derive.
pub(super) fn check_shadowed_globals<'a>(
    targets: &[DeriveTarget],
    names: impl Iterator<Item = ModuleName<'a>>,
    source: &str,
) -> Vec<Diagnostic> {
    let Some(first) = targets.iter().find(|target| is_builtin(target)) else {
        return Vec::new();
    };
    let mut shadowed: Vec<ModuleName<'a>> = names.filter(ModuleName::shadows_a_global).collect();
    shadowed.sort_by_key(|module_name| {
        (
            module_name.declared_at.map(|span| span.start),
            module_name.name,
        )
    });
    shadowed
        .into_iter()
        .map(|module_name| {
            let name = module_name.name;
            Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "this module gives the name `{name}` to something of its own, and the built-in derives in it read and write `{name}` as the global one"
                ),
                span: Some(module_name.declared_at.map_or_else(
                    || diagnostic_span_for_derive(first.decorator_span, source),
                    |declared_at| name_span(source, declared_at, name),
                )),
                notes: vec![],
                help: Some(format!(
                    "rename it, or import it under another name: `import {{ {name} as My{name} }}`"
                )),
            }
        })
        .collect()
}

/// The span of `name` where the declaration at `declared_at` writes it, or
/// the whole declaration's when it is not found.
fn name_span(source: &str, declared_at: SpanIR, name: &str) -> SpanIR {
    let range = declared_at.source_range();
    let is_ident = |letter: char| letter.is_alphanumeric() || matches!(letter, '_' | '$');
    let found = source.get(range.clone()).and_then(|declaration| {
        declaration.match_indices(name).find_map(|(offset, _)| {
            let before = declaration[..offset].chars().next_back();
            let after = declaration[offset + name.len()..].chars().next();
            (!before.is_some_and(is_ident) && !after.is_some_and(is_ident)).then_some(offset)
        })
    });
    let located = found.and_then(|offset| {
        let start = u32::try_from(range.start + offset).ok()?;
        let length = u32::try_from(name.len()).ok()?;
        Some(SpanIR::new(start + 1, start + length + 1))
    });
    located.unwrap_or(declared_at)
}

/// The names the declarations in `declarations` give, each with its span.
pub(super) fn declared_names<'a, Declaration: 'a>(
    declarations: impl Iterator<Item = &'a Declaration>,
    declares: Declares,
    name: impl Fn(&'a Declaration) -> &'a str,
    span: impl Fn(&'a Declaration) -> SpanIR,
) -> impl Iterator<Item = ModuleName<'a>> {
    declarations.map(move |declaration| ModuleName {
        name: name(declaration),
        declares,
        declared_at: Some(span(declaration)),
    })
}

/// The names `imports` binds, which carry no span. A macro's name is none
/// of the module's.
pub(super) fn imported_names(
    imports: &[SourceImportEntry],
) -> impl Iterator<Item = ModuleName<'_>> {
    imports.iter().map(|import| ModuleName {
        name: import.local_name.as_str(),
        declares: if import.is_type_only {
            Declares::Type
        } else {
            Declares::TypeAndValue
        },
        declared_at: None,
    })
}

/// An error for each member of a type alias with a built-in derive whose key
/// gives no name: a computed key or an index signature, in the alias's own
/// object type or in one written inside its union or intersection. A class's
/// and an interface's members are checked where their fields are gathered,
/// inherited ones with them.
pub(super) fn check_unnamed_alias_members(targets: &[DeriveTarget]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut reported = Vec::new();
    for target in targets.iter().filter(|target| is_builtin(target)) {
        let DeriveTargetIR::TypeAlias(alias) = &target.target_ir else {
            continue;
        };
        // A declaration's `@derive` directives each make a target.
        if reported.contains(&alias.span.start) {
            continue;
        }
        reported.push(alias.span.start);
        let mut unnamed = Vec::new();
        unnamed_in_body(&alias.body, &mut unnamed);
        diagnostics.extend(unnamed.into_iter().map(|field| Diagnostic {
            level: DiagnosticLevel::Error,
            message: unnamed_member_message(&field.name, &alias.name),
            span: Some(field.span),
            notes: vec![],
            help: None,
        }));
    }
    diagnostics
}

/// What a derive says of the member `member` of `owner`, whose key gives no
/// name: a computed key, or an index signature.
pub(crate) fn unnamed_member_message(member: &str, owner: &str) -> String {
    format!(
        "member `{member}` of '{owner}' has no name a derive can read it by: a computed key or an index signature is known only as the code runs. Give the member a name, or hold the entries in a field of their own, such as a `Record`"
    )
}

fn unnamed_in_body<'a>(body: &'a TypeBody, unnamed: &mut Vec<&'a InterfaceFieldIR>) {
    match body {
        TypeBody::Object { fields } => unnamed_in_fields(fields, unnamed),
        TypeBody::Union(members) | TypeBody::Intersection(members) => {
            unnamed_in_members(members, unnamed);
        }
        TypeBody::Newtype(inner) => unnamed_in_body(inner, unnamed),
        TypeBody::Tuple(_) | TypeBody::Alias(_) | TypeBody::Other(_) => {}
    }
}

fn unnamed_in_members<'a>(members: &'a [TypeMember], unnamed: &mut Vec<&'a InterfaceFieldIR>) {
    for member in members {
        match &member.kind {
            TypeMemberKind::Object { fields } => unnamed_in_fields(fields, unnamed),
            TypeMemberKind::Intersection(members) => unnamed_in_members(members, unnamed),
            TypeMemberKind::Literal(_) | TypeMemberKind::TypeRef(_) | TypeMemberKind::Brand(_) => {}
        }
    }
}

fn unnamed_in_fields<'a>(fields: &'a [InterfaceFieldIR], unnamed: &mut Vec<&'a InterfaceFieldIR>) {
    unnamed.extend(
        fields
            .iter()
            .filter(|field| field.key == MemberKey::Unnamed),
    );
}
