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

export type Weekday =
    | /** @default */ 'Monday'
    | 'Tuesday'
    | 'Wednesday'
    | 'Thursday'
    | 'Friday'
    | 'Saturday'
    | 'Sunday';

export function weekdayDefaultValue#0#0(): Weekday {
    return 'Monday';
}

export function weekdaySerialize(value: Weekday): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(weekdaySerializeWithContext(value, ctx));
}
export function weekdaySerializeWithContext(value: Weekday, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function weekdayDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Weekday } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = weekdayDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Weekday.deserialize: root cannot be a forward reference"
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
export function weekdayDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Weekday | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Weekday | __mf_PendingRef;
    }
    const allowedValues = [
        "'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Weekday"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Weekday;
}
export function weekdayIs(value: unknown): value is Weekday {
    const allowedValues = [
        "'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type WeekdayMondayErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekdayTuesdayErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekdayWednesdayErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekdayThursdayErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekdayFridayErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekdaySaturdayErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekdaySundayErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekdayMondayTainted = {
};
export type WeekdayTuesdayTainted = {
};
export type WeekdayWednesdayTainted = {
};
export type WeekdayThursdayTainted = {
};
export type WeekdayFridayTainted = {
};
export type WeekdaySaturdayTainted = {
};
export type WeekdaySundayTainted = {
};
export type WeekdayErrors = ({
    _value: "Monday";
} & WeekdayMondayErrors) | ({
    _value: "Tuesday";
} & WeekdayTuesdayErrors) | ({
    _value: "Wednesday";
} & WeekdayWednesdayErrors) | ({
    _value: "Thursday";
} & WeekdayThursdayErrors) | ({
    _value: "Friday";
} & WeekdayFridayErrors) | ({
    _value: "Saturday";
} & WeekdaySaturdayErrors) | ({
    _value: "Sunday";
} & WeekdaySundayErrors);
export type WeekdayTainted = ({
    _value: "Monday";
} & WeekdayMondayTainted) | ({
    _value: "Tuesday";
} & WeekdayTuesdayTainted) | ({
    _value: "Wednesday";
} & WeekdayWednesdayTainted) | ({
    _value: "Thursday";
} & WeekdayThursdayTainted) | ({
    _value: "Friday";
} & WeekdayFridayTainted) | ({
    _value: "Saturday";
} & WeekdaySaturdayTainted) | ({
    _value: "Sunday";
} & WeekdaySundayTainted);
export interface WeekdayMondayFieldControllers {
}
export interface WeekdayTuesdayFieldControllers {
}
export interface WeekdayWednesdayFieldControllers {
}
export interface WeekdayThursdayFieldControllers {
}
export interface WeekdayFridayFieldControllers {
}
export interface WeekdaySaturdayFieldControllers {
}
export interface WeekdaySundayFieldControllers {
}
export interface WeekdayGigaform {
    readonly currentVariant: "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" | "Saturday" | "Sunday";
    readonly data: Weekday;
    readonly errors: WeekdayErrors;
    readonly tainted: WeekdayTainted;
    readonly variants: WeekdayVariantFields;
    switchVariant(variant: "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" | "Saturday" | "Sunday"): void;
    validate(): Exit<Weekday, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Weekday>): void;
}
export interface WeekdayVariantFields {
    readonly Monday: {
        readonly fields: WeekdayMondayFieldControllers;
    };
    readonly Tuesday: {
        readonly fields: WeekdayTuesdayFieldControllers;
    };
    readonly Wednesday: {
        readonly fields: WeekdayWednesdayFieldControllers;
    };
    readonly Thursday: {
        readonly fields: WeekdayThursdayFieldControllers;
    };
    readonly Friday: {
        readonly fields: WeekdayFridayFieldControllers;
    };
    readonly Saturday: {
        readonly fields: WeekdaySaturdayFieldControllers;
    };
    readonly Sunday: {
        readonly fields: WeekdaySundayFieldControllers;
    };
}
function weekdayGetDefaultForVariant(variant: string): Weekday {
    if (variant === "Monday") {
        return "Monday" as Weekday;
    }
    if (variant === "Tuesday") {
        return "Tuesday" as Weekday;
    }
    if (variant === "Wednesday") {
        return "Wednesday" as Weekday;
    }
    if (variant === "Thursday") {
        return "Thursday" as Weekday;
    }
    if (variant === "Friday") {
        return "Friday" as Weekday;
    }
    if (variant === "Saturday") {
        return "Saturday" as Weekday;
    }
    if (variant === "Sunday") {
        return "Sunday" as Weekday;
    }
    return "Monday" as Weekday;
}
export function weekdayCreateForm(initial: Weekday): WeekdayGigaform {
    const initialVariant: "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" | "Saturday" | "Sunday" = (initial as "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" | "Saturday" | "Sunday") ?? "Monday";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "weekdayGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as WeekdayErrors);
    let tainted = $state<$MfPh10>({} as WeekdayTainted);
    const variants = {} as WeekdayVariantFields;
    variants[__expr__] = {
        fields: {} as WeekdayMondayFieldControllers
    };
    variants[__expr__] = {
        fields: {} as WeekdayTuesdayFieldControllers
    };
    variants[__expr__] = {
        fields: {} as WeekdayWednesdayFieldControllers
    };
    variants[__expr__] = {
        fields: {} as WeekdayThursdayFieldControllers
    };
    variants[__expr__] = {
        fields: {} as WeekdayFridayFieldControllers
    };
    variants[__expr__] = {
        fields: {} as WeekdaySaturdayFieldControllers
    };
    variants[__expr__] = {
        fields: {} as WeekdaySundayFieldControllers
    };
    function switchVariant(variant: "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" | "Saturday" | "Sunday"): void {
        currentVariant = variant;
        data = "weekdayGetDefaultForVariant"(variant);
        errors = {} as WeekdayErrors;
        tainted = {} as WeekdayTainted;
    }
    function validate(): Exit<Weekday, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(weekdayDeserialize(data));
    }
    function reset(overrides: Partial<Weekday>): void {
        data = overrides ? overrides as typeof data : weekdayGetDefaultForVariant(currentVariant);
        errors = {} as WeekdayErrors;
        tainted = {} as WeekdayTainted;
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
export function weekdayFromFormData(formData: FormData): Exit<Weekday, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" | "Saturday" | "Sunday" | null;
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
    return toExit(weekdayDeserialize(obj));
}

export const Weekday = {
  serialize: weekdaySerialize,
  serializeWithContext: weekdaySerializeWithContext,
  deserialize: weekdayDeserialize,
  deserializeWithContext: weekdayDeserializeWithContext,
  is: weekdayIs,
  createForm: weekdayCreateForm,
  fromFormData: weekdayFromFormData
} as const;