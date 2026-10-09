//! Rust's supertrait requirements between the built-in derives.

use std::collections::BTreeMap;

use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel};

use super::DERIVE_MODULE_PATH;
use super::derive_targets::{
    DeriveTarget, DeriveTargetIR, diagnostic_span_for_derive, find_macro_name_span,
};
use super::helpers::get_derive_target_start_span;

/// Each built-in derive with the built-in derives it requires, as in Rust:
/// `Eq: PartialEq`, `PartialOrd: PartialEq` and `Ord: Eq + PartialOrd`.
const REQUIREMENTS: &[(&str, &[&str])] = &[
    ("Eq", &["PartialEq"]),
    ("PartialOrd", &["PartialEq"]),
    ("Ord", &["Eq", "PartialOrd"]),
];

/// An error for each built-in derive whose declaration does not also derive
/// what it requires. A declaration's `@derive` directives count together.
pub(super) fn check_derive_requirements(targets: &[DeriveTarget], source: &str) -> Vec<Diagnostic> {
    let mut declarations: BTreeMap<u32, Vec<&DeriveTarget>> = BTreeMap::new();
    for target in targets {
        declarations
            .entry(get_derive_target_start_span(&target.target_ir))
            .or_default()
            .push(target);
    }

    let mut diagnostics = Vec::new();
    for directives in declarations.values() {
        let builtins = || {
            directives.iter().flat_map(|target| {
                target
                    .macro_names
                    .iter()
                    .filter(|(_, module)| module == DERIVE_MODULE_PATH)
                    .map(move |(name, _)| (*target, name.as_str()))
            })
        };
        for (target, name) in builtins() {
            let Some((_, required)) = REQUIREMENTS.iter().find(|(derive, _)| *derive == name)
            else {
                continue;
            };
            let missing: Vec<&str> = required
                .iter()
                .copied()
                .filter(|requirement| !builtins().any(|(_, derived)| derived == *requirement))
                .collect();
            if missing.is_empty() {
                continue;
            }
            let span = find_macro_name_span(source, target.decorator_span, name)
                .unwrap_or_else(|| diagnostic_span_for_derive(target.decorator_span, source));
            let missing = code_list(&missing);
            diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "`{name}` requires {missing}, which `{}` does not derive",
                    declaration_name(&target.target_ir)
                ),
                span: Some(span),
                notes: vec![],
                help: Some(format!("add {missing} to `@derive(...)`")),
            });
        }
    }
    diagnostics
}

fn declaration_name(target: &DeriveTargetIR) -> &str {
    match target {
        DeriveTargetIR::Class(class) => &class.name,
        DeriveTargetIR::Interface(interface) => &interface.name,
        DeriveTargetIR::Enum(enumeration) => &enumeration.name,
        DeriveTargetIR::TypeAlias(alias) => &alias.name,
    }
}

/// `names` as code spans joined for prose: "`A`" or "`A` and `B`".
fn code_list(names: &[&str]) -> String {
    names
        .iter()
        .map(|name| format!("`{name}`"))
        .collect::<Vec<_>>()
        .join(" and ")
}
