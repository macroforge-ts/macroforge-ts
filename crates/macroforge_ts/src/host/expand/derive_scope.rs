//! What the module around a built-in derive has to leave alone, and what its
//! declaration has to spell out, for the derive to read it.

use crate::ts_syn::abi::ir::class::MemberKey;
use crate::ts_syn::abi::ir::type_alias::{TypeBody, TypeMember, TypeMemberKind};
use crate::ts_syn::abi::{DecoratorIR, Diagnostic, DiagnosticLevel, InterfaceFieldIR, SpanIR};
use crate::ts_syn::import_registry::SourceImportEntry;

use super::DERIVE_MODULE_PATH;
use super::derive_targets::{DeriveTarget, DeriveTargetIR, diagnostic_span_for_derive};
use crate::builtin::derive::common::{
    DecoratorOption, Takes, decorator_options, extract_named_string, has_flag,
};
use crate::builtin::derive::endec::{ENDEC_OPTIONS, KNOWN_VALIDATORS, Placement, endec_option};

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

/// The functions each built-in derive names after the type it is on, as the
/// suffix that follows the type's name in camel case.
const GENERATED_SUFFIXES: &[(&str, &[&str])] = &[
    ("Debug", &["ToString"]),
    ("Clone", &["Clone"]),
    ("PartialEq", &["Equals"]),
    ("Hash", &["HashCode"]),
    ("PartialOrd", &["PartialCompare"]),
    ("Ord", &["Compare"]),
    ("Default", &["DefaultValue"]),
    (
        "Encode",
        &[
            "Encode",
            "EncodeWithContext",
            "EncodePartialWithContext",
            "EncodeFields",
        ],
    ),
    (
        "Decode",
        &[
            "Decode",
            "DecodeWithContext",
            "DecodePartialWithContext",
            "ValidateField",
            "ValidateFields",
            "HasShape",
            "Is",
        ],
    ),
];

fn target_name(target: &DeriveTarget) -> &str {
    match &target.target_ir {
        DeriveTargetIR::Class(class) => &class.name,
        DeriveTargetIR::Interface(interface) => &interface.name,
        DeriveTargetIR::Enum(declared) => &declared.name,
        DeriveTargetIR::TypeAlias(alias) => &alias.name,
    }
}

/// An error for each function a built-in derive would generate under a name
/// the module already has: one another derived type's function takes, as
/// `HTTPResponse` and `HttpResponse` both spell `httpResponseEncode`, or one
/// the module declares or imports itself. A second declaration of a function
/// replaces the first without a word.
pub(super) fn check_generated_names<'a>(
    targets: &[DeriveTarget],
    names: impl Iterator<Item = ModuleName<'a>>,
    source: &str,
) -> Vec<Diagnostic> {
    use convert_case::{Case, Casing};

    let module_names: Vec<&str> = names.map(|module_name| module_name.name).collect();
    // Each generated function, with the type that names it.
    let mut generated: Vec<(String, &str)> = Vec::new();
    // The pairs of types already reported, which collide on every function.
    let mut reported: Vec<(&str, &str)> = Vec::new();
    let mut diagnostics = Vec::new();
    for target in targets {
        let type_name = target_name(target);
        let prefix = type_name.to_case(Case::Camel);
        let span = diagnostic_span_for_derive(target.decorator_span, source);
        for (derive, module) in &target.macro_names {
            if module != DERIVE_MODULE_PATH {
                continue;
            }
            let Some((_, suffixes)) = GENERATED_SUFFIXES.iter().find(|(name, _)| name == derive)
            else {
                continue;
            };
            for suffix in *suffixes {
                let function = format!("{prefix}{suffix}");
                let other_type = generated
                    .iter()
                    .find(|(other, other_type)| *other == function && *other_type != type_name)
                    .map(|(_, other_type)| *other_type);
                if let Some(other_type) = other_type {
                    if !reported.contains(&(type_name, other_type)) {
                        reported.push((type_name, other_type));
                        diagnostics.push(Diagnostic {
                            level: DiagnosticLevel::Error,
                            message: format!(
                                "'{type_name}' and '{other_type}' both name their generated functions `{prefix}…`, as in `{function}`, so one type's would replace the other's"
                            ),
                            span: Some(span),
                            notes: vec![],
                            help: Some("rename one of the two types".to_string()),
                        });
                    }
                } else if module_names.contains(&function.as_str()) {
                    diagnostics.push(Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!(
                            "`@derive({derive})` on '{type_name}' generates `{function}`, a name this module already has"
                        ),
                        span: Some(span),
                        notes: vec![],
                        help: Some("rename the one the module declares or imports".to_string()),
                    });
                }
                generated.push((function, type_name));
            }
        }
    }
    diagnostics
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

