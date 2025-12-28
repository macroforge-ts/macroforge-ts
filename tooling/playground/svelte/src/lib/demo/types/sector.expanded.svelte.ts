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

export type Sector = /** @default */ 'Residential' | 'Commercial';

export function sectorDefaultValue#0#0(): Sector {
    return 'Residential';
}

export function sectorSerialize(value: Sector): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(sectorSerializeWithContext(value, ctx));
}
export function sectorSerializeWithContext(value: Sector, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function sectorDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Sector } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = sectorDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Sector.deserialize: root cannot be a forward reference"
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
export function sectorDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Sector | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Sector | __mf_PendingRef;
    }
    const allowedValues = [
        "'Residential', 'Commercial'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Sector"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Sector;
}
export function sectorIs(value: unknown): value is Sector {
    const allowedValues = [
        "'Residential', 'Commercial'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type SectorResidentialErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type SectorCommercialErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type SectorResidentialTainted = {
};
export type SectorCommercialTainted = {
};
export type SectorErrors = ({
    _value: "Residential";
} & SectorResidentialErrors) | ({
    _value: "Commercial";
} & SectorCommercialErrors);
export type SectorTainted = ({
    _value: "Residential";
} & SectorResidentialTainted) | ({
    _value: "Commercial";
} & SectorCommercialTainted);
export interface SectorResidentialFieldControllers {
}
export interface SectorCommercialFieldControllers {
}
export interface SectorGigaform {
    readonly currentVariant: "Residential" | "Commercial";
    readonly data: Sector;
    readonly errors: SectorErrors;
    readonly tainted: SectorTainted;
    readonly variants: SectorVariantFields;
    switchVariant(variant: "Residential" | "Commercial"): void;
    validate(): Exit<Sector, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Sector>): void;
}
export interface SectorVariantFields {
    readonly Residential: {
        readonly fields: SectorResidentialFieldControllers;
    };
    readonly Commercial: {
        readonly fields: SectorCommercialFieldControllers;
    };
}
function sectorGetDefaultForVariant(variant: string): Sector {
    if (variant === "Residential") {
        return "Residential" as Sector;
    }
    if (variant === "Commercial") {
        return "Commercial" as Sector;
    }
    return "Residential" as Sector;
}
export function sectorCreateForm(initial: Sector): SectorGigaform {
    const initialVariant: "Residential" | "Commercial" = (initial as "Residential" | "Commercial") ?? "Residential";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "sectorGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as SectorErrors);
    let tainted = $state<$MfPh10>({} as SectorTainted);
    const variants = {} as SectorVariantFields;
    variants[__expr__] = {
        fields: {} as SectorResidentialFieldControllers
    };
    variants[__expr__] = {
        fields: {} as SectorCommercialFieldControllers
    };
    function switchVariant(variant: "Residential" | "Commercial"): void {
        currentVariant = variant;
        data = "sectorGetDefaultForVariant"(variant);
        errors = {} as SectorErrors;
        tainted = {} as SectorTainted;
    }
    function validate(): Exit<Sector, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(sectorDeserialize(data));
    }
    function reset(overrides: Partial<Sector>): void {
        data = overrides ? overrides as typeof data : sectorGetDefaultForVariant(currentVariant);
        errors = {} as SectorErrors;
        tainted = {} as SectorTainted;
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
export function sectorFromFormData(formData: FormData): Exit<Sector, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Residential" | "Commercial" | null;
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
    return toExit(sectorDeserialize(obj));
}

export const Sector = {
  serialize: sectorSerialize,
  serializeWithContext: sectorSerializeWithContext,
  deserialize: sectorDeserialize,
  deserializeWithContext: sectorDeserializeWithContext,
  is: sectorIs,
  createForm: sectorCreateForm,
  fromFormData: sectorFromFormData
} as const;