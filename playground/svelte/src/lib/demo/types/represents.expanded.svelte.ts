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
import { Employee } from './employee.svelte';

export interface Represents {
    in: string | Employee;

    out: string | Account;
    id: string;
    dateStarted: string;
}

export function defaultValueRepresents(): Represents {
    return { in: '', out: '', id: '', dateStarted: '' } as Represents;
}

export function toStringifiedJSONRepresents(value: Represents): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeRepresents(value, ctx));
}
export function toObjectRepresents(value: Represents): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeRepresents(value, ctx);
}
export function __serializeRepresents(
    value: Represents,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Represents', __id };
    result['in'] = value.in;
    result['out'] = value.out;
    result['id'] = value.id;
    result['dateStarted'] = value.dateStarted;
    return result;
}

export function fromStringifiedJSONRepresents(
    json: string,
    opts?: DeserializeOptions
): Result<Represents, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectRepresents(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectRepresents(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Represents, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeRepresents(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Represents.fromObject: root cannot be a forward reference'
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
export function __deserializeRepresents(
    value: any,
    ctx: DeserializeContext
): Represents | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Represents.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('in' in obj)) {
        errors.push({ field: 'in', message: 'missing required field' });
    }
    if (!('out' in obj)) {
        errors.push({ field: 'out', message: 'missing required field' });
    }
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('dateStarted' in obj)) {
        errors.push({ field: 'dateStarted', message: 'missing required field' });
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
        const __raw_in = obj['in'] as string | Employee;
        instance.in = __raw_in;
    }
    {
        const __raw_out = obj['out'] as string | Account;
        instance.out = __raw_out;
    }
    {
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_dateStarted = obj['dateStarted'] as string;
        instance.dateStarted = __raw_dateStarted;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Represents;
}
export function validateFieldRepresents<K extends keyof Represents>(
    field: K,
    value: Represents[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsRepresents(
    partial: Partial<Represents>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeRepresents(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'in' in o && 'out' in o && 'id' in o && 'dateStarted' in o;
}
export function isRepresents(obj: unknown): obj is Represents {
    if (!hasShapeRepresents(obj)) {
        return false;
    }
    const result = fromObjectRepresents(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsRepresents = {
    _errors: Option<Array<string>>;
    in: Option<Array<string>>;
    out: Option<Array<string>>;
    id: Option<Array<string>>;
    dateStarted: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedRepresents = {
    in: Option<boolean>;
    out: Option<boolean>;
    id: Option<boolean>;
    dateStarted: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersRepresents {
    readonly in: FieldController<string | Employee>;
    readonly out: FieldController<string | Account>;
    readonly id: FieldController<string>;
    readonly dateStarted: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformRepresents {
    readonly data: Represents;
    readonly errors: ErrorsRepresents;
    readonly tainted: TaintedRepresents;
    readonly fields: FieldControllersRepresents;
    validate(): Result<Represents, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Represents>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormRepresents(overrides?: Partial<Represents>): GigaformRepresents {
    let data = $state({ ...defaultValueRepresents(), ...overrides });
    let errors = $state<ErrorsRepresents>({
        _errors: Option.none(),
        in: Option.none(),
        out: Option.none(),
        id: Option.none(),
        dateStarted: Option.none()
    });
    let tainted = $state<TaintedRepresents>({
        in: Option.none(),
        out: Option.none(),
        id: Option.none(),
        dateStarted: Option.none()
    });
    const fields: FieldControllersRepresents = {
        in: {
            path: ['in'] as const,
            name: 'in',
            constraints: { required: true },

            get: () => data.in,
            set: (value: string | Employee) => {
                data.in = value;
            },
            transform: (value: string | Employee): string | Employee => value,
            getError: () => errors.in,
            setError: (value: Option<Array<string>>) => {
                errors.in = value;
            },
            getTainted: () => tainted.in,
            setTainted: (value: Option<boolean>) => {
                tainted.in = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldRepresents('in', data.in);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        out: {
            path: ['out'] as const,
            name: 'out',
            constraints: { required: true },

            get: () => data.out,
            set: (value: string | Account) => {
                data.out = value;
            },
            transform: (value: string | Account): string | Account => value,
            getError: () => errors.out,
            setError: (value: Option<Array<string>>) => {
                errors.out = value;
            },
            getTainted: () => tainted.out,
            setTainted: (value: Option<boolean>) => {
                tainted.out = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldRepresents('out', data.out);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
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
                const fieldErrors = validateFieldRepresents('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        dateStarted: {
            path: ['dateStarted'] as const,
            name: 'dateStarted',
            constraints: { required: true },

            get: () => data.dateStarted,
            set: (value: string) => {
                data.dateStarted = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.dateStarted,
            setError: (value: Option<Array<string>>) => {
                errors.dateStarted = value;
            },
            getTainted: () => tainted.dateStarted,
            setTainted: (value: Option<boolean>) => {
                tainted.dateStarted = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldRepresents('dateStarted', data.dateStarted);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Represents, Array<{ field: string; message: string }>> {
        return fromObjectRepresents(data);
    }
    function reset(newOverrides?: Partial<Represents>): void {
        data = { ...defaultValueRepresents(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            in: Option.none(),
            out: Option.none(),
            id: Option.none(),
            dateStarted: Option.none()
        };
        tainted = {
            in: Option.none(),
            out: Option.none(),
            id: Option.none(),
            dateStarted: Option.none()
        };
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
export function fromFormDataRepresents(
    formData: FormData
): Result<Represents, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.in = formData.get('in') ?? '';
    obj.out = formData.get('out') ?? '';
    obj.id = formData.get('id') ?? '';
    obj.dateStarted = formData.get('dateStarted') ?? '';
    return fromStringifiedJSONRepresents(JSON.stringify(obj));
}
