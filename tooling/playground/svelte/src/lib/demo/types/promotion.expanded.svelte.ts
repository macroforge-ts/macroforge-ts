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


export interface Promotion {
    id: string;
    date: string;
}

export function promotionDefaultValue(): Promotion {
    return {
        id: "",
        date: ""
    } as Promotion;
}

export function promotionSerialize(value: Promotion): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(promotionSerializeWithContext(value, ctx));
}
export function promotionSerializeWithContext(value: Promotion, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Promotion"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"date"}`] = value.date;
    return result;
}

export function promotionDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Promotion } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = promotionDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Promotion.deserialize: root cannot be a forward reference"
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
export function promotionDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Promotion | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Promotion"}.deserializeWithContext: expected an object`
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
    return instance as Promotion;
}
export function promotionValidateField<K extends keyof Promotion>(_field: K, _value: Promotion[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function promotionValidateFields(_partial: Partial<Promotion>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function promotionHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "date" in o';
}
export function promotionIs(obj: unknown): obj is Promotion {
    if (!promotionHasShape(obj)) {
        return false;
    }
    const result = promotionDeserialize(obj);
    return result.success;
}

export function promotionFromFormData(formData: FormData): Exit<Promotion, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.date = formData.get(`${"date"}`) ?? "";
    return toExit("promotionDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Promotion;
    readonly errors: PromotionErrors;
    readonly tainted: PromotionTainted;
    readonly fields: PromotionFieldControllers;
    validate(): Exit<Promotion, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Promotion>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function promotionCreateForm(overrides: Partial<Promotion>): PromotionGigaform {}
let data = $state({
    ...promotionDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as PromotionErrors);
let tainted = $state<$MfPh3>({} as PromotionTainted);
const fields = {} as PromotionFieldControllers;
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
function validate(): Exit<Promotion, Array<{
    field: string;
    message: string;
}>> {
    return toExit("promotionDeserialize(data)");
    data = {
        ...promotionDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Promotion = {
  defaultValue: promotionDefaultValue,
  serialize: promotionSerialize,
  serializeWithContext: promotionSerializeWithContext,
  deserialize: promotionDeserialize,
  deserializeWithContext: promotionDeserializeWithContext,
  validateFields: promotionValidateFields,
  hasShape: promotionHasShape,
  is: promotionIs,
  fromFormData: promotionFromFormData,
  createForm: promotionCreateForm
} as const;