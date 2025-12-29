//! Tests for all template patterns used in macroforge_ts builtin macros.
//!
//! Each test contains the EXACT template from the corresponding builtin macro file.
//! Tests will FAIL if compilation takes more than 60 seconds.

use crate::compiler::compile_template;
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::from_secs(60);

fn test_template(name: &str, template: &str) {
    eprintln!("\n=== {} ===", name);
    eprintln!("Template length: {} chars", template.len());

    let start = Instant::now();
    let result = compile_template(template, "__stmts");
    let elapsed = start.elapsed();

    if elapsed > TIMEOUT {
        panic!(
            "{} TIMEOUT: compilation took {:?} (limit: {:?})",
            name, elapsed, TIMEOUT
        );
    }

    match result {
        Ok(code) => {
            let code_str = code.to_string();
            eprintln!("Compile time: {:?}", elapsed);
            eprintln!("Generated code size: {} bytes", code_str.len());

            // Print first 3000 chars of generated code for debugging
            if code_str.len() > 3000 {
                eprintln!("Generated code (first 3000 chars):\n{}", &code_str[..3000]);
            } else {
                eprintln!("Generated code:\n{}", code_str);
            }
        }
        Err(e) => {
            panic!("{} failed to compile after {:?}: {:?}", name, elapsed, e);
        }
    }
}

// ==================== derive_clone.rs:111-119 ====================
#[test]
fn test_derive_clone_class() {
    test_template(
        "derive_clone_class",
        r#"export function @{fn_name_ident}(value: @{class_ident}): @{class_ident} {
                    const cloned = Object.create(Object.getPrototypeOf(value));
                    {#for field in class.field_names().map(|f| ident!(f))}
                        cloned.@{field.clone()} = value.@{field};
                    {/for}
                    return cloned;
                }"#,
    );
}

// ==================== derive_clone.rs:130-133 ====================
#[test]
fn test_derive_clone_compose() {
    test_template(
        "derive_clone_compose",
        r#"{$typescript standalone}
                {$typescript class_body}"#,
    );
}

// ==================== derive_clone.rs:139-143 ====================
#[test]
fn test_derive_clone_enum() {
    test_template(
        "derive_clone_enum",
        r#"export function @{fn_name_ident}(value: @{ident!(enum_name)}): @{ident!(enum_name)} {
                    return value;
                }"#,
    );
}

// ==================== derive_clone.rs:150-158 ====================
#[test]
fn test_derive_clone_interface() {
    test_template(
        "derive_clone_interface",
        r#"export function @{fn_name_ident}(value: @{interface_ident}): @{interface_ident} {
                    const result = {} as any;
                    {#for field in interface.field_names().map(|f| ident!(f))}
                        result.@{field.clone()} = value.@{field};
                    {/for}
                    return result as @{interface_ident};
                }"#,
    );
}

// ==================== derive_clone.rs:166-174 ====================
#[test]
fn test_derive_clone_type_alias_object() {
    test_template(
        "derive_clone_type_alias_object",
        r#"export function @{fn_name_ident}(value: @{ident!(type_name)}): @{ident!(type_name)} {
                        const result = {} as any;
                        {#for field in type_alias.as_object().unwrap().iter().map(|f| ident!(f.name.as_str()))}
                            result.@{field.clone()} = value.@{field};
                        {/for}
                        return result as @{ident!(type_name)};
                    }"#,
    );
}

// ==================== derive_clone.rs:177-184 ====================
#[test]
fn test_derive_clone_type_alias_union() {
    test_template(
        "derive_clone_type_alias_union",
        r#"export function @{fn_name_ident}(value: @{ident!(type_name)}): @{ident!(type_name)} {
                        if (typeof value === "object" && value !== null) {
                            return { ...value } as @{ident!(type_name)};
                        }
                        return value;
                    }"#,
    );
}

