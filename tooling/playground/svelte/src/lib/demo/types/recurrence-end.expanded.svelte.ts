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

export type RecurrenceEnd = /** @default(0) */ number | string;

export function recurrenceEndDefaultValue#0#0(): RecurrenceEnd {
    return 0;
}

export function recurrenceEndSerialize(value: RecurrenceEnd): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(recurrenceEndSerializeWithContext(value, ctx));
}
export function recurrenceEndSerializeWithContext(value: RecurrenceEnd, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function recurrenceEndDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: RecurrenceEnd } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = recurrenceEndDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "RecurrenceEnd.deserialize: root cannot be a forward reference"
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
export function recurrenceEndDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): RecurrenceEnd | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RecurrenceEnd | __mf_PendingRef;
    }
    if (typeof value === "number") {
        return value as RecurrenceEnd;
    }
    if (typeof value === "string") {
        return value as RecurrenceEnd;
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"RecurrenceEnd"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function recurrenceEndIs(value: unknown): value is RecurrenceEnd {
    if (typeof value === "number") return true;
    if (typeof value === "string") return true;
    return false;
}

export type RecurrenceEndNumberErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type RecurrenceEndStringErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type RecurrenceEndNumberTainted = {
};
export type RecurrenceEndStringTainted = {
};
export type RecurrenceEndErrors = ({
    _type: "number";
} & RecurrenceEndNumberErrors) | ({
    _type: "string";
} & RecurrenceEndStringErrors);
export type RecurrenceEndTainted = ({
    _type: "number";
} & RecurrenceEndNumberTainted) | ({
    _type: "string";
} & RecurrenceEndStringTainted);
export interface RecurrenceEndNumberFieldControllers {
}
export interface RecurrenceEndStringFieldControllers {
}
export interface RecurrenceEndGigaform {
    readonly currentVariant: "number" | "string";
    readonly data: RecurrenceEnd;
    readonly errors: RecurrenceEndErrors;
    readonly tainted: RecurrenceEndTainted;
    readonly variants: RecurrenceEndVariantFields;
    switchVariant(variant: "number" | "string"): void;
    validate(): Exit<RecurrenceEnd, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<RecurrenceEnd>): void;
}
export interface RecurrenceEndVariantFields {
    readonly number: {
        readonly fields: RecurrenceEndNumberFieldControllers;
    };
    readonly string: {
        readonly fields: RecurrenceEndStringFieldControllers;
    };
}
function recurrenceEndGetDefaultForVariant(variant: string): RecurrenceEnd {
    if (variant === "number") {
        return 0 as RecurrenceEnd;
    }
    if (variant === "string") {
        return "" as RecurrenceEnd;
    }
    return 0 as RecurrenceEnd;
}
export function recurrenceEndCreateForm(initial: RecurrenceEnd): RecurrenceEndGigaform {
    const initialVariant: "number" | "string" = "number";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "recurrenceEndGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as RecurrenceEndErrors);
    let tainted = $state<$MfPh10>({} as RecurrenceEndTainted);
    const variants = {} as RecurrenceEndVariantFields;
    variants[__expr__] = {
        fields: {} as RecurrenceEndNumberFieldControllers
    };
    variants[__expr__] = {
        fields: {} as RecurrenceEndStringFieldControllers
    };
    function switchVariant(variant: "number" | "string"): void {
        currentVariant = variant;
        data = "recurrenceEndGetDefaultForVariant"(variant);
        errors = {} as RecurrenceEndErrors;
        tainted = {} as RecurrenceEndTainted;
    }
    function validate(): Exit<RecurrenceEnd, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(recurrenceEndDeserialize(data));
    }
    function reset(overrides: Partial<RecurrenceEnd>): void {
        data = overrides ? overrides as typeof data : recurrenceEndGetDefaultForVariant(currentVariant);
        errors = {} as RecurrenceEndErrors;
        tainted = {} as RecurrenceEndTainted;
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
export function recurrenceEndFromFormData(formData: FormData): Exit<RecurrenceEnd, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_type"}`) as "number" | "string" | null;
    if (!discriminant) {
        return toExit({
            success: false,
            errors: [
                {
                    field: `${"_type"}`,
                    message: "Missing discriminant field"
                }
            ]
        });
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    return toExit(recurrenceEndDeserialize(obj));
}

export const RecurrenceEnd = {
  serialize: recurrenceEndSerialize,
  serializeWithContext: recurrenceEndSerializeWithContext,
  deserialize: recurrenceEndDeserialize,
  deserializeWithContext: recurrenceEndDeserializeWithContext,
  is: recurrenceEndIs,
  createForm: recurrenceEndCreateForm,
  fromFormData: recurrenceEndFromFormData
} as const;