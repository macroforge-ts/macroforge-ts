//! Generates type definitions for Gigaform: Errors, Tainted, and Gigaform<T> types.

use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::TsStream;

use crate::gigaform::parser::{ParsedField, UnionConfig, UnionMode};
use crate::gigaform::GenericInfo;

/// Generates the Errors, Tainted, and Gigaform<T> type definitions.
pub fn generate(interface_name: &str, fields: &[ParsedField]) -> TsStream {
    let errors_fields = generate_errors_fields(fields);
    let tainted_fields = generate_tainted_fields(fields);
    let field_controller_types = generate_field_controller_types(fields);

    ts_template! {
        {>> "Nested error structure matching the data shape" <<}
        export type Errors = {
            _errors: Option<Array<string>>;
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
            getError(): Option<Array<string>>;
            setError(value: Option<Array<string>>): void;
            getTainted(): Option<boolean>;
            setTainted(value: Option<boolean>): void;
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

/// Generates type definitions with generic support.
pub fn generate_with_generics(interface_name: &str, fields: &[ParsedField], generics: &GenericInfo) -> TsStream {
    if generics.is_empty() {
        return generate(interface_name, fields);
    }

    let errors_fields = generate_errors_fields(fields);
    let tainted_fields = generate_tainted_fields(fields);
    let field_controller_types = generate_field_controller_types(fields);
    let generic_decl = generics.decl();
    let generic_args = generics.args();

    ts_template! {
        {>> "Nested error structure matching the data shape" <<}
        export type Errors@{generic_decl} = {
            _errors: Option<Array<string>>;
            @{errors_fields}
        };

        {>> "Nested boolean structure for tracking touched/dirty fields" <<}
        export type Tainted@{generic_decl} = {
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
            getError(): Option<Array<string>>;
            setError(value: Option<Array<string>>): void;
            getTainted(): Option<boolean>;
            setTainted(value: Option<boolean>): void;
            validate(): Array<string>;
        }

        {>> "Type-safe field controllers for this form" <<}
        export interface FieldControllers@{generic_decl} {
            @{field_controller_types}
        }

        {>> "Gigaform instance containing reactive state and field controllers" <<}
        export interface Gigaform@{generic_decl} {
            readonly data: @{interface_name}@{generic_args};
            readonly errors: Errors@{generic_args};
            readonly tainted: Tainted@{generic_args};
            readonly fields: FieldControllers@{generic_args};
            validate(): Result<@{interface_name}@{generic_args}, Array<{ field: string; message: string }>>;
            reset(overrides?: Partial<@{interface_name}@{generic_args}>): void;
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
            if field.is_array {
                // Array fields use ArrayFieldController with the element type
                let element_type = field.array_element_type.as_deref().unwrap_or("unknown");
                format!("readonly {name}: ArrayFieldController<{element_type}>;")
            } else {
                format!("readonly {name}: FieldController<{ts_type}>;")
            }
        })
        .collect::<Vec<_>>()
        .join("\n            ")
}

/// Generates the Errors type fields.
/// All fields use Option<Array<string>> for consistency with FieldController interface.
fn generate_errors_fields(fields: &[ParsedField]) -> String {
    fields
        .iter()
        .map(|field| {
            let name = &field.name;
            // All fields use Option<Array<string>> for FieldController compatibility
            format!("{name}: Option<Array<string>>;")
        })
        .collect::<Vec<_>>()
        .join("\n            ")
}

/// Generates the Tainted type fields.
/// All fields use Option<boolean> for consistency with FieldController interface.
fn generate_tainted_fields(fields: &[ParsedField]) -> String {
    fields
        .iter()
        .map(|field| {
            let name = &field.name;
            // All fields use Option<boolean> for FieldController compatibility
            format!("{name}: Option<boolean>;")
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

// =============================================================================
// Union Type Generation
// =============================================================================

/// Generates types for a discriminated union form with generic support.
/// Note: Unions typically don't have type parameters, so this delegates to generate_union.
pub fn generate_union_with_generics(type_name: &str, config: &UnionConfig, _generics: &GenericInfo) -> TsStream {
    // Unions are usually concrete types, generics are passed through but not commonly used
    generate_union(type_name, config)
}

/// Generates types for a discriminated union form.
pub fn generate_union(type_name: &str, config: &UnionConfig) -> TsStream {
    // Generate per-variant error types
    let variant_errors = generate_variant_errors(config);
    let variant_tainted = generate_variant_tainted(config);
    let variant_union_errors = generate_variant_union_type("Errors", config);
    let variant_union_tainted = generate_variant_union_type("Tainted", config);
    let variant_field_controllers = generate_variant_field_controllers(config);
    let variant_union_literal = generate_variant_literal_union(config);

    // Get discriminant field name
    let discriminant_field = match &config.mode {
        UnionMode::Tagged { field } => field.as_str(),
        UnionMode::Untagged => "_variant", // synthetic field for untagged
    };

    ts_template! {
        {>> "Per-variant error types" <<}
        @{variant_errors}

        {>> "Per-variant tainted types" <<}
        @{variant_tainted}

        {>> "Union error type" <<}
        export type Errors = @{variant_union_errors};

        {>> "Union tainted type" <<}
        export type Tainted = @{variant_union_tainted};

        {>> "Field controller interface" <<}
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
            getError(): Option<Array<string>>;
            setError(value: Option<Array<string>>): void;
            getTainted(): Option<boolean>;
            setTainted(value: Option<boolean>): void;
            validate(): Array<string>;
        }

        {>> "Per-variant field controller types" <<}
        @{variant_field_controllers}

        {>> "Union Gigaform interface with variant switching" <<}
        export interface Gigaform {
            readonly currentVariant: @{variant_union_literal};
            readonly data: @{type_name};
            readonly errors: Errors;
            readonly tainted: Tainted;
            readonly variants: VariantFields;
            switchVariant(variant: @{variant_union_literal}): void;
            validate(): Result<@{type_name}, Array<{ field: string; message: string }>>;
            reset(overrides?: Partial<@{type_name}>): void;
        }

        {>> "Variant fields container" <<}
        export interface VariantFields {
            @{generate_variant_fields_interface(config, discriminant_field)}
        }
    }
}

/// Generates per-variant error type definitions.
fn generate_variant_errors(config: &UnionConfig) -> String {
    config.variants.iter().map(|variant| {
        let variant_name = to_pascal_case(&variant.discriminant_value);
        let errors_fields = generate_errors_fields(&variant.fields);
        format!(
            "export type {variant_name}Errors = {{ _errors: Option<Array<string>>; {errors_fields} }};"
        )
    }).collect::<Vec<_>>().join("\n        ")
}

/// Generates per-variant tainted type definitions.
fn generate_variant_tainted(config: &UnionConfig) -> String {
    config.variants.iter().map(|variant| {
        let variant_name = to_pascal_case(&variant.discriminant_value);
        let tainted_fields = generate_tainted_fields(&variant.fields);
        format!(
            "export type {variant_name}Tainted = {{ {tainted_fields} }};"
        )
    }).collect::<Vec<_>>().join("\n        ")
}

/// Generates the union type for Errors or Tainted.
fn generate_variant_union_type(type_suffix: &str, config: &UnionConfig) -> String {
    let discriminant_field = match &config.mode {
        UnionMode::Tagged { field } => field.as_str(),
        UnionMode::Untagged => "_variant",
    };

    config.variants.iter().map(|variant| {
        let variant_name = to_pascal_case(&variant.discriminant_value);
        let value = &variant.discriminant_value;
        format!("({{ {discriminant_field}: \"{value}\" }} & {variant_name}{type_suffix})")
    }).collect::<Vec<_>>().join(" | ")
}

/// Generates per-variant field controller interfaces.
fn generate_variant_field_controllers(config: &UnionConfig) -> String {
    config.variants.iter().map(|variant| {
        let variant_name = to_pascal_case(&variant.discriminant_value);
        let field_types = generate_field_controller_types(&variant.fields);
        format!(
            "export interface {variant_name}FieldControllers {{ {field_types} }}"
        )
    }).collect::<Vec<_>>().join("\n        ")
}

/// Generates the literal union of variant discriminant values.
fn generate_variant_literal_union(config: &UnionConfig) -> String {
    config.variants.iter()
        .map(|v| format!("\"{}\"", v.discriminant_value))
        .collect::<Vec<_>>()
        .join(" | ")
}

/// Generates the VariantFields interface content.
fn generate_variant_fields_interface(config: &UnionConfig, _discriminant_field: &str) -> String {
    config.variants.iter().map(|variant| {
        let value = &variant.discriminant_value;
        let variant_name = to_pascal_case(&variant.discriminant_value);
        // Quote the property key if it contains special characters
        let prop_key = if needs_quoting(value) {
            format!("\"{}\"", value)
        } else {
            value.clone()
        };
        format!("readonly {prop_key}: {{ readonly fields: {variant_name}FieldControllers }};")
    }).collect::<Vec<_>>().join("\n            ")
}

/// Returns true if a string needs to be quoted when used as an object property key.
fn needs_quoting(s: &str) -> bool {
    // Needs quoting if it contains special chars or starts with a digit
    s.chars().any(|c| !c.is_alphanumeric() && c != '_') ||
    s.chars().next().map(|c| c.is_numeric()).unwrap_or(true)
}

/// Converts a string to a valid PascalCase TypeScript identifier.
/// Handles special characters like `|`, `(`, `)`, `&`, etc.
/// Uses "Or" for `|` and "And" for `&` to make identifiers more readable.
fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '|' {
            // Use "Or" for union types
            result.push_str("Or");
            capitalize_next = true;
        } else if c == '&' {
            // Use "And" for intersection types
            result.push_str("And");
            capitalize_next = true;
        } else if c == '_' || c == '-' || c == ' ' || c == '(' || c == ')' || c == '<' || c == '>' || c == ',' {
            // Skip these characters but capitalize the next letter
            capitalize_next = true;
        } else if c.is_alphanumeric() {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        // Skip any other non-alphanumeric characters
    }

    // Ensure the result is a valid identifier (starts with letter or underscore)
    if result.is_empty() || result.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
        format!("Variant{}", result)
    } else {
        result
    }
}

// =============================================================================
// Enum Type Generation
// =============================================================================

use crate::gigaform::parser::EnumFormConfig;

/// Generates types for an enum step form (wizard-style navigation).
pub fn generate_enum(enum_name: &str, config: &EnumFormConfig) -> TsStream {
    let variant_entries = generate_variant_entries(config);

    ts_template! {
        {>> "Variant metadata for step navigation" <<}
        export const variants = [
            @{variant_entries}
        ] as const;

        {>> "Type for a single variant entry" <<}
        export type VariantEntry = typeof variants[number];

        {>> "Enum step form interface with navigation" <<}
        export interface Gigaform {
            readonly currentStep: @{enum_name};
            readonly steps: typeof variants;
            setStep(step: @{enum_name}): void;
            nextStep(): boolean;
            prevStep(): boolean;
        }
    }
}

/// Generates the variant entries array content.
fn generate_variant_entries(config: &EnumFormConfig) -> String {
    config
        .variants
        .iter()
        .map(|variant| {
            let value_expr = &variant.value_expr;
            let label = &variant.label;
            if let Some(desc) = &variant.description {
                format!(
                    "{{ value: {value_expr}, label: \"{label}\", description: \"{desc}\" }}"
                )
            } else {
                format!("{{ value: {value_expr}, label: \"{label}\" }}")
            }
        })
        .collect::<Vec<_>>()
        .join(",\n            ")
}
