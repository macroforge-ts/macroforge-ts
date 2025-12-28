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

export type OverviewDisplay = /** @default */ 'Card' | 'Table';

export function overviewDisplayDefaultValue#0#0(): OverviewDisplay {
    return 'Card';
}

export function overviewDisplaySerialize(value: OverviewDisplay): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(overviewDisplaySerializeWithContext(value, ctx));
}
export function overviewDisplaySerializeWithContext(value: OverviewDisplay, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function overviewDisplayDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: OverviewDisplay } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = overviewDisplayDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "OverviewDisplay.deserialize: root cannot be a forward reference"
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
export function overviewDisplayDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): OverviewDisplay | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as OverviewDisplay | __mf_PendingRef;
    }
    const allowedValues = [
        "'Card', 'Table'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"OverviewDisplay"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as OverviewDisplay;
}
export function overviewDisplayIs(value: unknown): value is OverviewDisplay {
    const allowedValues = [
        "'Card', 'Table'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type OverviewDisplayCardErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type OverviewDisplayTableErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type OverviewDisplayCardTainted = {
};
export type OverviewDisplayTableTainted = {
};
export type OverviewDisplayErrors = ({
    _value: "Card";
} & OverviewDisplayCardErrors) | ({
    _value: "Table";
} & OverviewDisplayTableErrors);
export type OverviewDisplayTainted = ({
    _value: "Card";
} & OverviewDisplayCardTainted) | ({
    _value: "Table";
} & OverviewDisplayTableTainted);
export interface OverviewDisplayCardFieldControllers {
}
export interface OverviewDisplayTableFieldControllers {
}
export interface OverviewDisplayGigaform {
    readonly currentVariant: "Card" | "Table";
    readonly data: OverviewDisplay;
    readonly errors: OverviewDisplayErrors;
    readonly tainted: OverviewDisplayTainted;
    readonly variants: OverviewDisplayVariantFields;
    switchVariant(variant: "Card" | "Table"): void;
    validate(): Exit<OverviewDisplay, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<OverviewDisplay>): void;
}
export interface OverviewDisplayVariantFields {
    readonly Card: {
        readonly fields: OverviewDisplayCardFieldControllers;
    };
    readonly Table: {
        readonly fields: OverviewDisplayTableFieldControllers;
    };
}
function overviewDisplayGetDefaultForVariant(variant: string): OverviewDisplay {
    if (variant === "Card") {
        return "Card" as OverviewDisplay;
    }
    if (variant === "Table") {
        return "Table" as OverviewDisplay;
    }
    return "Card" as OverviewDisplay;
}
export function overviewDisplayCreateForm(initial: OverviewDisplay): OverviewDisplayGigaform {
    const initialVariant: "Card" | "Table" = (initial as "Card" | "Table") ?? "Card";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "overviewDisplayGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as OverviewDisplayErrors);
    let tainted = $state<$MfPh10>({} as OverviewDisplayTainted);
    const variants = {} as OverviewDisplayVariantFields;
    variants[__expr__] = {
        fields: {} as OverviewDisplayCardFieldControllers
    };
    variants[__expr__] = {
        fields: {} as OverviewDisplayTableFieldControllers
    };
    function switchVariant(variant: "Card" | "Table"): void {
        currentVariant = variant;
        data = "overviewDisplayGetDefaultForVariant"(variant);
        errors = {} as OverviewDisplayErrors;
        tainted = {} as OverviewDisplayTainted;
    }
    function validate(): Exit<OverviewDisplay, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(overviewDisplayDeserialize(data));
    }
    function reset(overrides: Partial<OverviewDisplay>): void {
        data = overrides ? overrides as typeof data : overviewDisplayGetDefaultForVariant(currentVariant);
        errors = {} as OverviewDisplayErrors;
        tainted = {} as OverviewDisplayTainted;
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
export function overviewDisplayFromFormData(formData: FormData): Exit<OverviewDisplay, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Card" | "Table" | null;
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
    return toExit(overviewDisplayDeserialize(obj));
}

export const OverviewDisplay = {
  serialize: overviewDisplaySerialize,
  serializeWithContext: overviewDisplaySerializeWithContext,
  deserialize: overviewDisplayDeserialize,
  deserializeWithContext: overviewDisplayDeserializeWithContext,
  is: overviewDisplayIs,
  createForm: overviewDisplayCreateForm,
  fromFormData: overviewDisplayFromFormData
} as const;