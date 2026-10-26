//! `Encode` for a class: standalone functions and static wrappers.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    DataClass, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident,
};

use super::field::prepare::to_encode_field;
use super::object_encoder::encode_with_context_functions;
use super::text::encode_text_body;
use super::types::EncodeField;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::derive::endec::EndecContainerOptions;
use crate::builtin::return_types::ENCODE_CONTEXT;

pub(super) fn handle_class(
    input: &DeriveInput,
    class: &DataClass,
) -> Result<TsStream, MacroforgeError> {
    let type_registry = &input.context.type_registry;
    let caller_file_path = input.context.file_name.as_str();
    let file_imports = input.context.import_registry.file_import_entries();
    let file_imports = file_imports.as_slice();
    let class_name = input.name();
    let class_names = TypeNames::new(class_name, class.type_params());
    let class_ident = class_names.full_type_ident();
    let encode_context_ident = ts_ident!(ENCODE_CONTEXT);
    let container_opts = EndecContainerOptions::from_decorators(&class.inner.decorators);
    let tag_field = container_opts.tag_field_or_default();

    // Generate function names (always prefix style)
    let fn_encode_ident = class_names.generic_function("Encode");
    let fn_encode_expr: Expr = ts_ident!(class_names.function("Encode")).into();
    let fn_encode_internal_call: Expr = ts_ident!(class_names.function("EncodeWithContext")).into();

    // Collect encodable fields with diagnostic collection
    let type_params = class_names.params.clone();
    let mut all_diagnostics = DiagnosticCollector::new();
    let fields: Vec<EncodeField> = class
        .fields()
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

    let encode_text = encode_text_body(&fn_encode_internal_call, tag_field);
    let with_context = encode_with_context_functions(&class_names, tag_field, &fields);
    let mut standalone = ts_template! {
        /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
        export function @{fn_encode_ident}(value: @{class_ident}, keepMetadata?: boolean): string {
            {$typescript encode_text}
        }

        {$typescript with_context}
    };

    // Generate static wrapper methods that delegate to standalone functions
    let fn_encode_internal_expr_class = fn_encode_internal_call;
    let class_body = ts_template!(Within {
        /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
        static @{class_names.generic_method("encode")}(value: @{class_ident}, keepMetadata?: boolean): string {
            return @{fn_encode_expr}(value, keepMetadata);
        }

        /** @internal Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
        static @{class_names.generic_method("encodeWithContext")}(value: @{class_ident}, ctx: @{encode_context_ident}): Record<string, unknown> {
            return @{fn_encode_internal_expr_class}(value, ctx);
        }
    });

    // Combine standalone functions with class body
    // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
    standalone.add_diagnostics(all_diagnostics.into_vec());
    Ok(standalone.merge(class_body))
}
