import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { Metadata } from './metadata.svelte';
import { Settings } from './settings.svelte';
import { AppPermissions } from './app-permissions.svelte';
import { UserRole } from './user-role.svelte';

export interface User {
    id: string;
    email: string | null;

    firstName: string;

    lastName: string;
    password: string | null;
    metadata: Metadata | null;
    settings: Settings;

    role: UserRole;
    emailVerified: boolean;
    verificationToken: string | null;
    verificationExpires: string | null;
    passwordResetToken: string | null;
    passwordResetExpires: string | null;
    permissions: AppPermissions;
}

export function defaultValueUser(): User {
    return {
        id: '',
        email: null,
        firstName: '',
        lastName: '',
        password: null,
        metadata: null,
        settings: Settings.defaultValue(),
        role: 'Administrator',
        emailVerified: false,
        verificationToken: null,
        verificationExpires: null,
        passwordResetToken: null,
        passwordResetExpires: null,
        permissions: AppPermissions.defaultValue()
    } as User;
}

export function toStringifiedJSONUser(value: User): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeUser(value, ctx));
}
export function toObjectUser(value: User): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeUser(value, ctx);
}
export function __serializeUser(value: User, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'User', __id };
    result['id'] = value.id;
    result['email'] = value.email;
    result['firstName'] = value.firstName;
    result['lastName'] = value.lastName;
    result['password'] = value.password;
    if (value.metadata !== null) {
        result['metadata'] =
            typeof (value.metadata as any)?.__serialize === 'function'
                ? (value.metadata as any).__serialize(ctx)
                : value.metadata;
    } else {
        result['metadata'] = null;
    }
    result['settings'] =
        typeof (value.settings as any)?.__serialize === 'function'
            ? (value.settings as any).__serialize(ctx)
            : value.settings;
    result['role'] =
        typeof (value.role as any)?.__serialize === 'function'
            ? (value.role as any).__serialize(ctx)
            : value.role;
    result['emailVerified'] = value.emailVerified;
    result['verificationToken'] = value.verificationToken;
    result['verificationExpires'] = value.verificationExpires;
    result['passwordResetToken'] = value.passwordResetToken;
    result['passwordResetExpires'] = value.passwordResetExpires;
    result['permissions'] =
        typeof (value.permissions as any)?.__serialize === 'function'
            ? (value.permissions as any).__serialize(ctx)
            : value.permissions;
    return result;
}

