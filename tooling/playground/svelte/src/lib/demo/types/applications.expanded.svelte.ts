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

export type Applications =
    | /** @default */ 'Sales'
    | 'Accounting'
    | 'Errand'
    | 'HumanResources'
    | 'Logistics'
    | 'Marketing'
    | 'Website';

export function applicationsDefaultValue#0#0(): Applications {
    return 'Sales';
}

export function applicationsSerialize(value: Applications): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(applicationsSerializeWithContext(value, ctx));
}
export function applicationsSerializeWithContext(value: Applications, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function applicationsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Applications } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = applicationsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Applications.deserialize: root cannot be a forward reference"
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
export function applicationsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Applications | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Applications | __mf_PendingRef;
    }
    const allowedValues = [
        "'Sales', 'Accounting', 'Errand', 'HumanResources', 'Logistics', 'Marketing', 'Website'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Applications"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Applications;
}
export function applicationsIs(value: unknown): value is Applications {
    const allowedValues = [
        "'Sales', 'Accounting', 'Errand', 'HumanResources', 'Logistics', 'Marketing', 'Website'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type ApplicationsSalesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ApplicationsAccountingErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ApplicationsErrandErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ApplicationsHumanResourcesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ApplicationsLogisticsErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ApplicationsMarketingErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ApplicationsWebsiteErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ApplicationsSalesTainted = {
};
export type ApplicationsAccountingTainted = {
};
export type ApplicationsErrandTainted = {
};
export type ApplicationsHumanResourcesTainted = {
};
export type ApplicationsLogisticsTainted = {
};
export type ApplicationsMarketingTainted = {
};
export type ApplicationsWebsiteTainted = {
};
export type ApplicationsErrors = ({
    _value: "Sales";
} & ApplicationsSalesErrors) | ({
    _value: "Accounting";
} & ApplicationsAccountingErrors) | ({
    _value: "Errand";
} & ApplicationsErrandErrors) | ({
    _value: "HumanResources";
} & ApplicationsHumanResourcesErrors) | ({
    _value: "Logistics";
} & ApplicationsLogisticsErrors) | ({
    _value: "Marketing";
} & ApplicationsMarketingErrors) | ({
    _value: "Website";
} & ApplicationsWebsiteErrors);
export type ApplicationsTainted = ({
    _value: "Sales";
} & ApplicationsSalesTainted) | ({
    _value: "Accounting";
} & ApplicationsAccountingTainted) | ({
    _value: "Errand";
} & ApplicationsErrandTainted) | ({
    _value: "HumanResources";
} & ApplicationsHumanResourcesTainted) | ({
    _value: "Logistics";
} & ApplicationsLogisticsTainted) | ({
    _value: "Marketing";
} & ApplicationsMarketingTainted) | ({
    _value: "Website";
} & ApplicationsWebsiteTainted);
export interface ApplicationsSalesFieldControllers {
}
export interface ApplicationsAccountingFieldControllers {
}
export interface ApplicationsErrandFieldControllers {
}
export interface ApplicationsHumanResourcesFieldControllers {
}
export interface ApplicationsLogisticsFieldControllers {
}
export interface ApplicationsMarketingFieldControllers {
}
export interface ApplicationsWebsiteFieldControllers {
}
export interface ApplicationsGigaform {
    readonly currentVariant: "Sales" | "Accounting" | "Errand" | "HumanResources" | "Logistics" | "Marketing" | "Website";
    readonly data: Applications;
    readonly errors: ApplicationsErrors;
    readonly tainted: ApplicationsTainted;
    readonly variants: ApplicationsVariantFields;
    switchVariant(variant: "Sales" | "Accounting" | "Errand" | "HumanResources" | "Logistics" | "Marketing" | "Website"): void;
    validate(): Exit<Applications, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Applications>): void;
}
export interface ApplicationsVariantFields {
    readonly Sales: {
        readonly fields: ApplicationsSalesFieldControllers;
    };
    readonly Accounting: {
        readonly fields: ApplicationsAccountingFieldControllers;
    };
    readonly Errand: {
        readonly fields: ApplicationsErrandFieldControllers;
    };
    readonly HumanResources: {
        readonly fields: ApplicationsHumanResourcesFieldControllers;
    };
    readonly Logistics: {
        readonly fields: ApplicationsLogisticsFieldControllers;
    };
    readonly Marketing: {
        readonly fields: ApplicationsMarketingFieldControllers;
    };
    readonly Website: {
        readonly fields: ApplicationsWebsiteFieldControllers;
    };
}
function applicationsGetDefaultForVariant(variant: string): Applications {
    if (variant === "Sales") {
        return "Sales" as Applications;
    }
    if (variant === "Accounting") {
        return "Accounting" as Applications;
    }
    if (variant === "Errand") {
        return "Errand" as Applications;
    }
    if (variant === "HumanResources") {
        return "HumanResources" as Applications;
    }
    if (variant === "Logistics") {
        return "Logistics" as Applications;
    }
    if (variant === "Marketing") {
        return "Marketing" as Applications;
    }
    if (variant === "Website") {
        return "Website" as Applications;
    }
    return "Sales" as Applications;
}
export function applicationsCreateForm(initial: Applications): ApplicationsGigaform {
    const initialVariant: "Sales" | "Accounting" | "Errand" | "HumanResources" | "Logistics" | "Marketing" | "Website" = (initial as "Sales" | "Accounting" | "Errand" | "HumanResources" | "Logistics" | "Marketing" | "Website") ?? "Sales";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "applicationsGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as ApplicationsErrors);
    let tainted = $state<$MfPh10>({} as ApplicationsTainted);
    const variants = {} as ApplicationsVariantFields;
    variants[__expr__] = {
        fields: {} as ApplicationsSalesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ApplicationsAccountingFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ApplicationsErrandFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ApplicationsHumanResourcesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ApplicationsLogisticsFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ApplicationsMarketingFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ApplicationsWebsiteFieldControllers
    };
    function switchVariant(variant: "Sales" | "Accounting" | "Errand" | "HumanResources" | "Logistics" | "Marketing" | "Website"): void {
        currentVariant = variant;
        data = "applicationsGetDefaultForVariant"(variant);
        errors = {} as ApplicationsErrors;
        tainted = {} as ApplicationsTainted;
    }
    function validate(): Exit<Applications, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(applicationsDeserialize(data));
    }
    function reset(overrides: Partial<Applications>): void {
        data = overrides ? overrides as typeof data : applicationsGetDefaultForVariant(currentVariant);
        errors = {} as ApplicationsErrors;
        tainted = {} as ApplicationsTainted;
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
export function applicationsFromFormData(formData: FormData): Exit<Applications, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Sales" | "Accounting" | "Errand" | "HumanResources" | "Logistics" | "Marketing" | "Website" | null;
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
    return toExit(applicationsDeserialize(obj));
}

export const Applications = {
  serialize: applicationsSerialize,
  serializeWithContext: applicationsSerializeWithContext,
  deserialize: applicationsDeserialize,
  deserializeWithContext: applicationsDeserializeWithContext,
  is: applicationsIs,
  createForm: applicationsCreateForm,
  fromFormData: applicationsFromFormData
} as const;