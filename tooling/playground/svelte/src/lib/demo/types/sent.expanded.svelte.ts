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


export interface Sent {
    recipient: string | null;
    method: string | null;
}

export function sentDefaultValue(): Sent {
    return {
        recipient: null,
        method: null
    } as Sent;
}

export function sentSerialize(value: Sent): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(sentSerializeWithContext(value, ctx));
}
export function sentSerializeWithContext(value: Sent, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Sent"}`,
        __id
    };
    result[`${"recipient"}`] = value.recipient;
    result[`${"method"}`] = value.method;
    return result;
}

export function sentDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Sent } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = sentDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Sent.deserialize: root cannot be a forward reference"
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
export function sentDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Sent | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Sent"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"recipient"}` in obj)) {
        errors.push({
            field: `${"recipient"}`,
            message: "missing required field"
        });
    }
    if (!(`${"method"}` in obj)) {
        errors.push({
            field: `${"method"}`,
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
        const __raw_recipient = obj[`${"recipient"}`] as string | null;
        instance.recipient = __raw_recipient;
    }
    {
        const __raw_method = obj[`${"method"}`] as string | null;
        instance.method = __raw_method;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Sent;
}
export function sentValidateField<K extends keyof Sent>(_field: K, _value: Sent[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function sentValidateFields(_partial: Partial<Sent>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function sentHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"recipient" in o && "method" in o';
}
export function sentIs(obj: unknown): obj is Sent {
    if (!sentHasShape(obj)) {
        return false;
    }
    const result = sentDeserialize(obj);
    return result.success;
}

export function sentFromFormData(formData: FormData): Exit<Sent, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.recipient = formData.get(`${"recipient"}`) ?? "";
    obj.method = formData.get(`${"method"}`) ?? "";
    return toExit("sentDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Sent;
    readonly errors: SentErrors;
    readonly tainted: SentTainted;
    readonly fields: SentFieldControllers;
    validate(): Exit<Sent, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Sent>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function sentCreateForm(overrides: Partial<Sent>): SentGigaform {}
let data = $state({
    ...sentDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as SentErrors);
let tainted = $state<$MfPh3>({} as SentTainted);
const fields = {} as SentFieldControllers;
fields.recipient = {
    label: `${"recipient"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.method = {
    label: `${"method"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Sent, Array<{
    field: string;
    message: string;
}>> {
    return toExit("sentDeserialize(data)");
    data = {
        ...sentDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Sent = {
  defaultValue: sentDefaultValue,
  serialize: sentSerialize,
  serializeWithContext: sentSerializeWithContext,
  deserialize: sentDeserialize,
  deserializeWithContext: sentDeserializeWithContext,
  validateFields: sentValidateFields,
  hasShape: sentHasShape,
  is: sentIs,
  fromFormData: sentFromFormData,
  createForm: sentCreateForm
} as const;