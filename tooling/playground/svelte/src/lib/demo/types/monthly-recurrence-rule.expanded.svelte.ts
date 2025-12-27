import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface MonthlyRecurrenceRule {
    quantityOfMonths: number;
    day: number;
    
    name: string;
}

export function monthlyRecurrenceRuleDefaultValue(): MonthlyRecurrenceRule {
    return {
        quantityOfMonths: 0,
        day: 0,
        name: ""
    } as MonthlyRecurrenceRule;
}

export function monthlyRecurrenceRuleSerialize(value: MonthlyRecurrenceRule): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(monthlyRecurrenceRuleSerializeWithContext(value, ctx));
}
export function monthlyRecurrenceRuleSerializeWithContext(value: MonthlyRecurrenceRule, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"MonthlyRecurrenceRule"}`,
        __id
    };
    result[`${"quantityOfMonths"}`] = value.quantityOfMonths;
    result[`${"day"}`] = value.day;
    result[`${"name"}`] = value.name;
    return result;
}

export function monthlyRecurrenceRuleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: MonthlyRecurrenceRule } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = monthlyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "MonthlyRecurrenceRule.deserialize: root cannot be a forward reference"
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
export function monthlyRecurrenceRuleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): MonthlyRecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"MonthlyRecurrenceRule"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"quantityOfMonths"}` in obj)) {
        errors.push({
            field: `${"quantityOfMonths"}`,
            message: "missing required field"
        });
    }
    if (!(`${"day"}` in obj)) {
        errors.push({
            field: `${"day"}`,
            message: "missing required field"
        });
    }
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
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
        const __raw_quantityOfMonths = obj[`${"quantityOfMonths"}`] as number;
        instance.quantityOfMonths = __raw_quantityOfMonths;
    }
    {
        const __raw_day = obj[`${"day"}`] as number;
        instance.day = __raw_day;
    }
    {
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "MonthlyRecurrenceRule.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as MonthlyRecurrenceRule;
}
export function monthlyRecurrenceRuleValidateField<K extends keyof MonthlyRecurrenceRule>(_field: K, _value: MonthlyRecurrenceRule[K]): Array<{
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
                message: "MonthlyRecurrenceRule.name must not be empty"
            });
        }
    }
    return errors;
}
export function monthlyRecurrenceRuleValidateFields(_partial: Partial<MonthlyRecurrenceRule>): Array<{
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
                message: "MonthlyRecurrenceRule.name must not be empty"
            });
        }
    }
    return errors;
}
export function monthlyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"quantityOfMonths" in o && "day" in o && "name" in o';
}
export function monthlyRecurrenceRuleIs(obj: unknown): obj is MonthlyRecurrenceRule {
    if (!monthlyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = monthlyRecurrenceRuleDeserialize(obj);
    return result.success;
}

export const MonthlyRecurrenceRule = {
  defaultValue: monthlyRecurrenceRuleDefaultValue,
  serialize: monthlyRecurrenceRuleSerialize,
  serializeWithContext: monthlyRecurrenceRuleSerializeWithContext,
  deserialize: monthlyRecurrenceRuleDeserialize,
  deserializeWithContext: monthlyRecurrenceRuleDeserializeWithContext,
  validateFields: monthlyRecurrenceRuleValidateFields,
  hasShape: monthlyRecurrenceRuleHasShape,
  is: monthlyRecurrenceRuleIs
} as const;