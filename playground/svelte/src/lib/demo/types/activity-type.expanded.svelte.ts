import { createdDefaultValue } from './created.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { commentedDeserializeWithContext } from './commented.svelte';
import { createdDeserializeWithContext } from './created.svelte';
import { editedDeserializeWithContext } from './edited.svelte';
import { paidDeserializeWithContext } from './paid.svelte';
import { sentDeserializeWithContext } from './sent.svelte';
import { viewedDeserializeWithContext } from './viewed.svelte';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
import { commentedDefaultValue } from './commented.svelte';
import { editedDefaultValue } from './edited.svelte';
import { paidDefaultValue } from './paid.svelte';
import { sentDefaultValue } from './sent.svelte';
import { viewedDefaultValue } from './viewed.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Edited } from './edited.svelte';
import type { Commented } from './commented.svelte';
import type { Viewed } from './viewed.svelte';
import type { Paid } from './paid.svelte';
import type { Created } from './created.svelte';
import type { Sent } from './sent.svelte';

export type ActivityType = /** @default */ Created | Edited | Sent | Viewed | Commented | Paid;

export function activityTypeDefaultValue(): ActivityType {
    return createdDefaultValue();
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function activityTypeSerialize(
    value: ActivityType
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(activityTypeSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function activityTypeSerializeWithContext(
    value: ActivityType,
    ctx: __mf_SerializeContext
): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function activityTypeDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, ActivityType> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = activityTypeDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'ActivityType.deserialize: root cannot be a forward reference'
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
export function activityTypeDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): ActivityType | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as ActivityType | __mf_PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'ActivityType.deserializeWithContext: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message:
                    'ActivityType.deserializeWithContext: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'Created') {
        return createdDeserializeWithContext(value, ctx) as ActivityType;
    }
    if (__typeName === 'Edited') {
        return editedDeserializeWithContext(value, ctx) as ActivityType;
    }
    if (__typeName === 'Sent') {
        return sentDeserializeWithContext(value, ctx) as ActivityType;
    }
    if (__typeName === 'Viewed') {
        return viewedDeserializeWithContext(value, ctx) as ActivityType;
    }
    if (__typeName === 'Commented') {
        return commentedDeserializeWithContext(value, ctx) as ActivityType;
    }
    if (__typeName === 'Paid') {
        return paidDeserializeWithContext(value, ctx) as ActivityType;
    }
    throw new __mf_DeserializeError([
        {
            field: '_root',
            message:
                'ActivityType.deserializeWithContext: unknown type "' +
                __typeName +
                '". Expected one of: Created, Edited, Sent, Viewed, Commented, Paid'
        }
    ]);
}
export function activityTypeIs(value: unknown): value is ActivityType {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    const __typeName = (value as any).__type;
    return (
        __typeName === 'Created' ||
        __typeName === 'Edited' ||
        __typeName === 'Sent' ||
        __typeName === 'Viewed' ||
        __typeName === 'Commented' ||
        __typeName === 'Paid'
    );
}

/** Per-variant error types */ export type ActivityTypeCreatedErrors = {
    _errors: Option<Array<string>>;
};
export type ActivityTypeEditedErrors = { _errors: Option<Array<string>> };
export type ActivityTypeSentErrors = { _errors: Option<Array<string>> };
export type ActivityTypeViewedErrors = { _errors: Option<Array<string>> };
export type ActivityTypeCommentedErrors = { _errors: Option<Array<string>> };
export type ActivityTypePaidErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type ActivityTypeCreatedTainted = {};
export type ActivityTypeEditedTainted = {};
export type ActivityTypeSentTainted = {};
export type ActivityTypeViewedTainted = {};
export type ActivityTypeCommentedTainted = {};
export type ActivityTypePaidTainted = {}; /** Union error type */
export type ActivityTypeErrors =
    | ({ _type: 'Created' } & ActivityTypeCreatedErrors)
    | ({ _type: 'Edited' } & ActivityTypeEditedErrors)
    | ({ _type: 'Sent' } & ActivityTypeSentErrors)
    | ({ _type: 'Viewed' } & ActivityTypeViewedErrors)
    | ({ _type: 'Commented' } & ActivityTypeCommentedErrors)
    | ({ _type: 'Paid' } & ActivityTypePaidErrors); /** Union tainted type */
export type ActivityTypeTainted =
    | ({ _type: 'Created' } & ActivityTypeCreatedTainted)
    | ({ _type: 'Edited' } & ActivityTypeEditedTainted)
    | ({ _type: 'Sent' } & ActivityTypeSentTainted)
    | ({ _type: 'Viewed' } & ActivityTypeViewedTainted)
    | ({ _type: 'Commented' } & ActivityTypeCommentedTainted)
    | ({ _type: 'Paid' } & ActivityTypePaidTainted); /** Per-variant field controller types */
