import { gradientDefaultValue } from "./gradient.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { cardinalDeserializeWithContext } from "./cardinal.svelte";
import { customDeserializeWithContext } from "./custom.svelte";
import { gradientDeserializeWithContext } from "./gradient.svelte";
import { ordinalDeserializeWithContext } from "./ordinal.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import { cardinalDefaultValue } from "./cardinal.svelte";
import { customDefaultValue } from "./custom.svelte";
import { ordinalDefaultValue } from "./ordinal.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Cardinal } from './cardinal.svelte';
import type { Custom } from './custom.svelte';
import type { Gradient } from './gradient.svelte';
import type { Ordinal } from './ordinal.svelte';


export type ColorsConfig = Cardinal | Ordinal | Custom | /** @default */ Gradient;

export function colorsConfigDefaultValue#0#0(): ColorsConfig {
    return gradientDefaultValue();
}

export function colorsConfigSerialize(value: ColorsConfig): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(colorsConfigSerializeWithContext(value, ctx));
}
export function colorsConfigSerializeWithContext(value: ColorsConfig, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function colorsConfigDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ColorsConfig } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = colorsConfigDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ColorsConfig.deserialize: root cannot be a forward reference"
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
export function colorsConfigDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ColorsConfig | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as ColorsConfig | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === "Cardinal") {
            return cardinalDeserializeWithContext(value, ctx) as ColorsConfig;
        }
        if (__typeName === "Ordinal") {
            return ordinalDeserializeWithContext(value, ctx) as ColorsConfig;
        }
        if (__typeName === "Custom") {
            return customDeserializeWithContext(value, ctx) as ColorsConfig;
        }
        if (__typeName === "Gradient") {
            return gradientDeserializeWithContext(value, ctx) as ColorsConfig;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"ColorsConfig"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function colorsConfigIs(value: unknown): value is ColorsConfig {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "Cardinal" || __typeName === "Ordinal" || __typeName === "Custom" || __typeName === "Gradient"') return true;
    }
    return false;
}
     }

export type ColorsConfigCardinalErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ColorsConfigOrdinalErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ColorsConfigCustomErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ColorsConfigGradientErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ColorsConfigCardinalTainted = {
};
export type ColorsConfigOrdinalTainted = {
};
export type ColorsConfigCustomTainted = {
};
export type ColorsConfigGradientTainted = {
};
export type ColorsConfigErrors = ({
    _type: "Cardinal";
} & ColorsConfigCardinalErrors) | ({
    _type: "Ordinal";
} & ColorsConfigOrdinalErrors) | ({
    _type: "Custom";
} & ColorsConfigCustomErrors) | ({
    _type: "Gradient";
} & ColorsConfigGradientErrors);
export type ColorsConfigTainted = ({
    _type: "Cardinal";
} & ColorsConfigCardinalTainted) | ({
    _type: "Ordinal";
} & ColorsConfigOrdinalTainted) | ({
    _type: "Custom";
} & ColorsConfigCustomTainted) | ({
    _type: "Gradient";
} & ColorsConfigGradientTainted);
export interface ColorsConfigCardinalFieldControllers {
}
export interface ColorsConfigOrdinalFieldControllers {
}
export interface ColorsConfigCustomFieldControllers {
}
export interface ColorsConfigGradientFieldControllers {
}
export interface ColorsConfigGigaform {
    readonly currentVariant: "Cardinal" | "Ordinal" | "Custom" | "Gradient";
    readonly data: ColorsConfig;
    readonly errors: ColorsConfigErrors;
    readonly tainted: ColorsConfigTainted;
    readonly variants: ColorsConfigVariantFields;
    switchVariant(variant: "Cardinal" | "Ordinal" | "Custom" | "Gradient"): void;
    validate(): Exit<ColorsConfig, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<ColorsConfig>): void;
}
export interface ColorsConfigVariantFields {
    readonly Cardinal: {
        readonly fields: ColorsConfigCardinalFieldControllers;
    };
    readonly Ordinal: {
        readonly fields: ColorsConfigOrdinalFieldControllers;
    };
    readonly Custom: {
        readonly fields: ColorsConfigCustomFieldControllers;
    };
    readonly Gradient: {
        readonly fields: ColorsConfigGradientFieldControllers;
    };
}
function colorsConfigGetDefaultForVariant(variant: string): ColorsConfig {
    if (variant === "Cardinal") {
        return cardinalDefaultValue() as ColorsConfig;
    }
    if (variant === "Ordinal") {
        return ordinalDefaultValue() as ColorsConfig;
    }
    if (variant === "Custom") {
        return customDefaultValue() as ColorsConfig;
    }
    if (variant === "Gradient") {
        return gradientDefaultValue() as ColorsConfig;
    }
    return cardinalDefaultValue() as ColorsConfig;
}
export function colorsConfigCreateForm(initial: ColorsConfig): ColorsConfigGigaform {
    const initialVariant: "Cardinal" | "Ordinal" | "Custom" | "Gradient" = "Cardinal";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "colorsConfigGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as ColorsConfigErrors);
    let tainted = $state<$MfPh10>({} as ColorsConfigTainted);
    const variants = {} as ColorsConfigVariantFields;
    variants[__expr__] = {
        fields: {} as ColorsConfigCardinalFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ColorsConfigOrdinalFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ColorsConfigCustomFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ColorsConfigGradientFieldControllers
    };
    function switchVariant(variant: "Cardinal" | "Ordinal" | "Custom" | "Gradient"): void {
        currentVariant = variant;
        data = "colorsConfigGetDefaultForVariant"(variant);
        errors = {} as ColorsConfigErrors;
        tainted = {} as ColorsConfigTainted;
    }
    function validate(): Exit<ColorsConfig, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(colorsConfigDeserialize(data));
    }
    function reset(overrides: Partial<ColorsConfig>): void {
        data = overrides ? overrides as typeof data : colorsConfigGetDefaultForVariant(currentVariant);
        errors = {} as ColorsConfigErrors;
        tainted = {} as ColorsConfigTainted;
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
export function colorsConfigFromFormData(formData: FormData): Exit<ColorsConfig, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_type"}`) as "Cardinal" | "Ordinal" | "Custom" | "Gradient" | null;
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
    return toExit(colorsConfigDeserialize(obj));
}

export const ColorsConfig = {
  serialize: colorsConfigSerialize,
  serializeWithContext: colorsConfigSerializeWithContext,
  deserialize: colorsConfigDeserialize,
  deserializeWithContext: colorsConfigDeserializeWithContext,
  is: colorsConfigIs,
  createForm: colorsConfigCreateForm,
  fromFormData: colorsConfigFromFormData
} as const;