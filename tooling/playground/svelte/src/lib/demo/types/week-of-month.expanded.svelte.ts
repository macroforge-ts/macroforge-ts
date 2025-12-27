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

export type WeekOfMonth = /** @default */ 'First' | 'Second' | 'Third' | 'Fourth' | 'Last';

export function weekOfMonthDefaultValue#0#0(): WeekOfMonth {
    return 'First';
}

export function weekOfMonthSerialize#0#0(value: WeekOfMonth): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(weekOfMonthSerializeWithContext(value, ctx));
}
export function weekOfMonthSerializeWithContext#0#0(value: WeekOfMonth, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function weekOfMonthDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: WeekOfMonth } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = weekOfMonthDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "WeekOfMonth.deserialize: root cannot be a forward reference"
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
export function weekOfMonthDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): WeekOfMonth | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as WeekOfMonth | __mf_PendingRef;
    }
    const allowedValues = [
        "'First', 'Second', 'Third', 'Fourth', 'Last'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"WeekOfMonth"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as WeekOfMonth;
}
export function weekOfMonthIs(value: unknown): value is WeekOfMonth {
    const allowedValues = [
        "'First', 'Second', 'Third', 'Fourth', 'Last'"
    ] as const;
    return allowedValues.includes(value as any);
}

function weekOfMonthGetDefaultForVariant(variant: string): WeekOfMonth {
    if (variant === `${"First"}`) {
        return `${"First"}` as WeekOfMonth;
    }
    if (variant === `${"Second"}`) {
        return `${"Second"}` as WeekOfMonth;
    }
    if (variant === `${"Third"}`) {
        return `${"Third"}` as WeekOfMonth;
    }
    if (variant === `${"Fourth"}`) {
        return `${"Fourth"}` as WeekOfMonth;
    }
    if (variant === `${"Last"}`) {
        return `${"Last"}` as WeekOfMonth;
    }
    return `${"First"}` as WeekOfMonth;
}
export function weekOfMonthCreateForm(initial: WeekOfMonth): WeekOfMonthGigaform {
    const initialVariant: "First" | "Second" | "Third" | "Fourth" | "Last" = '(initial as "First" | "Second" | "Third" | "Fourth" | "Last") ?? "First"';
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "weekOfMonthGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as WeekOfMonthErrors);
    let tainted = $state<$MfPh10>({} as WeekOfMonthTainted);
    const variants = {} as WeekOfMonthVariantFields;
    variants[First] = {
        fields: {} as WeekOfMonthFirstFieldControllers
    };
    variants[Second] = {
        fields: {} as WeekOfMonthSecondFieldControllers
    };
    variants[Third] = {
        fields: {} as WeekOfMonthThirdFieldControllers
    };
    variants[Fourth] = {
        fields: {} as WeekOfMonthFourthFieldControllers
    };
    variants[Last] = {
        fields: {} as WeekOfMonthLastFieldControllers
    };
    function switchVariant(variant: "First" | "Second" | "Third" | "Fourth" | "Last"): void {
        currentVariant = variant;
        data = "weekOfMonthGetDefaultForVariant"(variant);
        errors = {} as WeekOfMonthErrors;
        tainted = {} as WeekOfMonthTainted;
    }
    function validate(): Exit<WeekOfMonth, Array<{
        field: string;
        message: string;
    }>> {
        return toExit("weekOfMonthDeserialize(data)");
    }
    function reset(overrides: Partial<WeekOfMonth>): void {
        data = "overrides ? overrides as typeof data : weekOfMonthGetDefaultForVariant(currentVariant)";
        errors = {} as WeekOfMonthErrors;
        tainted = {} as WeekOfMonthTainted;
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
export function weekOfMonthFromFormData(formData: FormData): Exit<WeekOfMonth, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "First" | "Second" | "Third" | "Fourth" | "Last" | null;
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
    return toExit("weekOfMonthDeserialize(obj)");
}
export type $MfPh0 = $MfPh1;
export type $MfPh2 = $MfPh3;
export interface WeekOfMonthFirstFieldControllers {
}
export interface WeekOfMonthSecondFieldControllers {
}
export interface WeekOfMonthThirdFieldControllers {
}
export interface WeekOfMonthFourthFieldControllers {
}
export interface WeekOfMonthLastFieldControllers {
}
export interface $MfPh4 {
    readonly currentVariant: "First" | "Second" | "Third" | "Fourth" | "Last";
    readonly data: WeekOfMonth;
    readonly errors: WeekOfMonthErrors;
    readonly tainted: WeekOfMonthTainted;
    readonly variants: WeekOfMonthVariantFields;
    switchVariant(variant: "First" | "Second" | "Third" | "Fourth" | "Last"): void;
    validate(): Exit<WeekOfMonth, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<WeekOfMonth>): void;
}
export interface $MfPh13 {
}
export type WeekOfMonthFirstErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekOfMonthSecondErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekOfMonthThirdErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekOfMonthFourthErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type WeekOfMonthLastErrors = {
    _errors: __gf_Option<Array<string>>;
};
 };  };  };  };  }; export type WeekOfMonthFirstTainted = {
};
export type WeekOfMonthSecondTainted = {
};
export type WeekOfMonthThirdTainted = {
};
export type WeekOfMonthFourthTainted = {
};
export type WeekOfMonthLastTainted = {
};
 };  };  };  };  };

export const WeekOfMonth = {
  deserialize: weekOfMonthDeserialize,
  deserializeWithContext: weekOfMonthDeserializeWithContext,
  is: weekOfMonthIs,
  createForm: weekOfMonthCreateForm,
  fromFormData: weekOfMonthFromFormData
} as const;