import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Account } from './account.svelte';
import type { Order } from './order.svelte';


export interface Ordered {
    id: string;
    
    in: string | Account;
    
    out: string | Order;
    date: string;
}

export function orderedDefaultValue(): Ordered {
    return {
        id: "",
        in: "",
        out: "",
        date: ""
    } as Ordered;
}

export function orderedSerialize(value: Ordered): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(orderedSerializeWithContext(value, ctx));
}
export function orderedSerializeWithContext(value: Ordered, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Ordered"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"in"}`] = value.in;
    result[`${"out"}`] = value.out;
    result[`${"date"}`] = value.date;
    return result;
}

export function orderedDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Ordered } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = orderedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Ordered.deserialize: root cannot be a forward reference"
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
export function orderedDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Ordered | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Ordered"}.deserializeWithContext: expected an object`
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
    if (!(`${"in"}` in obj)) {
        errors.push({
            field: `${"in"}`,
            message: "missing required field"
        });
    }
    if (!(`${"out"}` in obj)) {
        errors.push({
            field: `${"out"}`,
            message: "missing required field"
        });
    }
    if (!(`${"date"}` in obj)) {
        errors.push({
            field: `${"date"}`,
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
        const __raw_in = obj[`${"in"}`] as string | Account;
        instance.in = __raw_in;
    }
    {
        const __raw_out = obj[`${"out"}`] as string | Order;
        instance.out = __raw_out;
    }
    {
        const __raw_date = obj[`${"date"}`] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Ordered;
}
export function orderedValidateField<K extends keyof Ordered>(_field: K, _value: Ordered[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function orderedValidateFields(_partial: Partial<Ordered>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function orderedHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "in" in o && "out" in o && "date" in o';
}
export function orderedIs(obj: unknown): obj is Ordered {
    if (!orderedHasShape(obj)) {
        return false;
    }
    const result = orderedDeserialize(obj);
    return result.success;
}

export function orderedFromFormData(formData: FormData): Exit<Ordered, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.in = formData.get(`${"in"}`) ?? "";
    obj.out = formData.get(`${"out"}`) ?? "";
    obj.date = formData.get(`${"date"}`) ?? "";
    return toExit("orderedDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Ordered;
    readonly errors: OrderedErrors;
    readonly tainted: OrderedTainted;
    readonly fields: OrderedFieldControllers;
    validate(): Exit<Ordered, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Ordered>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function orderedCreateForm(overrides: Partial<Ordered>): OrderedGigaform {}
let data = $state({
    ...orderedDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as OrderedErrors);
let tainted = $state<$MfPh3>({} as OrderedTainted);
const fields = {} as OrderedFieldControllers;
fields.id = {
    label: `${"id"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.in = {
    label: `${"in"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.out = {
    label: `${"out"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.date = {
    label: `${"date"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Ordered, Array<{
    field: string;
    message: string;
}>> {
    return toExit("orderedDeserialize(data)");
    data = {
        ...orderedDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Ordered = {
  defaultValue: orderedDefaultValue,
  serialize: orderedSerialize,
  serializeWithContext: orderedSerializeWithContext,
  deserialize: orderedDeserialize,
  deserializeWithContext: orderedDeserializeWithContext,
  validateFields: orderedValidateFields,
  hasShape: orderedHasShape,
  is: orderedIs,
  fromFormData: orderedFromFormData,
  createForm: orderedCreateForm
} as const;