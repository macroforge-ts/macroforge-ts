//! Generates type definitions for Gigaform as prefixed exports (e.g., UserErrors, UserTainted, UserGigaform, ...).

use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::TsStream;

use crate::gigaform::GenericInfo;
use crate::gigaform::parser::{ParsedField, UnionConfig, UnionMode};

fn prefixed(type_name: &str, suffix: &str) -> String {
    format!("{type_name}{suffix}")
}

/// Generates the Errors, Tainted, FieldControllers, and Gigaform type definitions.
pub fn generate(type_name: &str, fields: &[ParsedField]) -> TsStream {
    let errors_name = prefixed(type_name, "Errors");
    let tainted_name = prefixed(type_name, "Tainted");
    let field_controllers_name = prefixed(type_name, "FieldControllers");
    let gigaform_name = prefixed(type_name, "Gigaform");

    let errors_fields = generate_errors_fields(fields);
    let tainted_fields = generate_tainted_fields(fields);
    let field_controller_types = generate_field_controller_types(fields);

    ts_template! {
        {>> "Nested error structure matching the data shape" <<}
        export type @{errors_name} = {
            _errors: Option<Array<string>>;
            {$typescript errors_fields}
        };

        {>> "Nested boolean structure for tracking touched/dirty fields" <<}
        export type @{tainted_name} = {
            {$typescript tainted_fields}
        };

        {>> "Type-safe field controllers for this form" <<}
        export interface @{field_controllers_name} {
            {$typescript field_controller_types}
        }

        {>> "Gigaform instance containing reactive state and field controllers" <<}
        export interface @{gigaform_name} {
            readonly data: @{type_name};
            readonly errors: @{errors_name};
            readonly tainted: @{tainted_name};
            readonly fields: @{field_controllers_name};
            validate(): Result<@{type_name}, Array<{ field: string; message: string }>>;
            reset(overrides?: Partial<@{type_name}>): void;
        }
    }
}

/// Generates type definitions with generic support.
pub fn generate_with_generics(
    type_name: &str,
    fields: &[ParsedField],
    generics: &GenericInfo,
) -> TsStream {
    if generics.is_empty() {
        return generate(type_name, fields);
    }

    let errors_name = prefixed(type_name, "Errors");
    let tainted_name = prefixed(type_name, "Tainted");
    let field_controllers_name = prefixed(type_name, "FieldControllers");
    let gigaform_name = prefixed(type_name, "Gigaform");

    let errors_fields = generate_errors_fields(fields);
    let tainted_fields = generate_tainted_fields(fields);
    let field_controller_types = generate_field_controller_types(fields);
    let generic_decl = generics.decl();
    let generic_args = generics.args();

    ts_template! {
        {>> "Nested error structure matching the data shape" <<}
        export type @{errors_name}@{generic_decl} = {
            _errors: Option<Array<string>>;
            {$typescript errors_fields}
        };

        {>> "Nested boolean structure for tracking touched/dirty fields" <<}
        export type @{tainted_name}@{generic_decl} = {
            {$typescript tainted_fields}
        };

        {>> "Type-safe field controllers for this form" <<}
        export interface @{field_controllers_name}@{generic_decl} {
            {$typescript field_controller_types}
        }

        {>> "Gigaform instance containing reactive state and field controllers" <<}
        export interface @{gigaform_name}@{generic_decl} {
            readonly data: @{type_name}@{generic_args};
            readonly errors: @{errors_name}@{generic_args};
            readonly tainted: @{tainted_name}@{generic_args};
            readonly fields: @{field_controllers_name}@{generic_args};
            validate(): Result<@{type_name}@{generic_args}, Array<{ field: string; message: string }>>;
            reset(overrides?: Partial<@{type_name}@{generic_args}>): void;
        }
    }
}

