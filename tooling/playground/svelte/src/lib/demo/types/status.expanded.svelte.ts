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

export type Status = /** @default */ 'Scheduled' | 'OnDeck' | 'Waiting';

export function statusDefaultValue#0#0(): Status {
    return 'Scheduled';
}

export function statusSerialize(value: Status): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(statusSerializeWithContext(value, ctx));
}
export function statusSerializeWithContext(value: Status, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function statusDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Status } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = statusDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Status.deserialize: root cannot be a forward reference"
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
export function statusDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Status | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Status | __mf_PendingRef;
    }
    const allowedValues = [
        "'Scheduled', 'OnDeck', 'Waiting'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Status"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Status;
}
export function statusIs(value: unknown): value is Status {
    const allowedValues = [
        "'Scheduled', 'OnDeck', 'Waiting'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type StatusScheduledErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type StatusOnDeckErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type StatusWaitingErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type StatusScheduledTainted = {
};
export type StatusOnDeckTainted = {
};
export type StatusWaitingTainted = {
};
export type StatusErrors = ({
    _value: "Scheduled";
} & StatusScheduledErrors) | ({
    _value: "OnDeck";
} & StatusOnDeckErrors) | ({
    _value: "Waiting";
} & StatusWaitingErrors);
export type StatusTainted = ({
    _value: "Scheduled";
} & StatusScheduledTainted) | ({
    _value: "OnDeck";
} & StatusOnDeckTainted) | ({
    _value: "Waiting";
} & StatusWaitingTainted);
export interface StatusScheduledFieldControllers {
}
export interface StatusOnDeckFieldControllers {
}
export interface StatusWaitingFieldControllers {
}
export interface StatusGigaform {
    readonly currentVariant: "Scheduled" | "OnDeck" | "Waiting";
    readonly data: Status;
    readonly errors: StatusErrors;
    readonly tainted: StatusTainted;
    readonly variants: StatusVariantFields;
    switchVariant(variant: "Scheduled" | "OnDeck" | "Waiting"): void;
    validate(): Exit<Status, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Status>): void;
}
export interface StatusVariantFields {
    readonly Scheduled: {
        readonly fields: StatusScheduledFieldControllers;
    };
    readonly OnDeck: {
        readonly fields: StatusOnDeckFieldControllers;
    };
    readonly Waiting: {
        readonly fields: StatusWaitingFieldControllers;
    };
}
function statusGetDefaultForVariant(variant: string): Status {
    if (variant === "Scheduled") {
        return "Scheduled" as Status;
    }
    if (variant === "OnDeck") {
        return "OnDeck" as Status;
    }
    if (variant === "Waiting") {
        return "Waiting" as Status;
    }
    return "Scheduled" as Status;
}
export function statusCreateForm(initial: Status): StatusGigaform {
    const initialVariant: "Scheduled" | "OnDeck" | "Waiting" = (initial as "Scheduled" | "OnDeck" | "Waiting") ?? "Scheduled";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "statusGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as StatusErrors);
    let tainted = $state<$MfPh10>({} as StatusTainted);
    const variants = {} as StatusVariantFields;
    variants[__expr__] = {
        fields: {} as StatusScheduledFieldControllers
    };
    variants[__expr__] = {
        fields: {} as StatusOnDeckFieldControllers
    };
    variants[__expr__] = {
        fields: {} as StatusWaitingFieldControllers
    };
    function switchVariant(variant: "Scheduled" | "OnDeck" | "Waiting"): void {
        currentVariant = variant;
        data = "statusGetDefaultForVariant"(variant);
        errors = {} as StatusErrors;
        tainted = {} as StatusTainted;
    }
    function validate(): Exit<Status, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(statusDeserialize(data));
    }
    function reset(overrides: Partial<Status>): void {
        data = overrides ? overrides as typeof data : statusGetDefaultForVariant(currentVariant);
        errors = {} as StatusErrors;
        tainted = {} as StatusTainted;
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
export function statusFromFormData(formData: FormData): Exit<Status, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "Scheduled" | "OnDeck" | "Waiting" | null;
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
    return toExit(statusDeserialize(obj));
}

export const Status = {
  serialize: statusSerialize,
  serializeWithContext: statusSerializeWithContext,
  deserialize: statusDeserialize,
  deserializeWithContext: statusDeserializeWithContext,
  is: statusIs,
  createForm: statusCreateForm,
  fromFormData: statusFromFormData
} as const;