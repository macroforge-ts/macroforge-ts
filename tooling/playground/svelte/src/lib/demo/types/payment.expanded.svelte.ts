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


export interface Payment {
    id: string;
    date: string;
}

export function paymentDefaultValue(): Payment {
    return {
        id: "",
        date: ""
    } as Payment;
}

export function paymentSerialize(value: Payment): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(paymentSerializeWithContext(value, ctx));
}
export function paymentSerializeWithContext(value: Payment, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Payment"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"date"}`] = value.date;
    return result;
}

export function paymentDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Payment } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = paymentDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Payment.deserialize: root cannot be a forward reference"
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
export function paymentDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Payment | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Payment"}.deserializeWithContext: expected an object`
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
        const __raw_date = obj[`${"date"}`] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Payment;
}
export function paymentValidateField<K extends keyof Payment>(_field: K, _value: Payment[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function paymentValidateFields(_partial: Partial<Payment>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function paymentHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "date" in o';
}
export function paymentIs(obj: unknown): obj is Payment {
    if (!paymentHasShape(obj)) {
        return false;
    }
    const result = paymentDeserialize(obj);
    return result.success;
}

export function paymentFromFormData(formData: FormData): Exit<Payment, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.date = formData.get(`${"date"}`) ?? "";
    return toExit("paymentDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Payment;
    readonly errors: PaymentErrors;
    readonly tainted: PaymentTainted;
    readonly fields: PaymentFieldControllers;
    validate(): Exit<Payment, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Payment>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function paymentCreateForm(overrides: Partial<Payment>): PaymentGigaform {}
let data = $state({
    ...paymentDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as PaymentErrors);
let tainted = $state<$MfPh3>({} as PaymentTainted);
const fields = {} as PaymentFieldControllers;
fields.id = {
    label: `${"id"}`,
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
function validate(): Exit<Payment, Array<{
    field: string;
    message: string;
}>> {
    return toExit("paymentDeserialize(data)");
    data = {
        ...paymentDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Payment = {
  defaultValue: paymentDefaultValue,
  serialize: paymentSerialize,
  serializeWithContext: paymentSerializeWithContext,
  deserialize: paymentDeserialize,
  deserializeWithContext: paymentDeserializeWithContext,
  validateFields: paymentValidateFields,
  hasShape: paymentHasShape,
  is: paymentIs,
  fromFormData: paymentFromFormData,
  createForm: paymentCreateForm
} as const;