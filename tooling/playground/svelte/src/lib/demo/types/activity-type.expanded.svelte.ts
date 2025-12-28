import { createdDefaultValue } from "./created.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { commentedDeserializeWithContext } from "./commented.svelte";
import { createdDeserializeWithContext } from "./created.svelte";
import { editedDeserializeWithContext } from "./edited.svelte";
import { paidDeserializeWithContext } from "./paid.svelte";
import { sentDeserializeWithContext } from "./sent.svelte";
import { viewedDeserializeWithContext } from "./viewed.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import { commentedDefaultValue } from "./commented.svelte";
import { editedDefaultValue } from "./edited.svelte";
import { paidDefaultValue } from "./paid.svelte";
import { sentDefaultValue } from "./sent.svelte";
import { viewedDefaultValue } from "./viewed.svelte";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Commented } from './commented.svelte';
import type { Created } from './created.svelte';
import type { Edited } from './edited.svelte';
import type { Paid } from './paid.svelte';
import type { Sent } from './sent.svelte';
import type { Viewed } from './viewed.svelte';


export type ActivityType = /** @default */ Created | Edited | Sent | Viewed | Commented | Paid;

export function activityTypeDefaultValue#0#0(): ActivityType {
    return createdDefaultValue();
}

export function activityTypeSerialize(value: ActivityType): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(activityTypeSerializeWithContext(value, ctx));
}
export function activityTypeSerializeWithContext(value: ActivityType, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function activityTypeDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ActivityType } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = activityTypeDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ActivityType.deserialize: root cannot be a forward reference"
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
export function activityTypeDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ActivityType | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as ActivityType | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === "Created") {
            return createdDeserializeWithContext(value, ctx) as ActivityType;
        }
        if (__typeName === "Edited") {
            return editedDeserializeWithContext(value, ctx) as ActivityType;
        }
        if (__typeName === "Sent") {
            return sentDeserializeWithContext(value, ctx) as ActivityType;
        }
        if (__typeName === "Viewed") {
            return viewedDeserializeWithContext(value, ctx) as ActivityType;
        }
        if (__typeName === "Commented") {
            return commentedDeserializeWithContext(value, ctx) as ActivityType;
        }
        if (__typeName === "Paid") {
            return paidDeserializeWithContext(value, ctx) as ActivityType;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"ActivityType"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function activityTypeIs(value: unknown): value is ActivityType {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "Created" || __typeName === "Edited" || __typeName === "Sent" || __typeName === "Viewed" || __typeName === "Commented" || __typeName === "Paid"') return true;
    }
    return false;
}
     }