// ==================== derive_debug.rs:202-214 ====================
#[test]
fn test_derive_debug_class() {
    test_template(
        "derive_debug_class",
        r#"export function @{fn_name_ident}(value: @{class_ident}): string {
                    {#if !_debug_fields.is_empty()}
                        const parts: string[] = [];
                        {#for (label, name) in _debug_fields}
                            parts.push("@{label}: " + value.@{name});
                        {/for}
                        return "@{class_name} { " + parts.join(", ") + " }";
                    {:else}
                        return "@{class_name} {}";
                    {/if}
                }"#,
    );
}

// ==================== derive_debug.rs:241-253 ====================
#[test]
fn test_derive_debug_enum() {
    test_template(
        "derive_debug_enum",
        r#"export function @{fn_name_ident}(value: @{enum_ident}): string {
                    {#if !_variants.is_empty()}
                        const key = @{enum_ident.clone().into()}[value as unknown as keyof typeof @{enum_ident}];
                        if (key !== undefined) {
                            return "@{enum_name}." + key;
                        }
                        return "@{enum_name}(" + String(value) + ")";
                    {:else}
                        return "@{enum_name}(" + String(value) + ")";
                    {/if}
                }"#,
    );
}

// ==================== derive_default.rs:232-238 ====================
#[test]
fn test_derive_default_class() {
    test_template(
        "derive_default_class",
        r#"{$typescript class_body}

                export function @{fn_name_ident}(): @{class_ident} {
                    return @{class_expr}.defaultValue();
                }"#,
    );
}

// ==================== derive_default.rs:347-355 ====================
#[test]
fn test_derive_default_interface() {
    test_template(
        "derive_default_interface",
        r#"export function @{fn_name_ident}(): @{interface_ident} {
                        return {
                            {#for (name_ident, value_expr) in object_fields}
                                @{name_ident}: @{value_expr},
                            {/for}
                        } as @{interface_ident};
                    }"#,
    );
}

// ==================== derive_default.rs:450-458 ====================
#[test]
fn test_derive_default_type_alias_generic() {
    test_template(
        "derive_default_type_alias_generic",
        r#"export function @{fn_name_ident}@{generic_decl_ident}(): @{full_type_ident} {
                            return {
                                {#for (name_ident, value_expr) in object_fields}
                                    @{name_ident}: @{value_expr},
                                {/for}
                            } as @{full_type_ident};
                        }"#,
    );
}

// ==================== derive_hash.rs:298-308 ====================
#[test]
fn test_derive_hash_class() {
    test_template(
        "derive_hash_class",
        r#"export function @{fn_name_ident}(value: @{class_ident}): number {
                    let hash = 17;
                    {#if _has_fields}
                        {#for hash_expr in _hash_exprs}
                            hash = (hash * 31 + @{hash_expr}) | 0;
                        {/for}
                    {/if}
                    return hash;
                }"#,
    );
}

// ==================== derive_hash.rs:333-341 ====================
#[test]
fn test_derive_hash_enum_string() {
    test_template(
        "derive_hash_enum_string",
        r#"export function @{fn_name_ident}(value: @{ident!(enum_name)}): number {
                        let hash = 0;
                        for (let i = 0; i < value.length; i++) {
                            hash = (hash * 31 + value.charCodeAt(i)) | 0;
                        }
                        return hash;
                    }"#,
    );
}

// ==================== derive_ord.rs:262-271 ====================
#[test]
fn test_derive_ord_class() {
    test_template(
        "derive_ord_class",
        r#"export function @{fn_name_ident}(a: @{class_ident}, b: @{class_ident}): number {
                        if (a === b) return 0;
                        {#for (cmp_ident, cmp_expr) in &compare_steps}
                            const @{cmp_ident.clone()} = @{cmp_expr.clone()};
                            if (@{cmp_ident.clone()} !== 0) return @{cmp_ident.clone()};
                        {/for}
                        return 0;
                    }"#,
    );
}

// ==================== derive_ord.rs:299-311 ====================
#[test]
fn test_derive_ord_enum() {
    test_template(
        "derive_ord_enum",
        r#"export function @{fn_name_ident}(a: @{ident!(enum_name)}, b: @{ident!(enum_name)}): number {
                    // For enums, compare by value (numeric enums) or string
                    if (typeof a === "number" && typeof b === "number") {
                        return a < b ? -1 : a > b ? 1 : 0;
                    }
                    if (typeof a === "string" && typeof b === "string") {
                        const cmp = a.localeCompare(b);
                        return cmp < 0 ? -1 : cmp > 0 ? 1 : 0;
                    }
                    return 0;
                }"#,
    );
}

// ==================== derive_partial_ord.rs:287-293 ====================
#[test]
fn test_derive_partial_ord_class() {
    test_template(
        "derive_partial_ord_class",
        r#"export function @{fn_name_ident}(a: @{class_ident}, b: @{class_ident}): @{return_type_ident} {
                        if (a === b) return 0;
                        {$typescript TsStream::from_string(compare_body)}
                        return 0;
                    }"#,
    );
}

