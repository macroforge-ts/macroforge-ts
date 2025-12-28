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

export type OrderStage = /** @default */ 'Estimate' | 'Active' | 'Invoice';

export function orderStageDefaultValue#0#0(): OrderStage {
    return 'Estimate';
}

export function orderStageSerialize(value: OrderStage): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(orderStageSerializeWithContext(value, ctx));
}
export function orderStageSerializeWithContext(value: OrderStage, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function orderStageDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: OrderStage } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = orderStageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "OrderStage.deserialize: root cannot be a forward reference"
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
export function orderStageDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): OrderStage | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as OrderStage | __mf_PendingRef;
    }
    const allowedValues = [
        "'Estimate', 'Active', 'Invoice'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"OrderStage"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as OrderStage;
}
export function orderStageIs(value: unknown): value is OrderStage {
    const allowedValues = [
        "'Estimate', 'Active', 'Invoice'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type OrderStageEstimateErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type OrderStageActiveErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type OrderStageInvoiceErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type OrderStageEstimateTainted = {
};
export type OrderStageActiveTainted = {
};
export type OrderStageInvoiceTainted = {
};
export type OrderStageErrors = ({
    _value: "Estimate";
} & OrderStageEstimateErrors) | ({
    _value: "Active";
} & OrderStageActiveErrors) | ({
    _value: "Invoice";
} & OrderStageInvoiceErrors);
export type OrderStageTainted = ({
    _value: "Estimate";
} & OrderStageEstimateTainted) | ({
    _value: "Active";
} & OrderStageActiveTainted) | ({
    _value: "Invoice";
} & OrderStageInvoiceTainted);
export interface OrderStageEstimateFieldControllers {
}
export interface OrderStageActiveFieldControllers {
}
export interface OrderStageInvoiceFieldControllers {
}
export interface OrderStageGigaform {
    readonly currentVariant: "Estimate" | "Active" | "Invoice";
    readonly data: OrderStage;
    readonly errors: OrderStageErrors;
    readonly tainted: OrderStageTainted;
    readonly variants: OrderStageVariantFields;
    switchVariant(variant: "Estimate" | "Active" | "Invoice"): void;
    validate(): Exit<OrderStage, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<OrderStage>): void;
}
export interface OrderStageVariantFields {
    readonly Estimate: {
        readonly fields: OrderStageEstimateFieldControllers;
    };
    readonly Active: {
        readonly fields: OrderStageActiveFieldControllers;
    };
    readonly Invoice: {
        readonly fields: OrderStageInvoiceFieldControllers;
    };
}
function orderStageGetDefaultForVariant(variant: string): OrderStage {
    if (variant === "Estimate") {
        return "Estimate" as OrderStage;
    }
    if (variant === "Active") {
        return "Active" as OrderStage;
    }
    if (variant === "Invoice") {
        return "Invoice" as OrderStage;
    }
    return "Estimate" as OrderStage;
}
export function orderStageCreateForm(initial: OrderStage): OrderStageGigaform {
    const initialVariant: "Estimate" | "Active" | "Invoice" = (initial as "Estimate" | "Active" | "Invoice") ?? "Estimate";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "orderStageGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as OrderStageErrors);
    let tainted = $state<$MfPh10>({} as OrderStageTainted);
    const variants = {} as OrderStageVariantFields;
    variants[__expr__] = {
        fields: {} as OrderStageEstimateFieldControllers
    };
    variants[__expr__] = {
        fields: {} as OrderStageActiveFieldControllers
    };
    variants[__expr__] = {
        fields: {} as OrderStageInvoiceFieldControllers
    };
    function switchVariant(variant: "Estimate" | "Active" | "Invoice"): void {
        currentVariant = variant;
        data = "orderStageGetDefaultForVariant"(variant);
        errors = {} as OrderStageErrors;
        tainted = {} as OrderStageTainted;
    }
    function validate(): Exit<OrderStage, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(orderStageDeserialize(data));
    }
    function reset(overrides: Partial<OrderStage>): void {
        data = overrides ? overrides as typeof data : orderStageGetDefaultForVariant(currentVariant);
        errors = {} as OrderStageErrors;
        tainted = {} as OrderStageTainted;
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
export function orderStageFromFormData(formData: FormData): Exit<OrderStage, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Estimate" | "Active" | "Invoice" | null;
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
    return toExit(orderStageDeserialize(obj));
}

export const OrderStage = {
  serialize: orderStageSerialize,
  serializeWithContext: orderStageSerializeWithContext,
  deserialize: orderStageDeserialize,
  deserializeWithContext: orderStageDeserializeWithContext,
  is: orderStageIs,
  createForm: orderStageCreateForm,
  fromFormData: orderStageFromFormData
} as const;