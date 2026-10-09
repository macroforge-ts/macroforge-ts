//! The `Default` derive: dispatch by data kind, and enums.

use std::cell::RefCell;

use convert_case::{Case, Casing};

use crate::ast::Expr;
use crate::macros::{ts_macro_derive, ts_template};
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::ts_ident;
use crate::ts_syn::{
    Data, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, parse_ts_macro_input,
};

use super::class::class_default;
use super::object::object_default;
use super::type_alias::type_alias_default;
use super::values::DefaultSources;
use crate::builtin::derive::common::TypeNames;

#[ts_macro_derive(
    Default,
    description = "Generates a static defaultValue() factory method",
    attributes(default)
)]
pub fn derive_default_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let file_imports = input.context.import_registry.file_import_entries();
    let sources = DefaultSources {
        registry: &input.context.type_registry,
        caller_file_path: input.context.file_name.as_str(),
        file_imports: &file_imports,
        rejected: RefCell::default(),
    };

    let generated = match &input.data {
        Data::Class(class) => class_default(&input, class, &sources),
        Data::Enum(enum_data) => {
            let enum_name = input.name();
            let enum_ident = ts_ident!(enum_name);

            // Find variant with @default attribute (like Rust's #[default] on enums)
            let default_variant = enum_data.variants().iter().find(|v| {
                v.decorators
                    .iter()
                    .any(|d| d.name.eq_ignore_ascii_case("default"))
            });

            match default_variant {
                Some(variant) => {
                    let variant_name = &variant.name;
                    let fn_name_ident = ts_ident!("{}DefaultValue", enum_name.to_case(Case::Camel));
                    let enum_expr: Expr = ts_ident!(enum_name).into();
                    let variant_ident = ts_ident!(variant_name.as_str());
                    Ok(ts_template! {
                        export function @{fn_name_ident}(): @{enum_ident} {
                            return @{enum_expr}.@{variant_ident};
                        }
                    })
                }
                None => Err(MacroforgeError::new(
                    input.decorator_span(),
                    format!(
                        "@derive(Default) on enum requires exactly one variant with @default attribute. \
                        Add @default to one variant of {}",
                        enum_name
                    ),
                )),
            }
        }
        Data::Interface(interface) => object_default(
            &TypeNames::new(input.name(), interface.type_params()),
            interface.fields(),
            input.decorator_span(),
            &sources,
        ),
        Data::TypeAlias(type_alias) => type_alias_default(&input, type_alias, &sources),
    }?;
    let rejected = sources.rejected.take();
    if rejected.is_empty() {
        return Ok(generated);
    }
    let mut diagnostics = DiagnosticCollector::new();
    for message in rejected {
        diagnostics.error(input.decorator_span(), message);
    }
    Err(MacroforgeErrors::new(diagnostics.into_vec()).into())
}