export function fromStringifiedJSONUser(
    json: string,
    opts?: DeserializeOptions
): Result<User, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectUser(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectUser(
    obj: unknown,
    opts?: DeserializeOptions
): Result<User, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeUser(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'User.fromObject: root cannot be a forward reference' }
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
export function __deserializeUser(value: any, ctx: DeserializeContext): User | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'User.__deserialize: expected an object' }
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
        const __raw_email = obj['email'] as string | null;
        instance.email = __raw_email;
    }
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
    {
        const __raw_password = obj['password'] as string | null;
        instance.password = __raw_password;
    }
    {
        const __raw_metadata = obj['metadata'] as Metadata | null;
        if (__raw_metadata === null) {
            instance.metadata = null;
        } else {
            const __result = Metadata.__deserialize(__raw_metadata, ctx);
            ctx.assignOrDefer(instance, 'metadata', __result);
        }
    }
    {
        const __raw_settings = obj['settings'] as Settings;
        {
            const __result = Settings.__deserialize(__raw_settings, ctx);
            ctx.assignOrDefer(instance, 'settings', __result);
        }
    }
    {
        const __raw_role = obj['role'] as UserRole;
        {
            const __result = UserRole.__deserialize(__raw_role, ctx);
            ctx.assignOrDefer(instance, 'role', __result);
        }
    }
    {
        const __raw_emailVerified = obj['emailVerified'] as boolean;
        instance.emailVerified = __raw_emailVerified;
    }
    {
        const __raw_verificationToken = obj['verificationToken'] as string | null;
        instance.verificationToken = __raw_verificationToken;
    }
    {
        const __raw_verificationExpires = obj['verificationExpires'] as string | null;
        instance.verificationExpires = __raw_verificationExpires;
    }
    {
        const __raw_passwordResetToken = obj['passwordResetToken'] as string | null;
        instance.passwordResetToken = __raw_passwordResetToken;
    }
    {
        const __raw_passwordResetExpires = obj['passwordResetExpires'] as string | null;
        instance.passwordResetExpires = __raw_passwordResetExpires;
    }
    {
        const __raw_permissions = obj['permissions'] as AppPermissions;
        {
            const __result = AppPermissions.__deserialize(__raw_permissions, ctx);
            ctx.assignOrDefer(instance, 'permissions', __result);
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as User;
}
export function validateFieldUser<K extends keyof User>(
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
export function validateFieldsUser(
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
export function hasShapeUser(obj: unknown): boolean {
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
        'permissions' in o
    );
}
export function isUser(obj: unknown): obj is User {
    if (!hasShapeUser(obj)) {
        return false;
    }
    const result = fromObjectUser(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsUser = {
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
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedUser = {
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
}; /** Type-safe field controllers for this form */
export interface FieldControllersUser {
    readonly id: FieldController<string>;
    readonly email: FieldController<string | null>;
    readonly firstName: FieldController<string>;
    readonly lastName: FieldController<string>;
    readonly password: FieldController<string | null>;
    readonly metadata: FieldController<Metadata | null>;
    readonly settings: FieldController<Settings>;
    readonly role: FieldController<UserRole>;
    readonly emailVerified: FieldController<boolean>;
    readonly verificationToken: FieldController<string | null>;
    readonly verificationExpires: FieldController<string | null>;
    readonly passwordResetToken: FieldController<string | null>;
    readonly passwordResetExpires: FieldController<string | null>;
    readonly permissions: FieldController<AppPermissions>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformUser {
    readonly data: User;
    readonly errors: ErrorsUser;
    readonly tainted: TaintedUser;
    readonly fields: FieldControllersUser;
    validate(): Result<User, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<User>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormUser(overrides?: Partial<User>): GigaformUser {
    let data = $state({ ...User.defaultValue(), ...overrides });
    let errors = $state<ErrorsUser>({
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
        permissions: Option.none()
    });
    let tainted = $state<TaintedUser>({
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
        permissions: Option.none()
    });
    const fields: FieldControllersUser = {
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
                const fieldErrors = User.validateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        email: {
            path: ['email'] as const,
            name: 'email',
            constraints: { required: true },

            get: () => data.email,
            set: (value: string | null) => {
                data.email = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.email,
            setError: (value: Option<Array<string>>) => {
                errors.email = value;
            },
            getTainted: () => tainted.email,
            setTainted: (value: Option<boolean>) => {
                tainted.email = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = User.validateField('email', data.email);
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
                const fieldErrors = User.validateField('firstName', data.firstName);
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
                const fieldErrors = User.validateField('lastName', data.lastName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        password: {
            path: ['password'] as const,
            name: 'password',
            constraints: { required: true },

            get: () => data.password,
            set: (value: string | null) => {
                data.password = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.password,
            setError: (value: Option<Array<string>>) => {
                errors.password = value;
            },
            getTainted: () => tainted.password,
            setTainted: (value: Option<boolean>) => {
                tainted.password = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = User.validateField('password', data.password);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        metadata: {
            path: ['metadata'] as const,
            name: 'metadata',
            constraints: { required: true },

            get: () => data.metadata,
            set: (value: Metadata | null) => {
                data.metadata = value;
            },
            transform: (value: Metadata | null): Metadata | null => value,
            getError: () => errors.metadata,
            setError: (value: Option<Array<string>>) => {
                errors.metadata = value;
            },
            getTainted: () => tainted.metadata,
            setTainted: (value: Option<boolean>) => {
                tainted.metadata = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = User.validateField('metadata', data.metadata);
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
                const fieldErrors = User.validateField('settings', data.settings);
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
                const fieldErrors = User.validateField('role', data.role);
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
                const fieldErrors = User.validateField('emailVerified', data.emailVerified);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        verificationToken: {
            path: ['verificationToken'] as const,
            name: 'verificationToken',
            constraints: { required: true },

            get: () => data.verificationToken,
            set: (value: string | null) => {
                data.verificationToken = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.verificationToken,
            setError: (value: Option<Array<string>>) => {
                errors.verificationToken = value;
            },
            getTainted: () => tainted.verificationToken,
            setTainted: (value: Option<boolean>) => {
                tainted.verificationToken = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = User.validateField('verificationToken', data.verificationToken);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        verificationExpires: {
            path: ['verificationExpires'] as const,
            name: 'verificationExpires',
            constraints: { required: true },

            get: () => data.verificationExpires,
            set: (value: string | null) => {
                data.verificationExpires = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.verificationExpires,
            setError: (value: Option<Array<string>>) => {
                errors.verificationExpires = value;
            },
            getTainted: () => tainted.verificationExpires,
            setTainted: (value: Option<boolean>) => {
                tainted.verificationExpires = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = User.validateField(
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
            set: (value: string | null) => {
                data.passwordResetToken = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.passwordResetToken,
            setError: (value: Option<Array<string>>) => {
                errors.passwordResetToken = value;
            },
            getTainted: () => tainted.passwordResetToken,
            setTainted: (value: Option<boolean>) => {
                tainted.passwordResetToken = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = User.validateField(
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
            set: (value: string | null) => {
                data.passwordResetExpires = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.passwordResetExpires,
            setError: (value: Option<Array<string>>) => {
                errors.passwordResetExpires = value;
            },
            getTainted: () => tainted.passwordResetExpires,
            setTainted: (value: Option<boolean>) => {
                tainted.passwordResetExpires = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = User.validateField(
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
                const fieldErrors = User.validateField('permissions', data.permissions);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<User, Array<{ field: string; message: string }>> {
        return User.fromObject(data);
    }
    function reset(newOverrides?: Partial<User>): void {
        data = { ...User.defaultValue(), ...newOverrides };
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
            permissions: Option.none()
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
            permissions: Option.none()
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
export function fromFormDataUser(
    formData: FormData
): Result<User, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.email = formData.get('email') ?? '';
    obj.firstName = formData.get('firstName') ?? '';
    obj.lastName = formData.get('lastName') ?? '';
    obj.password = formData.get('password') ?? '';
    obj.metadata = formData.get('metadata') ?? '';
    {
        // Collect nested object fields with prefix "settings."
        const settingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('settings.')) {
                const fieldName = key.slice('settings.'.length);
                // Handle deeper nesting by splitting on dots
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
        // Collect nested object fields with prefix "role."
        const roleObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('role.')) {
                const fieldName = key.slice('role.'.length);
                // Handle deeper nesting by splitting on dots
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
    obj.verificationToken = formData.get('verificationToken') ?? '';
    obj.verificationExpires = formData.get('verificationExpires') ?? '';
    obj.passwordResetToken = formData.get('passwordResetToken') ?? '';
    obj.passwordResetExpires = formData.get('passwordResetExpires') ?? '';
    {
        // Collect nested object fields with prefix "permissions."
        const permissionsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('permissions.')) {
                const fieldName = key.slice('permissions.'.length);
                // Handle deeper nesting by splitting on dots
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
    return User.fromStringifiedJSON(JSON.stringify(obj));
}
