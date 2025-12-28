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


export interface Paid {
    amount: number | null;
    currency: string | null;
    paymentMethod: string | null;
}

export function paidDefaultValue(): Paid {
    return {
        amount: null,
        currency: null,
        paymentMethod: null
    } as Paid;
}

export function paidSerialize(value: Paid): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(paidSerializeWithContext(value, ctx));
}
export function paidSerializeWithContext(value: Paid, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "Paid",
        __id
    };
    result.amount = value.amount;
    result.currency = value.currency;
    result.paymentMethod = value.paymentMethod;
    return result;
}

export function paidDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Paid } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = paidDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Paid.deserialize: root cannot be a forward reference"
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
export function paidDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Paid | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Paid"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("amount" in obj)) {
        errors.push({
            field: "amount",
            message: "missing required field"
        });
    }
    if (!("currency" in obj)) {
        errors.push({
            field: "currency",
            message: "missing required field"
        });
    }
    if (!("paymentMethod" in obj)) {
        errors.push({
            field: "paymentMethod",
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
        const __raw_amount = obj["amount"] as number | null;
        instance.amount = __raw_amount;
    }
    {
        const __raw_currency = obj["currency"] as string | null;
        instance.currency = __raw_currency;
    }
    {
        const __raw_paymentMethod = obj["paymentMethod"] as string | null;
        instance.paymentMethod = __raw_paymentMethod;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Paid;
}
export function paidValidateField<K extends keyof Paid>(_field: K, _value: Paid[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function paidValidateFields(_partial: Partial<Paid>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function paidHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"amount" in o && "currency" in o && "paymentMethod" in o';
}
export function paidIs(obj: unknown): obj is Paid {
    if (!paidHasShape(obj)) {
        return false;
    }
    const result = paidDeserialize(obj);
    return result.success;
}

export type PaidErrors = {
    _errors: __gf_Option<Array<string>>;
    amount: __gf_Option<Array<string>>;
    currency: __gf_Option<Array<string>>;
    paymentMethod: __gf_Option<Array<string>>;
};
export type PaidTainted = {
    amount: __gf_Option<boolean>;
    currency: __gf_Option<boolean>;
    paymentMethod: __gf_Option<boolean>;
};
export interface PaidFieldControllers {
    readonly amount: FieldController<number | null>;
    readonly currency: FieldController<string | null>;
    readonly paymentMethod: FieldController<string | null>;
}
export interface PaidGigaform {
    readonly data: Paid;
    readonly errors: PaidErrors;
    readonly tainted: PaidTainted;
    readonly fields: PaidFieldControllers;
    validate(): Exit<Paid, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Paid>): void;
}
export function paidCreateForm(overrides?: Partial<Paid>): PaidGigaform {
    let data = $state({
        ...paidDefaultValue(),
        ...overrides
    });
    let errors = $state<PaidErrors>({
        _errors: optionNone(),
        amount: optionNone(),
        currency: optionNone(),
        paymentMethod: optionNone()
    } as PaidErrors);
    let tainted = $state<PaidTainted>({
        amount: optionNone(),
        currency: optionNone(),
        paymentMethod: optionNone()
    } as PaidTainted);
    const fields = {
        amount: {
            path: [
                "amount"
            ] as const,
            name: "amount",
            constraints: {
                required: true
            },
            get: ()=>data.amount,
            set: (value: number | null)=>{
                data.amount = value;
            },
            transform: (value: number | null): number | null =>value,
            getError: ()=>errors.amount,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.amount = value;
            },
            getTainted: ()=>tainted.amount,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.amount = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = paidValidateField("amount", data.amount);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        currency: {
            path: [
                "currency"
            ] as const,
            name: "currency",
            constraints: {
                required: true
            },
            get: ()=>data.currency,
            set: (value: string | null)=>{
                data.currency = value;
            },
            transform: (value: string | null): string | null =>value,
            getError: ()=>errors.currency,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.currency = value;
            },
            getTainted: ()=>tainted.currency,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.currency = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = paidValidateField("currency", data.currency);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        paymentMethod: {
            path: [
                "paymentMethod"
            ] as const,
            name: "paymentMethod",
            constraints: {
                required: true
            },
            get: ()=>data.paymentMethod,
            set: (value: string | null)=>{
                data.paymentMethod = value;
            },
            transform: (value: string | null): string | null =>value,
            getError: ()=>errors.paymentMethod,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.paymentMethod = value;
            },
            getTainted: ()=>tainted.paymentMethod,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.paymentMethod = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = paidValidateField("paymentMethod", data.paymentMethod);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as PaidFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<Paid, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(paidDeserialize(data));
    }
    function reset(newOverrides?: Partial<Paid>): void {
        data = {
            ...paidDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            amount: optionNone(),
            currency: optionNone(),
            paymentMethod: optionNone()
        };
        tainted = {
            amount: optionNone(),
            currency: optionNone(),
            paymentMethod: optionNone()
        };
    }
    return {
        get data () {
            return data;
        },
        set data (v){
            data = v;
        },
        get errors () {
            return errors;
        },
        set errors (v){
            errors = v;
        },
        get tainted () {
            return tainted;
        },
        set tainted (v){
            tainted = v;
        },
        fields,
        validate,
        reset
    };
}
export function paidFromFormData(formData: FormData): Exit<Paid, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<Paid, Array<{ field: string; message: string }>>";
    {
        const amountStr = formData.get(`${"amount"}`);
        obj.amount = amountStr ? parseFloat(amountStr as string) : $MfPh5;
        if (obj.amount !== undefined && isNaN(obj.amount as number)) obj.amount = "0";
    }
    obj.currency = formData.get(`${"currency"}`) ?? "";
    obj.paymentMethod = formData.get(`${"paymentMethod"}`) ?? "";
    return toExit(paidDeserialize(obj));
}

export const Paid = {
  defaultValue: paidDefaultValue,
  serialize: paidSerialize,
  serializeWithContext: paidSerializeWithContext,
  deserialize: paidDeserialize,
  deserializeWithContext: paidDeserializeWithContext,
  validateFields: paidValidateFields,
  hasShape: paidHasShape,
  is: paidIs,
  createForm: paidCreateForm,
  fromFormData: paidFromFormData
} as const;