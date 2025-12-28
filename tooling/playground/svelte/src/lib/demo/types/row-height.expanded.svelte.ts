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

export type RowHeight = 'ExtraSmall' | 'Small' | /** @default */ 'Medium' | 'Large';

export function rowHeightDefaultValue#0#0(): RowHeight {
    return 'Medium';
}

export function rowHeightSerialize(value: RowHeight): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(rowHeightSerializeWithContext(value, ctx));
}
export function rowHeightSerializeWithContext(value: RowHeight, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function rowHeightDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: RowHeight } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = rowHeightDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "RowHeight.deserialize: root cannot be a forward reference"
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
export function rowHeightDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): RowHeight | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RowHeight | __mf_PendingRef;
    }
    const allowedValues = [
        "'ExtraSmall', 'Small', 'Medium', 'Large'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"RowHeight"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as RowHeight;
}
export function rowHeightIs(value: unknown): value is RowHeight {
    const allowedValues = [
        "'ExtraSmall', 'Small', 'Medium', 'Large'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type RowHeightExtraSmallErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type RowHeightSmallErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type RowHeightMediumErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type RowHeightLargeErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type RowHeightExtraSmallTainted = {
};
export type RowHeightSmallTainted = {
};
export type RowHeightMediumTainted = {
};
export type RowHeightLargeTainted = {
};
export type RowHeightErrors = ({
    _value: "ExtraSmall";
} & RowHeightExtraSmallErrors) | ({
    _value: "Small";
} & RowHeightSmallErrors) | ({
    _value: "Medium";
} & RowHeightMediumErrors) | ({
    _value: "Large";
} & RowHeightLargeErrors);
export type RowHeightTainted = ({
    _value: "ExtraSmall";
} & RowHeightExtraSmallTainted) | ({
    _value: "Small";
} & RowHeightSmallTainted) | ({
    _value: "Medium";
} & RowHeightMediumTainted) | ({
    _value: "Large";
} & RowHeightLargeTainted);
export interface RowHeightExtraSmallFieldControllers {
}
export interface RowHeightSmallFieldControllers {
}
export interface RowHeightMediumFieldControllers {
}
export interface RowHeightLargeFieldControllers {
}
export interface RowHeightGigaform {
    readonly currentVariant: "ExtraSmall" | "Small" | "Medium" | "Large";
    readonly data: RowHeight;
    readonly errors: RowHeightErrors;
    readonly tainted: RowHeightTainted;
    readonly variants: RowHeightVariantFields;
    switchVariant(variant: "ExtraSmall" | "Small" | "Medium" | "Large"): void;
    validate(): Exit<RowHeight, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<RowHeight>): void;
}
export interface RowHeightVariantFields {
    readonly ExtraSmall: {
        readonly fields: RowHeightExtraSmallFieldControllers;
    };
    readonly Small: {
        readonly fields: RowHeightSmallFieldControllers;
    };
    readonly Medium: {
        readonly fields: RowHeightMediumFieldControllers;
    };
    readonly Large: {
        readonly fields: RowHeightLargeFieldControllers;
    };
}
function rowHeightGetDefaultForVariant(variant: string): RowHeight {
    if (variant === "ExtraSmall") {
        return "ExtraSmall" as RowHeight;
    }
    if (variant === "Small") {
        return "Small" as RowHeight;
    }
    if (variant === "Medium") {
        return "Medium" as RowHeight;
    }
    if (variant === "Large") {
        return "Large" as RowHeight;
    }
    return "ExtraSmall" as RowHeight;
}
export function rowHeightCreateForm(initial: RowHeight): RowHeightGigaform {
    const initialVariant: "ExtraSmall" | "Small" | "Medium" | "Large" = (initial as "ExtraSmall" | "Small" | "Medium" | "Large") ?? "ExtraSmall";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "rowHeightGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as RowHeightErrors);
    let tainted = $state<$MfPh10>({} as RowHeightTainted);
    const variants = {} as RowHeightVariantFields;
    variants[__expr__] = {
        fields: {} as RowHeightExtraSmallFieldControllers
    };
    variants[__expr__] = {
        fields: {} as RowHeightSmallFieldControllers
    };
    variants[__expr__] = {
        fields: {} as RowHeightMediumFieldControllers
    };
    variants[__expr__] = {
        fields: {} as RowHeightLargeFieldControllers
    };
    function switchVariant(variant: "ExtraSmall" | "Small" | "Medium" | "Large"): void {
        currentVariant = variant;
        data = "rowHeightGetDefaultForVariant"(variant);
        errors = {} as RowHeightErrors;
        tainted = {} as RowHeightTainted;
    }
    function validate(): Exit<RowHeight, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(rowHeightDeserialize(data));
    }
    function reset(overrides: Partial<RowHeight>): void {
        data = overrides ? overrides as typeof data : rowHeightGetDefaultForVariant(currentVariant);
        errors = {} as RowHeightErrors;
        tainted = {} as RowHeightTainted;
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
export function rowHeightFromFormData(formData: FormData): Exit<RowHeight, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "ExtraSmall" | "Small" | "Medium" | "Large" | null;
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
    return toExit(rowHeightDeserialize(obj));
}

export const RowHeight = {
  serialize: rowHeightSerialize,
  serializeWithContext: rowHeightSerializeWithContext,
  deserialize: rowHeightDeserialize,
  deserializeWithContext: rowHeightDeserializeWithContext,
  is: rowHeightIs,
  createForm: rowHeightCreateForm,
  fromFormData: rowHeightFromFormData
} as const;