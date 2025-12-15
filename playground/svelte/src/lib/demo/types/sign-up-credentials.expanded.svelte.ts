import { defaultValueEmailParts } from './email-parts.svelte';
import { defaultValueFirstName } from './first-name.svelte';
import { defaultValueLastName } from './last-name.svelte';
import { defaultValuePassword } from './password.svelte';
import { SerializeContext } from 'macroforge/serde';
import { __serializeEmailParts } from './email-parts.svelte';
import { __serializeFirstName } from './first-name.svelte';
import { __serializeLastName } from './last-name.svelte';
import { __serializePassword } from './password.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeEmailParts } from './email-parts.svelte';
import { __deserializeFirstName } from './first-name.svelte';
import { __deserializeLastName } from './last-name.svelte';
import { __deserializePassword } from './password.svelte';
import { Option } from 'macroforge/utils';
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

export function defaultValueSignUpCredentials(): SignUpCredentials {
    return {
        firstName: defaultValueFirstName(),
        lastName: defaultValueLastName(),
        email: defaultValueEmailParts(),
        password: defaultValuePassword(),
        rememberMe: false
    } as SignUpCredentials;
}

export function toStringifiedJSONSignUpCredentials(value: SignUpCredentials): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeSignUpCredentials(value, ctx));
}
export function toObjectSignUpCredentials(value: SignUpCredentials): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeSignUpCredentials(value, ctx);
}
export function __serializeSignUpCredentials(
    value: SignUpCredentials,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'SignUpCredentials', __id };
    result['firstName'] = __serializeFirstName(value.firstName, ctx);
    result['lastName'] = __serializeLastName(value.lastName, ctx);
    result['email'] = __serializeEmailParts(value.email, ctx);
    result['password'] = __serializePassword(value.password, ctx);
    result['rememberMe'] = value.rememberMe;
    return result;
}

export function fromStringifiedJSONSignUpCredentials(
    json: string,
    opts?: DeserializeOptions
): Result<SignUpCredentials, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectSignUpCredentials(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectSignUpCredentials(
    obj: unknown,
    opts?: DeserializeOptions
): Result<SignUpCredentials, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeSignUpCredentials(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'SignUpCredentials.fromObject: root cannot be a forward reference'
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
export function __deserializeSignUpCredentials(
    value: any,
    ctx: DeserializeContext
): SignUpCredentials | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'SignUpCredentials.__deserialize: expected an object' }
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
        throw new DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_firstName = obj['firstName'] as FirstName;
        {
            const __result = __deserializeFirstName(__raw_firstName, ctx);
            ctx.assignOrDefer(instance, 'firstName', __result);
        }
    }
    {
        const __raw_lastName = obj['lastName'] as LastName;
        {
            const __result = __deserializeLastName(__raw_lastName, ctx);
            ctx.assignOrDefer(instance, 'lastName', __result);
        }
    }
    {
        const __raw_email = obj['email'] as EmailParts;
        {
            const __result = __deserializeEmailParts(__raw_email, ctx);
            ctx.assignOrDefer(instance, 'email', __result);
        }
    }
    {
        const __raw_password = obj['password'] as Password;
        {
            const __result = __deserializePassword(__raw_password, ctx);
            ctx.assignOrDefer(instance, 'password', __result);
        }
    }
    {
        const __raw_rememberMe = obj['rememberMe'] as boolean;
        instance.rememberMe = __raw_rememberMe;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as SignUpCredentials;
}
export function validateFieldSignUpCredentials<K extends keyof SignUpCredentials>(
    field: K,
    value: SignUpCredentials[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsSignUpCredentials(
    partial: Partial<SignUpCredentials>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeSignUpCredentials(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'firstName' in o && 'lastName' in o && 'email' in o && 'password' in o && 'rememberMe' in o
    );
}
export function isSignUpCredentials(obj: unknown): obj is SignUpCredentials {
    if (!hasShapeSignUpCredentials(obj)) {
        return false;
    }
    const result = fromObjectSignUpCredentials(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsSignUpCredentials = {
    _errors: Option<Array<string>>;
    firstName: Option<Array<string>>;
    lastName: Option<Array<string>>;
    email: Option<Array<string>>;
    password: Option<Array<string>>;
    rememberMe: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedSignUpCredentials = {
    firstName: Option<boolean>;
    lastName: Option<boolean>;
    email: Option<boolean>;
    password: Option<boolean>;
    rememberMe: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersSignUpCredentials {
    readonly firstName: FieldController<FirstName>;
    readonly lastName: FieldController<LastName>;
    readonly email: FieldController<EmailParts>;
    readonly password: FieldController<Password>;
    readonly rememberMe: FieldController<boolean>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformSignUpCredentials {
    readonly data: SignUpCredentials;
    readonly errors: ErrorsSignUpCredentials;
    readonly tainted: TaintedSignUpCredentials;
    readonly fields: FieldControllersSignUpCredentials;
    validate(): Result<SignUpCredentials, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<SignUpCredentials>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormSignUpCredentials(
    overrides?: Partial<SignUpCredentials>
): GigaformSignUpCredentials {
    let data = $state({ ...defaultValueSignUpCredentials(), ...overrides });
    let errors = $state<ErrorsSignUpCredentials>({
        _errors: Option.none(),
        firstName: Option.none(),
        lastName: Option.none(),
        email: Option.none(),
        password: Option.none(),
        rememberMe: Option.none()
    });
    let tainted = $state<TaintedSignUpCredentials>({
        firstName: Option.none(),
        lastName: Option.none(),
        email: Option.none(),
        password: Option.none(),
        rememberMe: Option.none()
    });
    const fields: FieldControllersSignUpCredentials = {
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
                const fieldErrors = validateFieldSignUpCredentials('firstName', data.firstName);
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
                const fieldErrors = validateFieldSignUpCredentials('lastName', data.lastName);
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
                const fieldErrors = validateFieldSignUpCredentials('email', data.email);
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
                const fieldErrors = validateFieldSignUpCredentials('password', data.password);
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
                const fieldErrors = validateFieldSignUpCredentials('rememberMe', data.rememberMe);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<SignUpCredentials, Array<{ field: string; message: string }>> {
        return fromObjectSignUpCredentials(data);
    }
    function reset(newOverrides?: Partial<SignUpCredentials>): void {
        data = { ...defaultValueSignUpCredentials(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataSignUpCredentials(
    formData: FormData
): Result<SignUpCredentials, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        // Collect nested object fields with prefix "firstName."
        const firstNameObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('firstName.')) {
                const fieldName = key.slice('firstName.'.length);
                // Handle deeper nesting by splitting on dots
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
        // Collect nested object fields with prefix "lastName."
        const lastNameObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('lastName.')) {
                const fieldName = key.slice('lastName.'.length);
                // Handle deeper nesting by splitting on dots
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
        // Collect nested object fields with prefix "email."
        const emailObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('email.')) {
                const fieldName = key.slice('email.'.length);
                // Handle deeper nesting by splitting on dots
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
        // Collect nested object fields with prefix "password."
        const passwordObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('password.')) {
                const fieldName = key.slice('password.'.length);
                // Handle deeper nesting by splitting on dots
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
    return fromStringifiedJSONSignUpCredentials(JSON.stringify(obj));
}
