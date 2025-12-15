import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type Status = /** @default */ 'Scheduled' | 'OnDeck' | 'Waiting';

export function defaultValueStatus(): Status {
    return 'Scheduled';
}

export function toStringifiedJSONStatus(value: Status): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeStatus(value, ctx));
}
export function toObjectStatus(value: Status): unknown {
    const ctx = SerializeContext.create();
    return __serializeStatus(value, ctx);
}
export function __serializeStatus(value: Status, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONStatus(
    json: string,
    opts?: DeserializeOptions
): Result<Status, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectStatus(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectStatus(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Status, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeStatus(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Status.fromObject: root cannot be a forward reference' }
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
export function __deserializeStatus(value: any, ctx: DeserializeContext): Status | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Status | PendingRef;
    }
    const allowedValues = ['Scheduled', 'OnDeck', 'Waiting'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
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
export function isStatus(value: unknown): value is Status {
    const allowedValues = ['Scheduled', 'OnDeck', 'Waiting'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type ScheduledErrorsStatus = {
    _errors: Option<Array<string>>;
};
export type OnDeckErrorsStatus = { _errors: Option<Array<string>> };
export type WaitingErrorsStatus = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type ScheduledTaintedStatus = {};
export type OnDeckTaintedStatus = {};
export type WaitingTaintedStatus = {}; /** Union error type */
export type ErrorsStatus =
    | ({ _value: 'Scheduled' } & ScheduledErrorsStatus)
    | ({ _value: 'OnDeck' } & OnDeckErrorsStatus)
    | ({ _value: 'Waiting' } & WaitingErrorsStatus); /** Union tainted type */
export type TaintedStatus =
    | ({ _value: 'Scheduled' } & ScheduledTaintedStatus)
    | ({ _value: 'OnDeck' } & OnDeckTaintedStatus)
    | ({ _value: 'Waiting' } & WaitingTaintedStatus); /** Per-variant field controller types */
export interface ScheduledFieldControllersStatus {}
export interface OnDeckFieldControllersStatus {}
export interface WaitingFieldControllersStatus {} /** Union Gigaform interface with variant switching */
export interface GigaformStatus {
    readonly currentVariant: 'Scheduled' | 'OnDeck' | 'Waiting';
    readonly data: Status;
    readonly errors: ErrorsStatus;
    readonly tainted: TaintedStatus;
    readonly variants: VariantFieldsStatus;
    switchVariant(variant: 'Scheduled' | 'OnDeck' | 'Waiting'): void;
    validate(): Result<Status, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Status>): void;
} /** Variant fields container */
export interface VariantFieldsStatus {
    readonly Scheduled: { readonly fields: ScheduledFieldControllersStatus };
    readonly OnDeck: { readonly fields: OnDeckFieldControllersStatus };
    readonly Waiting: { readonly fields: WaitingFieldControllersStatus };
} /** Gets default value for a specific variant */
function getDefaultForVariantStatus(variant: string): Status {
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
export function createFormStatus(initial?: Status): GigaformStatus {
    const initialVariant: 'Scheduled' | 'OnDeck' | 'Waiting' =
        (initial as 'Scheduled' | 'OnDeck' | 'Waiting') ?? 'Scheduled';
    let currentVariant = $state<'Scheduled' | 'OnDeck' | 'Waiting'>(initialVariant);
    let data = $state<Status>(initial ?? getDefaultForVariantStatus(initialVariant));
    let errors = $state<ErrorsStatus>({} as ErrorsStatus);
    let tainted = $state<TaintedStatus>({} as TaintedStatus);
    const variants: VariantFieldsStatus = {
        Scheduled: {
            fields: {} as ScheduledFieldControllersStatus
        },
        OnDeck: {
            fields: {} as OnDeckFieldControllersStatus
        },
        Waiting: {
            fields: {} as WaitingFieldControllersStatus
        }
    };
    function switchVariant(variant: 'Scheduled' | 'OnDeck' | 'Waiting'): void {
        currentVariant = variant;
        data = getDefaultForVariantStatus(variant);
        errors = {} as ErrorsStatus;
        tainted = {} as TaintedStatus;
    }
    function validate(): Result<Status, Array<{ field: string; message: string }>> {
        return fromObjectStatus(data);
    }
    function reset(overrides?: Partial<Status>): void {
        data = overrides ? (overrides as typeof data) : getDefaultForVariantStatus(currentVariant);
        errors = {} as ErrorsStatus;
        tainted = {} as TaintedStatus;
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
export function fromFormDataStatus(
    formData: FormData
): Result<Status, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as 'Scheduled' | 'OnDeck' | 'Waiting' | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Scheduled') {
    } else if (discriminant === 'OnDeck') {
    } else if (discriminant === 'Waiting') {
    }
    return fromStringifiedJSONStatus(JSON.stringify(obj));
}
