import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Password {
    password: string;
}

export function passwordDefaultValue(): Password {
    return { password: '' } as Password;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function passwordSerialize(
    value: Password
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(passwordSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function passwordSerializeWithContext(
    value: Password,
    ctx: __mf_SerializeContext
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

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function passwordDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Password> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = passwordDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Password.deserialize: root cannot be a forward reference'
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
export function passwordDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Password | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Password.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('password' in obj)) {
        errors.push({ field: 'password', message: 'missing required field' });
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
        const __raw_password = obj['password'] as string;
        if (__raw_password.length === 0) {
            errors.push({ field: 'password', message: 'must not be empty' });
        }
        instance.password = __raw_password;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Password;
}
export function passwordValidateField<K extends keyof Password>(
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
export function passwordValidateFields(
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
export function passwordHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'password' in o;
}
export function passwordIs(obj: unknown): obj is Password {
    if (!passwordHasShape(obj)) {
        return false;
    }
    const result = passwordDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type PasswordErrors = {
    _errors: Option<Array<string>>;
    password: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type PasswordTainted = {
    password: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface PasswordFieldControllers {
    readonly password: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface PasswordGigaform {
    readonly data: Password;
    readonly errors: PasswordErrors;
    readonly tainted: PasswordTainted;
    readonly fields: PasswordFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Password>;
    reset(overrides?: Partial<Password>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function passwordCreateForm(overrides?: Partial<Password>): PasswordGigaform {
    let data = $state({ ...passwordDefaultValue(), ...overrides });
    let errors = $state<PasswordErrors>({ _errors: optionNone(), password: optionNone() });
    let tainted = $state<PasswordTainted>({ password: optionNone() });
    const fields: PasswordFieldControllers = {
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
                const fieldErrors = passwordValidateField('password', data.password);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Password> {
        return passwordDeserialize(data);
    }
    function reset(newOverrides?: Partial<Password>): void {
        data = { ...passwordDefaultValue(), ...newOverrides };
        errors = { _errors: optionNone(), password: optionNone() };
        tainted = { password: optionNone() };
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
export function passwordFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Password> {
    const obj: Record<string, unknown> = {};
    obj.password = formData.get('password') ?? '';
    return passwordDeserialize(obj);
}

export const Password = {
    defaultValue: passwordDefaultValue,
    serialize: passwordSerialize,
    serializeWithContext: passwordSerializeWithContext,
    deserialize: passwordDeserialize,
    deserializeWithContext: passwordDeserializeWithContext,
    validateFields: passwordValidateFields,
    hasShape: passwordHasShape,
    is: passwordIs,
    createForm: passwordCreateForm,
    fromFormData: passwordFromFormData
} as const;
