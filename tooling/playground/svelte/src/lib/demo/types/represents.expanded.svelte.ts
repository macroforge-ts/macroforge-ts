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
import type { Employee } from './employee.svelte';


export interface Represents {
    
    in: string | Employee;
    
    out: string | Account;
    id: string;
    dateStarted: string;
}

export function representsDefaultValue(): Represents {
    return {
        in: "",
        out: "",
        id: "",
        dateStarted: ""
    } as Represents;
}

export function representsSerialize(value: Represents): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(representsSerializeWithContext(value, ctx));
}
export function representsSerializeWithContext(value: Represents, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Represents"}`,
        __id
    };
    result[`${"in"}`] = value.in;
    result[`${"out"}`] = value.out;
    result[`${"id"}`] = value.id;
    result[`${"dateStarted"}`] = value.dateStarted;
    return result;
}

export function representsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Represents } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = representsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Represents.deserialize: root cannot be a forward reference"
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
export function representsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Represents | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Represents"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
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
    if (!(`${"id"}` in obj)) {
        errors.push({
            field: `${"id"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dateStarted"}` in obj)) {
        errors.push({
            field: `${"dateStarted"}`,
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
        const __raw_in = obj[`${"in"}`] as string | Employee;
        instance.in = __raw_in;
    }
    {
        const __raw_out = obj[`${"out"}`] as string | Account;
        instance.out = __raw_out;
    }
    {
        const __raw_id = obj[`${"id"}`] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_dateStarted = obj[`${"dateStarted"}`] as string;
        instance.dateStarted = __raw_dateStarted;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Represents;
}
export function representsValidateField<K extends keyof Represents>(_field: K, _value: Represents[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function representsValidateFields(_partial: Partial<Represents>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function representsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"in" in o && "out" in o && "id" in o && "dateStarted" in o';
}
export function representsIs(obj: unknown): obj is Represents {
    if (!representsHasShape(obj)) {
        return false;
    }
    const result = representsDeserialize(obj);
    return result.success;
}

export function representsFromFormData(formData: FormData): Exit<Represents, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.in = formData.get(`${"in"}`) ?? "";
    obj.out = formData.get(`${"out"}`) ?? "";
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.dateStarted = formData.get(`${"dateStarted"}`) ?? "";
    return toExit("representsDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Represents;
    readonly errors: RepresentsErrors;
    readonly tainted: RepresentsTainted;
    readonly fields: RepresentsFieldControllers;
    validate(): Exit<Represents, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Represents>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function representsCreateForm(overrides: Partial<Represents>): RepresentsGigaform {}
let data = $state({
    ...representsDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as RepresentsErrors);
let tainted = $state<$MfPh3>({} as RepresentsTainted);
const fields = {} as RepresentsFieldControllers;
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
fields.id = {
    label: `${"id"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.dateStarted = {
    label: `${"dateStarted"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Represents, Array<{
    field: string;
    message: string;
}>> {
    return toExit("representsDeserialize(data)");
    data = {
        ...representsDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Represents = {
  defaultValue: representsDefaultValue,
  serialize: representsSerialize,
  serializeWithContext: representsSerializeWithContext,
  deserialize: representsDeserialize,
  deserializeWithContext: representsDeserializeWithContext,
  validateFields: representsValidateFields,
  hasShape: representsHasShape,
  is: representsIs,
  fromFormData: representsFromFormData,
  createForm: representsCreateForm
} as const;