import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { Account } from './account.svelte';
import { Order } from './order.svelte';

export interface Ordered {
    id: string;

    in: string | Account;

    out: string | Order;
    date: string;
}

export function defaultValueOrdered(): Ordered {
    return { id: '', in: '', out: '', date: '' } as Ordered;
}

export function toStringifiedJSONOrdered(value: Ordered): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeOrdered(value, ctx));
}
export function toObjectOrdered(value: Ordered): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeOrdered(value, ctx);
}
export function __serializeOrdered(value: Ordered, ctx: SerializeContext): Record<string, unknown> {
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

export function fromStringifiedJSONOrdered(
    json: string,
    opts?: DeserializeOptions
): Result<Ordered, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectOrdered(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectOrdered(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Ordered, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeOrdered(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Ordered.fromObject: root cannot be a forward reference'
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
export function __deserializeOrdered(value: any, ctx: DeserializeContext): Ordered | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Ordered.__deserialize: expected an object' }
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
        throw new DeserializeError(errors);
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
        throw new DeserializeError(errors);
    }
    return instance as Ordered;
}
export function validateFieldOrdered<K extends keyof Ordered>(
    field: K,
    value: Ordered[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsOrdered(
    partial: Partial<Ordered>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeOrdered(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'id' in o && 'in' in o && 'out' in o && 'date' in o;
}
export function isOrdered(obj: unknown): obj is Ordered {
    if (!hasShapeOrdered(obj)) {
        return false;
    }
    const result = fromObjectOrdered(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsOrdered = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    in: Option<Array<string>>;
    out: Option<Array<string>>;
    date: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedOrdered = {
    id: Option<boolean>;
    in: Option<boolean>;
    out: Option<boolean>;
    date: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersOrdered {
    readonly id: FieldController<string>;
    readonly in: FieldController<string | Account>;
    readonly out: FieldController<string | Order>;
    readonly date: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformOrdered {
    readonly data: Ordered;
    readonly errors: ErrorsOrdered;
    readonly tainted: TaintedOrdered;
    readonly fields: FieldControllersOrdered;
    validate(): Result<Ordered, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Ordered>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormOrdered(overrides?: Partial<Ordered>): GigaformOrdered {
    let data = $state({ ...defaultValueOrdered(), ...overrides });
    let errors = $state<ErrorsOrdered>({
        _errors: Option.none(),
        id: Option.none(),
        in: Option.none(),
        out: Option.none(),
        date: Option.none()
    });
    let tainted = $state<TaintedOrdered>({
        id: Option.none(),
        in: Option.none(),
        out: Option.none(),
        date: Option.none()
    });
    const fields: FieldControllersOrdered = {
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
                const fieldErrors = validateFieldOrdered('id', data.id);
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
                const fieldErrors = validateFieldOrdered('in', data.in);
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
                const fieldErrors = validateFieldOrdered('out', data.out);
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
                const fieldErrors = validateFieldOrdered('date', data.date);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Ordered, Array<{ field: string; message: string }>> {
        return fromObjectOrdered(data);
    }
    function reset(newOverrides?: Partial<Ordered>): void {
        data = { ...defaultValueOrdered(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataOrdered(
    formData: FormData
): Result<Ordered, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.in = formData.get('in') ?? '';
    obj.out = formData.get('out') ?? '';
    obj.date = formData.get('date') ?? '';
    return fromStringifiedJSONOrdered(JSON.stringify(obj));
}
