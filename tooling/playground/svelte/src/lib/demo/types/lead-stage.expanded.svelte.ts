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

export type LeadStage =
    | /** @default */ 'Open'
    | 'InitialContact'
    | 'Qualified'
    | 'Estimate'
    | 'Negotiation';

export function leadStageDefaultValue#0#0(): LeadStage {
    return 'Open';
}

export function leadStageSerialize(value: LeadStage): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(leadStageSerializeWithContext(value, ctx));
}
export function leadStageSerializeWithContext(value: LeadStage, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function leadStageDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: LeadStage } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = leadStageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "LeadStage.deserialize: root cannot be a forward reference"
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
export function leadStageDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): LeadStage | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as LeadStage | __mf_PendingRef;
    }
    const allowedValues = [
        "'Open', 'InitialContact', 'Qualified', 'Estimate', 'Negotiation'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"LeadStage"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as LeadStage;
}
export function leadStageIs(value: unknown): value is LeadStage {
    const allowedValues = [
        "'Open', 'InitialContact', 'Qualified', 'Estimate', 'Negotiation'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type LeadStageOpenErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type LeadStageInitialContactErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type LeadStageQualifiedErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type LeadStageEstimateErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type LeadStageNegotiationErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type LeadStageOpenTainted = {
};
export type LeadStageInitialContactTainted = {
};
export type LeadStageQualifiedTainted = {
};
export type LeadStageEstimateTainted = {
};
export type LeadStageNegotiationTainted = {
};
export type LeadStageErrors = ({
    _value: "Open";
} & LeadStageOpenErrors) | ({
    _value: "InitialContact";
} & LeadStageInitialContactErrors) | ({
    _value: "Qualified";
} & LeadStageQualifiedErrors) | ({
    _value: "Estimate";
} & LeadStageEstimateErrors) | ({
    _value: "Negotiation";
} & LeadStageNegotiationErrors);
export type LeadStageTainted = ({
    _value: "Open";
} & LeadStageOpenTainted) | ({
    _value: "InitialContact";
} & LeadStageInitialContactTainted) | ({
    _value: "Qualified";
} & LeadStageQualifiedTainted) | ({
    _value: "Estimate";
} & LeadStageEstimateTainted) | ({
    _value: "Negotiation";
} & LeadStageNegotiationTainted);
export interface LeadStageOpenFieldControllers {
}
export interface LeadStageInitialContactFieldControllers {
}
export interface LeadStageQualifiedFieldControllers {
}
export interface LeadStageEstimateFieldControllers {
}
export interface LeadStageNegotiationFieldControllers {
}
export interface LeadStageGigaform {
    readonly currentVariant: "Open" | "InitialContact" | "Qualified" | "Estimate" | "Negotiation";
    readonly data: LeadStage;
    readonly errors: LeadStageErrors;
    readonly tainted: LeadStageTainted;
    readonly variants: LeadStageVariantFields;
    switchVariant(variant: "Open" | "InitialContact" | "Qualified" | "Estimate" | "Negotiation"): void;
    validate(): Exit<LeadStage, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<LeadStage>): void;
}
export interface LeadStageVariantFields {
    readonly Open: {
        readonly fields: LeadStageOpenFieldControllers;
    };
    readonly InitialContact: {
        readonly fields: LeadStageInitialContactFieldControllers;
    };
    readonly Qualified: {
        readonly fields: LeadStageQualifiedFieldControllers;
    };
    readonly Estimate: {
        readonly fields: LeadStageEstimateFieldControllers;
    };
    readonly Negotiation: {
        readonly fields: LeadStageNegotiationFieldControllers;
    };
}
function leadStageGetDefaultForVariant(variant: string): LeadStage {
    if (variant === "Open") {
        return "Open" as LeadStage;
    }
    if (variant === "InitialContact") {
        return "InitialContact" as LeadStage;
    }
    if (variant === "Qualified") {
        return "Qualified" as LeadStage;
    }
    if (variant === "Estimate") {
        return "Estimate" as LeadStage;
    }
    if (variant === "Negotiation") {
        return "Negotiation" as LeadStage;
    }
    return "Open" as LeadStage;
}
export function leadStageCreateForm(initial: LeadStage): LeadStageGigaform {
    const initialVariant: "Open" | "InitialContact" | "Qualified" | "Estimate" | "Negotiation" = (initial as "Open" | "InitialContact" | "Qualified" | "Estimate" | "Negotiation") ?? "Open";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "leadStageGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as LeadStageErrors);
    let tainted = $state<$MfPh10>({} as LeadStageTainted);
    const variants = {} as LeadStageVariantFields;
    variants[__expr__] = {
        fields: {} as LeadStageOpenFieldControllers
    };
    variants[__expr__] = {
        fields: {} as LeadStageInitialContactFieldControllers
    };
    variants[__expr__] = {
        fields: {} as LeadStageQualifiedFieldControllers
    };
    variants[__expr__] = {
        fields: {} as LeadStageEstimateFieldControllers
    };
    variants[__expr__] = {
        fields: {} as LeadStageNegotiationFieldControllers
    };
    function switchVariant(variant: "Open" | "InitialContact" | "Qualified" | "Estimate" | "Negotiation"): void {
        currentVariant = variant;
        data = "leadStageGetDefaultForVariant"(variant);
        errors = {} as LeadStageErrors;
        tainted = {} as LeadStageTainted;
    }
    function validate(): Exit<LeadStage, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(leadStageDeserialize(data));
    }
    function reset(overrides: Partial<LeadStage>): void {
        data = overrides ? overrides as typeof data : leadStageGetDefaultForVariant(currentVariant);
        errors = {} as LeadStageErrors;
        tainted = {} as LeadStageTainted;
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
export function leadStageFromFormData(formData: FormData): Exit<LeadStage, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Open" | "InitialContact" | "Qualified" | "Estimate" | "Negotiation" | null;
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
    return toExit(leadStageDeserialize(obj));
}

export const LeadStage = {
  serialize: leadStageSerialize,
  serializeWithContext: leadStageSerializeWithContext,
  deserialize: leadStageDeserialize,
  deserializeWithContext: leadStageDeserializeWithContext,
  is: leadStageIs,
  createForm: leadStageCreateForm,
  fromFormData: leadStageFromFormData
} as const;