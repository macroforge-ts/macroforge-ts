import { emailPartsDefaultValue } from './email-parts.svelte';
import { firstNameDefaultValue } from './first-name.svelte';
import { lastNameDefaultValue } from './last-name.svelte';
import { passwordDefaultValue } from './password.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { emailPartsSerializeWithContext } from './email-parts.svelte';
import { firstNameSerializeWithContext } from './first-name.svelte';
import { lastNameSerializeWithContext } from './last-name.svelte';
import { passwordSerializeWithContext } from './password.svelte';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { emailPartsDeserializeWithContext } from './email-parts.svelte';
import { firstNameDeserializeWithContext } from './first-name.svelte';
import { lastNameDeserializeWithContext } from './last-name.svelte';
import { passwordDeserializeWithContext } from './password.svelte';
import { Result } from 'macroforge/reexports';
import { Option } from 'macroforge/reexports';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { FirstName } from './first-name.svelte';
import type { Password } from './password.svelte';
import type { EmailParts } from './email-parts.svelte';
import type { LastName } from './last-name.svelte';

export interface SignUpCredentials {
    firstName: FirstName;
    lastName: LastName;
    email: EmailParts;
    password: Password;
    rememberMe: boolean;
}

export function signUpCredentialsDefaultValue(): SignUpCredentials {
    return {
        firstName: firstNameDefaultValue(),
        lastName: lastNameDefaultValue(),
        email: emailPartsDefaultValue(),
        password: passwordDefaultValue(),
        rememberMe: false
    } as SignUpCredentials;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function signUpCredentialsSerialize(
    value: SignUpCredentials
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(signUpCredentialsSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function signUpCredentialsSerializeWithContext(
    value: SignUpCredentials,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'SignUpCredentials', __id };
    result['firstName'] = firstNameSerializeWithContext(value.firstName, ctx);
    result['lastName'] = lastNameSerializeWithContext(value.lastName, ctx);
    result['email'] = emailPartsSerializeWithContext(value.email, ctx);
    result['password'] = passwordSerializeWithContext(value.password, ctx);
    result['rememberMe'] = value.rememberMe;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function signUpCredentialsDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, SignUpCredentials> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = signUpCredentialsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'SignUpCredentials.deserialize: root cannot be a forward reference'
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
export function signUpCredentialsDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): SignUpCredentials | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'SignUpCredentials.deserializeWithContext: expected an object'
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('firstName' in obj)) {
        errors.push({ field: 'firstName', message: 'missing required field' });
    }
    if (!('lastName' in obj)) {
        errors.push({ field: 'lastName', message: 'missing required field' });
    }
    if (!('email' in obj)) {
        errors.push({ field: 'email', message: 'missing required field' });
    }
    if (!('password' in obj)) {
        errors.push({ field: 'password', message: 'missing required field' });
    }
    if (!('rememberMe' in obj)) {
        errors.push({ field: 'rememberMe', message: 'missing required field' });
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
        const __raw_firstName = obj['firstName'] as FirstName;
        {
            const __result = firstNameDeserializeWithContext(__raw_firstName, ctx);
            ctx.assignOrDefer(instance, 'firstName', __result);
        }
    }
    {
        const __raw_lastName = obj['lastName'] as LastName;
        {
            const __result = lastNameDeserializeWithContext(__raw_lastName, ctx);
            ctx.assignOrDefer(instance, 'lastName', __result);
        }
    }
    {
        const __raw_email = obj['email'] as EmailParts;
        {
            const __result = emailPartsDeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, 'email', __result);
        }
    }
    {
        const __raw_password = obj['password'] as Password;
        {
            const __result = passwordDeserializeWithContext(__raw_password, ctx);
            ctx.assignOrDefer(instance, 'password', __result);
        }
    }
    {
        const __raw_rememberMe = obj['rememberMe'] as boolean;
        instance.rememberMe = __raw_rememberMe;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as SignUpCredentials;
}
export function signUpCredentialsValidateField<K extends keyof SignUpCredentials>(
    field: K,
    value: SignUpCredentials[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function signUpCredentialsValidateFields(
    partial: Partial<SignUpCredentials>
): Array<{ field: string; message: string }> {
    return [];
}
export function signUpCredentialsHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'firstName' in o && 'lastName' in o && 'email' in o && 'password' in o && 'rememberMe' in o
    );
}
export function signUpCredentialsIs(obj: unknown): obj is SignUpCredentials {
    if (!signUpCredentialsHasShape(obj)) {
        return false;
    }
    const result = signUpCredentialsDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type SignUpCredentialsErrors = {
    _errors: Option<Array<string>>;
    firstName: Option<Array<string>>;
    lastName: Option<Array<string>>;
    email: Option<Array<string>>;
    password: Option<Array<string>>;
    rememberMe: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type SignUpCredentialsTainted = {
    firstName: Option<boolean>;
    lastName: Option<boolean>;
    email: Option<boolean>;
    password: Option<boolean>;
    rememberMe: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface SignUpCredentialsFieldControllers {
    readonly firstName: FieldController<FirstName>;
    readonly lastName: FieldController<LastName>;
    readonly email: FieldController<EmailParts>;
    readonly password: FieldController<Password>;
    readonly rememberMe: FieldController<boolean>;
} /** Gigaform instance containing reactive state and field controllers */
export interface SignUpCredentialsGigaform {
    readonly data: SignUpCredentials;
    readonly errors: SignUpCredentialsErrors;
    readonly tainted: SignUpCredentialsTainted;
    readonly fields: SignUpCredentialsFieldControllers;
    validate(): Result<SignUpCredentials, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<SignUpCredentials>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function signUpCredentialsCreateForm(
    overrides?: Partial<SignUpCredentials>
): SignUpCredentialsGigaform {
    let data = $state({ ...signUpCredentialsDefaultValue(), ...overrides });
    let errors = $state<SignUpCredentialsErrors>({
        _errors: Option.none(),
        firstName: Option.none(),
        lastName: Option.none(),
        email: Option.none(),
        password: Option.none(),
        rememberMe: Option.none()
    });
    let tainted = $state<SignUpCredentialsTainted>({
        firstName: Option.none(),
        lastName: Option.none(),
        email: Option.none(),
        password: Option.none(),
        rememberMe: Option.none()
    });
    const fields: SignUpCredentialsFieldControllers = {
        firstName: {
            path: ['firstName'] as const,
            name: 'firstName',
            constraints: { required: true },
            get: () => data.firstName,
            set: (value: FirstName) => {
                data.firstName = value;
            },
            transform: (value: FirstName): FirstName => value,
            getError: () => errors.firstName,
            setError: (value: Option<Array<string>>) => {
                errors.firstName = value;
            },
            getTainted: () => tainted.firstName,
            setTainted: (value: Option<boolean>) => {
                tainted.firstName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = signUpCredentialsValidateField('firstName', data.firstName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        lastName: {
            path: ['lastName'] as const,
            name: 'lastName',
            constraints: { required: true },
            get: () => data.lastName,
            set: (value: LastName) => {
                data.lastName = value;
            },
            transform: (value: LastName): LastName => value,
            getError: () => errors.lastName,
            setError: (value: Option<Array<string>>) => {
                errors.lastName = value;
            },
            getTainted: () => tainted.lastName,
            setTainted: (value: Option<boolean>) => {
                tainted.lastName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = signUpCredentialsValidateField('lastName', data.lastName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        email: {
            path: ['email'] as const,
            name: 'email',
            constraints: { required: true },
            get: () => data.email,
            set: (value: EmailParts) => {
                data.email = value;
            },
            transform: (value: EmailParts): EmailParts => value,
            getError: () => errors.email,
            setError: (value: Option<Array<string>>) => {
                errors.email = value;
            },
            getTainted: () => tainted.email,
            setTainted: (value: Option<boolean>) => {
                tainted.email = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = signUpCredentialsValidateField('email', data.email);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        password: {
            path: ['password'] as const,
            name: 'password',
            constraints: { required: true },
            get: () => data.password,
            set: (value: Password) => {
                data.password = value;
            },
            transform: (value: Password): Password => value,
            getError: () => errors.password,
            setError: (value: Option<Array<string>>) => {
                errors.password = value;
            },
            getTainted: () => tainted.password,
            setTainted: (value: Option<boolean>) => {
                tainted.password = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = signUpCredentialsValidateField('password', data.password);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        rememberMe: {
            path: ['rememberMe'] as const,
            name: 'rememberMe',
            constraints: { required: true },
            get: () => data.rememberMe,
            set: (value: boolean) => {
                data.rememberMe = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.rememberMe,
            setError: (value: Option<Array<string>>) => {
                errors.rememberMe = value;
            },
            getTainted: () => tainted.rememberMe,
            setTainted: (value: Option<boolean>) => {
                tainted.rememberMe = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = signUpCredentialsValidateField('rememberMe', data.rememberMe);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<SignUpCredentials, Array<{ field: string; message: string }>> {
        return signUpCredentialsDeserialize(data);
    }
    function reset(newOverrides?: Partial<SignUpCredentials>): void {
        data = { ...signUpCredentialsDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            firstName: Option.none(),
            lastName: Option.none(),
            email: Option.none(),
            password: Option.none(),
            rememberMe: Option.none()
        };
        tainted = {
            firstName: Option.none(),
            lastName: Option.none(),
            email: Option.none(),
            password: Option.none(),
            rememberMe: Option.none()
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function signUpCredentialsFromFormData(
    formData: FormData
): Result<SignUpCredentials, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const firstNameObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('firstName.')) {
                const fieldName = key.slice('firstName.'.length);
                const parts = fieldName.split('.');
                let current = firstNameObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.firstName = firstNameObj;
    }
    {
        const lastNameObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('lastName.')) {
                const fieldName = key.slice('lastName.'.length);
                const parts = fieldName.split('.');
                let current = lastNameObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.lastName = lastNameObj;
    }
    {
        const emailObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('email.')) {
                const fieldName = key.slice('email.'.length);
                const parts = fieldName.split('.');
                let current = emailObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.email = emailObj;
    }
    {
        const passwordObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('password.')) {
                const fieldName = key.slice('password.'.length);
                const parts = fieldName.split('.');
                let current = passwordObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.password = passwordObj;
    }
    {
        const rememberMeVal = formData.get('rememberMe');
        obj.rememberMe =
            rememberMeVal === 'true' || rememberMeVal === 'on' || rememberMeVal === '1';
    }
    return signUpCredentialsDeserialize(obj);
}

export const SignUpCredentials = {
    defaultValue: signUpCredentialsDefaultValue,
    serialize: signUpCredentialsSerialize,
    serializeWithContext: signUpCredentialsSerializeWithContext,
    deserialize: signUpCredentialsDeserialize,
    deserializeWithContext: signUpCredentialsDeserializeWithContext,
    validateFields: signUpCredentialsValidateFields,
    hasShape: signUpCredentialsHasShape,
    is: signUpCredentialsIs,
    createForm: signUpCredentialsCreateForm,
    fromFormData: signUpCredentialsFromFormData
} as const;
