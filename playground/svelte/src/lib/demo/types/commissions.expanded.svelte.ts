import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Commissions {
    technician: string;

    salesRep: string;
}

export function defaultValueCommissions(): Commissions {
    return { technician: '', salesRep: '' } as Commissions;
}

export function toStringifiedJSONCommissions(value: Commissions): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeCommissions(value, ctx));
}
export function toObjectCommissions(value: Commissions): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeCommissions(value, ctx);
}
export function __serializeCommissions(
    value: Commissions,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Commissions', __id };
    result['technician'] = value.technician;
    result['salesRep'] = value.salesRep;
    return result;
}

export function fromStringifiedJSONCommissions(
    json: string,
    opts?: DeserializeOptions
): Result<Commissions, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectCommissions(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectCommissions(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Commissions, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeCommissions(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Commissions.fromObject: root cannot be a forward reference'
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
export function __deserializeCommissions(
    value: any,
    ctx: DeserializeContext
): Commissions | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Commissions.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('technician' in obj)) {
        errors.push({ field: 'technician', message: 'missing required field' });
    }
    if (!('salesRep' in obj)) {
        errors.push({ field: 'salesRep', message: 'missing required field' });
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
        const __raw_technician = obj['technician'] as string;
        if (__raw_technician.length === 0) {
            errors.push({ field: 'technician', message: 'must not be empty' });
        }
        instance.technician = __raw_technician;
    }
    {
        const __raw_salesRep = obj['salesRep'] as string;
        if (__raw_salesRep.length === 0) {
            errors.push({ field: 'salesRep', message: 'must not be empty' });
        }
        instance.salesRep = __raw_salesRep;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Commissions;
}
export function validateFieldCommissions<K extends keyof Commissions>(
    field: K,
    value: Commissions[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'technician': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'technician', message: 'must not be empty' });
            }
            break;
        }
        case 'salesRep': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'salesRep', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsCommissions(
    partial: Partial<Commissions>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('technician' in partial && partial.technician !== undefined) {
        const __val = partial.technician as string;
        if (__val.length === 0) {
            errors.push({ field: 'technician', message: 'must not be empty' });
        }
    }
    if ('salesRep' in partial && partial.salesRep !== undefined) {
        const __val = partial.salesRep as string;
        if (__val.length === 0) {
            errors.push({ field: 'salesRep', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeCommissions(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'technician' in o && 'salesRep' in o;
}
export function isCommissions(obj: unknown): obj is Commissions {
    if (!hasShapeCommissions(obj)) {
        return false;
    }
    const result = fromObjectCommissions(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsCommissions = {
    _errors: Option<Array<string>>;
    technician: Option<Array<string>>;
    salesRep: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedCommissions = {
    technician: Option<boolean>;
    salesRep: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersCommissions {
    readonly technician: FieldController<string>;
    readonly salesRep: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformCommissions {
    readonly data: Commissions;
    readonly errors: ErrorsCommissions;
    readonly tainted: TaintedCommissions;
    readonly fields: FieldControllersCommissions;
    validate(): Result<Commissions, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Commissions>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormCommissions(overrides?: Partial<Commissions>): GigaformCommissions {
    let data = $state({ ...defaultValueCommissions(), ...overrides });
    let errors = $state<ErrorsCommissions>({
        _errors: Option.none(),
        technician: Option.none(),
        salesRep: Option.none()
    });
    let tainted = $state<TaintedCommissions>({
        technician: Option.none(),
        salesRep: Option.none()
    });
    const fields: FieldControllersCommissions = {
        technician: {
            path: ['technician'] as const,
            name: 'technician',
            constraints: { required: true },

            get: () => data.technician,
            set: (value: string) => {
                data.technician = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.technician,
            setError: (value: Option<Array<string>>) => {
                errors.technician = value;
            },
            getTainted: () => tainted.technician,
            setTainted: (value: Option<boolean>) => {
                tainted.technician = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldCommissions('technician', data.technician);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        salesRep: {
            path: ['salesRep'] as const,
            name: 'salesRep',
            constraints: { required: true },

            get: () => data.salesRep,
            set: (value: string) => {
                data.salesRep = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.salesRep,
            setError: (value: Option<Array<string>>) => {
                errors.salesRep = value;
            },
            getTainted: () => tainted.salesRep,
            setTainted: (value: Option<boolean>) => {
                tainted.salesRep = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldCommissions('salesRep', data.salesRep);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Commissions, Array<{ field: string; message: string }>> {
        return fromObjectCommissions(data);
    }
    function reset(newOverrides?: Partial<Commissions>): void {
        data = { ...defaultValueCommissions(), ...newOverrides };
        errors = { _errors: Option.none(), technician: Option.none(), salesRep: Option.none() };
        tainted = { technician: Option.none(), salesRep: Option.none() };
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
export function fromFormDataCommissions(
    formData: FormData
): Result<Commissions, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.technician = formData.get('technician') ?? '';
    obj.salesRep = formData.get('salesRep') ?? '';
    return fromStringifiedJSONCommissions(JSON.stringify(obj));
}
