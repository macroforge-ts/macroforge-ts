import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { itemSerializeWithContext } from "./item.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { itemDeserializeWithContext } from "./item.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Item } from './item.svelte';


export interface BilledItem {
    
    
    item: Item;
    
    quantity: number;
    
    taxed: boolean;
    
    upsale: boolean;
}

export function billedItemDefaultValue(): BilledItem {
    return {
        item: "",
        quantity: 0,
        taxed: false,
        upsale: false
    } as BilledItem;
}

export function billedItemSerialize(value: BilledItem): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(billedItemSerializeWithContext(value, ctx));
}
export function billedItemSerializeWithContext(value: BilledItem, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"BilledItem"}`,
        __id
    };
    result[`${"item"}`] = itemSerializeWithContext(value.item, ctx);
    result[`${"quantity"}`] = value.quantity;
    result[`${"taxed"}`] = value.taxed;
    result[`${"upsale"}`] = value.upsale;
    return result;
}

export function billedItemDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: BilledItem } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = billedItemDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "BilledItem.deserialize: root cannot be a forward reference"
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
export function billedItemDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): BilledItem | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"BilledItem"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"item"}` in obj)) {
        errors.push({
            field: `${"item"}`,
            message: "missing required field"
        });
    }
    if (!(`${"quantity"}` in obj)) {
        errors.push({
            field: `${"quantity"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxed"}` in obj)) {
        errors.push({
            field: `${"taxed"}`,
            message: "missing required field"
        });
    }
    if (!(`${"upsale"}` in obj)) {
        errors.push({
            field: `${"upsale"}`,
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
        const __raw_item = obj[`${"item"}`] as Item;
        {
            const __result = itemDeserializeWithContext(__raw_item, ctx);
            ctx.assignOrDefer(instance, `${"item"}`, __result);
        }
    }
    {
        const __raw_quantity = obj[`${"quantity"}`] as number;
        instance.quantity = __raw_quantity;
    }
    {
        const __raw_taxed = obj[`${"taxed"}`] as boolean;
        instance.taxed = __raw_taxed;
    }
    {
        const __raw_upsale = obj[`${"upsale"}`] as boolean;
        instance.upsale = __raw_upsale;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as BilledItem;
}
export function billedItemValidateField<K extends keyof BilledItem>(_field: K, _value: BilledItem[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function billedItemValidateFields(_partial: Partial<BilledItem>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function billedItemHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"item" in o && "quantity" in o && "taxed" in o && "upsale" in o';
}
export function billedItemIs(obj: unknown): obj is BilledItem {
    if (!billedItemHasShape(obj)) {
        return false;
    }
    const result = billedItemDeserialize(obj);
    return result.success;
}

export function billedItemFromFormData(formData: FormData): Exit<BilledItem, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const itemObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"item"}.`)) {
                const fieldName = key.slice(`${"item"}.`.length);
                const parts = fieldName.split(".");
                let current = itemObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.item = itemObj;
    }
    {
        const quantityStr = formData.get(`${"quantity"}`);
        obj.quantity = quantityStr ? parseFloat(quantityStr as string) : $MfPh5;
        if (obj.quantity !== undefined && isNaN(obj.quantity as number)) obj.quantity = "0";
    }
    {
        const taxedVal = formData.get(`${"taxed"}`);
        obj.taxed = taxedVal === "true" || taxedVal === "on" || taxedVal === "1";
    }
    {
        const upsaleVal = formData.get(`${"upsale"}`);
        obj.upsale = upsaleVal === "true" || upsaleVal === "on" || upsaleVal === "1";
    }
    return toExit("billedItemDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: BilledItem;
    readonly errors: BilledItemErrors;
    readonly tainted: BilledItemTainted;
    readonly fields: BilledItemFieldControllers;
    validate(): Exit<BilledItem, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<BilledItem>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function billedItemCreateForm(overrides: Partial<BilledItem>): BilledItemGigaform {}
let data = $state({
    ...billedItemDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as BilledItemErrors);
let tainted = $state<$MfPh3>({} as BilledItemTainted);
const fields = {} as BilledItemFieldControllers;
fields.item = {
    label: `${"item"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.quantity = {
    label: `${"quantity"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.taxed = {
    label: `${"taxed"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.upsale = {
    label: `${"upsale"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
function validate(): Exit<BilledItem, Array<{
    field: string;
    message: string;
}>> {
    return toExit("billedItemDeserialize(data)");
    data = {
        ...billedItemDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const BilledItem = {
  defaultValue: billedItemDefaultValue,
  serialize: billedItemSerialize,
  serializeWithContext: billedItemSerializeWithContext,
  deserialize: billedItemDeserialize,
  deserializeWithContext: billedItemDeserializeWithContext,
  validateFields: billedItemValidateFields,
  hasShape: billedItemHasShape,
  is: billedItemIs,
  fromFormData: billedItemFromFormData,
  createForm: billedItemCreateForm
} as const;