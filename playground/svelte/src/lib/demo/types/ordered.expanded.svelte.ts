import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from 'macroforge/reexports';
import { Option } from 'macroforge/reexports';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Account } from './account.svelte';
import type { Order } from './order.svelte';

export interface Ordered {
    id: string;

    in: string | Account;

    out: string | Order;
    date: string;
}

export function orderedDefaultValue(): Ordered {
    return { id: '', in: '', out: '', date: '' } as Ordered;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function orderedSerialize(
    value: Ordered
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(orderedSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function orderedSerializeWithContext(
    value: Ordered,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Ordered', __id };
    result['id'] = value.id;
    result['in'] = value.in;
    result['out'] = value.out;
    result['date'] = value.date;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function orderedDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Ordered> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = orderedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Ordered.deserialize: root cannot be a forward reference'
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
export function orderedDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Ordered | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Ordered.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('in' in obj)) {
        errors.push({ field: 'in', message: 'missing required field' });
    }
    if (!('out' in obj)) {
        errors.push({ field: 'out', message: 'missing required field' });
    }
    if (!('date' in obj)) {
        errors.push({ field: 'date', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_in = obj['in'] as string | Account;
        instance.in = __raw_in;
    }
    {
        const __raw_out = obj['out'] as string | Order;
        instance.out = __raw_out;
    }
    {
        const __raw_date = obj['date'] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Ordered;
}
export function orderedValidateField<K extends keyof Ordered>(
    field: K,
    value: Ordered[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function orderedValidateFields(
    partial: Partial<Ordered>
): Array<{ field: string; message: string }> {
    return [];
}
export function orderedHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'id' in o && 'in' in o && 'out' in o && 'date' in o;
}
export function orderedIs(obj: unknown): obj is Ordered {
    if (!orderedHasShape(obj)) {
        return false;
    }
    const result = orderedDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type OrderedErrors = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    in: Option<Array<string>>;
    out: Option<Array<string>>;
    date: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type OrderedTainted = {
    id: Option<boolean>;
    in: Option<boolean>;
    out: Option<boolean>;
    date: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface OrderedFieldControllers {
    readonly id: FieldController<string>;
    readonly in: FieldController<string | Account>;
    readonly out: FieldController<string | Order>;
    readonly date: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface OrderedGigaform {
    readonly data: Ordered;
    readonly errors: OrderedErrors;
    readonly tainted: OrderedTainted;
    readonly fields: OrderedFieldControllers;
    validate(): Result<Ordered, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Ordered>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function orderedCreateForm(overrides?: Partial<Ordered>): OrderedGigaform {
    let data = $state({ ...orderedDefaultValue(), ...overrides });
    let errors = $state<OrderedErrors>({
        _errors: Option.none(),
        id: Option.none(),
        in: Option.none(),
        out: Option.none(),
        date: Option.none()
    });
    let tainted = $state<OrderedTainted>({
        id: Option.none(),
        in: Option.none(),
        out: Option.none(),
        date: Option.none()
    });
    const fields: OrderedFieldControllers = {
        id: {
            path: ['id'] as const,
            name: 'id',
            constraints: { required: true },
            get: () => data.id,
            set: (value: string) => {
                data.id = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.id,
            setError: (value: Option<Array<string>>) => {
                errors.id = value;
            },
            getTainted: () => tainted.id,
            setTainted: (value: Option<boolean>) => {
                tainted.id = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = orderedValidateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        in: {
            path: ['in'] as const,
            name: 'in',
            constraints: { required: true },
            get: () => data.in,
            set: (value: string | Account) => {
                data.in = value;
            },
            transform: (value: string | Account): string | Account => value,
            getError: () => errors.in,
            setError: (value: Option<Array<string>>) => {
                errors.in = value;
            },
            getTainted: () => tainted.in,
            setTainted: (value: Option<boolean>) => {
                tainted.in = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = orderedValidateField('in', data.in);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        out: {
            path: ['out'] as const,
            name: 'out',
            constraints: { required: true },
            get: () => data.out,
            set: (value: string | Order) => {
                data.out = value;
            },
            transform: (value: string | Order): string | Order => value,
            getError: () => errors.out,
            setError: (value: Option<Array<string>>) => {
                errors.out = value;
            },
            getTainted: () => tainted.out,
            setTainted: (value: Option<boolean>) => {
                tainted.out = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = orderedValidateField('out', data.out);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        date: {
            path: ['date'] as const,
            name: 'date',
            constraints: { required: true },
            get: () => data.date,
            set: (value: string) => {
                data.date = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.date,
            setError: (value: Option<Array<string>>) => {
                errors.date = value;
            },
            getTainted: () => tainted.date,
            setTainted: (value: Option<boolean>) => {
                tainted.date = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = orderedValidateField('date', data.date);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Ordered, Array<{ field: string; message: string }>> {
        return orderedDeserialize(data);
    }
    function reset(newOverrides?: Partial<Ordered>): void {
        data = { ...orderedDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            id: Option.none(),
            in: Option.none(),
            out: Option.none(),
            date: Option.none()
        };
        tainted = { id: Option.none(), in: Option.none(), out: Option.none(), date: Option.none() };
    }
    return {
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
        fields,
        validate,
        reset
    };
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function orderedFromFormData(
    formData: FormData
): Result<Ordered, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.in = formData.get('in') ?? '';
    obj.out = formData.get('out') ?? '';
    obj.date = formData.get('date') ?? '';
    return orderedDeserialize(obj);
}

export const Ordered = {
    defaultValue: orderedDefaultValue,
    serialize: orderedSerialize,
    serializeWithContext: orderedSerializeWithContext,
    deserialize: orderedDeserialize,
    deserializeWithContext: orderedDeserializeWithContext,
    validateFields: orderedValidateFields,
    hasShape: orderedHasShape,
    is: orderedIs,
    createForm: orderedCreateForm,
    fromFormData: orderedFromFormData
} as const;
