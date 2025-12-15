import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Password {
    password: string;
}

export function defaultValuePassword(): Password {
    return { password: '' } as Password;
}

export function toStringifiedJSONPassword(value: Password): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializePassword(value, ctx));
}
export function toObjectPassword(value: Password): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializePassword(value, ctx);
}
export function __serializePassword(
    value: Password,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Password', __id };
    result['password'] = value.password;
    return result;
}

export function fromStringifiedJSONPassword(
    json: string,
    opts?: DeserializeOptions
): Result<Password, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectPassword(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectPassword(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Password, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializePassword(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Password.fromObject: root cannot be a forward reference'
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
export function __deserializePassword(value: any, ctx: DeserializeContext): Password | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Password.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('password' in obj)) {
        errors.push({ field: 'password', message: 'missing required field' });
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
        const __raw_password = obj['password'] as string;
        if (__raw_password.length === 0) {
            errors.push({ field: 'password', message: 'must not be empty' });
        }
        instance.password = __raw_password;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Password;
}
export function validateFieldPassword<K extends keyof Password>(
    field: K,
    value: Password[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'password': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'password', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsPassword(
    partial: Partial<Password>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('password' in partial && partial.password !== undefined) {
        const __val = partial.password as string;
        if (__val.length === 0) {
            errors.push({ field: 'password', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapePassword(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'password' in o;
}
export function isPassword(obj: unknown): obj is Password {
    if (!hasShapePassword(obj)) {
        return false;
    }
    const result = fromObjectPassword(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsPassword = {
    _errors: Option<Array<string>>;
    password: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedPassword = {
    password: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersPassword {
    readonly password: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformPassword {
    readonly data: Password;
    readonly errors: ErrorsPassword;
    readonly tainted: TaintedPassword;
    readonly fields: FieldControllersPassword;
    validate(): Result<Password, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Password>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormPassword(overrides?: Partial<Password>): GigaformPassword {
    let data = $state({ ...defaultValuePassword(), ...overrides });
    let errors = $state<ErrorsPassword>({ _errors: Option.none(), password: Option.none() });
    let tainted = $state<TaintedPassword>({ password: Option.none() });
    const fields: FieldControllersPassword = {
        password: {
            path: ['password'] as const,
            name: 'password',
            constraints: { required: true },

            get: () => data.password,
            set: (value: string) => {
                data.password = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.password,
            setError: (value: Option<Array<string>>) => {
                errors.password = value;
            },
            getTainted: () => tainted.password,
            setTainted: (value: Option<boolean>) => {
                tainted.password = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldPassword('password', data.password);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Password, Array<{ field: string; message: string }>> {
        return fromObjectPassword(data);
    }
    function reset(newOverrides?: Partial<Password>): void {
        data = { ...defaultValuePassword(), ...newOverrides };
        errors = { _errors: Option.none(), password: Option.none() };
        tainted = { password: Option.none() };
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
export function fromFormDataPassword(
    formData: FormData
): Result<Password, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.password = formData.get('password') ?? '';
    return fromStringifiedJSONPassword(JSON.stringify(obj));
}
