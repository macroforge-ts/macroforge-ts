// All dependencies are re-exported from macroforge_ts - no need for separate crate imports!
use macroforge_ts::macros::{body, ts_macro_derive};
use macroforge_ts::ts_syn::{
    Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input,
};

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[ts_macro_derive(
    JSON,
    description = "Generates a toJSON() implementation that returns a plain object with all fields"
)]
pub fn derive_json_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            // Use Rust-style templating for clean code generation!
            // body! wrapper ensures this gets inserted as class members
            Ok(body! {
                toJSON(): Record<string, unknown> {

                    const result: Record<string, unknown> = {};

                    {#for field in class.field_names()}
                        result.@{field} = this.@{field};
                    {/for}

                    return result;
                }
            })
        }
        Data::Enum(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(JSON) */ can only target classes",
        )),
        Data::Interface(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(JSON) */ can only target classes, not interfaces",
        )),
        Data::TypeAlias(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(JSON) */ can only target classes, not type aliases",
        )),
    }
}

/// A test macro that generates self-contained code for testing macroforge edge cases.
///
/// Tests:
/// - Generic type parameters
/// - Optional fields (T | null)
/// - Array types
/// - Conditional logic in templates
/// - Field decorators with arguments
/// - Static methods
/// - Instance methods
#[ts_macro_derive(
    Inspect,
    description = "Generates inspection utilities for debugging and testing edge cases",
    attributes(inspect, label)
)]
pub fn inspect_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

            // Collect field info with decorator data
            let field_info: Vec<_> = class
                .fields()
                .iter()
                .map(|field| {
                    // Extract label from @label("...") decorator - args_src contains the raw string
                    let label = field
                        .decorators
                        .iter()
                        .find(|d| d.name == "label")
                        .map(|d| d.args_src.trim_matches('"').to_string())
                        .filter(|s| !s.is_empty())
                        .unwrap_or_else(|| capitalize(&field.name));

                    let should_inspect = field
                        .decorators
                        .iter()
                        .any(|d| d.name == "inspect");

                    let is_optional = field.ts_type.contains("null") || field.ts_type.contains("undefined");
                    let is_array = field.ts_type.contains("[]") || field.ts_type.contains("Array<");

                    (
                        field.name.clone(),
                        label,
                        should_inspect,
                        is_optional,
                        is_array,
                        field.ts_type.trim_end_matches(';').trim().to_string(),
                    )
                })
                .collect();

            let stream = body! {
                {>> "Returns field metadata for inspection" <<}
                static fieldMetadata(): Array<{ name: string; label: string; optional: boolean; array: boolean; type: string }> {
                    return [
                        {#for (name, label, _inspect, optional, array, ts_type) in &field_info}
                        { name: "@{name}", label: "@{label}", optional: @{optional}, array: @{array}, type: "@{ts_type}" },
                        {/for}
                    ];
                }

                {>> "Returns only the inspectable fields" <<}
                getInspectableFields(): Record<string, unknown> {
                    const result: Record<string, unknown> = {};
                    {#for (name, _label, inspect, _optional, _array, _ts_type) in &field_info}
                        {#if *inspect}
                        result["@{name}"] = this.@{name};
                        {/if}
                    {/for}
                    return result;
                }

                {>> "Deep clones only array fields" <<}
                cloneArrayFields(): Partial<@{class_name}> {
                    const result: Partial<@{class_name}> = {};
                    {#for (name, _label, _inspect, _optional, array, _ts_type) in &field_info}
                        {#if *array}
                        result.@{name} = [...(this.@{name} ?? [])] as typeof this.@{name};
                        {/if}
                    {/for}
                    return result;
                }

                {>> "Returns non-null field count" <<}
                countPopulatedFields(): number {
                    let count = 0;
                    {#for (name, _label, _inspect, optional, _array, _ts_type) in &field_info}
                        {#if *optional}
                        if (this.@{name} != null) count++;
                        {:else}
                        count++;
                        {/if}
                    {/for}
                    return count;
                }
            };
            Ok(stream)
        }
        Data::Enum(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(Inspect) */ can only target classes",
        )),
        Data::Interface(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(Inspect) */ can only target classes, not interfaces",
        )),
        Data::TypeAlias(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(Inspect) */ can only target classes, not type aliases",
        )),
    }
}
