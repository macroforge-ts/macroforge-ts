//! /** @derive(Serialize) */ macro implementation
//!
//! Generates JSON serialization methods with cycle detection:
//! - For classes: `toStringifiedJSON()`, `toJSON()`, and `__serialize(ctx)`
//! - For interfaces: `namespace InterfaceName { toStringifiedJSON, __serialize }`
//!
//! Uses `__id` and `__ref` markers for object identity tracking.

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, parse_ts_macro_input};
use crate::ts_syn::abi::DiagnosticCollector;

use super::{SerdeContainerOptions, SerdeFieldOptions, TypeCategory};

/// Field info for serialization
#[derive(Clone)]
struct SerializeField {
    json_key: String,
    field_name: String,
    type_cat: TypeCategory,
    optional: bool,
    flatten: bool,
}

#[ts_macro_derive(
    Serialize,
    description = "Generates serialization methods with cycle detection (toStringifiedJSON, __serialize)",
    attributes((serde, "Configure serialization for this field. Options: skip, rename, flatten"))
)]
pub fn derive_serialize_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let container_opts = SerdeContainerOptions::from_decorators(&class.inner.decorators);

            // Collect serializable fields with diagnostic collection
            let mut all_diagnostics = DiagnosticCollector::new();
            let fields: Vec<SerializeField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let parse_result = SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                    all_diagnostics.extend(parse_result.diagnostics);
                    let opts = parse_result.options;

                    if !opts.should_serialize() {
                        return None;
                    }

                    let json_key = opts
                        .rename
                        .clone()
                        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                    let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                    Some(SerializeField {
                        json_key,
                        field_name: field.name.clone(),
                        type_cat,
                        optional: field.optional,
                        flatten: opts.flatten,
                    })
                })
                .collect();

            // Check for errors in field parsing before continuing
            if all_diagnostics.has_errors() {
                return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
            }

            // Separate regular fields from flattened fields
            let regular_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
            let flatten_fields: Vec<_> = fields.iter().filter(|f| f.flatten).cloned().collect();

            let has_regular = !regular_fields.is_empty();
            let has_flatten = !flatten_fields.is_empty();

            let mut result = body! {
                toStringifiedJSON(): string {
                    const ctx = SerializeContext.create();
                    return JSON.stringify(this.__serialize(ctx));
                }

                toObject(): Record<string, unknown> {
                    const ctx = SerializeContext.create();
                    return this.__serialize(ctx);
                }

                __serialize(ctx: SerializeContext): Record<string, unknown> {
                    // Check if already serialized (cycle detection)
                    const existingId = ctx.getId(this);
                    if (existingId !== undefined) {
                        return { __ref: existingId };
                    }

                    // Register this object
                    const __id = ctx.register(this);

                    const result: Record<string, unknown> = {
                        __type: "@{class_name}",
                        __id,
                    };

                    {#if has_regular}
                        {#for field in regular_fields}
                            {#match &field.type_cat}
                                {:case TypeCategory::Primitive}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = this.@{field.field_name};
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = this.@{field.field_name};
                                    {/if}

                                {:case TypeCategory::Date}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = this.@{field.field_name}.toISOString();
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = this.@{field.field_name}.toISOString();
                                    {/if}

                                {:case TypeCategory::Array(_)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = this.@{field.field_name}.map(
                                                (item: any) => typeof item?.__serialize === "function"
                                                    ? item.__serialize(ctx)
                                                    : item
                                            );
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = this.@{field.field_name}.map(
                                            (item: any) => typeof item?.__serialize === "function"
                                                ? item.__serialize(ctx)
                                                : item
                                        );
                                    {/if}

                                {:case TypeCategory::Map(_, _)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = Object.fromEntries(
                                                Array.from(this.@{field.field_name}.entries()).map(
                                                    ([k, v]) => [k, typeof (v as any)?.__serialize === "function"
                                                        ? (v as any).__serialize(ctx)
                                                        : v]
                                                )
                                            );
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = Object.fromEntries(
                                            Array.from(this.@{field.field_name}.entries()).map(
                                                ([k, v]) => [k, typeof (v as any)?.__serialize === "function"
                                                    ? (v as any).__serialize(ctx)
                                                    : v]
                                            )
                                        );
                                    {/if}

                                {:case TypeCategory::Set(_)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = Array.from(this.@{field.field_name}).map(
                                                (item: any) => typeof item?.__serialize === "function"
                                                    ? item.__serialize(ctx)
                                                    : item
                                            );
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = Array.from(this.@{field.field_name}).map(
                                            (item: any) => typeof item?.__serialize === "function"
                                                ? item.__serialize(ctx)
                                                : item
                                        );
                                    {/if}

                                {:case TypeCategory::Optional(_)}
                                    if (this.@{field.field_name} !== undefined) {
                                        result["@{field.json_key}"] = typeof (this.@{field.field_name} as any)?.__serialize === "function"
                                            ? (this.@{field.field_name} as any).__serialize(ctx)
                                            : this.@{field.field_name};
                                    }

                                {:case TypeCategory::Nullable(_)}
                                    if (this.@{field.field_name} !== null) {
                                        result["@{field.json_key}"] = typeof (this.@{field.field_name} as any)?.__serialize === "function"
                                            ? (this.@{field.field_name} as any).__serialize(ctx)
                                            : this.@{field.field_name};
                                    } else {
                                        result["@{field.json_key}"] = null;
                                    }

                                {:case TypeCategory::Serializable(_)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = typeof (this.@{field.field_name} as any)?.__serialize === "function"
                                                ? (this.@{field.field_name} as any).__serialize(ctx)
                                                : this.@{field.field_name};
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = typeof (this.@{field.field_name} as any)?.__serialize === "function"
                                            ? (this.@{field.field_name} as any).__serialize(ctx)
                                            : this.@{field.field_name};
                                    {/if}

                                {:case TypeCategory::Unknown}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = this.@{field.field_name};
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = this.@{field.field_name};
                                    {/if}
                            {/match}
                        {/for}
                    {/if}

                    {#if has_flatten}
                        {#for field in flatten_fields}
                            {#if field.optional}
                                if (this.@{field.field_name} !== undefined) {
                                    const __flattened = typeof (this.@{field.field_name} as any)?.__serialize === "function"
                                        ? (this.@{field.field_name} as any).__serialize(ctx)
                                        : this.@{field.field_name};
                                    // Remove __type and __id from flattened object
                                    const { __type: _, __id: __, ...rest } = __flattened as any;
                                    Object.assign(result, rest);
                                }
                            {:else}
                                {
                                    const __flattened = typeof (this.@{field.field_name} as any)?.__serialize === "function"
                                        ? (this.@{field.field_name} as any).__serialize(ctx)
                                        : this.@{field.field_name};
                                    // Remove __type and __id from flattened object
                                    const { __type: _, __id: __, ...rest } = __flattened as any;
                                    Object.assign(result, rest);
                                }
                            {/if}
                        {/for}
                    {/if}

                    return result;
                }
            };
            result.add_import("SerializeContext", "macroforge/serde");
            Ok(result)
        }
        Data::Enum(_) => {
            // Enums: return the underlying value directly
            let enum_name = input.name();
            Ok(ts_template! {
                export namespace @{enum_name} {
                    export function toStringifiedJSON(value: @{enum_name}): string {
                        return JSON.stringify(value);
                    }

                    export function __serialize(_ctx: SerializeContext): string | number {
                        return value;
                    }
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let container_opts =
                SerdeContainerOptions::from_decorators(&interface.inner.decorators);

            // Collect serializable fields from interface with diagnostic collection
            let mut all_diagnostics = DiagnosticCollector::new();
            let fields: Vec<SerializeField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let parse_result = SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                    all_diagnostics.extend(parse_result.diagnostics);
                    let opts = parse_result.options;

                    if !opts.should_serialize() {
                        return None;
                    }

                    let json_key = opts
                        .rename
                        .clone()
                        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                    let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                    Some(SerializeField {
                        json_key,
                        field_name: field.name.clone(),
                        type_cat,
                        optional: field.optional,
                        flatten: opts.flatten,
                    })
                })
                .collect();

            // Check for errors in field parsing before continuing
            if all_diagnostics.has_errors() {
                return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
            }

            // Separate regular fields from flattened fields
            let regular_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
            let flatten_fields: Vec<_> = fields.iter().filter(|f| f.flatten).cloned().collect();

            let has_regular = !regular_fields.is_empty();
            let has_flatten = !flatten_fields.is_empty();

            let mut result = ts_template! {
                export namespace @{interface_name} {
                    export function toStringifiedJSON(self: @{interface_name}): string {
                        const ctx = SerializeContext.create();
                        return JSON.stringify(__serialize(self, ctx));
                    }

                    export function toObject(self: @{interface_name}): Record<string, unknown> {
                        const ctx = SerializeContext.create();
                        return __serialize(self, ctx);
                    }

                    export function __serialize(self: @{interface_name}, ctx: SerializeContext): Record<string, unknown> {
                        // Check if already serialized (cycle detection)
                        const existingId = ctx.getId(self);
                        if (existingId !== undefined) {
                            return { __ref: existingId };
                        }

                        // Register this object
                        const __id = ctx.register(self);

                        const result: Record<string, unknown> = {
                            __type: "@{interface_name}",
                            __id,
                        };

                        {#if has_regular}
                            {#for field in regular_fields}
                                {#match &field.type_cat}
                                    {:case TypeCategory::Primitive}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = self.@{field.field_name};
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = self.@{field.field_name};
                                        {/if}

                                    {:case TypeCategory::Date}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = self.@{field.field_name}.toISOString();
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = self.@{field.field_name}.toISOString();
                                        {/if}

                                    {:case TypeCategory::Array(_)}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = self.@{field.field_name}.map(
                                                    (item: any) => typeof item?.__serialize === "function"
                                                        ? item.__serialize(ctx)
                                                        : item
                                                );
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = self.@{field.field_name}.map(
                                                (item: any) => typeof item?.__serialize === "function"
                                                    ? item.__serialize(ctx)
                                                    : item
                                            );
                                        {/if}

                                    {:case TypeCategory::Map(_, _)}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = Object.fromEntries(
                                                    Array.from(self.@{field.field_name}.entries()).map(
                                                        ([k, v]) => [k, typeof (v as any)?.__serialize === "function"
                                                            ? (v as any).__serialize(ctx)
                                                            : v]
                                                    )
                                                );
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = Object.fromEntries(
                                                Array.from(self.@{field.field_name}.entries()).map(
                                                    ([k, v]) => [k, typeof (v as any)?.__serialize === "function"
                                                        ? (v as any).__serialize(ctx)
                                                        : v]
                                                )
                                            );
                                        {/if}

                                    {:case TypeCategory::Set(_)}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = Array.from(self.@{field.field_name}).map(
                                                    (item: any) => typeof item?.__serialize === "function"
                                                        ? item.__serialize(ctx)
                                                        : item
                                                );
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = Array.from(self.@{field.field_name}).map(
                                                (item: any) => typeof item?.__serialize === "function"
                                                    ? item.__serialize(ctx)
                                                    : item
                                            );
                                        {/if}

                                    {:case TypeCategory::Optional(_)}
                                        if (self.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = typeof (self.@{field.field_name} as any)?.__serialize === "function"
                                                ? (self.@{field.field_name} as any).__serialize(ctx)
                                                : self.@{field.field_name};
                                        }

                                    {:case TypeCategory::Nullable(_)}
                                        if (self.@{field.field_name} !== null) {
                                            result["@{field.json_key}"] = typeof (self.@{field.field_name} as any)?.__serialize === "function"
                                                ? (self.@{field.field_name} as any).__serialize(ctx)
                                                : self.@{field.field_name};
                                        } else {
                                            result["@{field.json_key}"] = null;
                                        }

                                    {:case TypeCategory::Serializable(_)}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = typeof (self.@{field.field_name} as any)?.__serialize === "function"
                                                    ? (self.@{field.field_name} as any).__serialize(ctx)
                                                    : self.@{field.field_name};
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = typeof (self.@{field.field_name} as any)?.__serialize === "function"
                                                ? (self.@{field.field_name} as any).__serialize(ctx)
                                                : self.@{field.field_name};
                                        {/if}

                                    {:case TypeCategory::Unknown}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = self.@{field.field_name};
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = self.@{field.field_name};
                                        {/if}
                                {/match}
                            {/for}
                        {/if}

                        {#if has_flatten}
                            {#for field in flatten_fields}
                                {#if field.optional}
                                    if (self.@{field.field_name} !== undefined) {
                                        const __flattened = typeof (self.@{field.field_name} as any)?.__serialize === "function"
                                            ? (self.@{field.field_name} as any).__serialize(ctx)
                                            : self.@{field.field_name};
                                        const { __type: _, __id: __, ...rest } = __flattened as any;
                                        Object.assign(result, rest);
                                    }
                                {:else}
                                    {
                                        const __flattened = typeof (self.@{field.field_name} as any)?.__serialize === "function"
                                            ? (self.@{field.field_name} as any).__serialize(ctx)
                                            : self.@{field.field_name};
                                        const { __type: _, __id: __, ...rest } = __flattened as any;
                                        Object.assign(result, rest);
                                    }
                                {/if}
                            {/for}
                        {/if}

                        return result;
                    }
                }
            };
            result.add_import("SerializeContext", "macroforge/serde");
            Ok(result)
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            if type_alias.is_object() {
                // Object type: serialize fields
                let container_opts =
                    SerdeContainerOptions::from_decorators(&type_alias.inner.decorators);

                // Collect serializable fields with diagnostic collection
                let mut all_diagnostics = DiagnosticCollector::new();
                let fields: Vec<SerializeField> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter_map(|field| {
                        let parse_result = SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                        all_diagnostics.extend(parse_result.diagnostics);
                        let opts = parse_result.options;

                        if !opts.should_serialize() {
                            return None;
                        }

                        let json_key = opts
                            .rename
                            .clone()
                            .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                        let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                        Some(SerializeField {
                            json_key,
                            field_name: field.name.clone(),
                            type_cat,
                            optional: field.optional,
                            flatten: opts.flatten,
                        })
                    })
                    .collect();

                // Check for errors in field parsing before continuing
                if all_diagnostics.has_errors() {
                    return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
                }

                let regular_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
                let has_regular = !regular_fields.is_empty();

                let mut result = ts_template! {
                    export namespace @{type_name} {
                        export function toStringifiedJSON(value: @{type_name}): string {
                            const ctx = SerializeContext.create();
                            return JSON.stringify(__serialize(value, ctx));
                        }

                        export function toObject(value: @{type_name}): Record<string, unknown> {
                            const ctx = SerializeContext.create();
                            return __serialize(value, ctx);
                        }

                        export function __serialize(value: @{type_name}, ctx: SerializeContext): Record<string, unknown> {
                            const existingId = ctx.getId(value);
                            if (existingId !== undefined) {
                                return { __ref: existingId };
                            }

                            const __id = ctx.register(value);
                            const result: Record<string, unknown> = {
                                __type: "@{type_name}",
                                __id,
                            };

                            {#if has_regular}
                                {#for field in regular_fields}
                                    {#if field.optional}
                                        if (value.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = value.@{field.field_name};
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = value.@{field.field_name};
                                    {/if}
                                {/for}
                            {/if}

                            return result;
                        }
                    }
                };
                result.add_import("SerializeContext", "macroforge/serde");
                Ok(result)
            } else {
                // Union, tuple, or simple alias: delegate to inner type's __serialize if available
                let mut result = ts_template! {
                    export namespace @{type_name} {
                        export function toStringifiedJSON(value: @{type_name}): string {
                            const ctx = SerializeContext.create();
                            return JSON.stringify(__serialize(value, ctx));
                        }

                        export function toObject(value: @{type_name}): unknown {
                            const ctx = SerializeContext.create();
                            return __serialize(value, ctx);
                        }

                        export function __serialize(value: @{type_name}, ctx: SerializeContext): unknown {
                            if (typeof (value as any)?.__serialize === "function") {
                                return (value as any).__serialize(ctx);
                            }
                            return value;
                        }
                    }
                };
                result.add_import("SerializeContext", "macroforge/serde");
                Ok(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_field_struct() {
        let field = SerializeField {
            json_key: "name".into(),
            field_name: "name".into(),
            type_cat: TypeCategory::Primitive,
            optional: false,
            flatten: false,
        };
        assert_eq!(field.json_key, "name");
        assert!(!field.optional);
    }
}
