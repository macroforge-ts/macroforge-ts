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

export type NextStep = /** @default */ 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation';

export function nextStepDefaultValue#0#0(): NextStep {
    return 'InitialContact';
}

export function nextStepSerialize(value: NextStep): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(nextStepSerializeWithContext(value, ctx));
}
export function nextStepSerializeWithContext(value: NextStep, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function nextStepDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: NextStep } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = nextStepDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "NextStep.deserialize: root cannot be a forward reference"
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
export function nextStepDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): NextStep | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as NextStep | __mf_PendingRef;
    }
    const allowedValues = [
        "'InitialContact', 'Qualified', 'Estimate', 'Negotiation'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"NextStep"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as NextStep;
}
export function nextStepIs(value: unknown): value is NextStep {
    const allowedValues = [
        "'InitialContact', 'Qualified', 'Estimate', 'Negotiation'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type NextStepInitialContactErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type NextStepQualifiedErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type NextStepEstimateErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type NextStepNegotiationErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type NextStepInitialContactTainted = {
};
export type NextStepQualifiedTainted = {
};
export type NextStepEstimateTainted = {
};
export type NextStepNegotiationTainted = {
};
export type NextStepErrors = ({
    _value: "InitialContact";
} & NextStepInitialContactErrors) | ({
    _value: "Qualified";
} & NextStepQualifiedErrors) | ({
    _value: "Estimate";
} & NextStepEstimateErrors) | ({
    _value: "Negotiation";
} & NextStepNegotiationErrors);
export type NextStepTainted = ({
    _value: "InitialContact";
} & NextStepInitialContactTainted) | ({
    _value: "Qualified";
} & NextStepQualifiedTainted) | ({
    _value: "Estimate";
} & NextStepEstimateTainted) | ({
    _value: "Negotiation";
} & NextStepNegotiationTainted);
export interface NextStepInitialContactFieldControllers {
}
export interface NextStepQualifiedFieldControllers {
}
export interface NextStepEstimateFieldControllers {
}
export interface NextStepNegotiationFieldControllers {
}
export interface NextStepGigaform {
    readonly currentVariant: "InitialContact" | "Qualified" | "Estimate" | "Negotiation";
    readonly data: NextStep;
    readonly errors: NextStepErrors;
    readonly tainted: NextStepTainted;
    readonly variants: NextStepVariantFields;
    switchVariant(variant: "InitialContact" | "Qualified" | "Estimate" | "Negotiation"): void;
    validate(): Exit<NextStep, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<NextStep>): void;
}
export interface NextStepVariantFields {
    readonly InitialContact: {
        readonly fields: NextStepInitialContactFieldControllers;
    };
    readonly Qualified: {
        readonly fields: NextStepQualifiedFieldControllers;
    };
    readonly Estimate: {
        readonly fields: NextStepEstimateFieldControllers;
    };
    readonly Negotiation: {
        readonly fields: NextStepNegotiationFieldControllers;
    };
}
function nextStepGetDefaultForVariant(variant: string): NextStep {
    if (variant === "InitialContact") {
        return "InitialContact" as NextStep;
    }
    if (variant === "Qualified") {
        return "Qualified" as NextStep;
    }
    if (variant === "Estimate") {
        return "Estimate" as NextStep;
    }
    if (variant === "Negotiation") {
        return "Negotiation" as NextStep;
    }
    return "InitialContact" as NextStep;
}
export function nextStepCreateForm(initial: NextStep): NextStepGigaform {
    const initialVariant: "InitialContact" | "Qualified" | "Estimate" | "Negotiation" = (initial as "InitialContact" | "Qualified" | "Estimate" | "Negotiation") ?? "InitialContact";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "nextStepGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as NextStepErrors);
    let tainted = $state<$MfPh10>({} as NextStepTainted);
    const variants = {} as NextStepVariantFields;
    variants[__expr__] = {
        fields: {} as NextStepInitialContactFieldControllers
    };
    variants[__expr__] = {
        fields: {} as NextStepQualifiedFieldControllers
    };
    variants[__expr__] = {
        fields: {} as NextStepEstimateFieldControllers
    };
    variants[__expr__] = {
        fields: {} as NextStepNegotiationFieldControllers
    };
    function switchVariant(variant: "InitialContact" | "Qualified" | "Estimate" | "Negotiation"): void {
        currentVariant = variant;
        data = "nextStepGetDefaultForVariant"(variant);
        errors = {} as NextStepErrors;
        tainted = {} as NextStepTainted;
    }
    function validate(): Exit<NextStep, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(nextStepDeserialize(data));
    }
    function reset(overrides: Partial<NextStep>): void {
        data = overrides ? overrides as typeof data : nextStepGetDefaultForVariant(currentVariant);
        errors = {} as NextStepErrors;
        tainted = {} as NextStepTainted;
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
export function nextStepFromFormData(formData: FormData): Exit<NextStep, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "InitialContact" | "Qualified" | "Estimate" | "Negotiation" | null;
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
    return toExit(nextStepDeserialize(obj));
}

export const NextStep = {
  serialize: nextStepSerialize,
  serializeWithContext: nextStepSerializeWithContext,
  deserialize: nextStepDeserialize,
  deserializeWithContext: nextStepDeserializeWithContext,
  is: nextStepIs,
  createForm: nextStepCreateForm,
  fromFormData: nextStepFromFormData
} as const;