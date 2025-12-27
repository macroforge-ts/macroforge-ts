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

export type Priority = /** @default */ 'Medium' | 'High' | 'Low';

export function priorityDefaultValue#0#0(): Priority {
    return 'Medium';
}

export function prioritySerialize#0#0(value: Priority): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(prioritySerializeWithContext(value, ctx));
}
export function prioritySerializeWithContext#0#0(value: Priority, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function priorityDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Priority } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = priorityDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Priority.deserialize: root cannot be a forward reference"
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
export function priorityDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Priority | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Priority | __mf_PendingRef;
    }
    const allowedValues = [
        "'Medium', 'High', 'Low'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Priority"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Priority;
}
export function priorityIs(value: unknown): value is Priority {
    const allowedValues = [
        "'Medium', 'High', 'Low'"
    ] as const;
    return allowedValues.includes(value as any);
}

function priorityGetDefaultForVariant(variant: string): Priority {
    if (variant === `${"Medium"}`) {
        return `${"Medium"}` as Priority;
    }
    if (variant === `${"High"}`) {
        return `${"High"}` as Priority;
    }
    if (variant === `${"Low"}`) {
        return `${"Low"}` as Priority;
    }
    return `${"Medium"}` as Priority;
}
export function priorityCreateForm(initial: Priority): PriorityGigaform {
    const initialVariant: "Medium" | "High" | "Low" = '(initial as "Medium" | "High" | "Low") ?? "Medium"';
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "priorityGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as PriorityErrors);
    let tainted = $state<$MfPh10>({} as PriorityTainted);
    const variants = {} as PriorityVariantFields;
    variants[Medium] = {
        fields: {} as PriorityMediumFieldControllers
    };
    variants[High] = {
        fields: {} as PriorityHighFieldControllers
    };
    variants[Low] = {
        fields: {} as PriorityLowFieldControllers
    };
    function switchVariant(variant: "Medium" | "High" | "Low"): void {
        currentVariant = variant;
        data = "priorityGetDefaultForVariant"(variant);
        errors = {} as PriorityErrors;
        tainted = {} as PriorityTainted;
    }
    function validate(): Exit<Priority, Array<{
        field: string;
        message: string;
    }>> {
        return toExit("priorityDeserialize(data)");
    }
    function reset(overrides: Partial<Priority>): void {
        data = "overrides ? overrides as typeof data : priorityGetDefaultForVariant(currentVariant)";
        errors = {} as PriorityErrors;
        tainted = {} as PriorityTainted;
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
export function priorityFromFormData(formData: FormData): Exit<Priority, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Medium" | "High" | "Low" | null;
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
    return toExit("priorityDeserialize(obj)");
}
export type $MfPh0 = $MfPh1;
export type $MfPh2 = $MfPh3;
export interface PriorityMediumFieldControllers {
}
export interface PriorityHighFieldControllers {
}
export interface PriorityLowFieldControllers {
}
export interface $MfPh4 {
    readonly currentVariant: "Medium" | "High" | "Low";
    readonly data: Priority;
    readonly errors: PriorityErrors;
    readonly tainted: PriorityTainted;
    readonly variants: PriorityVariantFields;
    switchVariant(variant: "Medium" | "High" | "Low"): void;
    validate(): Exit<Priority, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Priority>): void;
}
export interface $MfPh13 {
}
export type PriorityMediumErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PriorityHighErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PriorityLowErrors = {
    _errors: __gf_Option<Array<string>>;
};
 };  };  }; export type PriorityMediumTainted = {
};
export type PriorityHighTainted = {
};
export type PriorityLowTainted = {
};
 };  };  };

export const Priority = {
  deserialize: priorityDeserialize,
  deserializeWithContext: priorityDeserializeWithContext,
  is: priorityIs,
  createForm: priorityCreateForm,
  fromFormData: priorityFromFormData
} as const;