/// The options each value-trait decorator takes on a field.
const DECORATOR_OPTIONS: &[(&str, &[(&str, Takes)])] = &[
    ("debug", &[("skip", Takes::Flag), ("rename", Takes::Text)]),
    ("hash", &[("skip", Takes::Flag)]),
    ("partialEq", &[("skip", Takes::Flag)]),
    ("ord", &[("skip", Takes::Flag)]),
];

/// The options the value-trait decorator `decorator` takes, when it is one.
fn trait_options(decorator: &DecoratorIR) -> Option<&'static [(&'static str, Takes)]> {
    DECORATOR_OPTIONS
        .iter()
        .find(|(name, _)| decorator.name.eq_ignore_ascii_case(name))
        .map(|(_, takes)| *takes)
}

fn is_endec(decorator: &&DecoratorIR) -> bool {
    decorator.name.eq_ignore_ascii_case("endec")
}

/// What a message calls an option: its name, or all of it when it has none.
fn written<'a>(option: &DecoratorOption<'a>) -> &'a str {
    if option.name.is_empty() {
        option.text
    } else {
        option.name
    }
}

fn option_error(decorator: &DecoratorIR, message: String) -> Diagnostic {
    Diagnostic {
        level: DiagnosticLevel::Error,
        message,
        span: Some(decorator.span),
        notes: vec![],
        help: None,
    }
}

/// The decorators of each member of `target`, at any depth of an alias's
/// body: one slice a member.
fn member_decorators(target: &DeriveTargetIR) -> Vec<&[DecoratorIR]> {
    fn of_fields<'a>(fields: &'a [InterfaceFieldIR], found: &mut Vec<&'a [DecoratorIR]>) {
        found.extend(fields.iter().map(|field| field.decorators.as_slice()));
    }
    fn of_members<'a>(members: &'a [TypeMember], found: &mut Vec<&'a [DecoratorIR]>) {
        for member in members {
            found.push(&member.decorators);
            match &member.kind {
                TypeMemberKind::Object { fields } => of_fields(fields, found),
                TypeMemberKind::Intersection(members) => of_members(members, found),
                TypeMemberKind::Literal(_)
                | TypeMemberKind::TypeRef(_)
                | TypeMemberKind::Brand(_) => {}
            }
        }
    }
    fn of_body<'a>(body: &'a TypeBody, found: &mut Vec<&'a [DecoratorIR]>) {
        match body {
            TypeBody::Object { fields } => of_fields(fields, found),
            TypeBody::Union(members) | TypeBody::Intersection(members) => {
                of_members(members, found)
            }
            TypeBody::Newtype(inner) => of_body(inner, found),
            TypeBody::Tuple(_) | TypeBody::Alias(_) | TypeBody::Other(_) => {}
        }
    }
    let mut found = Vec::new();
    match target {
        DeriveTargetIR::Class(class) => {
            found.extend(class.fields.iter().map(|field| field.decorators.as_slice()));
        }
        DeriveTargetIR::Interface(interface) => {
            of_fields(&interface.fields, &mut found);
            found.extend(
                interface
                    .methods
                    .iter()
                    .map(|method| method.decorators.as_slice()),
            );
        }
        DeriveTargetIR::Enum(declared) => {
            found.extend(
                declared
                    .variants
                    .iter()
                    .map(|variant| variant.decorators.as_slice()),
            );
        }
        DeriveTargetIR::TypeAlias(alias) => of_body(&alias.body, &mut found),
    }
    found
}

