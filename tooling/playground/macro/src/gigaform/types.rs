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

    ts_template! {
        /** Nested error structure matching the data shape */
        export type @{errors_name} = {
            _errors: __gigaform_reexport_Option<Array<string>>;
            {#for field in fields}
                @{&field.name}: __gigaform_reexport_Option<Array<string>>;
            {/for}
        };

        /** Nested boolean structure for tracking touched/dirty fields */
        export type @{tainted_name} = {
            {#for field in fields}
                @{&field.name}: __gigaform_reexport_Option<boolean>;
            {/for}
        };

        /** Type-safe field controllers for this form */
        export interface @{field_controllers_name} {
            {#for field in fields}
                {#if field.is_array}
                    {$let element_type = field.array_element_type.as_deref().unwrap_or("unknown")}
                    readonly @{&field.name}: ArrayFieldController<@{element_type}>;
                {:else}
                    readonly @{&field.name}: FieldController<@{&field.ts_type}>;
                {/if}
            {/for}
        }

        /** Gigaform instance containing reactive state and field controllers */
        export interface @{gigaform_name} {
            readonly data: @{type_name};
            readonly errors: @{errors_name};
            readonly tainted: @{tainted_name};
            readonly fields: @{field_controllers_name};
            validate(): Exit<@{type_name}, Array<{ field: string; message: string }>>;
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

    let generic_decl = generics.decl();
    let generic_args = generics.args();

    ts_template! {
        /** Nested error structure matching the data shape */
        export interface @{type_name}Errors@{generic_decl}{
            _errors: __gigaform_reexport_Option<Array<string>>;
            {#for field in fields}
                @{&field.name}: __gigaform_reexport_Option<Array<string>>;
            {/for}
        };

        /** Nested boolean structure for tracking touched/dirty fields */
        export interface @{type_name}Tainted@{generic_decl}{
            {#for field in fields}
                @{&field.name}: __gigaform_reexport_Option<boolean>;
            {/for}
        };

        /** Type-safe field controllers for this form */
        export interface @{type_name}FieldControllers@{generic_decl} {
            {#for field in fields}
                {#if field.is_array}
                    {$let element_type = field.array_element_type.as_deref().unwrap_or("unknown")}
                    readonly @{&field.name}: ArrayFieldController<@{element_type}>;
                {:else}
                    readonly @{&field.name}: FieldController<@{&field.ts_type}>;
                {/if}
            {/for}
        }

        /** Gigaform instance containing reactive state and field controllers */
        export interface @{type_name}Gigaform@{generic_decl} {
            readonly data: @{type_name}@{generic_args};
            readonly errors: @{type_name}Errors@{generic_args};
            readonly tainted: @{type_name}Tainted@{generic_args};
            readonly fields: @{type_name}FieldControllers@{generic_args};
            validate(): Exit<@{type_name}@{generic_args}, Array<{ field: string; message: string }>>;
            reset(overrides?: Partial<@{type_name}@{generic_args}>): void;
        }
    }
}

// =============================================================================
// Union Type Generation
// =============================================================================

/// Generates types for a discriminated union form with generic support.
pub fn generate_union_with_generics(
    type_name: &str,
    config: &UnionConfig,
    generics: &GenericInfo,
) -> TsStream {
    if generics.is_empty() {
        return generate_union(type_name, config);
    }

    let generic_decl = generics.decl();
    let generic_args = generics.args();

    let errors_name = prefixed(type_name, "Errors");
    let tainted_name = prefixed(type_name, "Tainted");
    let gigaform_name = prefixed(type_name, "Gigaform");
    let variant_fields_name = prefixed(type_name, "VariantFields");

    let variant_errors = generate_variant_errors(type_name, config, &generic_decl);
    let variant_tainted = generate_variant_tainted(type_name, config, &generic_decl);
    let variant_union_errors =
        generate_variant_union_type("Errors", type_name, config, &generic_args);
    let variant_union_tainted =
        generate_variant_union_type("Tainted", type_name, config, &generic_args);
    let variant_field_controllers =
        generate_variant_field_controllers(type_name, config, &generic_decl);
    let variant_union_literal = generate_variant_literal_union(config);

    ts_template! {
        /** Per-variant error types */
        {$typescript variant_errors}

        /** Per-variant tainted types */
        {$typescript variant_tainted}

        /** Union error type */
        export type @{errors_name}@{generic_decl} = @{variant_union_errors};

        /** Union tainted type */
        export type @{tainted_name}@{generic_decl} = @{variant_union_tainted};

        /** Per-variant field controller types */
        {$typescript variant_field_controllers}

        /** Union Gigaform interface with variant switching */
        export interface @{gigaform_name}@{generic_decl} {
            readonly currentVariant: @{variant_union_literal};
            readonly data: @{type_name}@{generic_args};
            readonly errors: @{errors_name}@{generic_args};
            readonly tainted: @{tainted_name}@{generic_args};
            readonly variants: @{variant_fields_name}@{generic_args};
            switchVariant(variant: @{variant_union_literal}): void;
            validate(): Exit<@{type_name}@{generic_args}, Array<{ field: string; message: string }>>;
            reset(overrides?: Partial<@{type_name}@{generic_args}>): void;
        }

        /** Variant fields container */
        export interface @{variant_fields_name}@{generic_decl} {
            {#for variant in &config.variants}
                {$let value = &variant.discriminant_value}
                {$let variant_name = to_pascal_case(&variant.discriminant_value)}
                {$let prop_key = if needs_quoting(value) { format!("\"{}\"", value) } else { value.clone() }}
                readonly @{prop_key}: { readonly fields: @{type_name}@{variant_name}FieldControllers@{generic_args} };
            {/for}
        }
    }
}

/// Generates types for a discriminated union form.
pub fn generate_union(type_name: &str, config: &UnionConfig) -> TsStream {
    let errors_name = prefixed(type_name, "Errors");
    let tainted_name = prefixed(type_name, "Tainted");
    let gigaform_name = prefixed(type_name, "Gigaform");
    let variant_fields_name = prefixed(type_name, "VariantFields");

    // Generate per-variant error types
    let variant_errors = generate_variant_errors(type_name, config, "");
    let variant_tainted = generate_variant_tainted(type_name, config, "");
    let variant_union_errors = generate_variant_union_type("Errors", type_name, config, "");
    let variant_union_tainted = generate_variant_union_type("Tainted", type_name, config, "");
    let variant_field_controllers = generate_variant_field_controllers(type_name, config, "");
    let variant_union_literal = generate_variant_literal_union(config);

    ts_template! {
        /** Per-variant error types */
        {$typescript variant_errors}

        /** Per-variant tainted types */
        {$typescript variant_tainted}

        /** Union error type */
        export type @{errors_name} = @{variant_union_errors};

        /** Union tainted type */
        export type @{tainted_name} = @{variant_union_tainted};

        /** Per-variant field controller types */
        {$typescript variant_field_controllers}

        /** Union Gigaform interface with variant switching */
        export interface @{gigaform_name} {
            readonly currentVariant: @{variant_union_literal};
            readonly data: @{type_name};
            readonly errors: @{errors_name};
            readonly tainted: @{tainted_name};
            readonly variants: @{variant_fields_name};
            switchVariant(variant: @{variant_union_literal}): void;
            validate(): Exit<@{type_name}, Array<{ field: string; message: string }>>;
            reset(overrides?: Partial<@{type_name}>): void;
        }

        /** Variant fields container */
        export interface @{variant_fields_name} {
            {#for variant in &config.variants}
                {$let value = &variant.discriminant_value}
                {$let variant_name = to_pascal_case(&variant.discriminant_value)}
                {$let prop_key = if needs_quoting(value) { format!("\"{}\"", value) } else { value.clone() }}
                readonly @{prop_key}: { readonly fields: @{type_name}@{variant_name}FieldControllers };
            {/for}
        }
    }
}

/// Generates per-variant error type definitions.
fn generate_variant_errors(type_name: &str, config: &UnionConfig, generic_decl: &str) -> TsStream {
    ts_template! {
        {#for variant in &config.variants}
            {$let variant_name = to_pascal_case(&variant.discriminant_value)}
            export type @{type_name}@{variant_name}Errors@{generic_decl} = {
                _errors: __gigaform_reexport_Option<Array<string>>;
                {#for field in &variant.fields}
                    @{&field.name}: __gigaform_reexport_Option<Array<string>>;
                {/for}
            };
        {/for}
    }
}

/// Generates per-variant tainted type definitions.
fn generate_variant_tainted(type_name: &str, config: &UnionConfig, generic_decl: &str) -> TsStream {
    ts_template! {
        {#for variant in &config.variants}
            {$let variant_name = to_pascal_case(&variant.discriminant_value)}
            export type @{type_name}@{variant_name}Tainted@{generic_decl} = {
                {#for field in &variant.fields}
                    @{&field.name}: __gigaform_reexport_Option<boolean>;
                {/for}
            };
        {/for}
    }
}

/// Generates the union type for Errors or Tainted.
fn generate_variant_union_type(
    type_suffix: &str,
    type_name: &str,
    config: &UnionConfig,
    generic_args: &str,
) -> String {
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
                "({{ {discriminant_field}: \"{value}\" }} & {type_name}{variant_name}{type_suffix}{generic_args})"
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

/// Generates per-variant field controller interfaces.
fn generate_variant_field_controllers(
    type_name: &str,
    config: &UnionConfig,
    generic_decl: &str,
) -> TsStream {
    ts_template! {
        {#for variant in &config.variants}
            {$let variant_name = to_pascal_case(&variant.discriminant_value)}
            export interface @{type_name}@{variant_name}FieldControllers@{generic_decl} {
                {#for field in &variant.fields}
                    {#if field.is_array}
                        {$let element_type = field.array_element_type.as_deref().unwrap_or("unknown")}
                        readonly @{&field.name}: ArrayFieldController<@{element_type}>;
                    {:else}
                        readonly @{&field.name}: FieldController<@{&field.ts_type}>;
                    {/if}
                {/for}
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
    let variants_name = prefixed(enum_name, "Variants");
    let variant_entry_name = prefixed(enum_name, "VariantEntry");
    let gigaform_name = prefixed(enum_name, "Gigaform");

    ts_template! {
        /** Variant metadata for step navigation */
        const @{variants_name}A: Array<{ value: @{enum_name}; label: string; description?: string }> = [];
        {#for variant in &config.variants}
            {$let entry = generate_variant_entry(variant, enum_name)}
            {$typescript entry}
        {/for}
        export const @{variants_name} = @{variants_name}Arr as const;

        /** Type for a single variant entry */
        export type @{variant_entry_name} = typeof @{variants_name}[number];

        /** Enum step form interface with navigation */
        export interface @{gigaform_name} {
            readonly currentStep: @{enum_name};
            readonly steps: typeof @{variants_name};
            setStep(step: @{enum_name}): void;
            nextStep(): boolean;
            prevStep(): boolean;
        }
    }
}

/// Generates a single variant entry push statement.
fn generate_variant_entry(
    variant: &crate::gigaform::parser::EnumVariantFormConfig,
    enum_name: &str,
) -> TsStream {
    let value_expr = &variant.value_expr;
    let label = &variant.label;
    let variants_name = prefixed(enum_name, "Variants");

    if let Some(desc) = &variant.description {
        ts_template! {
            @{variants_name}Arr.push({ value: @{value_expr}, label: "@{label}", description: "@{desc}" });
        }
    } else {
        ts_template! {
            @{variants_name}Arr.push({ value: @{value_expr}, label: "@{label}" });
        }
    }
}