export interface ActivityTypeCreatedFieldControllers {}
export interface ActivityTypeEditedFieldControllers {}
export interface ActivityTypeSentFieldControllers {}
export interface ActivityTypeViewedFieldControllers {}
export interface ActivityTypeCommentedFieldControllers {}
export interface ActivityTypePaidFieldControllers {} /** Union Gigaform interface with variant switching */
export interface ActivityTypeGigaform {
    readonly currentVariant: 'Created' | 'Edited' | 'Sent' | 'Viewed' | 'Commented' | 'Paid';
    readonly data: ActivityType;
    readonly errors: ActivityTypeErrors;
    readonly tainted: ActivityTypeTainted;
    readonly variants: ActivityTypeVariantFields;
    switchVariant(variant: 'Created' | 'Edited' | 'Sent' | 'Viewed' | 'Commented' | 'Paid'): void;
    validate(): Result<ActivityType, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<ActivityType>): void;
} /** Variant fields container */
export interface ActivityTypeVariantFields {
    readonly Created: { readonly fields: ActivityTypeCreatedFieldControllers };
    readonly Edited: { readonly fields: ActivityTypeEditedFieldControllers };
    readonly Sent: { readonly fields: ActivityTypeSentFieldControllers };
    readonly Viewed: { readonly fields: ActivityTypeViewedFieldControllers };
    readonly Commented: { readonly fields: ActivityTypeCommentedFieldControllers };
    readonly Paid: { readonly fields: ActivityTypePaidFieldControllers };
} /** Gets default value for a specific variant */
function activityTypeGetDefaultForVariant(variant: string): ActivityType {
    switch (variant) {
        case 'Created':
            return createdDefaultValue() as ActivityType;
        case 'Edited':
            return editedDefaultValue() as ActivityType;
        case 'Sent':
            return sentDefaultValue() as ActivityType;
        case 'Viewed':
            return viewedDefaultValue() as ActivityType;
        case 'Commented':
            return commentedDefaultValue() as ActivityType;
        case 'Paid':
            return paidDefaultValue() as ActivityType;
        default:
            return createdDefaultValue() as ActivityType;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function activityTypeCreateForm(initial?: ActivityType): ActivityTypeGigaform {
    const initialVariant: 'Created' | 'Edited' | 'Sent' | 'Viewed' | 'Commented' | 'Paid' =
        'Created';
    let currentVariant = $state<'Created' | 'Edited' | 'Sent' | 'Viewed' | 'Commented' | 'Paid'>(
        initialVariant
    );
    let data = $state<ActivityType>(initial ?? activityTypeGetDefaultForVariant(initialVariant));
    let errors = $state<ActivityTypeErrors>({} as ActivityTypeErrors);
    let tainted = $state<ActivityTypeTainted>({} as ActivityTypeTainted);
    const variants: ActivityTypeVariantFields = {
        Created: { fields: {} as ActivityTypeCreatedFieldControllers },
        Edited: { fields: {} as ActivityTypeEditedFieldControllers },
        Sent: { fields: {} as ActivityTypeSentFieldControllers },
        Viewed: { fields: {} as ActivityTypeViewedFieldControllers },
        Commented: { fields: {} as ActivityTypeCommentedFieldControllers },
        Paid: { fields: {} as ActivityTypePaidFieldControllers }
    };
    function switchVariant(
        variant: 'Created' | 'Edited' | 'Sent' | 'Viewed' | 'Commented' | 'Paid'
    ): void {
        currentVariant = variant;
        data = activityTypeGetDefaultForVariant(variant);
        errors = {} as ActivityTypeErrors;
        tainted = {} as ActivityTypeTainted;
    }
    function validate(): Result<ActivityType, Array<{ field: string; message: string }>> {
        return activityTypeDeserialize(data);
    }
    function reset(overrides?: Partial<ActivityType>): void {
        data = overrides
            ? (overrides as typeof data)
            : activityTypeGetDefaultForVariant(currentVariant);
        errors = {} as ActivityTypeErrors;
        tainted = {} as ActivityTypeTainted;
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
export function activityTypeFromFormData(
    formData: FormData
): Result<ActivityType, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_type') as
        | 'Created'
        | 'Edited'
        | 'Sent'
        | 'Viewed'
        | 'Commented'
        | 'Paid'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_type', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    if (discriminant === 'Created') {
    } else if (discriminant === 'Edited') {
    } else if (discriminant === 'Sent') {
    } else if (discriminant === 'Viewed') {
    } else if (discriminant === 'Commented') {
    } else if (discriminant === 'Paid') {
    }
    return activityTypeDeserialize(obj);
}

export const ActivityType = {
    defaultValue: activityTypeDefaultValue,
    serialize: activityTypeSerialize,
    serializeWithContext: activityTypeSerializeWithContext,
    deserialize: activityTypeDeserialize,
    deserializeWithContext: activityTypeDeserializeWithContext,
    is: activityTypeIs,
    createForm: activityTypeCreateForm,
    fromFormData: activityTypeFromFormData
} as const;
