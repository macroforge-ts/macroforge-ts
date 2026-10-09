//! `Encode` for a class: standalone functions and static wrappers.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    DataClass, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident,
};

use super::field::prepare::to_encode_field;
use super::field::statements::encode_fields;
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
    let fn_encode_internal_ident = class_names.generic_function("EncodeWithContext");
    let fn_encode_internal_call: Expr = ts_ident!(class_names.function("EncodeWithContext")).into();

    // Create Expr version of ENCODE_CONTEXT for expression positions
    let encode_context_expr: Expr = encode_context_ident.clone().into();

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

    let field_statements = encode_fields(&fields, tag_field);

    // Generate standalone functions
    // Clone ident for use in standalone template and later for class body
    let fn_encode_internal_ident_standalone = fn_encode_internal_ident;
    let fn_encode_internal_expr_standalone = fn_encode_internal_call.clone();
    let mut standalone = ts_template! {
        /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
        export function @{fn_encode_ident}(value: @{class_ident}, keepMetadata?: boolean): string {
            const ctx = @{encode_context_expr}.create();
            const __raw = @{fn_encode_internal_expr_standalone}(value, ctx);
            if (keepMetadata) return JSON.stringify(__raw);
            return JSON.stringify(__raw, (key, val) => key === "@{tag_field}" || key === "__id" ? undefined : val);
        }

        /** @internal Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
        export function @{fn_encode_internal_ident_standalone}(value: @{class_ident}, ctx: @{encode_context_ident}): Record<string, unknown> {
            // Check if already encoded (cycle detection)
            const existingId = ctx.getId(value);
            if (existingId !== undefined) {
                return { __ref: existingId };
            }

            // Register this object
            const __id = ctx.register(value);

            const result: Record<string, unknown> = {
                "@{tag_field}": "@{class_name}",
                __id,
            };

            {$typescript field_statements}

            return result;
        }
    };
    standalone.add_aliased_import("EncodeContext", crate::package::ENDEC);

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
