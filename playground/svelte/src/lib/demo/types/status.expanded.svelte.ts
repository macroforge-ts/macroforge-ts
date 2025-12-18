import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { exitFail } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';

export type Status = /** @default */ 'Scheduled' | 'OnDeck' | 'Waiting';

export function statusDefaultValue(): Status {
    return 'Scheduled';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function statusSerialize(
    value: Status
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(statusSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function statusSerializeWithContext(value: Status, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function statusDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Status> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = statusDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Status.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function statusDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Status | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Status | __mf_PendingRef;
    }
    const allowedValues = ['Scheduled', 'OnDeck', 'Waiting'] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for Status: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as Status;
}
export function statusIs(value: unknown): value is Status {
    const allowedValues = ['Scheduled', 'OnDeck', 'Waiting'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type StatusScheduledErrors = {
    _errors: Option<Array<string>>;
};
export type StatusOnDeckErrors = { _errors: Option<Array<string>> };
export type StatusWaitingErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type StatusScheduledTainted = {};
export type StatusOnDeckTainted = {};
export type StatusWaitingTainted = {}; /** Union error type */
export type StatusErrors =
    | ({ _value: 'Scheduled' } & StatusScheduledErrors)
    | ({ _value: 'OnDeck' } & StatusOnDeckErrors)
    | ({ _value: 'Waiting' } & StatusWaitingErrors); /** Union tainted type */
export type StatusTainted =
    | ({ _value: 'Scheduled' } & StatusScheduledTainted)
    | ({ _value: 'OnDeck' } & StatusOnDeckTainted)
    | ({ _value: 'Waiting' } & StatusWaitingTainted); /** Per-variant field controller types */
export interface StatusScheduledFieldControllers {}
export interface StatusOnDeckFieldControllers {}
export interface StatusWaitingFieldControllers {} /** Union Gigaform interface with variant switching */
export interface StatusGigaform {
    readonly currentVariant: 'Scheduled' | 'OnDeck' | 'Waiting';
    readonly data: Status;
    readonly errors: StatusErrors;
    readonly tainted: StatusTainted;
    readonly variants: StatusVariantFields;
    switchVariant(variant: 'Scheduled' | 'OnDeck' | 'Waiting'): void;
    validate(): Exit<Array<{ field: string; message: string }>, Status>;
    reset(overrides?: Partial<Status>): void;
} /** Variant fields container */
export interface StatusVariantFields {
    readonly Scheduled: { readonly fields: StatusScheduledFieldControllers };
    readonly OnDeck: { readonly fields: StatusOnDeckFieldControllers };
    readonly Waiting: { readonly fields: StatusWaitingFieldControllers };
} /** Gets default value for a specific variant */
function statusGetDefaultForVariant(variant: string): Status {
    switch (variant) {
        case 'Scheduled':
            return 'Scheduled' as Status;
        case 'OnDeck':
            return 'OnDeck' as Status;
        case 'Waiting':
            return 'Waiting' as Status;
        default:
            return 'Scheduled' as Status;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function statusCreateForm(initial?: Status): StatusGigaform {
    const initialVariant: 'Scheduled' | 'OnDeck' | 'Waiting' =
        (initial as 'Scheduled' | 'OnDeck' | 'Waiting') ?? 'Scheduled';
    let currentVariant = $state<'Scheduled' | 'OnDeck' | 'Waiting'>(initialVariant);
    let data = $state<Status>(initial ?? statusGetDefaultForVariant(initialVariant));
    let errors = $state<StatusErrors>({} as StatusErrors);
    let tainted = $state<StatusTainted>({} as StatusTainted);
    const variants: StatusVariantFields = {
        Scheduled: { fields: {} as StatusScheduledFieldControllers },
        OnDeck: { fields: {} as StatusOnDeckFieldControllers },
        Waiting: { fields: {} as StatusWaitingFieldControllers }
    };
    function switchVariant(variant: 'Scheduled' | 'OnDeck' | 'Waiting'): void {
        currentVariant = variant;
        data = statusGetDefaultForVariant(variant);
        errors = {} as StatusErrors;
        tainted = {} as StatusTainted;
    }
    function validate(): Exit<Array<{ field: string; message: string }>, Status> {
        return statusDeserialize(data);
    }
    function reset(overrides?: Partial<Status>): void {
        data = overrides ? (overrides as typeof data) : statusGetDefaultForVariant(currentVariant);
        errors = {} as StatusErrors;
        tainted = {} as StatusTainted;
    }
    return {
        get currentVariant() {
            return currentVariant;
        },
        get data() {
            return data;
        },
        set data(v) {
            data = v;
        },
        get errors() {
            return errors;
        },
        set errors(v) {
            errors = v;
        },
        get tainted() {
            return tainted;
        },
        set tainted(v) {
            tainted = v;
        },
        variants,
        switchVariant,
        validate,
        reset
    };
} /** Parses FormData for union type, determining variant from discriminant field */
export function statusFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Status> {
    const discriminant = formData.get('_value') as 'Scheduled' | 'OnDeck' | 'Waiting' | null;
    if (!discriminant) {
        return exitFail([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Scheduled') {
    } else if (discriminant === 'OnDeck') {
    } else if (discriminant === 'Waiting') {
    }
    return statusDeserialize(obj);
}

export const Status = {
    defaultValue: statusDefaultValue,
    serialize: statusSerialize,
    serializeWithContext: statusSerializeWithContext,
    deserialize: statusDeserialize,
    deserializeWithContext: statusDeserializeWithContext,
    is: statusIs,
    createForm: statusCreateForm,
    fromFormData: statusFromFormData
} as const;
