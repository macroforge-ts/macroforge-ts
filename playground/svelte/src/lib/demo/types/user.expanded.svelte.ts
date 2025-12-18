import { appPermissionsDefaultValue } from './app-permissions.svelte';
import { settingsDefaultValue } from './settings.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { appPermissionsSerializeWithContext } from './app-permissions.svelte';
import { settingsSerializeWithContext } from './settings.svelte';
import { userRoleSerializeWithContext } from './user-role.svelte';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { appPermissionsDeserializeWithContext } from './app-permissions.svelte';
import { settingsDeserializeWithContext } from './settings.svelte';
import { userRoleDeserializeWithContext } from './user-role.svelte';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { DateTime } from 'effect';
import type { Option } from 'effect/Option';
import type { Metadata } from './metadata.svelte';
import type { Settings } from './settings.svelte';
import type { AppPermissions } from './app-permissions.svelte';
import type { UserRole } from './user-role.svelte';

export interface User {
    id: string;
    email: Option<string>;

    firstName: string;

    lastName: string;
    password: Option<string>;
    metadata: Option<Metadata>;
    settings: Settings;

    role: UserRole;
    emailVerified: boolean;
    verificationToken: Option<string>;
    verificationExpires: Option<DateTime.DateTime>;
    passwordResetToken: Option<string>;
    passwordResetExpires: Option<DateTime.DateTime>;
    permissions: AppPermissions;
    createdAt: DateTime.DateTime;
    lastLoginAt: Option<DateTime.DateTime>;
}

