import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeCommented } from './commented.svelte';
import { __deserializeCreated } from './created.svelte';
import { __deserializeEdited } from './edited.svelte';
import { __deserializePaid } from './paid.svelte';
import { __deserializeSent } from './sent.svelte';
import { __deserializeViewed } from './viewed.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import { defaultValueCommented } from './commented.svelte';
import { defaultValueCreated } from './created.svelte';
import { defaultValueEdited } from './edited.svelte';
import { defaultValuePaid } from './paid.svelte';
import { defaultValueSent } from './sent.svelte';
import { defaultValueViewed } from './viewed.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

import { Edited } from './edited.svelte';
import { Commented } from './commented.svelte';
import { Viewed } from './viewed.svelte';
import { Paid } from './paid.svelte';
import { Created } from './created.svelte';
import { Sent } from './sent.svelte';

export type ActivityType = /** @default */ Created | Edited | Sent | Viewed | Commented | Paid;

export function defaultValueActivityType(): ActivityType {
    return Created.defaultValue();
}

export function toStringifiedJSONActivityType(value: ActivityType): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeActivityType(value, ctx));
}
export function toObjectActivityType(value: ActivityType): unknown {
    const ctx = SerializeContext.create();
    return __serializeActivityType(value, ctx);
}
export function __serializeActivityType(value: ActivityType, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONActivityType(
    json: string,
    opts?: DeserializeOptions
): Result<ActivityType, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectActivityType(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectActivityType(
    obj: unknown,
    opts?: DeserializeOptions
): Result<ActivityType, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeActivityType(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'ActivityType.fromObject: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function __deserializeActivityType(
    value: any,
    ctx: DeserializeContext
): ActivityType | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as ActivityType | PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new DeserializeError([
            { field: '_root', message: 'ActivityType.__deserialize: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'ActivityType.__deserialize: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'Created') {
        return __deserializeCreated(value, ctx) as ActivityType;
    }
    if (__typeName === 'Edited') {
        return __deserializeEdited(value, ctx) as ActivityType;
    }
    if (__typeName === 'Sent') {
        return __deserializeSent(value, ctx) as ActivityType;
    }
    if (__typeName === 'Viewed') {
        return __deserializeViewed(value, ctx) as ActivityType;
    }
    if (__typeName === 'Commented') {
        return __deserializeCommented(value, ctx) as ActivityType;
    }
    if (__typeName === 'Paid') {
        return __deserializePaid(value, ctx) as ActivityType;
    }
    throw new DeserializeError([
        {
            field: '_root',
            message:
                'ActivityType.__deserialize: unknown type "' +
                __typeName +
                '". Expected one of: Created, Edited, Sent, Viewed, Commented, Paid'
        }
    ]);
}
export function isActivityType(value: unknown): value is ActivityType {
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

/** Per-variant error types */ export type CreatedErrorsActivityType = {
    _errors: Option<Array<string>>;
};
export type EditedErrorsActivityType = { _errors: Option<Array<string>> };
export type SentErrorsActivityType = { _errors: Option<Array<string>> };
export type ViewedErrorsActivityType = { _errors: Option<Array<string>> };
export type CommentedErrorsActivityType = { _errors: Option<Array<string>> };
export type PaidErrorsActivityType = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type CreatedTaintedActivityType = {};
export type EditedTaintedActivityType = {};
export type SentTaintedActivityType = {};
export type ViewedTaintedActivityType = {};
export type CommentedTaintedActivityType = {};
export type PaidTaintedActivityType = {}; /** Union error type */
export type ErrorsActivityType =
    | ({ _type: 'Created' } & CreatedErrorsActivityType)
    | ({ _type: 'Edited' } & EditedErrorsActivityType)
    | ({ _type: 'Sent' } & SentErrorsActivityType)
    | ({ _type: 'Viewed' } & ViewedErrorsActivityType)
    | ({ _type: 'Commented' } & CommentedErrorsActivityType)
    | ({ _type: 'Paid' } & PaidErrorsActivityType); /** Union tainted type */
export type TaintedActivityType =
    | ({ _type: 'Created' } & CreatedTaintedActivityType)
    | ({ _type: 'Edited' } & EditedTaintedActivityType)
    | ({ _type: 'Sent' } & SentTaintedActivityType)
    | ({ _type: 'Viewed' } & ViewedTaintedActivityType)
    | ({ _type: 'Commented' } & CommentedTaintedActivityType)
    | ({ _type: 'Paid' } & PaidTaintedActivityType); /** Per-variant field controller types */
export interface CreatedFieldControllersActivityType {}
export interface EditedFieldControllersActivityType {}
export interface SentFieldControllersActivityType {}
export interface ViewedFieldControllersActivityType {}
export interface CommentedFieldControllersActivityType {}
export interface PaidFieldControllersActivityType {} /** Union Gigaform interface with variant switching */
export interface GigaformActivityType {
    readonly currentVariant: 'Created' | 'Edited' | 'Sent' | 'Viewed' | 'Commented' | 'Paid';
    readonly data: ActivityType;
    readonly errors: ErrorsActivityType;
    readonly tainted: TaintedActivityType;
    readonly variants: VariantFieldsActivityType;
    switchVariant(variant: 'Created' | 'Edited' | 'Sent' | 'Viewed' | 'Commented' | 'Paid'): void;
    validate(): Result<ActivityType, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<ActivityType>): void;
} /** Variant fields container */
export interface VariantFieldsActivityType {
    readonly Created: { readonly fields: CreatedFieldControllersActivityType };
    readonly Edited: { readonly fields: EditedFieldControllersActivityType };
    readonly Sent: { readonly fields: SentFieldControllersActivityType };
    readonly Viewed: { readonly fields: ViewedFieldControllersActivityType };
    readonly Commented: { readonly fields: CommentedFieldControllersActivityType };
    readonly Paid: { readonly fields: PaidFieldControllersActivityType };
} /** Gets default value for a specific variant */
function getDefaultForVariantActivityType(variant: string): ActivityType {
    switch (variant) {
        case 'Created':
            return defaultValueCreated() as ActivityType;
        case 'Edited':
            return defaultValueEdited() as ActivityType;
        case 'Sent':
            return defaultValueSent() as ActivityType;
        case 'Viewed':
            return defaultValueViewed() as ActivityType;
        case 'Commented':
            return defaultValueCommented() as ActivityType;
        case 'Paid':
            return defaultValuePaid() as ActivityType;
        default:
            return defaultValueCreated() as ActivityType;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormActivityType(initial?: ActivityType): GigaformActivityType {
    const initialVariant: 'Created' | 'Edited' | 'Sent' | 'Viewed' | 'Commented' | 'Paid' =
        'Created';
    let currentVariant = $state<'Created' | 'Edited' | 'Sent' | 'Viewed' | 'Commented' | 'Paid'>(
        initialVariant
    );
    let data = $state<ActivityType>(initial ?? getDefaultForVariantActivityType(initialVariant));
    let errors = $state<ErrorsActivityType>({} as ErrorsActivityType);
    let tainted = $state<TaintedActivityType>({} as TaintedActivityType);
    const variants: VariantFieldsActivityType = {
        Created: {
            fields: {} as CreatedFieldControllersActivityType
        },
        Edited: {
            fields: {} as EditedFieldControllersActivityType
        },
        Sent: {
            fields: {} as SentFieldControllersActivityType
        },
        Viewed: {
            fields: {} as ViewedFieldControllersActivityType
        },
        Commented: {
            fields: {} as CommentedFieldControllersActivityType
        },
        Paid: {
            fields: {} as PaidFieldControllersActivityType
        }
    };
    function switchVariant(
        variant: 'Created' | 'Edited' | 'Sent' | 'Viewed' | 'Commented' | 'Paid'
    ): void {
        currentVariant = variant;
        data = getDefaultForVariantActivityType(variant);
        errors = {} as ErrorsActivityType;
        tainted = {} as TaintedActivityType;
    }
    function validate(): Result<ActivityType, Array<{ field: string; message: string }>> {
        return fromObjectActivityType(data);
    }
    function reset(overrides?: Partial<ActivityType>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantActivityType(currentVariant);
        errors = {} as ErrorsActivityType;
        tainted = {} as TaintedActivityType;
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
export function fromFormDataActivityType(
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
    return fromStringifiedJSONActivityType(JSON.stringify(obj));
}
