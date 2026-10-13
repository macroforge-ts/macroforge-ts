//! `Encode` for a class: static methods and the standalone functions that run them.

use crate::ast::Expr;
use crate::builtin::derive::endec::coder::TypeScope;
use crate::builtin::derive::endec::reserved_keys::check_field_keys;
use crate::builtin::derive::endec::source_field::SourceField;
use crate::builtin::derive::endec::union_guard::Direction;
use crate::builtin::derive::endec::untyped_fields::check_untyped_fields;
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    DataClass, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident,
};

use super::field::prepare::to_encode_field;
use super::object_encoder::{Home, encode_with_context_functions};
use super::text::encode_text_body;
use super::types::EncodeField;
use crate::builtin::derive::common::{ClassUse, TypeNames, derive_class_fields, inherited_statics};
use crate::builtin::derive::endec::EndecContainerOptions;
use crate::builtin::derive::endec::type_arguments::{declared_param_coders, passed_param_coders};

pub(super) fn handle_class(
    input: &DeriveInput,
    class: &DataClass,
) -> Result<TsStream, MacroforgeError> {
    let type_registry = &input.context.type_registry;
    let caller_file_path = input.context.file_name.as_str();
    let file_imports = input.context.import_registry.file_import_entries();
    let file_imports = file_imports.as_slice();
    let class_name = input.name();
    let class_names = TypeNames::new(class_name, class.type_params())
        .inheriting(inherited_statics(input, class, "Encode"));
    let class_ident = class_names.full_type_ident();
    let container_opts = EndecContainerOptions::from_decorators(&class.inner.decorators);
    let tag_field = container_opts.tag_field_or_default();

    // Generate function names (always prefix style)
    let fn_encode_ident = class_names.generic_function("Encode");
    let fn_encode_expr: Expr = ts_ident!(class_names.function("Encode")).into();
    let fn_encode_internal_call: Expr = ts_ident!(class_names.function("EncodeWithContext")).into();

    // Collect encodable fields with diagnostic collection
    let type_params = class_names.params.clone();
    let class_fields = derive_class_fields(input, class, "Encode", ClassUse::Reads)?;
    let mut all_diagnostics = DiagnosticCollector::new();
    let key_scope = TypeScope {
        registry: type_registry,
        file_path: caller_file_path,
        file_imports,
        type_params: &type_params,
    };
    check_field_keys(
        Direction::Encode,
        &container_opts,
        class_fields.iter().map(SourceField::from),
        &key_scope,
        &mut all_diagnostics,
    );
    check_untyped_fields(Direction::Encode, &class_fields, &mut all_diagnostics);
    let fields: Vec<EncodeField> = class_fields
        .iter()
        .filter_map(|field| {
            to_encode_field(
                field.into(),
                &container_opts,
                &mut all_diagnostics,
                type_registry,
                caller_file_path,
                file_imports,
                &type_params,
            )
        })
        .collect();

    // Check for errors in field parsing before continuing
    if all_diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
    }

    let coders = declared_param_coders(Direction::Encode, &class_names.params);
    let passed_coders = passed_param_coders(Direction::Encode, &class_names.params);
    let encode_text = encode_text_body(&fn_encode_internal_call, tag_field, &passed_coders);
    let with_context = encode_with_context_functions(&class_names, tag_field, &fields, Home::Class);
    let standalone = ts_template! {
        /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
        export function @{fn_encode_ident}(value: @{&class_ident}, keepMetadata?: boolean@{&coders}): string {
            {$typescript encode_text}
        }
    };
    let class_body = ts_template!(Within {
        /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
        static @{class_names.generic_method("encode")}(value: @{&class_ident}, keepMetadata?: boolean@{&coders}): string {
            return @{fn_encode_expr}(value, keepMetadata@{&passed_coders});
        }
    });

    // Every standalone function comes before the class members, so the
    // functions land below the class and the members inside it.
    let mut result = standalone.merge(with_context).merge(class_body);
    result.add_diagnostics(all_diagnostics.into_vec());
    Ok(result)
}