fn type_decorators(target: &DeriveTargetIR) -> &[DecoratorIR] {
    match target {
        DeriveTargetIR::Class(class) => &class.decorators,
        DeriveTargetIR::Interface(interface) => &interface.decorators,
        DeriveTargetIR::Enum(declared) => &declared.decorators,
        DeriveTargetIR::TypeAlias(alias) => &alias.decorators,
    }
}

/// The name of the type `target` declares, and where its declaration starts.
fn declared(target: &DeriveTargetIR) -> (&str, u32) {
    match target {
        DeriveTargetIR::Class(class) => (&class.name, class.span.start),
        DeriveTargetIR::Interface(interface) => (&interface.name, interface.span.start),
        DeriveTargetIR::Enum(declared) => (&declared.name, declared.span.start),
        DeriveTargetIR::TypeAlias(alias) => (&alias.name, alias.span.start),
    }
}

/// An error for each option of a built-in decorator that the decorator does
/// not take, is given a value of the wrong kind, is given twice or is
/// written where it does nothing, on a type with a built-in derive or on a
/// member of one. A misspelled `skip` would otherwise skip nothing, and a
/// `skip: "no"` skip what it was told to keep.
pub(super) fn check_decorator_options(targets: &[DeriveTarget]) -> Vec<Diagnostic> {
    let mut checked = Vec::new();
    let mut diagnostics = Vec::new();
    for target in targets.iter().filter(|target| is_builtin(target)) {
        // A declaration's `@derive` directives each make a target.
        let (_, declared_at) = declared(&target.target_ir);
        if checked.contains(&declared_at) {
            continue;
        }
        checked.push(declared_at);
        let on_type = type_decorators(&target.target_ir);
        let on_members = member_decorators(&target.target_ir);
        for group in std::iter::once(on_type).chain(on_members.iter().copied()) {
            for decorator in group {
                let Some(takes) = trait_options(decorator) else {
                    continue;
                };
                for option in decorator_options(&decorator.args_src) {
                    let known = takes
                        .iter()
                        .find(|(known, _)| known.eq_ignore_ascii_case(option.name));
                    let message = match known {
                        Some((known, takes)) => takes
                            .refuses(option.value)
                            .map(|why| format!("`@{}`: `{known}` {why}", decorator.name)),
                        None => Some(format!(
                            "`@{}` takes `{}`, not `{}`",
                            decorator.name,
                            takes
                                .iter()
                                .map(|(known, _)| *known)
                                .collect::<Vec<_>>()
                                .join("`, `"),
                            written(&option)
                        )),
                    };
                    diagnostics.extend(message.map(|message| option_error(decorator, message)));
                }
            }
            diagnostics.extend(repeated_options(group));
        }
        check_endec_options(target, on_type, &on_members, &mut diagnostics);
    }
    diagnostics
}

/// An error for each option the decorators of one member, or of the type
/// itself, give more than once: which of the two holds is not said.
fn repeated_options(group: &[DecoratorIR]) -> Vec<Diagnostic> {
    let mut given: Vec<(&str, &str)> = Vec::new();
    let mut diagnostics = Vec::new();
    for decorator in group {
        let endec = is_endec(&decorator);
        let takes = trait_options(decorator);
        for option in decorator_options(&decorator.args_src) {
            let known = match takes {
                Some(takes) => takes
                    .iter()
                    .map(|(known, _)| *known)
                    .find(|known| known.eq_ignore_ascii_case(option.name)),
                None if endec => endec_option(option.name).map(|known| known.name),
                None => None,
            };
            let Some(known) = known else {
                continue;
            };
            let entry = (decorator.name.as_str(), known);
            if given
                .iter()
                .any(|(name, option)| name.eq_ignore_ascii_case(entry.0) && *option == known)
            {
                diagnostics.push(option_error(
                    decorator,
                    format!("`@{}` is given `{known}` more than once", decorator.name),
                ));
            } else {
                given.push(entry);
            }
        }
    }
    diagnostics
}

