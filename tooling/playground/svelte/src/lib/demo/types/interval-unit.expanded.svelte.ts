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

export type IntervalUnit = /** @default */ 'Day' | 'Week' | 'Month' | 'Year';

export function intervalUnitDefaultValue#0#0(): IntervalUnit {
    return 'Day';
}

export function intervalUnitSerialize(value: IntervalUnit): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(intervalUnitSerializeWithContext(value, ctx));
}
export function intervalUnitSerializeWithContext(value: IntervalUnit, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function intervalUnitDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: IntervalUnit } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = intervalUnitDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "IntervalUnit.deserialize: root cannot be a forward reference"
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
export function intervalUnitDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): IntervalUnit | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as IntervalUnit | __mf_PendingRef;
    }
    const allowedValues = [
        "'Day', 'Week', 'Month', 'Year'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"IntervalUnit"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as IntervalUnit;
}
export function intervalUnitIs(value: unknown): value is IntervalUnit {
    const allowedValues = [
        "'Day', 'Week', 'Month', 'Year'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type IntervalUnitDayErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type IntervalUnitWeekErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type IntervalUnitMonthErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type IntervalUnitYearErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type IntervalUnitDayTainted = {
};
export type IntervalUnitWeekTainted = {
};
export type IntervalUnitMonthTainted = {
};
export type IntervalUnitYearTainted = {
};
export type IntervalUnitErrors = ({
    _value: "Day";
} & IntervalUnitDayErrors) | ({
    _value: "Week";
} & IntervalUnitWeekErrors) | ({
    _value: "Month";
} & IntervalUnitMonthErrors) | ({
    _value: "Year";
} & IntervalUnitYearErrors);
export type IntervalUnitTainted = ({
    _value: "Day";
} & IntervalUnitDayTainted) | ({
    _value: "Week";
} & IntervalUnitWeekTainted) | ({
    _value: "Month";
} & IntervalUnitMonthTainted) | ({
    _value: "Year";
} & IntervalUnitYearTainted);
export interface IntervalUnitDayFieldControllers {
}
export interface IntervalUnitWeekFieldControllers {
}
export interface IntervalUnitMonthFieldControllers {
}
export interface IntervalUnitYearFieldControllers {
}
export interface IntervalUnitGigaform {
    readonly currentVariant: "Day" | "Week" | "Month" | "Year";
    readonly data: IntervalUnit;
    readonly errors: IntervalUnitErrors;
    readonly tainted: IntervalUnitTainted;
    readonly variants: IntervalUnitVariantFields;
    switchVariant(variant: "Day" | "Week" | "Month" | "Year"): void;
    validate(): Exit<IntervalUnit, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<IntervalUnit>): void;
}
export interface IntervalUnitVariantFields {
    readonly Day: {
        readonly fields: IntervalUnitDayFieldControllers;
    };
    readonly Week: {
        readonly fields: IntervalUnitWeekFieldControllers;
    };
    readonly Month: {
        readonly fields: IntervalUnitMonthFieldControllers;
    };
    readonly Year: {
        readonly fields: IntervalUnitYearFieldControllers;
    };
}
function intervalUnitGetDefaultForVariant(variant: string): IntervalUnit {
    if (variant === "Day") {
        return "Day" as IntervalUnit;
    }
    if (variant === "Week") {
        return "Week" as IntervalUnit;
    }
    if (variant === "Month") {
        return "Month" as IntervalUnit;
    }
    if (variant === "Year") {
        return "Year" as IntervalUnit;
    }
    return "Day" as IntervalUnit;
}
export function intervalUnitCreateForm(initial: IntervalUnit): IntervalUnitGigaform {
    const initialVariant: "Day" | "Week" | "Month" | "Year" = (initial as "Day" | "Week" | "Month" | "Year") ?? "Day";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "intervalUnitGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as IntervalUnitErrors);
    let tainted = $state<$MfPh10>({} as IntervalUnitTainted);
    const variants = {} as IntervalUnitVariantFields;
    variants[__expr__] = {
        fields: {} as IntervalUnitDayFieldControllers
    };
    variants[__expr__] = {
        fields: {} as IntervalUnitWeekFieldControllers
    };
    variants[__expr__] = {
        fields: {} as IntervalUnitMonthFieldControllers
    };
    variants[__expr__] = {
        fields: {} as IntervalUnitYearFieldControllers
    };
    function switchVariant(variant: "Day" | "Week" | "Month" | "Year"): void {
        currentVariant = variant;
        data = "intervalUnitGetDefaultForVariant"(variant);
        errors = {} as IntervalUnitErrors;
        tainted = {} as IntervalUnitTainted;
    }
    function validate(): Exit<IntervalUnit, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(intervalUnitDeserialize(data));
    }
    function reset(overrides: Partial<IntervalUnit>): void {
        data = overrides ? overrides as typeof data : intervalUnitGetDefaultForVariant(currentVariant);
        errors = {} as IntervalUnitErrors;
        tainted = {} as IntervalUnitTainted;
    }
    return {
        get currentVariant () {
            return currentVariant;
        },
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
        variants,
        switchVariant,
        validate,
        reset
    };
}
export function intervalUnitFromFormData(formData: FormData): Exit<IntervalUnit, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Day" | "Week" | "Month" | "Year" | null;
    if (!discriminant) {
        return toExit({
            success: false,
            errors: [
                {
                    field: `${"_value"}`,
                    message: "Missing discriminant field"
                }
            ]
        });
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    return toExit(intervalUnitDeserialize(obj));
}

export const IntervalUnit = {
  serialize: intervalUnitSerialize,
  serializeWithContext: intervalUnitSerializeWithContext,
  deserialize: intervalUnitDeserialize,
  deserializeWithContext: intervalUnitDeserializeWithContext,
  is: intervalUnitIs,
  createForm: intervalUnitCreateForm,
  fromFormData: intervalUnitFromFormData
} as const;