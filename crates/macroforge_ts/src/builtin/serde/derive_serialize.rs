//! /** @derive(Serialize) */ macro implementation
//!
//! Generates JSON serialization methods:
//! - For classes: `toJSON(): Record<string, unknown>`
//! - For interfaces: `namespace InterfaceName { export function toJSON(self: InterfaceName): Record<string, unknown> }`

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

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
    description = "Generates a toJSON() method for JSON serialization",
    attributes(serde)
)]
pub fn derive_serialize_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let container_opts = SerdeContainerOptions::from_decorators(&class.inner.decorators);

            // Collect serializable fields
            let fields: Vec<SerializeField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = SerdeFieldOptions::from_decorators(&field.decorators);
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

            // Separate regular fields from flattened fields
            let regular_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
            let flatten_fields: Vec<_> = fields.iter().filter(|f| f.flatten).cloned().collect();

            let has_regular = !regular_fields.is_empty();
            let has_flatten = !flatten_fields.is_empty();

            Ok(body! {
                toJSON(): Record<string, unknown> {
                    const result: Record<string, unknown> = {};

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
                                                (item: any) => typeof item?.toJSON === "function" ? item.toJSON() : item
                                            );
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = this.@{field.field_name}.map(
                                            (item: any) => typeof item?.toJSON === "function" ? item.toJSON() : item
                                        );
                                    {/if}

                                {:case TypeCategory::Map(_, _)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = Object.fromEntries(
                                                Array.from(this.@{field.field_name}.entries()).map(
                                                    ([k, v]) => [k, typeof (v as any)?.toJSON === "function" ? (v as any).toJSON() : v]
                                                )
                                            );
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = Object.fromEntries(
                                            Array.from(this.@{field.field_name}.entries()).map(
                                                ([k, v]) => [k, typeof (v as any)?.toJSON === "function" ? (v as any).toJSON() : v]
                                            )
                                        );
                                    {/if}

                                {:case TypeCategory::Set(_)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = Array.from(this.@{field.field_name}).map(
                                                (item: any) => typeof item?.toJSON === "function" ? item.toJSON() : item
                                            );
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = Array.from(this.@{field.field_name}).map(
                                            (item: any) => typeof item?.toJSON === "function" ? item.toJSON() : item
                                        );
                                    {/if}

                                {:case TypeCategory::Optional(_)}
                                    if (this.@{field.field_name} !== undefined) {
                                        result["@{field.json_key}"] = typeof (this.@{field.field_name} as any)?.toJSON === "function"
                                            ? (this.@{field.field_name} as any).toJSON()
                                            : this.@{field.field_name};
                                    }

                                {:case TypeCategory::Nullable(_)}
                                    result["@{field.json_key}"] = this.@{field.field_name} !== null
                                        && typeof (this.@{field.field_name} as any)?.toJSON === "function"
                                        ? (this.@{field.field_name} as any).toJSON()
                                        : this.@{field.field_name};

                                {:case TypeCategory::Serializable(_)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = typeof (this.@{field.field_name} as any)?.toJSON === "function"
                                                ? (this.@{field.field_name} as any).toJSON()
                                                : this.@{field.field_name};
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = typeof (this.@{field.field_name} as any)?.toJSON === "function"
                                            ? (this.@{field.field_name} as any).toJSON()
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
                                    Object.assign(result, typeof (this.@{field.field_name} as any)?.toJSON === "function"
                                        ? (this.@{field.field_name} as any).toJSON()
                                        : this.@{field.field_name});
                                }
                            {:else}
                                Object.assign(result, typeof (this.@{field.field_name} as any)?.toJSON === "function"
                                    ? (this.@{field.field_name} as any).toJSON()
                                    : this.@{field.field_name});
                            {/if}
                        {/for}
                    {/if}

                    return result;
                }
            })
        }
        Data::Enum(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(Serialize) */ can only be applied to classes or interfaces",
        )),
        Data::Interface(interface) => {
            let interface_name = input.name();
            let container_opts =
                SerdeContainerOptions::from_decorators(&interface.inner.decorators);

            // Collect serializable fields from interface
            let fields: Vec<SerializeField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = SerdeFieldOptions::from_decorators(&field.decorators);
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

            // Separate regular fields from flattened fields
            let regular_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
            let flatten_fields: Vec<_> = fields.iter().filter(|f| f.flatten).cloned().collect();

            let has_regular = !regular_fields.is_empty();
            let has_flatten = !flatten_fields.is_empty();

            Ok(ts_template! {
                export namespace @{interface_name} {
                    export function toJSON(self: @{interface_name}): Record<string, unknown> {
                        const result: Record<string, unknown> = {};

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
                                                    (item: any) => typeof item?.toJSON === "function" ? item.toJSON() : item
                                                );
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = self.@{field.field_name}.map(
                                                (item: any) => typeof item?.toJSON === "function" ? item.toJSON() : item
                                            );
                                        {/if}

                                    {:case TypeCategory::Map(_, _)}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = Object.fromEntries(
                                                    Array.from(self.@{field.field_name}.entries()).map(
                                                        ([k, v]) => [k, typeof (v as any)?.toJSON === "function" ? (v as any).toJSON() : v]
                                                    )
                                                );
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = Object.fromEntries(
                                                Array.from(self.@{field.field_name}.entries()).map(
                                                    ([k, v]) => [k, typeof (v as any)?.toJSON === "function" ? (v as any).toJSON() : v]
                                                )
                                            );
                                        {/if}

                                    {:case TypeCategory::Set(_)}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = Array.from(self.@{field.field_name}).map(
                                                    (item: any) => typeof item?.toJSON === "function" ? item.toJSON() : item
                                                );
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = Array.from(self.@{field.field_name}).map(
                                                (item: any) => typeof item?.toJSON === "function" ? item.toJSON() : item
                                            );
                                        {/if}

                                    {:case TypeCategory::Optional(_)}
                                        if (self.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = typeof (self.@{field.field_name} as any)?.toJSON === "function"
                                                ? (self.@{field.field_name} as any).toJSON()
                                                : self.@{field.field_name};
                                        }

                                    {:case TypeCategory::Nullable(_)}
                                        result["@{field.json_key}"] = self.@{field.field_name} !== null
                                            && typeof (self.@{field.field_name} as any)?.toJSON === "function"
                                            ? (self.@{field.field_name} as any).toJSON()
                                            : self.@{field.field_name};

                                    {:case TypeCategory::Serializable(_)}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = typeof (self.@{field.field_name} as any)?.toJSON === "function"
                                                    ? (self.@{field.field_name} as any).toJSON()
                                                    : self.@{field.field_name};
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = typeof (self.@{field.field_name} as any)?.toJSON === "function"
                                                ? (self.@{field.field_name} as any).toJSON()
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
                                        Object.assign(result, typeof (self.@{field.field_name} as any)?.toJSON === "function"
                                            ? (self.@{field.field_name} as any).toJSON()
                                            : self.@{field.field_name});
                                    }
                                {:else}
                                    Object.assign(result, typeof (self.@{field.field_name} as any)?.toJSON === "function"
                                        ? (self.@{field.field_name} as any).toJSON()
                                        : self.@{field.field_name});
                                {/if}
                            {/for}
                        {/if}

                        return result;
                    }
                }
            })
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