/// The errors of `@endec` options on `target` and its members: a value of
/// the wrong kind, an option of a field on the type or of the type on a
/// field, and ways of telling a union's members apart that exclude one
/// another or have no union to apply to.
fn check_endec_options(
    target: &DeriveTarget,
    on_type: &[DecoratorIR],
    on_members: &[&[DecoratorIR]],
    diagnostics: &mut Vec<Diagnostic>,
) {
    let (type_name, _) = declared(&target.target_ir);
    let endec_on_type: Vec<&DecoratorIR> = on_type.iter().filter(is_endec).collect();
    let endec_on_members = || {
        on_members
            .iter()
            .flat_map(|group| group.iter())
            .filter(is_endec)
    };
    if matches!(target.target_ir, DeriveTargetIR::Enum(_)) {
        // An enum's values are coded as they are.
        diagnostics.extend(
            endec_on_type
                .iter()
                .copied()
                .chain(endec_on_members())
                .filter(|decorator| !decorator.args_src.trim().is_empty())
                .map(|decorator| {
                    option_error(
                        decorator,
                        format!(
                            "`@endec` configures the fields of a type and the members of a union, and the enum '{type_name}' has neither"
                        ),
                    )
                }),
        );
        return;
    }
    // `Decode` reads an alias's own `@endec` for validators, and reports
    // what it does not know there.
    let read_by_decode = matches!(target.target_ir, DeriveTargetIR::TypeAlias(_))
        && target.macro_names.iter().any(|(name, _)| name == "Decode");
    for decorator in &endec_on_type {
        for option in decorator_options(&decorator.args_src) {
            let message = match endec_option(option.name) {
                Some(known) if known.placement == Placement::Field => Some(format!(
                    "`@endec`: `{}` is an option of a field, and does nothing on the type '{type_name}'",
                    known.name
                )),
                Some(known) => known
                    .takes
                    .refuses(option.value)
                    .map(|why| format!("`@endec`: `{}` {why}", known.name)),
                None if read_by_decode
                    || KNOWN_VALIDATORS
                        .iter()
                        .any(|known| known.eq_ignore_ascii_case(option.name)) =>
                {
                    None
                }
                None => Some(format!(
                    "`@endec` takes `{}` on a type, not `{}`",
                    ENDEC_OPTIONS
                        .iter()
                        .filter(|known| known.placement != Placement::Field)
                        .map(|known| known.name)
                        .collect::<Vec<_>>()
                        .join("`, `"),
                    written(&option)
                )),
            };
            diagnostics.extend(message.map(|message| option_error(decorator, message)));
        }
    }
    for decorator in endec_on_members() {
        for option in decorator_options(&decorator.args_src) {
            let Some(known) = endec_option(option.name) else {
                continue;
            };
            let message = if known.placement == Placement::Type {
                Some(format!(
                    "`@endec`: `{}` is an option of a type, and does nothing on a member of '{type_name}'",
                    known.name
                ))
            } else {
                known
                    .takes
                    .refuses(option.value)
                    .map(|why| format!("`@endec`: `{}` {why}", known.name))
            };
            diagnostics.extend(message.map(|message| option_error(decorator, message)));
        }
    }

    let Some(first) = endec_on_type.first() else {
        return;
    };
    let on = |flag: &str| {
        endec_on_type
            .iter()
            .any(|decorator| has_flag(&decorator.args_src, flag))
    };
    let text = |name: &str| {
        endec_on_type
            .iter()
            .find_map(|decorator| extract_named_string(&decorator.args_src, name))
    };
    let (tag, content) = (text("tag"), text("content"));
    let modes: Vec<&str> = [
        ("untagged", on("untagged")),
        ("externallyTagged", on("externallyTagged")),
        ("tag", tag.is_some()),
    ]
    .into_iter()
    .filter_map(|(mode, given)| given.then_some(mode))
    .collect();
    if modes.len() > 1 {
        diagnostics.push(option_error(
            first,
            format!(
                "`@endec`: `{}` each say how the members of a union are told apart; give one of them",
                modes.join("` and `")
            ),
        ));
    }
    match (&tag, &content) {
        (None, Some(_)) => diagnostics.push(option_error(
            first,
            "`@endec`: `content` names the key a tagged member's value is held under, and is given without `tag`"
                .to_string(),
        )),
        (Some(tag), Some(content)) if tag == content => diagnostics.push(option_error(
            first,
            format!(
                "`@endec`: `tag` and `content` are both \"{tag}\", and a member's tag and its value cannot be held under one key"
            ),
        )),
        (None, None) | (Some(_), None) | (Some(_), Some(_)) => {}
    }
    // An object type is one shape: it is tagged, and that is all.
    let is_one_shape = match &target.target_ir {
        DeriveTargetIR::Class(_) | DeriveTargetIR::Interface(_) => true,
        DeriveTargetIR::TypeAlias(alias) => {
            matches!(alias.body.unbranded(), TypeBody::Object { .. })
        }
        DeriveTargetIR::Enum(_) => false,
    };
    if is_one_shape {
        let unions_only = [
            ("untagged", on("untagged")),
            ("externallyTagged", on("externallyTagged")),
            ("content", content.is_some()),
        ];
        diagnostics.extend(unions_only.into_iter().filter(|(_, given)| *given).map(
            |(option, _)| {
                option_error(
                    first,
                    format!(
                        "`@endec`: `{option}` says how the members of a union are told apart, and '{type_name}' is not a union"
                    ),
                )
            },
        ));
    }
}

