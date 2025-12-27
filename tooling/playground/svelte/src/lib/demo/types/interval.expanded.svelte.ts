import { dailyRecurrenceRuleDefaultValue } from "./daily-recurrence-rule.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { dailyRecurrenceRuleDeserializeWithContext } from "./daily-recurrence-rule.svelte";
import { monthlyRecurrenceRuleDeserializeWithContext } from "./monthly-recurrence-rule.svelte";
import { weeklyRecurrenceRuleDeserializeWithContext } from "./weekly-recurrence-rule.svelte";
import { yearlyRecurrenceRuleDeserializeWithContext } from "./yearly-recurrence-rule.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { DailyRecurrenceRule } from './daily-recurrence-rule.svelte';
import type { MonthlyRecurrenceRule } from './monthly-recurrence-rule.svelte';
import type { WeeklyRecurrenceRule } from './weekly-recurrence-rule.svelte';
import type { YearlyRecurrenceRule } from './yearly-recurrence-rule.svelte';


export type Interval =
    | /** @default */ DailyRecurrenceRule
    | WeeklyRecurrenceRule
    | MonthlyRecurrenceRule
    | YearlyRecurrenceRule;

export function intervalDefaultValue#0#0(): Interval {
    return dailyRecurrenceRuleDefaultValue();
}

export function intervalSerialize#0#0(value: Interval): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(intervalSerializeWithContext(value, ctx));
}
export function intervalSerializeWithContext#0#0(value: Interval, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function intervalDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Interval } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = intervalDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Interval.deserialize: root cannot be a forward reference"
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
export function intervalDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Interval | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Interval | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"DailyRecurrenceRule"}`) {
            return dailyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
        }
        if (__typeName === `${"WeeklyRecurrenceRule"}`) {
            return weeklyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
        }
        if (__typeName === `${"MonthlyRecurrenceRule"}`) {
            return monthlyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
        }
        if (__typeName === `${"YearlyRecurrenceRule"}`) {
            return yearlyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"Interval"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function intervalIs(value: unknown): value is Interval {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "DailyRecurrenceRule" || __typeName === "WeeklyRecurrenceRule" || __typeName === "MonthlyRecurrenceRule" || __typeName === "YearlyRecurrenceRule"') return true;
    }
    return false;
}
     }

export const Interval = {
  deserialize: intervalDeserialize,
  deserializeWithContext: intervalDeserializeWithContext,
  is: intervalIs
} as const;