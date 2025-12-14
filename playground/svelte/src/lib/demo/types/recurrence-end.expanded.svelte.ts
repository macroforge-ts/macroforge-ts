import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type RecurrenceEnd = /** @default(0) */ number | string;

export function defaultValueRecurrenceEnd(): RecurrenceEnd {
    return 0;
}

export function toStringifiedJSONRecurrenceEnd(value: RecurrenceEnd): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeRecurrenceEnd(value, ctx));
}
export function toObjectRecurrenceEnd(value: RecurrenceEnd): unknown {
    const ctx = SerializeContext.create();
    return __serializeRecurrenceEnd(value, ctx);
}
export function __serializeRecurrenceEnd(value: RecurrenceEnd, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONRecurrenceEnd(
    json: string,
    opts?: DeserializeOptions
): Result<RecurrenceEnd, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectRecurrenceEnd(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectRecurrenceEnd(
    obj: unknown,
    opts?: DeserializeOptions
): Result<RecurrenceEnd, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeRecurrenceEnd(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'RecurrenceEnd.fromObject: root cannot be a forward reference'
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
export function __deserializeRecurrenceEnd(
    value: any,
    ctx: DeserializeContext
): RecurrenceEnd | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RecurrenceEnd | PendingRef;
    }
    if (typeof value === 'number') {
        return value as RecurrenceEnd;
    }
    if (typeof value === 'string') {
        return value as RecurrenceEnd;
    }
    throw new DeserializeError([
        {
            field: '_root',
            message: 'RecurrenceEnd.__deserialize: expected number, string, got ' + typeof value
        }
    ]);
}
export function isRecurrenceEnd(value: unknown): value is RecurrenceEnd {
    return typeof value === 'number' || typeof value === 'string';
}

/** Per-variant error types */ export type NumberErrorsRecurrenceEnd = {
    _errors: Option<Array<string>>;
};
export type StringErrorsRecurrenceEnd = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type NumberTaintedRecurrenceEnd = {};
export type StringTaintedRecurrenceEnd = {}; /** Union error type */
export type ErrorsRecurrenceEnd =
    | ({ _type: 'number' } & NumberErrorsRecurrenceEnd)
    | ({ _type: 'string' } & StringErrorsRecurrenceEnd); /** Union tainted type */
export type TaintedRecurrenceEnd =
    | ({ _type: 'number' } & NumberTaintedRecurrenceEnd)
    | ({ _type: 'string' } & StringTaintedRecurrenceEnd); /** Per-variant field controller types */
export interface NumberFieldControllersRecurrenceEnd {}
export interface StringFieldControllersRecurrenceEnd {} /** Union Gigaform interface with variant switching */
export interface GigaformRecurrenceEnd {
    readonly currentVariant: 'number' | 'string';
    readonly data: RecurrenceEnd;
    readonly errors: ErrorsRecurrenceEnd;
    readonly tainted: TaintedRecurrenceEnd;
    readonly variants: VariantFieldsRecurrenceEnd;
    switchVariant(variant: 'number' | 'string'): void;
    validate(): Result<RecurrenceEnd, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<RecurrenceEnd>): void;
} /** Variant fields container */
export interface VariantFieldsRecurrenceEnd {
    readonly number: { readonly fields: NumberFieldControllersRecurrenceEnd };
    readonly string: { readonly fields: StringFieldControllersRecurrenceEnd };
} /** Gets default value for a specific variant */
function getDefaultForVariantRecurrenceEnd(variant: string): RecurrenceEnd {
    switch (variant) {
        case 'number':
            return 0 as RecurrenceEnd;
        case 'string':
            return '' as RecurrenceEnd;
        default:
            return 0 as RecurrenceEnd;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormRecurrenceEnd(initial?: RecurrenceEnd): GigaformRecurrenceEnd {
    const initialVariant: 'number' | 'string' = 'number';
    let currentVariant = $state<'number' | 'string'>(initialVariant);
    let data = $state<RecurrenceEnd>(initial ?? getDefaultForVariantRecurrenceEnd(initialVariant));
    let errors = $state<ErrorsRecurrenceEnd>({} as ErrorsRecurrenceEnd);
    let tainted = $state<TaintedRecurrenceEnd>({} as TaintedRecurrenceEnd);
    const variants: VariantFieldsRecurrenceEnd = {
        number: {
            fields: {} as NumberFieldControllersRecurrenceEnd
        },
        string: {
            fields: {} as StringFieldControllersRecurrenceEnd
        }
    };
    function switchVariant(variant: 'number' | 'string'): void {
        currentVariant = variant;
        data = getDefaultForVariantRecurrenceEnd(variant);
        errors = {} as ErrorsRecurrenceEnd;
        tainted = {} as TaintedRecurrenceEnd;
    }
    function validate(): Result<RecurrenceEnd, Array<{ field: string; message: string }>> {
        return RecurrenceEnd.fromObject(data);
    }
    function reset(overrides?: Partial<RecurrenceEnd>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantRecurrenceEnd(currentVariant);
        errors = {} as ErrorsRecurrenceEnd;
        tainted = {} as TaintedRecurrenceEnd;
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
export function fromFormDataRecurrenceEnd(
    formData: FormData
): Result<RecurrenceEnd, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_type') as 'number' | 'string' | null;
    if (!discriminant) {
        return Result.err([{ field: '_type', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    if (discriminant === 'number') {
    } else if (discriminant === 'string') {
    }
    return RecurrenceEnd.fromStringifiedJSON(JSON.stringify(obj));
}
