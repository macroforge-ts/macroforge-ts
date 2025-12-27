import { productDefaultsDefaultValue } from "./product-defaults.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { productDefaultsSerializeWithContext } from "./product-defaults.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { productDefaultsDeserializeWithContext } from "./product-defaults.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { ProductDefaults } from './product-defaults.svelte';


export interface Product {
    
    id: string;
    
    
    name: string;
    
    
    quickCode: string;
    
    group: string | null;
    
    subgroup: string | null;
    
    unit: string | null;
    
    active: boolean;
    
    commission: boolean;
    
    favorite: boolean;
    defaults: ProductDefaults;
}

export function productDefaultValue(): Product {
    return {
        id: "",
        name: "",
        quickCode: "",
        group: null,
        subgroup: null,
        unit: null,
        active: false,
        commission: false,
        favorite: false,
        defaults: productDefaultsDefaultValue()
    } as Product;
}

export function productSerialize(value: Product): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(productSerializeWithContext(value, ctx));
}
export function productSerializeWithContext(value: Product, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Product"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"name"}`] = value.name;
    result[`${"quickCode"}`] = value.quickCode;
    result[`${"group"}`] = value.group;
    result[`${"subgroup"}`] = value.subgroup;
    result[`${"unit"}`] = value.unit;
    result[`${"active"}`] = value.active;
    result[`${"commission"}`] = value.commission;
    result[`${"favorite"}`] = value.favorite;
    result[`${"defaults"}`] = productDefaultsSerializeWithContext(value.defaults, ctx);
    return result;
}

export function productDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Product } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = productDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Product.deserialize: root cannot be a forward reference"
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return {
            success: true,
            value: resultOrRef
        };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return {
                success: false,
                errors: e.errors
            };
        }
        const message = e instanceof Error ? e.message : String(e);
        return {
            success: false,
            errors: [
                {
                    field: "_root",
                    message
                }
            ]
        };
    }
}
export function productDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Product | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Product"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"id"}` in obj)) {
        errors.push({
            field: `${"id"}`,
            message: "missing required field"
        });
    }
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
            message: "missing required field"
        });
    }
    if (!(`${"quickCode"}` in obj)) {
        errors.push({
            field: `${"quickCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"group"}` in obj)) {
        errors.push({
            field: `${"group"}`,
            message: "missing required field"
        });
    }
    if (!(`${"subgroup"}` in obj)) {
        errors.push({
            field: `${"subgroup"}`,
            message: "missing required field"
        });
    }
    if (!(`${"unit"}` in obj)) {
        errors.push({
            field: `${"unit"}`,
            message: "missing required field"
        });
    }
    if (!(`${"active"}` in obj)) {
        errors.push({
            field: `${"active"}`,
            message: "missing required field"
        });
    }
    if (!(`${"commission"}` in obj)) {
        errors.push({
            field: `${"commission"}`,
            message: "missing required field"
        });
    }
    if (!(`${"favorite"}` in obj)) {
        errors.push({
            field: `${"favorite"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaults"}` in obj)) {
        errors.push({
            field: `${"defaults"}`,
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_id = obj[`${"id"}`] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Product.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_quickCode = obj[`${"quickCode"}`] as string;
        if (__raw_quickCode.trim().length === 0) {
            errors.push({
                field: "quickCode",
                message: "Product.quickCode must not be empty"
            });
        }
        instance.quickCode = __raw_quickCode;
    }
    {
        const __raw_group = obj[`${"group"}`] as string | null;
        instance.group = __raw_group;
    }
    {
        const __raw_subgroup = obj[`${"subgroup"}`] as string | null;
        instance.subgroup = __raw_subgroup;
    }
    {
        const __raw_unit = obj[`${"unit"}`] as string | null;
        instance.unit = __raw_unit;
    }
    {
        const __raw_active = obj[`${"active"}`] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_commission = obj[`${"commission"}`] as boolean;
        instance.commission = __raw_commission;
    }
    {
        const __raw_favorite = obj[`${"favorite"}`] as boolean;
        instance.favorite = __raw_favorite;
    }
    {
        const __raw_defaults = obj[`${"defaults"}`] as ProductDefaults;
        {
            const __result = productDefaultsDeserializeWithContext(__raw_defaults, ctx);
            ctx.assignOrDefer(instance, `${"defaults"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Product;
}
export function productValidateField<K extends keyof Product>(_field: K, _value: Product[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"name"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Product.name must not be empty"
            });
        }
    }
    if (_field === `${"quickCode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "quickCode",
                message: "Product.quickCode must not be empty"
            });
        }
    }
    return errors;
}
export function productValidateFields(_partial: Partial<Product>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"name"}` in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Product.name must not be empty"
            });
        }
    }
    if (`${"quickCode"}` in _partial && _partial.quickCode !== undefined) {
        const __val = _partial.quickCode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "quickCode",
                message: "Product.quickCode must not be empty"
            });
        }
    }
    return errors;
}
export function productHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "name" in o && "quickCode" in o && "group" in o && "subgroup" in o && "unit" in o && "active" in o && "commission" in o && "favorite" in o && "defaults" in o';
}
export function productIs(obj: unknown): obj is Product {
    if (!productHasShape(obj)) {
        return false;
    }
    const result = productDeserialize(obj);
    return result.success;
}

export const Product = {
  defaultValue: productDefaultValue,
  serialize: productSerialize,
  serializeWithContext: productSerializeWithContext,
  deserialize: productDeserialize,
  deserializeWithContext: productDeserializeWithContext,
  validateFields: productValidateFields,
  hasShape: productHasShape,
  is: productIs
} as const;