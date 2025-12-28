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

export type JobTitle =
    | /** @default */ 'Technician'
    | 'SalesRepresentative'
    | 'HumanResources'
    | 'InformationTechnology';

export function jobTitleDefaultValue#0#0(): JobTitle {
    return 'Technician';
}

export function jobTitleSerialize(value: JobTitle): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(jobTitleSerializeWithContext(value, ctx));
}
export function jobTitleSerializeWithContext(value: JobTitle, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function jobTitleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: JobTitle } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = jobTitleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "JobTitle.deserialize: root cannot be a forward reference"
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
export function jobTitleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): JobTitle | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as JobTitle | __mf_PendingRef;
    }
    const allowedValues = [
        "'Technician', 'SalesRepresentative', 'HumanResources', 'InformationTechnology'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"JobTitle"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as JobTitle;
}
export function jobTitleIs(value: unknown): value is JobTitle {
    const allowedValues = [
        "'Technician', 'SalesRepresentative', 'HumanResources', 'InformationTechnology'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type JobTitleTechnicianErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type JobTitleSalesRepresentativeErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type JobTitleHumanResourcesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type JobTitleInformationTechnologyErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type JobTitleTechnicianTainted = {
};
export type JobTitleSalesRepresentativeTainted = {
};
export type JobTitleHumanResourcesTainted = {
};
export type JobTitleInformationTechnologyTainted = {
};
export type JobTitleErrors = ({
    _value: "Technician";
} & JobTitleTechnicianErrors) | ({
    _value: "SalesRepresentative";
} & JobTitleSalesRepresentativeErrors) | ({
    _value: "HumanResources";
} & JobTitleHumanResourcesErrors) | ({
    _value: "InformationTechnology";
} & JobTitleInformationTechnologyErrors);
export type JobTitleTainted = ({
    _value: "Technician";
} & JobTitleTechnicianTainted) | ({
    _value: "SalesRepresentative";
} & JobTitleSalesRepresentativeTainted) | ({
    _value: "HumanResources";
} & JobTitleHumanResourcesTainted) | ({
    _value: "InformationTechnology";
} & JobTitleInformationTechnologyTainted);
export interface JobTitleTechnicianFieldControllers {
}
export interface JobTitleSalesRepresentativeFieldControllers {
}
export interface JobTitleHumanResourcesFieldControllers {
}
export interface JobTitleInformationTechnologyFieldControllers {
}
export interface JobTitleGigaform {
    readonly currentVariant: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology";
    readonly data: JobTitle;
    readonly errors: JobTitleErrors;
    readonly tainted: JobTitleTainted;
    readonly variants: JobTitleVariantFields;
    switchVariant(variant: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology"): void;
    validate(): Exit<JobTitle, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<JobTitle>): void;
}
export interface JobTitleVariantFields {
    readonly Technician: {
        readonly fields: JobTitleTechnicianFieldControllers;
    };
    readonly SalesRepresentative: {
        readonly fields: JobTitleSalesRepresentativeFieldControllers;
    };
    readonly HumanResources: {
        readonly fields: JobTitleHumanResourcesFieldControllers;
    };
    readonly InformationTechnology: {
        readonly fields: JobTitleInformationTechnologyFieldControllers;
    };
}
function jobTitleGetDefaultForVariant(variant: string): JobTitle {
    if (variant === "Technician") {
        return "Technician" as JobTitle;
    }
    if (variant === "SalesRepresentative") {
        return "SalesRepresentative" as JobTitle;
    }
    if (variant === "HumanResources") {
        return "HumanResources" as JobTitle;
    }
    if (variant === "InformationTechnology") {
        return "InformationTechnology" as JobTitle;
    }
    return "Technician" as JobTitle;
}
export function jobTitleCreateForm(initial: JobTitle): JobTitleGigaform {
    const initialVariant: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology" = (initial as "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology") ?? "Technician";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "jobTitleGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as JobTitleErrors);
    let tainted = $state<$MfPh10>({} as JobTitleTainted);
    const variants = {} as JobTitleVariantFields;
    variants[__expr__] = {
        fields: {} as JobTitleTechnicianFieldControllers
    };
    variants[__expr__] = {
        fields: {} as JobTitleSalesRepresentativeFieldControllers
    };
    variants[__expr__] = {
        fields: {} as JobTitleHumanResourcesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as JobTitleInformationTechnologyFieldControllers
    };
    function switchVariant(variant: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology"): void {
        currentVariant = variant;
        data = "jobTitleGetDefaultForVariant"(variant);
        errors = {} as JobTitleErrors;
        tainted = {} as JobTitleTainted;
    }
    function validate(): Exit<JobTitle, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(jobTitleDeserialize(data));
    }
    function reset(overrides: Partial<JobTitle>): void {
        data = overrides ? overrides as typeof data : jobTitleGetDefaultForVariant(currentVariant);
        errors = {} as JobTitleErrors;
        tainted = {} as JobTitleTainted;
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
export function jobTitleFromFormData(formData: FormData): Exit<JobTitle, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology" | null;
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
    return toExit(jobTitleDeserialize(obj));
}

export const JobTitle = {
  serialize: jobTitleSerialize,
  serializeWithContext: jobTitleSerializeWithContext,
  deserialize: jobTitleDeserialize,
  deserializeWithContext: jobTitleDeserializeWithContext,
  is: jobTitleIs,
  createForm: jobTitleCreateForm,
  fromFormData: jobTitleFromFormData
} as const;