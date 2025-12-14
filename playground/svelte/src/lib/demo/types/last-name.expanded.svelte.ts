import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface LastName {
    name: string;
}

export function defaultValueLastName(): LastName {
    return { name: '' } as LastName;
}

export function toStringifiedJSONLastName(value: LastName): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeLastName(value, ctx));
}
export function toObjectLastName(value: LastName): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeLastName(value, ctx);
}
export function __serializeLastName(
    value: LastName,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'LastName', __id };
    result['name'] = value.name;
    return result;
}

export function fromStringifiedJSONLastName(
    json: string,
    opts?: DeserializeOptions
): Result<LastName, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectLastName(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectLastName(
    obj: unknown,
    opts?: DeserializeOptions
): Result<LastName, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeLastName(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'LastName.fromObject: root cannot be a forward reference'
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
export function __deserializeLastName(value: any, ctx: DeserializeContext): LastName | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'LastName.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('name' in obj)) {
        errors.push({ field: 'name', message: 'missing required field' });
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
        const __raw_name = obj['name'] as string;
        if (__raw_name.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
        instance.name = __raw_name;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as LastName;
}
export function validateFieldLastName<K extends keyof LastName>(
    field: K,
    value: LastName[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'name': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'name', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsLastName(
    partial: Partial<LastName>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('name' in partial && partial.name !== undefined) {
        const __val = partial.name as string;
        if (__val.length === 0) {
            errors.push({ field: 'name', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeLastName(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'name' in o;
}
export function isLastName(obj: unknown): obj is LastName {
    if (!hasShapeLastName(obj)) {
        return false;
    }
    const result = fromObjectLastName(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsLastName = {
    _errors: Option<Array<string>>;
    name: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedLastName = {
    name: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersLastName {
    readonly name: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformLastName {
    readonly data: LastName;
    readonly errors: ErrorsLastName;
    readonly tainted: TaintedLastName;
    readonly fields: FieldControllersLastName;
    validate(): Result<LastName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<LastName>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormLastName(overrides?: Partial<LastName>): GigaformLastName {
    let data = $state({ ...LastName.defaultValue(), ...overrides });
    let errors = $state<ErrorsLastName>({ _errors: Option.none(), name: Option.none() });
    let tainted = $state<TaintedLastName>({ name: Option.none() });
    const fields: FieldControllersLastName = {
        name: {
            path: ['name'] as const,
            name: 'name',
            constraints: { required: true },

            get: () => data.name,
            set: (value: string) => {
                data.name = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.name,
            setError: (value: Option<Array<string>>) => {
                errors.name = value;
            },
            getTainted: () => tainted.name,
            setTainted: (value: Option<boolean>) => {
                tainted.name = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = LastName.validateField('name', data.name);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<LastName, Array<{ field: string; message: string }>> {
        return LastName.fromObject(data);
    }
    function reset(newOverrides?: Partial<LastName>): void {
        data = { ...LastName.defaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), name: Option.none() };
        tainted = { name: Option.none() };
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
export function fromFormDataLastName(
    formData: FormData
): Result<LastName, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.name = formData.get('name') ?? '';
    return LastName.fromStringifiedJSON(JSON.stringify(obj));
}