/// An error for each `declare enum` with a built-in derive. The functions a
/// derive generates read the enum's members as the code runs, and the module
/// a `declare enum` is written in has no such value.
pub(super) fn check_ambient_enums(targets: &[DeriveTarget], source: &str) -> Vec<Diagnostic> {
    let mut reported = Vec::new();
    targets
        .iter()
        .filter(|target| is_builtin(target))
        .filter_map(|target| {
            let DeriveTargetIR::Enum(declared) = &target.target_ir else {
                return None;
            };
            // A declaration's `@derive` directives each make a target.
            if !declared.is_declare || reported.contains(&declared.span.start) {
                return None;
            }
            reported.push(declared.span.start);
            Some(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "'{}' is a `declare enum`: it describes an enum this module does not hold, so the functions a derive generates would have no members to read",
                    declared.name
                ),
                span: Some(diagnostic_span_for_derive(target.decorator_span, source)),
                notes: vec![],
                help: Some("declare it as an `enum`, or derive where it is defined".to_string()),
            })
        })
        .collect()
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
            message: if field.key == MemberKey::Unnamed {
                unnamed_member_message(&field.name, &alias.name)
            } else {
                prototype_member_message(&alias.name)
            },
            span: Some(field.span),
            notes: vec![],
            help: None,
        }));
    }
    diagnostics
}

/// What a derive says of a field of `owner` named `__proto__`: the name
/// reaches the prototype every object has, not a field of its own.
pub(crate) fn prototype_member_message(owner: &str) -> String {
    format!(
        "field '__proto__' of '{owner}' has the name every object reaches its prototype by, so reading or assigning it would not reach a field; give it another name"
    )
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
            .filter(|field| field.key == MemberKey::Unnamed || field.name == "__proto__"),
    );
}