/// Generates the FieldControllers type entries.
fn generate_field_controller_types(fields: &[ParsedField]) -> TsStream {
    ts_template! {
        {#for field in fields}
            {#if field.is_array}
                {$let element_type = field.array_element_type.as_deref().unwrap_or("unknown")}
                readonly @{&field.name}: ArrayFieldController<@{element_type}>;
            {:else}
                readonly @{&field.name}: FieldController<@{&field.ts_type}>;
            {/if}
        {/for}
    }
}

/// Generates the Errors type fields.
/// All fields use Option<Array<string>> for consistency with FieldController interface.
fn generate_errors_fields(fields: &[ParsedField]) -> TsStream {
    ts_template! {
        {#for field in fields}
            @{&field.name}: Option<Array<string>>;
        {/for}
    }
}

/// Generates the Tainted type fields.
/// All fields use Option<boolean> for consistency with FieldController interface.
fn generate_tainted_fields(fields: &[ParsedField]) -> TsStream {
    ts_template! {
        {#for field in fields}
            @{&field.name}: Option<boolean>;
        {/for}
    }
}

// =============================================================================
// Union Type Generation
// =============================================================================

/// Generates types for a discriminated union form with generic support.
/// Note: Unions typically don't have type parameters, so this delegates to generate_union.
pub fn generate_union_with_generics(
    type_name: &str,
    config: &UnionConfig,
    _generics: &GenericInfo,
) -> TsStream {
    // Unions are usually concrete types, generics are passed through but not commonly used
    generate_union(type_name, config)
}

/// Generates types for a discriminated union form.
pub fn generate_union(type_name: &str, config: &UnionConfig) -> TsStream {
    let errors_name = prefixed(type_name, "Errors");
    let tainted_name = prefixed(type_name, "Tainted");
    let gigaform_name = prefixed(type_name, "Gigaform");
    let variant_fields_name = prefixed(type_name, "VariantFields");

    // Generate per-variant error types
    let variant_errors = generate_variant_errors(type_name, config);
    let variant_tainted = generate_variant_tainted(type_name, config);
    let variant_union_errors = generate_variant_union_type("Errors", type_name, config);
    let variant_union_tainted = generate_variant_union_type("Tainted", type_name, config);
    let variant_field_controllers = generate_variant_field_controllers(type_name, config);
    let variant_union_literal = generate_variant_literal_union(config);

    // Get discriminant field name
    let discriminant_field = match &config.mode {
        UnionMode::Tagged { field } => field.as_str(),
        UnionMode::Untagged => "_variant", // synthetic field for untagged
    };

    let variant_fields_interface = generate_variant_fields_interface(type_name, config, discriminant_field);

    ts_template! {
        {>> "Per-variant error types" <<}
        {$typescript variant_errors}

        {>> "Per-variant tainted types" <<}
        {$typescript variant_tainted}

        {>> "Union error type" <<}
        export type @{errors_name} = @{variant_union_errors};

        {>> "Union tainted type" <<}
        export type @{tainted_name} = @{variant_union_tainted};

        {>> "Per-variant field controller types" <<}
        {$typescript variant_field_controllers}

        {>> "Union Gigaform interface with variant switching" <<}
        export interface @{gigaform_name} {
            readonly currentVariant: @{variant_union_literal};
            readonly data: @{type_name};
            readonly errors: @{errors_name};
            readonly tainted: @{tainted_name};
            readonly variants: @{variant_fields_name};
            switchVariant(variant: @{variant_union_literal}): void;
            validate(): Result<@{type_name}, Array<{ field: string; message: string }>>;
            reset(overrides?: Partial<@{type_name}>): void;
        }

        {>> "Variant fields container" <<}
        export interface @{variant_fields_name} {
            {$typescript variant_fields_interface}
        }
    }
}

/// Generates per-variant error type definitions.
fn generate_variant_errors(type_name: &str, config: &UnionConfig) -> TsStream {
    ts_template! {
        {#for variant in &config.variants}
            {$let variant_name = to_pascal_case(&variant.discriminant_value)}
            {$let errors_fields = generate_errors_fields(&variant.fields)}
            export type {|@{type_name}@{variant_name}Errors|} = {
                _errors: Option<Array<string>>;
                {$typescript errors_fields}
            };
        {/for}
    }
}

/// Generates per-variant tainted type definitions.
fn generate_variant_tainted(type_name: &str, config: &UnionConfig) -> TsStream {
    ts_template! {
        {#for variant in &config.variants}
            {$let variant_name = to_pascal_case(&variant.discriminant_value)}
            {$let tainted_fields = generate_tainted_fields(&variant.fields)}
            export type {|@{type_name}@{variant_name}Tainted|} = {
                {$typescript tainted_fields}
            };
        {/for}
    }
}

/// Generates the union type for Errors or Tainted.
fn generate_variant_union_type(type_suffix: &str, type_name: &str, config: &UnionConfig) -> String {
    let discriminant_field = match &config.mode {
        UnionMode::Tagged { field } => field.as_str(),
        UnionMode::Untagged => "_variant",
    };

    config
        .variants
        .iter()
        .map(|variant| {
            let variant_name = to_pascal_case(&variant.discriminant_value);
            let value = &variant.discriminant_value;
            format!(
                "({{ {discriminant_field}: \"{value}\" }} & {type_name}{variant_name}{type_suffix})"
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

/// Generates per-variant field controller interfaces.
fn generate_variant_field_controllers(type_name: &str, config: &UnionConfig) -> TsStream {
    ts_template! {
        {#for variant in &config.variants}
            {$let variant_name = to_pascal_case(&variant.discriminant_value)}
            {$let field_types = generate_field_controller_types(&variant.fields)}
            export interface {|@{type_name}@{variant_name}FieldControllers|} {
                {$typescript field_types}
            }
        {/for}
    }
}

/// Generates the literal union of variant discriminant values.
fn generate_variant_literal_union(config: &UnionConfig) -> String {
    config
        .variants
        .iter()
        .map(|v| format!("\"{}\"", v.discriminant_value))
        .collect::<Vec<_>>()
        .join(" | ")
}

/// Generates the VariantFields interface content.
fn generate_variant_fields_interface(
    type_name: &str,
    config: &UnionConfig,
    _discriminant_field: &str,
) -> TsStream {
    ts_template! {
        {#for variant in &config.variants}
            {$let value = &variant.discriminant_value}
            {$let variant_name = to_pascal_case(&variant.discriminant_value)}
            {$let prop_key = if needs_quoting(value) { format!("\"{}\"", value) } else { value.clone() }}
            readonly @{prop_key}: { readonly fields: {|@{type_name}@{variant_name}FieldControllers|} };
        {/for}
    }
}

/// Returns true if a string needs to be quoted when used as an object property key.
fn needs_quoting(s: &str) -> bool {
    // Needs quoting if it contains special chars or starts with a digit
    s.chars().any(|c| !c.is_alphanumeric() && c != '_')
        || s.chars().next().map(|c| c.is_numeric()).unwrap_or(true)
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
        } else if c == '_'
            || c == '-'
            || c == ' '
            || c == '('
            || c == ')'
            || c == '<'
            || c == '>'
            || c == ','
        {
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
    if result.is_empty()
        || result
            .chars()
            .next()
            .map(|c| c.is_numeric())
            .unwrap_or(false)
    {
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
    let variants_name = prefixed(enum_name, "Variants");
    let variant_entry_name = prefixed(enum_name, "VariantEntry");
    let gigaform_name = prefixed(enum_name, "Gigaform");

    ts_template! {
        {>> "Variant metadata for step navigation" <<}
        export const @{variants_name} = [
            {$typescript variant_entries}
        ] as const;

        {>> "Type for a single variant entry" <<}
        export type @{variant_entry_name} = typeof @{variants_name}[number];

        {>> "Enum step form interface with navigation" <<}
        export interface @{gigaform_name} {
            readonly currentStep: @{enum_name};
            readonly steps: typeof @{variants_name};
            setStep(step: @{enum_name}): void;
            nextStep(): boolean;
            prevStep(): boolean;
        }
    }
}

/// Generates the variant entries array content.
fn generate_variant_entries(config: &EnumFormConfig) -> TsStream {
    ts_template! {
        {#for variant in &config.variants}
            {#if let Some(desc) = &variant.description}
                { value: @{&variant.value_expr}, label: "@{&variant.label}", description: "@{desc}" },
            {:else}
                { value: @{&variant.value_expr}, label: "@{&variant.label}" },
            {/if}
        {/for}
    }
}
