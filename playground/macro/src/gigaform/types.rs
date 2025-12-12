//! Generates type definitions for Gigaform: Errors, Tainted, and Gigaform<T> types.

use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::TsStream;

use crate::gigaform::parser::ParsedField;

/// Generates the Errors, Tainted, and Gigaform<T> type definitions.
pub fn generate(interface_name: &str, fields: &[ParsedField]) -> TsStream {
    let errors_fields = generate_errors_fields(fields);
    let tainted_fields = generate_tainted_fields(fields);
    let field_controller_types = generate_field_controller_types(fields);

    ts_template! {
        {>> "Nested error structure matching the data shape" <<}
        export type Errors = {
            _errors?: Array<string>;
            @{errors_fields}
        };

        {>> "Nested boolean structure for tracking touched/dirty fields" <<}
        export type Tainted = {
            @{tainted_fields}
        };

        {>> "Field controller interface for a single field" <<}
        export interface FieldController<T> {
            readonly path: ReadonlyArray<string | number>;
            readonly name: string;
            readonly constraints: Record<string, unknown>;
            readonly label?: string;
            readonly description?: string;
            readonly placeholder?: string;
            readonly disabled?: boolean;
            readonly readonly?: boolean;
            get(): T;
            set(value: T): void;
            getError(): Array<string> | undefined;
            setError(value: Array<string> | undefined): void;
            getTainted(): boolean;
            setTainted(value: boolean): void;
            validate(): Array<string>;
        }

        {>> "Type-safe field controllers for this form" <<}
        export interface FieldControllers {
            @{field_controller_types}
        }

        {>> "Gigaform instance containing reactive state and field controllers" <<}
        export interface Gigaform {
            readonly data: @{interface_name};
            readonly errors: Errors;
            readonly tainted: Tainted;
            readonly fields: FieldControllers;
            validate(): Result<@{interface_name}, Array<{ field: string; message: string }>>;
            reset(overrides?: Partial<@{interface_name}>): void;
        }
    }
}

/// Generates the FieldControllers type entries.
fn generate_field_controller_types(fields: &[ParsedField]) -> String {
    fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let ts_type = &field.ts_type;
            format!("readonly {name}: FieldController<{ts_type}>;")
        })
        .collect::<Vec<_>>()
        .join("\n            ")
}

/// Generates the Errors type fields.
fn generate_errors_fields(fields: &[ParsedField]) -> String {
    fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let optional = "?";

            if field.is_array {
                // Array fields: { _errors?: string[]; [index: number]: string[] | NestedType.Errors }
                let element_type = field.array_element_type.as_deref().unwrap_or("unknown");
                let is_element_nested = is_nested_type(element_type);

                if is_element_nested {
                    format!(
                        "{name}{optional}: {{ _errors?: Array<string>; [index: number]: {element_type}.Errors }};"
                    )
                } else {
                    format!(
                        "{name}{optional}: {{ _errors?: Array<string>; [index: number]: Array<string> }};"
                    )
                }
            } else if field.is_nested {
                // Nested object: reference child's Errors type
                let nested_type = field.nested_type.as_deref().unwrap_or("unknown");
                format!("{name}{optional}: {nested_type}.Errors;")
            } else {
                // Primitive field: simple string array
                format!("{name}{optional}: Array<string>;")
            }
        })
        .collect::<Vec<_>>()
        .join("\n            ")
}

/// Generates the Tainted type fields.
fn generate_tainted_fields(fields: &[ParsedField]) -> String {
    fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let optional = "?";

            if field.is_array {
                // Array fields: { [index: number]: boolean | NestedType.Tainted }
                let element_type = field.array_element_type.as_deref().unwrap_or("unknown");
                let is_element_nested = is_nested_type(element_type);

                if is_element_nested {
                    format!("{name}{optional}: {{ [index: number]: {element_type}.Tainted }};")
                } else {
                    format!("{name}{optional}: {{ [index: number]: boolean }};")
                }
            } else if field.is_nested {
                // Nested object: reference child's Tainted type
                let nested_type = field.nested_type.as_deref().unwrap_or("unknown");
                format!("{name}{optional}: {nested_type}.Tainted;")
            } else {
                // Primitive field: simple boolean
                format!("{name}{optional}: boolean;")
            }
        })
        .collect::<Vec<_>>()
        .join("\n            ")
}

/// Checks if a type name represents a nested (non-primitive) type.
fn is_nested_type(type_name: &str) -> bool {
    let trimmed = type_name.trim();

    // Primitives are not nested
    let primitives = [
        "string",
        "number",
        "boolean",
        "Date",
        "bigint",
        "symbol",
        "undefined",
        "null",
        "unknown",
        "any",
        "never",
        "void",
    ];
    if primitives.contains(&trimmed) {
        return false;
    }

    // PascalCase identifiers are assumed to be nested types
    trimmed
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
}