// ==================== derive_partial_eq.rs - need to read ====================
#[test]
fn test_derive_partial_eq_class() {
    test_template(
        "derive_partial_eq_class",
        r#"export function @{fn_name_ident}(a: @{class_ident}, b: @{class_ident}): boolean {
                    if (a === b) return true;
                    {#for eq_expr in _eq_exprs}
                        if (!(@{eq_expr})) return false;
                    {/for}
                    return true;
                }"#,
    );
}

// ==================== serde/derive_serialize.rs:487-920 (MASSIVE) ====================
#[test]
fn test_serde_serialize_class() {
    test_template(
        "serde_serialize_class",
        r#"/** Serializes a value to a JSON string. @param value - The value to serialize @returns JSON string representation with cycle detection metadata */
                export function @{fn_serialize_ident}(value: @{class_ident}): string {
                    const ctx = @{serialize_context_expr}.create();
                    return JSON.stringify(@{fn_serialize_internal_expr_standalone}(value, ctx));
                }

                /** @internal Serializes with an existing context for nested/cyclic object graphs. @param value - The value to serialize @param ctx - The serialization context */
                export function @{fn_serialize_internal_ident_standalone}(value: @{class_ident}, ctx: @{serialize_context_ident}): Record<string, unknown> {
                    // Check if already serialized (cycle detection)
                    const existingId = ctx.getId(value);
                    if (existingId !== undefined) {
                        return { __ref: existingId };
                    }

                    // Register this object
                    const __id = ctx.register(value);

                    const result: Record<string, unknown> = {
                        __type: "@{class_name}",
                        __id,
                    };

                    {#if has_regular}
                        {#for field in regular_fields}
                            {#if let Some(fn_name) = &field.serialize_with}
                                // Custom serialization function (serializeWith) - wrapped as IIFE for arrow functions
                                {#if field.optional}
                                    if (value.@{field._field_ident} !== undefined) {
                                        result["@{field._json_key}"] = (@{fn_name})(value.@{field._field_ident});
                                    }
                                {:else}
                                    result["@{field._json_key}"] = (@{fn_name})(value.@{field._field_ident});
                                {/if}
                            {:else}
                            {#match &field._type_cat}
                                {:case TypeCategory::Primitive}
                                    {#if field.optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            result["@{field._json_key}"] = value.@{field._field_ident};
                                        }
                                    {:else}
                                        result["@{field._json_key}"] = value.@{field._field_ident};
                                    {/if}

                                {:case TypeCategory::Date}
                                    {#if field.optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            result["@{field._json_key}"] = value.@{field._field_ident}.toISOString();
                                        }
                                    {:else}
                                        result["@{field._json_key}"] = value.@{field._field_ident}.toISOString();
                                    {/if}

                                {:case TypeCategory::Array(_)}
                                    {#if field.optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result["@{field._json_key}"] = value.@{field._field_ident};
                                                {:case SerdeValueKind::Date}
                                                    result["@{field._json_key}"] = value.@{field._field_ident}.map((item: Date) => item.toISOString());
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.array_elem_serializable_type}
                                                        {$let serialize_with_context_elem: Expr = ident!(nested_serialize_fn_name(elem_type)).into()}
                                                        result["@{field._json_key}"] = value.@{field._field_ident}.map(
                                                            (item) => @{serialize_with_context_elem}(item, ctx)
                                                        );
                                                    {:else}
                                                        result["@{field._json_key}"] = value.@{field._field_ident};
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result["@{field._json_key}"] = value.@{field._field_ident};
                                            {:case SerdeValueKind::Date}
                                                result["@{field._json_key}"] = value.@{field._field_ident}.map((item: Date) => item.toISOString());
                                            {:case _}
                                                {#if let Some(elem_type) = &field.array_elem_serializable_type}
                                                    {$let serialize_with_context_elem: Expr = ident!(nested_serialize_fn_name(elem_type)).into()}
                                                    result["@{field._json_key}"] = value.@{field._field_ident}.map(
                                                        (item) => @{serialize_with_context_elem}(item, ctx)
                                                    );
                                                {:else}
                                                    result["@{field._json_key}"] = value.@{field._field_ident};
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Serializable(inner_type)}
                                    {$let serialize_with_context: Expr = ident!(nested_serialize_fn_name(inner_type)).into()}
                                    {#if field.optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            result["@{field._json_key}"] = @{serialize_with_context}(value.@{field._field_ident}, ctx);
                                        }
                                    {:else}
                                        result["@{field._json_key}"] = @{serialize_with_context}(value.@{field._field_ident}, ctx);
                                    {/if}

                                {:case TypeCategory::Nullable(_)}
                                    {#match field._nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                        {:case SerdeValueKind::PrimitiveLike}
                                            {#if field.optional}
                                                if (value.@{field._field_ident} !== undefined) {
                                                    result["@{field._json_key}"] = value.@{field._field_ident};
                                                }
                                            {:else}
                                                result["@{field._json_key}"] = value.@{field._field_ident};
                                            {/if}
                                        {:case SerdeValueKind::Date}
                                            {#if field.optional}
                                                if (value.@{field._field_ident} !== undefined) {
                                                    result["@{field._json_key}"] = value.@{field._field_ident} === null ? null : value.@{field._field_ident}.toISOString();
                                                }
                                            {:else}
                                                result["@{field._json_key}"] = value.@{field._field_ident} === null ? null : value.@{field._field_ident}.toISOString();
                                            {/if}
                                        {:case _}
                                            {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                {$let serialize_with_context_nullable: Expr = ident!(nested_serialize_fn_name(inner_type)).into()}
                                                {#if field.optional}
                                                    if (value.@{field._field_ident} !== undefined) {
                                                        result["@{field._json_key}"] = value.@{field._field_ident} === null ? null : @{serialize_with_context_nullable}(value.@{field._field_ident}, ctx);
                                                    }
                                                {:else}
                                                    result["@{field._json_key}"] = value.@{field._field_ident} === null ? null : @{serialize_with_context_nullable}(value.@{field._field_ident}, ctx);
                                                {/if}
                                            {:else}
                                                {#if field.optional}
                                                    if (value.@{field._field_ident} !== undefined) {
                                                        result["@{field._json_key}"] = value.@{field._field_ident};
                                                    }
                                                {:else}
                                                    result["@{field._json_key}"] = value.@{field._field_ident};
                                                {/if}
                                            {/if}
                                    {/match}

                                {:case _}
                                    {#if field.optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            result["@{field._json_key}"] = value.@{field._field_ident};
                                        }
                                    {:else}
                                        result["@{field._json_key}"] = value.@{field._field_ident};
                                    {/if}
                            {/match}
                            {/if}
                        {/for}
                    {/if}

                    return result;
                }"#,
    );
}

// ==================== serde/derive_deserialize.rs:1288-1303 ====================
#[test]
fn test_serde_deserialize_standalone() {
    test_template(
        "serde_deserialize_standalone",
        r#"/** Deserializes input to an instance. Automatically detects whether input is a JSON string or object. @param input - JSON string or object to deserialize @param opts - Optional deserialization options @returns Result containing the deserialized instance or validation errors */
                export function @{fn_deserialize_ident}(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
                    return @{&class_expr}.deserialize(input, opts);
                }

                /** Deserializes with an existing context for nested/cyclic object graphs. @param value - The raw value to deserialize @param ctx - The deserialization context */
                export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{class_ident} | @{pending_ref_ident} {
                    return @{&class_expr}.deserializeWithContext(value, ctx);
                }

                /** Type guard: checks if a value can be successfully deserialized. @param value - The value to check @returns True if the value can be deserialized to this type */
                export function @{fn_is_ident}(value: unknown): value is @{class_ident} {
                    return @{&class_expr}.is(value);
                }"#,
    );
}

// ==================== serde/derive_deserialize.rs:1324-1351 ====================
#[test]
fn test_serde_deserialize_enum() {
    test_template(
        "serde_deserialize_enum",
        r#"/** Deserializes input to an enum value. Automatically detects whether input is a JSON string or value. @param input - JSON string or value to deserialize @returns The enum value @throws Error if the value is not a valid enum member */
                export function @{fn_deserialize_ident}(input: unknown): @{&enum_ident} {
                    const data = typeof input === "string" ? JSON.parse(input) : input;
                    return @{fn_deserialize_internal_expr}(data);
                }

                /** Deserializes with an existing context (for consistency with other types). */
                export function @{fn_deserialize_internal_ident}(data: unknown): @{&enum_ident} {
                    for (const key of Object.keys(@{&enum_expr})) {
                        const enumValue = @{&enum_expr}[key as keyof typeof @{&enum_ident}];
                        if (enumValue === data) {
                            return data as @{&enum_ident};
                        }
                    }
                    throw new Error("Invalid @{enum_name} value: " + JSON.stringify(data));
                }

                export function @{fn_is_ident}(value: unknown): value is @{&enum_ident} {
                    for (const key of Object.keys(@{&enum_expr})) {
                        const enumValue = @{&enum_expr}[key as keyof typeof @{&enum_ident}];
                        if (enumValue === value) {
                            return true;
                        }
                    }
                    return false;
                }"#,
    );
}

// ==================== body! macro tests ====================
#[test]
fn test_body_default() {
    test_template(
        "body_default",
        r#"static defaultValue(): @{class_ident} {
                    const instance = new @{class_expr}();
                    {#for (name_ident, value_expr) in field_data}
                        instance.@{name_ident} = @{value_expr};
                    {/for}
                    return instance;
                }"#,
    );
}

#[test]
fn test_body_hash() {
    test_template(
        "body_hash",
        r#"hashCode(): number {
                let hash = 17;
                {#if _has_fields}
                    {#for hash_expr in _hash_exprs}
                        hash = (hash * 31 + @{hash_expr}) | 0;
                    {/for}
                {/if}
                return hash;
            }"#,
    );
}

// ==================== Match pattern binding test ====================
#[test]
fn test_match_with_pattern_binding() {
    test_template(
        "match_with_pattern_binding",
        r#"{#match type_cat}
            {:case TypeCategory::Primitive}
                instance.@{field_ident} = raw_value;
            {:case TypeCategory::Array(inner)}
                instance.@{field_ident} = raw_value as @{inner}[];
        {/match}"#,
    );
}

// ==================== Function name placeholder test ====================
#[test]
fn test_export_function_name() {
    test_template(
        "export_function_name",
        r#"export function @{fn_name_ident}(value: @{class_ident}): number {
                let hash = 17;
                return hash;
            }"#,
    );
}

// ==================== Function with doc comment test ====================
#[test]
fn test_export_function_with_doc() {
    test_template(
        "export_function_with_doc",
        r#"/** Serializes a value to a JSON string. */
export function @{fn_serialize_ident}(value: @{class_ident}): string {
    const ctx = @{serialize_context_expr}.create();
    return ctx.serialize();
}"#,
    );
}

// ==================== JSDoc with @param test ====================
#[test]
fn test_jsdoc_with_at_param() {
    test_template(
        "jsdoc_with_at_param",
        r#"/** Deserializes input. @param input - JSON string @returns The deserialized value */
export function @{fn_deserialize_ident}(input: unknown, opts?: @{opts_type}): @{return_type} {
    return null;
}"#,
    );
}

// ==================== Debug: Constructor with for loop ====================
#[test]
fn test_debug_constructor_for_loop() {
    // This mimics the pattern from derive_deserialize.rs that generates empty constructor
    test_template(
        "debug_constructor_for_loop",
        r#"class __MF_DUMMY__ {
    constructor(props: Record<string, unknown>) {
        {#for field in &all_fields}
            {#if field.optional}
                this.@{field._field_ident} = props.@{field._field_ident} as @{field.ts_type};
            {:else}
                this.@{field._field_ident} = props.@{field._field_ident};
            {/if}
        {/for}
    }
}"#,
    );
}

// ==================== For-only template with external variable ====================
// This tests the pattern where a template ONLY contains a for loop that uses
// an external variable. This pattern causes "unused variable" warnings in Rust
// because the variable capture might not be happening correctly.
#[test]
fn test_for_only_template_with_external_var() {
    // Matches pattern from gigaform/types.rs generate_variant_errors
    test_template(
        "for_only_template_with_external_var",
        r#"{#for v in &config.variants}export type @{type_name}@{v.name}Errors = {};{/for}"#,
    );
}

#[test]
fn test_for_only_template_concatenated_ident() {
    // Tests that variables used in concatenated identifiers inside for loops are captured
    test_template(
        "for_only_template_concatenated_ident",
        r#"{#for item in items}export interface @{prefix}@{item.suffix}Controller {}{/for}"#,
    );
}

#[test]
fn test_for_only_with_let_binding() {
    // Tests for loop with $let that uses external variable
    test_template(
        "for_only_with_let_binding",
        r#"{#for v in &config.variants}{$let variant_name = to_pascal_case(&v.value)}export type @{type_name}@{variant_name}Tainted = {};{/for}"#,
    );
}

// ==================== Optional parameter preservation ====================
#[test]
fn test_optional_parameter_preserved() {
    // Tests that optional parameter markers (?) are preserved in codegen
    test_template(
        "optional_parameter",
        r#"interface FormStore {
    reset(overrides?: Partial<@{type_name}>): void;
}"#,
    );
}
