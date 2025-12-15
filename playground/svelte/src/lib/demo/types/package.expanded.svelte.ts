import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Package {
    id: string;

    date: string;
}

export function defaultValuePackage(): Package {
    return { id: '', date: '' } as Package;
}

export function toStringifiedJSONPackage(value: Package): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializePackage(value, ctx));
}
export function toObjectPackage(value: Package): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializePackage(value, ctx);
}
export function __serializePackage(value: Package, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Package', __id };
    result['id'] = value.id;
    result['date'] = value.date;
    return result;
}

export function fromStringifiedJSONPackage(
    json: string,
    opts?: DeserializeOptions
): Result<Package, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectPackage(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectPackage(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Package, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializePackage(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Package.fromObject: root cannot be a forward reference'
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
export function __deserializePackage(value: any, ctx: DeserializeContext): Package | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Package.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
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
        const __raw_date = obj['date'] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Package;
}
export function validateFieldPackage<K extends keyof Package>(
    field: K,
    value: Package[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsPackage(
    partial: Partial<Package>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapePackage(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'id' in o && 'date' in o;
}
export function isPackage(obj: unknown): obj is Package {
    if (!hasShapePackage(obj)) {
        return false;
    }
    const result = fromObjectPackage(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsPackage = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    date: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedPackage = {
    id: Option<boolean>;
    date: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersPackage {
    readonly id: FieldController<string>;
    readonly date: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformPackage {
    readonly data: Package;
    readonly errors: ErrorsPackage;
    readonly tainted: TaintedPackage;
    readonly fields: FieldControllersPackage;
    validate(): Result<Package, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Package>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormPackage(overrides?: Partial<Package>): GigaformPackage {
    let data = $state({ ...defaultValuePackage(), ...overrides });
    let errors = $state<ErrorsPackage>({
        _errors: Option.none(),
        id: Option.none(),
        date: Option.none()
    });
    let tainted = $state<TaintedPackage>({ id: Option.none(), date: Option.none() });
    const fields: FieldControllersPackage = {
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
                const fieldErrors = validateFieldPackage('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        date: {
            path: ['date'] as const,
            name: 'date',
            constraints: { required: true },
            label: 'Date',
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
                const fieldErrors = validateFieldPackage('date', data.date);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Package, Array<{ field: string; message: string }>> {
        return fromObjectPackage(data);
    }
    function reset(newOverrides?: Partial<Package>): void {
        data = { ...defaultValuePackage(), ...newOverrides };
        errors = { _errors: Option.none(), id: Option.none(), date: Option.none() };
        tainted = { id: Option.none(), date: Option.none() };
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
export function fromFormDataPackage(
    formData: FormData
): Result<Package, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.date = formData.get('date') ?? '';
    return fromStringifiedJSONPackage(JSON.stringify(obj));
}