export function userDefaultValue(): User {
    return {
        id: '',
        email: (() => Option.none())(),
        firstName: '',
        lastName: '',
        password: (() => Option.none())(),
        metadata: (() => Option.none())(),
        settings: settingsDefaultValue(),
        role: 'Administrator',
        emailVerified: false,
        verificationToken: (() => Option.none())(),
        verificationExpires: (() => Option.none())(),
        passwordResetToken: (() => Option.none())(),
        passwordResetExpires: (() => Option.none())(),
        permissions: appPermissionsDefaultValue(),
        createdAt: (() => DateTime.unsafeNow())(),
        lastLoginAt: (() => Option.none())()
    } as User;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function userSerialize(
    value: User
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(userSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function userSerializeWithContext(
    value: User,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'User', __id };
    result['id'] = value.id;
    result['email'] = ((v: Option.Option<unknown>) => Option.getOrNull(v))(value.email);
    result['firstName'] = value.firstName;
    result['lastName'] = value.lastName;
    result['password'] = ((v: Option.Option<unknown>) => Option.getOrNull(v))(value.password);
    result['metadata'] = ((v: Option.Option<unknown>) => Option.getOrNull(v))(value.metadata);
    result['settings'] = settingsSerializeWithContext(value.settings, ctx);
    result['role'] = userRoleSerializeWithContext(value.role, ctx);
    result['emailVerified'] = value.emailVerified;
    result['verificationToken'] = ((v: Option.Option<unknown>) => Option.getOrNull(v))(
        value.verificationToken
    );
    result['verificationExpires'] = ((v: Option.Option<unknown>) => Option.getOrNull(v))(
        value.verificationExpires
    );
    result['passwordResetToken'] = ((v: Option.Option<unknown>) => Option.getOrNull(v))(
        value.passwordResetToken
    );
    result['passwordResetExpires'] = ((v: Option.Option<unknown>) => Option.getOrNull(v))(
        value.passwordResetExpires
    );
    result['permissions'] = appPermissionsSerializeWithContext(value.permissions, ctx);
    result['createdAt'] = ((v: DateTime.DateTime) => DateTime.formatIso(v))(value.createdAt);
    result['lastLoginAt'] = ((v: Option.Option<unknown>) => Option.getOrNull(v))(value.lastLoginAt);
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function userDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, User> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = userDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                { field: '_root', message: 'User.deserialize: root cannot be a forward reference' }
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
export function userDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): User | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'User.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('email' in obj)) {
        errors.push({ field: 'email', message: 'missing required field' });
    }
    if (!('firstName' in obj)) {
        errors.push({ field: 'firstName', message: 'missing required field' });
    }
    if (!('lastName' in obj)) {
        errors.push({ field: 'lastName', message: 'missing required field' });
    }
    if (!('password' in obj)) {
        errors.push({ field: 'password', message: 'missing required field' });
    }
    if (!('metadata' in obj)) {
        errors.push({ field: 'metadata', message: 'missing required field' });
    }
    if (!('settings' in obj)) {
        errors.push({ field: 'settings', message: 'missing required field' });
    }
    if (!('role' in obj)) {
        errors.push({ field: 'role', message: 'missing required field' });
    }
    if (!('emailVerified' in obj)) {
        errors.push({ field: 'emailVerified', message: 'missing required field' });
    }
    if (!('verificationToken' in obj)) {
        errors.push({ field: 'verificationToken', message: 'missing required field' });
    }
    if (!('verificationExpires' in obj)) {
        errors.push({ field: 'verificationExpires', message: 'missing required field' });
    }
    if (!('passwordResetToken' in obj)) {
        errors.push({ field: 'passwordResetToken', message: 'missing required field' });
    }
    if (!('passwordResetExpires' in obj)) {
        errors.push({ field: 'passwordResetExpires', message: 'missing required field' });
    }
    if (!('permissions' in obj)) {
        errors.push({ field: 'permissions', message: 'missing required field' });
    }
    if (!('createdAt' in obj)) {
        errors.push({ field: 'createdAt', message: 'missing required field' });
    }
    if (!('lastLoginAt' in obj)) {
        errors.push({ field: 'lastLoginAt', message: 'missing required field' });
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
    instance.email = ((raw: unknown) => (raw === null ? Option.none() : Option.some(raw)))(
        obj['email']
    );
    {
        const __raw_firstName = obj['firstName'] as string;
        if (__raw_firstName.length === 0) {
            errors.push({ field: 'firstName', message: 'must not be empty' });
        }
        instance.firstName = __raw_firstName;
    }
    {
        const __raw_lastName = obj['lastName'] as string;
        if (__raw_lastName.length === 0) {
            errors.push({ field: 'lastName', message: 'must not be empty' });
        }
        instance.lastName = __raw_lastName;
    }
    instance.password = ((raw: unknown) => (raw === null ? Option.none() : Option.some(raw)))(
        obj['password']
    );
    instance.metadata = ((raw: unknown) => (raw === null ? Option.none() : Option.some(raw)))(
        obj['metadata']
    );
    {
        const __raw_settings = obj['settings'] as Settings;
        {
            const __result = settingsDeserializeWithContext(__raw_settings, ctx);
            ctx.assignOrDefer(instance, 'settings', __result);
        }
    }
    {
        const __raw_role = obj['role'] as UserRole;
        {
            const __result = userRoleDeserializeWithContext(__raw_role, ctx);
            ctx.assignOrDefer(instance, 'role', __result);
        }
    }
    {
        const __raw_emailVerified = obj['emailVerified'] as boolean;
        instance.emailVerified = __raw_emailVerified;
    }
    instance.verificationToken = ((raw: unknown) =>
        raw === null ? Option.none() : Option.some(raw))(obj['verificationToken']);
    instance.verificationExpires = ((raw: unknown) =>
        raw === null ? Option.none() : Option.some(raw))(obj['verificationExpires']);
    instance.passwordResetToken = ((raw: unknown) =>
        raw === null ? Option.none() : Option.some(raw))(obj['passwordResetToken']);
    instance.passwordResetExpires = ((raw: unknown) =>
        raw === null ? Option.none() : Option.some(raw))(obj['passwordResetExpires']);
    {
        const __raw_permissions = obj['permissions'] as AppPermissions;
        {
            const __result = appPermissionsDeserializeWithContext(__raw_permissions, ctx);
            ctx.assignOrDefer(instance, 'permissions', __result);
        }
    }
    instance.createdAt = ((raw: unknown) => DateTime.unsafeFromDate(new Date(raw as string)))(
        obj['createdAt']
    );
    instance.lastLoginAt = ((raw: unknown) => (raw === null ? Option.none() : Option.some(raw)))(
        obj['lastLoginAt']
    );
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as User;
}
export function userValidateField<K extends keyof User>(
    field: K,
    value: User[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'firstName': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'firstName', message: 'must not be empty' });
            }
            break;
        }
        case 'lastName': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'lastName', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function userValidateFields(
    partial: Partial<User>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('firstName' in partial && partial.firstName !== undefined) {
        const __val = partial.firstName as string;
        if (__val.length === 0) {
            errors.push({ field: 'firstName', message: 'must not be empty' });
        }
    }
    if ('lastName' in partial && partial.lastName !== undefined) {
        const __val = partial.lastName as string;
        if (__val.length === 0) {
            errors.push({ field: 'lastName', message: 'must not be empty' });
        }
    }
    return errors;
}
export function userHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'email' in o &&
        'firstName' in o &&
        'lastName' in o &&
        'password' in o &&
        'metadata' in o &&
        'settings' in o &&
        'role' in o &&
        'emailVerified' in o &&
        'verificationToken' in o &&
        'verificationExpires' in o &&
        'passwordResetToken' in o &&
        'passwordResetExpires' in o &&
        'permissions' in o &&
        'createdAt' in o &&
        'lastLoginAt' in o
    );
}
export function userIs(obj: unknown): obj is User {
    if (!userHasShape(obj)) {
        return false;
    }
    const result = userDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type UserErrors = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    email: Option<Array<string>>;
    firstName: Option<Array<string>>;
    lastName: Option<Array<string>>;
    password: Option<Array<string>>;
    metadata: Option<Array<string>>;
    settings: Option<Array<string>>;
    role: Option<Array<string>>;
    emailVerified: Option<Array<string>>;
    verificationToken: Option<Array<string>>;
    verificationExpires: Option<Array<string>>;
    passwordResetToken: Option<Array<string>>;
    passwordResetExpires: Option<Array<string>>;
    permissions: Option<Array<string>>;
    createdAt: Option<Array<string>>;
    lastLoginAt: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type UserTainted = {
    id: Option<boolean>;
    email: Option<boolean>;
    firstName: Option<boolean>;
    lastName: Option<boolean>;
    password: Option<boolean>;
    metadata: Option<boolean>;
    settings: Option<boolean>;
    role: Option<boolean>;
    emailVerified: Option<boolean>;
    verificationToken: Option<boolean>;
    verificationExpires: Option<boolean>;
    passwordResetToken: Option<boolean>;
    passwordResetExpires: Option<boolean>;
    permissions: Option<boolean>;
    createdAt: Option<boolean>;
    lastLoginAt: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface UserFieldControllers {
    readonly id: FieldController<string>;
    readonly email: FieldController<Option<string>>;
    readonly firstName: FieldController<string>;
    readonly lastName: FieldController<string>;
    readonly password: FieldController<Option<string>>;
    readonly metadata: FieldController<Option<Metadata>>;
    readonly settings: FieldController<Settings>;
    readonly role: FieldController<UserRole>;
    readonly emailVerified: FieldController<boolean>;
    readonly verificationToken: FieldController<Option<string>>;
    readonly verificationExpires: FieldController<Option<DateTime.DateTime>>;
    readonly passwordResetToken: FieldController<Option<string>>;
    readonly passwordResetExpires: FieldController<Option<DateTime.DateTime>>;
    readonly permissions: FieldController<AppPermissions>;
    readonly createdAt: FieldController<DateTime.DateTime>;
    readonly lastLoginAt: FieldController<Option<DateTime.DateTime>>;
} /** Gigaform instance containing reactive state and field controllers */
export interface UserGigaform {
    readonly data: User;
    readonly errors: UserErrors;
    readonly tainted: UserTainted;
    readonly fields: UserFieldControllers;
    validate(): Result<User, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<User>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function userCreateForm(overrides?: Partial<User>): UserGigaform {
    let data = $state({ ...userDefaultValue(), ...overrides });
    let errors = $state<UserErrors>({
        _errors: Option.none(),
        id: Option.none(),
        email: Option.none(),
        firstName: Option.none(),
        lastName: Option.none(),
        password: Option.none(),
        metadata: Option.none(),
        settings: Option.none(),
        role: Option.none(),
        emailVerified: Option.none(),
        verificationToken: Option.none(),
        verificationExpires: Option.none(),
        passwordResetToken: Option.none(),
        passwordResetExpires: Option.none(),
        permissions: Option.none(),
        createdAt: Option.none(),
        lastLoginAt: Option.none()
    });
    let tainted = $state<UserTainted>({
        id: Option.none(),
        email: Option.none(),
        firstName: Option.none(),
        lastName: Option.none(),
        password: Option.none(),
        metadata: Option.none(),
        settings: Option.none(),
        role: Option.none(),
        emailVerified: Option.none(),
        verificationToken: Option.none(),
        verificationExpires: Option.none(),
        passwordResetToken: Option.none(),
        passwordResetExpires: Option.none(),
        permissions: Option.none(),
        createdAt: Option.none(),
        lastLoginAt: Option.none()
    });
    const fields: UserFieldControllers = {
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
                const fieldErrors = userValidateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        email: {
            path: ['email'] as const,
            name: 'email',
            constraints: { required: true },
            get: () => data.email,
            set: (value: Option<string>) => {
                data.email = value;
            },
            transform: (value: Option<string>): Option<string> => value,
            getError: () => errors.email,
            setError: (value: Option<Array<string>>) => {
                errors.email = value;
            },
            getTainted: () => tainted.email,
            setTainted: (value: Option<boolean>) => {
                tainted.email = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('email', data.email);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        firstName: {
            path: ['firstName'] as const,
            name: 'firstName',
            constraints: { required: true },
            get: () => data.firstName,
            set: (value: string) => {
                data.firstName = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.firstName,
            setError: (value: Option<Array<string>>) => {
                errors.firstName = value;
            },
            getTainted: () => tainted.firstName,
            setTainted: (value: Option<boolean>) => {
                tainted.firstName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('firstName', data.firstName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        lastName: {
            path: ['lastName'] as const,
            name: 'lastName',
            constraints: { required: true },
            get: () => data.lastName,
            set: (value: string) => {
                data.lastName = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.lastName,
            setError: (value: Option<Array<string>>) => {
                errors.lastName = value;
            },
            getTainted: () => tainted.lastName,
            setTainted: (value: Option<boolean>) => {
                tainted.lastName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('lastName', data.lastName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        password: {
            path: ['password'] as const,
            name: 'password',
            constraints: { required: true },
            get: () => data.password,
            set: (value: Option<string>) => {
                data.password = value;
            },
            transform: (value: Option<string>): Option<string> => value,
            getError: () => errors.password,
            setError: (value: Option<Array<string>>) => {
                errors.password = value;
            },
            getTainted: () => tainted.password,
            setTainted: (value: Option<boolean>) => {
                tainted.password = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('password', data.password);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        metadata: {
            path: ['metadata'] as const,
            name: 'metadata',
            constraints: { required: true },
            get: () => data.metadata,
            set: (value: Option<Metadata>) => {
                data.metadata = value;
            },
            transform: (value: Option<Metadata>): Option<Metadata> => value,
            getError: () => errors.metadata,
            setError: (value: Option<Array<string>>) => {
                errors.metadata = value;
            },
            getTainted: () => tainted.metadata,
            setTainted: (value: Option<boolean>) => {
                tainted.metadata = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('metadata', data.metadata);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        settings: {
            path: ['settings'] as const,
            name: 'settings',
            constraints: { required: true },
            get: () => data.settings,
            set: (value: Settings) => {
                data.settings = value;
            },
            transform: (value: Settings): Settings => value,
            getError: () => errors.settings,
            setError: (value: Option<Array<string>>) => {
                errors.settings = value;
            },
            getTainted: () => tainted.settings,
            setTainted: (value: Option<boolean>) => {
                tainted.settings = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('settings', data.settings);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        role: {
            path: ['role'] as const,
            name: 'role',
            constraints: { required: true },
            get: () => data.role,
            set: (value: UserRole) => {
                data.role = value;
            },
            transform: (value: UserRole): UserRole => value,
            getError: () => errors.role,
            setError: (value: Option<Array<string>>) => {
                errors.role = value;
            },
            getTainted: () => tainted.role,
            setTainted: (value: Option<boolean>) => {
                tainted.role = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('role', data.role);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        emailVerified: {
            path: ['emailVerified'] as const,
            name: 'emailVerified',
            constraints: { required: true },
            get: () => data.emailVerified,
            set: (value: boolean) => {
                data.emailVerified = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.emailVerified,
            setError: (value: Option<Array<string>>) => {
                errors.emailVerified = value;
            },
            getTainted: () => tainted.emailVerified,
            setTainted: (value: Option<boolean>) => {
                tainted.emailVerified = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('emailVerified', data.emailVerified);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        verificationToken: {
            path: ['verificationToken'] as const,
            name: 'verificationToken',
            constraints: { required: true },
            get: () => data.verificationToken,
            set: (value: Option<string>) => {
                data.verificationToken = value;
            },
            transform: (value: Option<string>): Option<string> => value,
            getError: () => errors.verificationToken,
            setError: (value: Option<Array<string>>) => {
                errors.verificationToken = value;
            },
            getTainted: () => tainted.verificationToken,
            setTainted: (value: Option<boolean>) => {
                tainted.verificationToken = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('verificationToken', data.verificationToken);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        verificationExpires: {
            path: ['verificationExpires'] as const,
            name: 'verificationExpires',
            constraints: { required: true },
            get: () => data.verificationExpires,
            set: (value: Option<DateTime.DateTime>) => {
                data.verificationExpires = value;
            },
            transform: (value: Option<DateTime.DateTime>): Option<DateTime.DateTime> => value,
            getError: () => errors.verificationExpires,
            setError: (value: Option<Array<string>>) => {
                errors.verificationExpires = value;
            },
            getTainted: () => tainted.verificationExpires,
            setTainted: (value: Option<boolean>) => {
                tainted.verificationExpires = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField(
                    'verificationExpires',
                    data.verificationExpires
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        passwordResetToken: {
            path: ['passwordResetToken'] as const,
            name: 'passwordResetToken',
            constraints: { required: true },
            get: () => data.passwordResetToken,
            set: (value: Option<string>) => {
                data.passwordResetToken = value;
            },
            transform: (value: Option<string>): Option<string> => value,
            getError: () => errors.passwordResetToken,
            setError: (value: Option<Array<string>>) => {
                errors.passwordResetToken = value;
            },
            getTainted: () => tainted.passwordResetToken,
            setTainted: (value: Option<boolean>) => {
                tainted.passwordResetToken = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField(
                    'passwordResetToken',
                    data.passwordResetToken
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        passwordResetExpires: {
            path: ['passwordResetExpires'] as const,
            name: 'passwordResetExpires',
            constraints: { required: true },
            get: () => data.passwordResetExpires,
            set: (value: Option<DateTime.DateTime>) => {
                data.passwordResetExpires = value;
            },
            transform: (value: Option<DateTime.DateTime>): Option<DateTime.DateTime> => value,
            getError: () => errors.passwordResetExpires,
            setError: (value: Option<Array<string>>) => {
                errors.passwordResetExpires = value;
            },
            getTainted: () => tainted.passwordResetExpires,
            setTainted: (value: Option<boolean>) => {
                tainted.passwordResetExpires = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField(
                    'passwordResetExpires',
                    data.passwordResetExpires
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        permissions: {
            path: ['permissions'] as const,
            name: 'permissions',
            constraints: { required: true },
            get: () => data.permissions,
            set: (value: AppPermissions) => {
                data.permissions = value;
            },
            transform: (value: AppPermissions): AppPermissions => value,
            getError: () => errors.permissions,
            setError: (value: Option<Array<string>>) => {
                errors.permissions = value;
            },
            getTainted: () => tainted.permissions,
            setTainted: (value: Option<boolean>) => {
                tainted.permissions = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('permissions', data.permissions);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        createdAt: {
            path: ['createdAt'] as const,
            name: 'createdAt',
            constraints: { required: true },
            get: () => data.createdAt,
            set: (value: DateTime.DateTime) => {
                data.createdAt = value;
            },
            transform: (value: DateTime.DateTime): DateTime.DateTime => value,
            getError: () => errors.createdAt,
            setError: (value: Option<Array<string>>) => {
                errors.createdAt = value;
            },
            getTainted: () => tainted.createdAt,
            setTainted: (value: Option<boolean>) => {
                tainted.createdAt = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('createdAt', data.createdAt);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        lastLoginAt: {
            path: ['lastLoginAt'] as const,
            name: 'lastLoginAt',
            constraints: { required: true },
            get: () => data.lastLoginAt,
            set: (value: Option<DateTime.DateTime>) => {
                data.lastLoginAt = value;
            },
            transform: (value: Option<DateTime.DateTime>): Option<DateTime.DateTime> => value,
            getError: () => errors.lastLoginAt,
            setError: (value: Option<Array<string>>) => {
                errors.lastLoginAt = value;
            },
            getTainted: () => tainted.lastLoginAt,
            setTainted: (value: Option<boolean>) => {
                tainted.lastLoginAt = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = userValidateField('lastLoginAt', data.lastLoginAt);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<User, Array<{ field: string; message: string }>> {
        return userDeserialize(data);
    }
    function reset(newOverrides?: Partial<User>): void {
        data = { ...userDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            id: Option.none(),
            email: Option.none(),
            firstName: Option.none(),
            lastName: Option.none(),
            password: Option.none(),
            metadata: Option.none(),
            settings: Option.none(),
            role: Option.none(),
            emailVerified: Option.none(),
            verificationToken: Option.none(),
            verificationExpires: Option.none(),
            passwordResetToken: Option.none(),
            passwordResetExpires: Option.none(),
            permissions: Option.none(),
            createdAt: Option.none(),
            lastLoginAt: Option.none()
        };
        tainted = {
            id: Option.none(),
            email: Option.none(),
            firstName: Option.none(),
            lastName: Option.none(),
            password: Option.none(),
            metadata: Option.none(),
            settings: Option.none(),
            role: Option.none(),
            emailVerified: Option.none(),
            verificationToken: Option.none(),
            verificationExpires: Option.none(),
            passwordResetToken: Option.none(),
            passwordResetExpires: Option.none(),
            permissions: Option.none(),
            createdAt: Option.none(),
            lastLoginAt: Option.none()
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
export function userFromFormData(
    formData: FormData
): Result<User, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
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
    obj.firstName = formData.get('firstName') ?? '';
    obj.lastName = formData.get('lastName') ?? '';
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
        const metadataObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('metadata.')) {
                const fieldName = key.slice('metadata.'.length);
                const parts = fieldName.split('.');
                let current = metadataObj;
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
        obj.metadata = metadataObj;
    }
    {
        const settingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('settings.')) {
                const fieldName = key.slice('settings.'.length);
                const parts = fieldName.split('.');
                let current = settingsObj;
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
        obj.settings = settingsObj;
    }
    {
        const roleObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('role.')) {
                const fieldName = key.slice('role.'.length);
                const parts = fieldName.split('.');
                let current = roleObj;
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
        obj.role = roleObj;
    }
    {
        const emailVerifiedVal = formData.get('emailVerified');
        obj.emailVerified =
            emailVerifiedVal === 'true' || emailVerifiedVal === 'on' || emailVerifiedVal === '1';
    }
    {
        const verificationTokenObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('verificationToken.')) {
                const fieldName = key.slice('verificationToken.'.length);
                const parts = fieldName.split('.');
                let current = verificationTokenObj;
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
        obj.verificationToken = verificationTokenObj;
    }
    {
        const verificationExpiresObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('verificationExpires.')) {
                const fieldName = key.slice('verificationExpires.'.length);
                const parts = fieldName.split('.');
                let current = verificationExpiresObj;
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
        obj.verificationExpires = verificationExpiresObj;
    }
    {
        const passwordResetTokenObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('passwordResetToken.')) {
                const fieldName = key.slice('passwordResetToken.'.length);
                const parts = fieldName.split('.');
                let current = passwordResetTokenObj;
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
        obj.passwordResetToken = passwordResetTokenObj;
    }
    {
        const passwordResetExpiresObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('passwordResetExpires.')) {
                const fieldName = key.slice('passwordResetExpires.'.length);
                const parts = fieldName.split('.');
                let current = passwordResetExpiresObj;
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
        obj.passwordResetExpires = passwordResetExpiresObj;
    }
    {
        const permissionsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('permissions.')) {
                const fieldName = key.slice('permissions.'.length);
                const parts = fieldName.split('.');
                let current = permissionsObj;
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
        obj.permissions = permissionsObj;
    }
    {
        const createdAtObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('createdAt.')) {
                const fieldName = key.slice('createdAt.'.length);
                const parts = fieldName.split('.');
                let current = createdAtObj;
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
        obj.createdAt = createdAtObj;
    }
    {
        const lastLoginAtObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('lastLoginAt.')) {
                const fieldName = key.slice('lastLoginAt.'.length);
                const parts = fieldName.split('.');
                let current = lastLoginAtObj;
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
        obj.lastLoginAt = lastLoginAtObj;
    }
    return userDeserialize(obj);
}

export const User = {
    defaultValue: userDefaultValue,
    serialize: userSerialize,
    serializeWithContext: userSerializeWithContext,
    deserialize: userDeserialize,
    deserializeWithContext: userDeserializeWithContext,
    validateFields: userValidateFields,
    hasShape: userHasShape,
    is: userIs,
    createForm: userCreateForm,
    fromFormData: userFromFormData
} as const;