export type ActivityTypeCreatedErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ActivityTypeEditedErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ActivityTypeSentErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ActivityTypeViewedErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ActivityTypeCommentedErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ActivityTypePaidErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type ActivityTypeCreatedTainted = {
};
export type ActivityTypeEditedTainted = {
};
export type ActivityTypeSentTainted = {
};
export type ActivityTypeViewedTainted = {
};
export type ActivityTypeCommentedTainted = {
};
export type ActivityTypePaidTainted = {
};
export type ActivityTypeErrors = ({
    _type: "Created";
} & ActivityTypeCreatedErrors) | ({
    _type: "Edited";
} & ActivityTypeEditedErrors) | ({
    _type: "Sent";
} & ActivityTypeSentErrors) | ({
    _type: "Viewed";
} & ActivityTypeViewedErrors) | ({
    _type: "Commented";
} & ActivityTypeCommentedErrors) | ({
    _type: "Paid";
} & ActivityTypePaidErrors);
export type ActivityTypeTainted = ({
    _type: "Created";
} & ActivityTypeCreatedTainted) | ({
    _type: "Edited";
} & ActivityTypeEditedTainted) | ({
    _type: "Sent";
} & ActivityTypeSentTainted) | ({
    _type: "Viewed";
} & ActivityTypeViewedTainted) | ({
    _type: "Commented";
} & ActivityTypeCommentedTainted) | ({
    _type: "Paid";
} & ActivityTypePaidTainted);
export interface ActivityTypeCreatedFieldControllers {
}
export interface ActivityTypeEditedFieldControllers {
}
export interface ActivityTypeSentFieldControllers {
}
export interface ActivityTypeViewedFieldControllers {
}
export interface ActivityTypeCommentedFieldControllers {
}
export interface ActivityTypePaidFieldControllers {
}
export interface ActivityTypeGigaform {
    readonly currentVariant: "Created" | "Edited" | "Sent" | "Viewed" | "Commented" | "Paid";
    readonly data: ActivityType;
    readonly errors: ActivityTypeErrors;
    readonly tainted: ActivityTypeTainted;
    readonly variants: ActivityTypeVariantFields;
    switchVariant(variant: "Created" | "Edited" | "Sent" | "Viewed" | "Commented" | "Paid"): void;
    validate(): Exit<ActivityType, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<ActivityType>): void;
}
export interface ActivityTypeVariantFields {
    readonly Created: {
        readonly fields: ActivityTypeCreatedFieldControllers;
    };
    readonly Edited: {
        readonly fields: ActivityTypeEditedFieldControllers;
    };
    readonly Sent: {
        readonly fields: ActivityTypeSentFieldControllers;
    };
    readonly Viewed: {
        readonly fields: ActivityTypeViewedFieldControllers;
    };
    readonly Commented: {
        readonly fields: ActivityTypeCommentedFieldControllers;
    };
    readonly Paid: {
        readonly fields: ActivityTypePaidFieldControllers;
    };
}
function activityTypeGetDefaultForVariant(variant: string): ActivityType {
    if (variant === "Created") {
        return createdDefaultValue() as ActivityType;
    }
    if (variant === "Edited") {
        return editedDefaultValue() as ActivityType;
    }
    if (variant === "Sent") {
        return sentDefaultValue() as ActivityType;
    }
    if (variant === "Viewed") {
        return viewedDefaultValue() as ActivityType;
    }
    if (variant === "Commented") {
        return commentedDefaultValue() as ActivityType;
    }
    if (variant === "Paid") {
        return paidDefaultValue() as ActivityType;
    }
    return createdDefaultValue() as ActivityType;
}
export function activityTypeCreateForm(initial: ActivityType): ActivityTypeGigaform {
    const initialVariant: "Created" | "Edited" | "Sent" | "Viewed" | "Commented" | "Paid" = "Created";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "activityTypeGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as ActivityTypeErrors);
    let tainted = $state<$MfPh10>({} as ActivityTypeTainted);
    const variants = {} as ActivityTypeVariantFields;
    variants[__expr__] = {
        fields: {} as ActivityTypeCreatedFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ActivityTypeEditedFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ActivityTypeSentFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ActivityTypeViewedFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ActivityTypeCommentedFieldControllers
    };
    variants[__expr__] = {
        fields: {} as ActivityTypePaidFieldControllers
    };
    function switchVariant(variant: "Created" | "Edited" | "Sent" | "Viewed" | "Commented" | "Paid"): void {
        currentVariant = variant;
        data = "activityTypeGetDefaultForVariant"(variant);
        errors = {} as ActivityTypeErrors;
        tainted = {} as ActivityTypeTainted;
    }
    function validate(): Exit<ActivityType, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(activityTypeDeserialize(data));
    }
    function reset(overrides: Partial<ActivityType>): void {
        data = overrides ? overrides as typeof data : activityTypeGetDefaultForVariant(currentVariant);
        errors = {} as ActivityTypeErrors;
        tainted = {} as ActivityTypeTainted;
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
export function activityTypeFromFormData(formData: FormData): Exit<ActivityType, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_type"}`) as "Created" | "Edited" | "Sent" | "Viewed" | "Commented" | "Paid" | null;
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
    return toExit(activityTypeDeserialize(obj));
}

export const ActivityType = {
  serialize: activityTypeSerialize,
  serializeWithContext: activityTypeSerializeWithContext,
  deserialize: activityTypeDeserialize,
  deserializeWithContext: activityTypeDeserializeWithContext,
  is: activityTypeIs,
  createForm: activityTypeCreateForm,
  fromFormData: activityTypeFromFormData
} as const;