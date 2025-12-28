import { appPermissionsDefaultValue } from "./app-permissions.svelte";
import { settingsDefaultValue } from "./settings.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { appPermissionsSerializeWithContext } from "./app-permissions.svelte";
import { settingsSerializeWithContext } from "./settings.svelte";
import { userRoleSerializeWithContext } from "./user-role.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { appPermissionsDeserializeWithContext } from "./app-permissions.svelte";
import { settingsDeserializeWithContext } from "./settings.svelte";
import { userRoleDeserializeWithContext } from "./user-role.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { DateTime } from 'effect';
import type { Option } from 'effect/Option';
import type { AppPermissions } from './app-permissions.svelte';
import type { Metadata } from './metadata.svelte';
import type { Settings } from './settings.svelte';
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
        id: "",
        email: optionDefaultValue<string>(),
        firstName: "",
        lastName: "",
        password: optionDefaultValue<string>(),
        metadata: optionDefaultValue<Metadata>(),
        settings: settingsDefaultValue(),
        role: "Administrator",
        emailVerified: false,
        verificationToken: optionDefaultValue<string>(),
        verificationExpires: optionDefaultValue<DateTime.DateTime>(),
        passwordResetToken: optionDefaultValue<string>(),
        passwordResetExpires: optionDefaultValue<DateTime.DateTime>(),
        permissions: appPermissionsDefaultValue(),
        createdAt: dateTime.dateTimeDefaultValue(),
        lastLoginAt: optionDefaultValue<DateTime.DateTime>()
    } as User;
}

export function userSerialize(value: User): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(userSerializeWithContext(value, ctx));
}
export function userSerializeWithContext(value: User, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: "User",
        __id
    };
    result.id = value.id;
    result.email = option<string>SerializeWithContext(value.email, ctx);
    result.firstName = value.firstName;
    result.lastName = value.lastName;
    result.password = option<string>SerializeWithContext(value.password, ctx);
    result.metadata = option<metadata>SerializeWithContext(value.metadata, ctx);
    result.settings = settingsSerializeWithContext(value.settings, ctx);
    result.role = userRoleSerializeWithContext(value.role, ctx);
    result.emailVerified = value.emailVerified;
    result.verificationToken = option<string>SerializeWithContext(value.verificationToken, ctx);
    result.verificationExpires = option<dateTime.dateTime>SerializeWithContext(value.verificationExpires, ctx);
    result.passwordResetToken = option<string>SerializeWithContext(value.passwordResetToken, ctx);
    result.passwordResetExpires = option<dateTime.dateTime>SerializeWithContext(value.passwordResetExpires, ctx);
    result.permissions = appPermissionsSerializeWithContext(value.permissions, ctx);
    result.createdAt = dateTime.dateTimeSerializeWithContext(value.createdAt, ctx);
    result.lastLoginAt = option<dateTime.dateTime>SerializeWithContext(value.lastLoginAt, ctx);
    return result;
}

export function userDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: User } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = userDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "User.deserialize: root cannot be a forward reference"
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return {
            success: true,
            value: resultOrRef
        };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return {
                success: false,
                errors: e.errors
            };
        }
        const message = e instanceof Error ? e.message : String(e);
        return {
            success: false,
            errors: [
                {
                    field: "_root",
                    message
                }
            ]
        };
    }
}
export function userDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): User | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"User"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("id" in obj)) {
        errors.push({
            field: "id",
            message: "missing required field"
        });
    }
    if (!("email" in obj)) {
        errors.push({
            field: "email",
            message: "missing required field"
        });
    }
    if (!("firstName" in obj)) {
        errors.push({
            field: "firstName",
            message: "missing required field"
        });
    }
    if (!("lastName" in obj)) {
        errors.push({
            field: "lastName",
            message: "missing required field"
        });
    }
    if (!("password" in obj)) {
        errors.push({
            field: "password",
            message: "missing required field"
        });
    }
    if (!("metadata" in obj)) {
        errors.push({
            field: "metadata",
            message: "missing required field"
        });
    }
    if (!("settings" in obj)) {
        errors.push({
            field: "settings",
            message: "missing required field"
        });
    }
    if (!("role" in obj)) {
        errors.push({
            field: "role",
            message: "missing required field"
        });
    }
    if (!("emailVerified" in obj)) {
        errors.push({
            field: "emailVerified",
            message: "missing required field"
        });
    }
    if (!("verificationToken" in obj)) {
        errors.push({
            field: "verificationToken",
            message: "missing required field"
        });
    }
    if (!("verificationExpires" in obj)) {
        errors.push({
            field: "verificationExpires",
            message: "missing required field"
        });
    }
    if (!("passwordResetToken" in obj)) {
        errors.push({
            field: "passwordResetToken",
            message: "missing required field"
        });
    }
    if (!("passwordResetExpires" in obj)) {
        errors.push({
            field: "passwordResetExpires",
            message: "missing required field"
        });
    }
    if (!("permissions" in obj)) {
        errors.push({
            field: "permissions",
            message: "missing required field"
        });
    }
    if (!("createdAt" in obj)) {
        errors.push({
            field: "createdAt",
            message: "missing required field"
        });
    }
    if (!("lastLoginAt" in obj)) {
        errors.push({
            field: "lastLoginAt",
            message: "missing required field"
        });
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
        const __raw_id = obj["id"] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_email = obj["email"] as Option<string>;
        {
            const __result = option<string>DeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, "email", __result);
        }
    }
    {
        const __raw_firstName = obj["firstName"] as string;
        if (__raw_firstName.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "User.firstName must not be empty"
            });
        }
        instance.firstName = __raw_firstName;
    }
    {
        const __raw_lastName = obj["lastName"] as string;
        if (__raw_lastName.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "User.lastName must not be empty"
            });
        }
        instance.lastName = __raw_lastName;
    }
    {
        const __raw_password = obj["password"] as Option<string>;
        {
            const __result = option<string>DeserializeWithContext(__raw_password, ctx);
            ctx.assignOrDefer(instance, "password", __result);
        }
    }
    {
        const __raw_metadata = obj["metadata"] as Option<Metadata>;
        {
            const __result = option<metadata>DeserializeWithContext(__raw_metadata, ctx);
            ctx.assignOrDefer(instance, "metadata", __result);
        }
    }
    {
        const __raw_settings = obj["settings"] as Settings;
        {
            const __result = settingsDeserializeWithContext(__raw_settings, ctx);
            ctx.assignOrDefer(instance, "settings", __result);
        }
    }
    {
        const __raw_role = obj["role"] as UserRole;
        {
            const __result = userRoleDeserializeWithContext(__raw_role, ctx);
            ctx.assignOrDefer(instance, "role", __result);
        }
    }
    {
        const __raw_emailVerified = obj["emailVerified"] as boolean;
        instance.emailVerified = __raw_emailVerified;
    }
    {
        const __raw_verificationToken = obj["verificationToken"] as Option<string>;
        {
            const __result = option<string>DeserializeWithContext(__raw_verificationToken, ctx);
            ctx.assignOrDefer(instance, "verificationToken", __result);
        }
    }
    {
        const __raw_verificationExpires = obj["verificationExpires"] as Option<DateTime.DateTime>;
        {
            const __result = option<dateTime.dateTime>DeserializeWithContext(__raw_verificationExpires, ctx);
            ctx.assignOrDefer(instance, "verificationExpires", __result);
        }
    }
    {
        const __raw_passwordResetToken = obj["passwordResetToken"] as Option<string>;
        {
            const __result = option<string>DeserializeWithContext(__raw_passwordResetToken, ctx);
            ctx.assignOrDefer(instance, "passwordResetToken", __result);
        }
    }
    {
        const __raw_passwordResetExpires = obj["passwordResetExpires"] as Option<DateTime.DateTime>;
        {
            const __result = option<dateTime.dateTime>DeserializeWithContext(__raw_passwordResetExpires, ctx);
            ctx.assignOrDefer(instance, "passwordResetExpires", __result);
        }
    }
    {
        const __raw_permissions = obj["permissions"] as AppPermissions;
        {
            const __result = appPermissionsDeserializeWithContext(__raw_permissions, ctx);
            ctx.assignOrDefer(instance, "permissions", __result);
        }
    }
    {
        const __raw_createdAt = obj["createdAt"] as DateTime.DateTime;
        {
            const __result = dateTime.dateTimeDeserializeWithContext(__raw_createdAt, ctx);
            ctx.assignOrDefer(instance, "createdAt", __result);
        }
    }
    {
        const __raw_lastLoginAt = obj["lastLoginAt"] as Option<DateTime.DateTime>;
        {
            const __result = option<dateTime.dateTime>DeserializeWithContext(__raw_lastLoginAt, ctx);
            ctx.assignOrDefer(instance, "lastLoginAt", __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as User;
}
export function userValidateField<K extends keyof User>(_field: K, _value: User[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === "firstName") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "User.firstName must not be empty"
            });
        }
    }
    if (_field === "lastName") {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "User.lastName must not be empty"
            });
        }
    }
    return errors;
}
export function userValidateFields(_partial: Partial<User>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("firstName" in _partial && _partial.firstName !== undefined) {
        const __val = _partial.firstName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "User.firstName must not be empty"
            });
        }
    }
    if ("lastName" in _partial && _partial.lastName !== undefined) {
        const __val = _partial.lastName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "User.lastName must not be empty"
            });
        }
    }
    return errors;
}
export function userHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "email" in o && "firstName" in o && "lastName" in o && "password" in o && "metadata" in o && "settings" in o && "role" in o && "emailVerified" in o && "verificationToken" in o && "verificationExpires" in o && "passwordResetToken" in o && "passwordResetExpires" in o && "permissions" in o && "createdAt" in o && "lastLoginAt" in o';
}
export function userIs(obj: unknown): obj is User {
    if (!userHasShape(obj)) {
        return false;
    }
    const result = userDeserialize(obj);
    return result.success;
}

export type UserErrors = {
    _errors: __gf_Option<Array<string>>;
    id: __gf_Option<Array<string>>;
    email: __gf_Option<Array<string>>;
    firstName: __gf_Option<Array<string>>;
    lastName: __gf_Option<Array<string>>;
    password: __gf_Option<Array<string>>;
    metadata: __gf_Option<Array<string>>;
    settings: __gf_Option<Array<string>>;
    role: __gf_Option<Array<string>>;
    emailVerified: __gf_Option<Array<string>>;
    verificationToken: __gf_Option<Array<string>>;
    verificationExpires: __gf_Option<Array<string>>;
    passwordResetToken: __gf_Option<Array<string>>;
    passwordResetExpires: __gf_Option<Array<string>>;
    permissions: __gf_Option<Array<string>>;
    createdAt: __gf_Option<Array<string>>;
    lastLoginAt: __gf_Option<Array<string>>;
};
export type UserTainted = {
    id: __gf_Option<boolean>;
    email: __gf_Option<boolean>;
    firstName: __gf_Option<boolean>;
    lastName: __gf_Option<boolean>;
    password: __gf_Option<boolean>;
    metadata: __gf_Option<boolean>;
    settings: __gf_Option<boolean>;
    role: __gf_Option<boolean>;
    emailVerified: __gf_Option<boolean>;
    verificationToken: __gf_Option<boolean>;
    verificationExpires: __gf_Option<boolean>;
    passwordResetToken: __gf_Option<boolean>;
    passwordResetExpires: __gf_Option<boolean>;
    permissions: __gf_Option<boolean>;
    createdAt: __gf_Option<boolean>;
    lastLoginAt: __gf_Option<boolean>;
};
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
}
export interface UserGigaform {
    readonly data: User;
    readonly errors: UserErrors;
    readonly tainted: UserTainted;
    readonly fields: UserFieldControllers;
    validate(): Exit<User, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<User>): void;
}
export function userCreateForm(overrides?: Partial<User>): UserGigaform {
    let data = $state({
        ...userDefaultValue(),
        ...overrides
    });
    let errors = $state<UserErrors>({
        _errors: optionNone(),
        id: optionNone(),
        email: optionNone(),
        firstName: optionNone(),
        lastName: optionNone(),
        password: optionNone(),
        metadata: optionNone(),
        settings: optionNone(),
        role: optionNone(),
        emailVerified: optionNone(),
        verificationToken: optionNone(),
        verificationExpires: optionNone(),
        passwordResetToken: optionNone(),
        passwordResetExpires: optionNone(),
        permissions: optionNone(),
        createdAt: optionNone(),
        lastLoginAt: optionNone()
    } as UserErrors);
    let tainted = $state<UserTainted>({
        id: optionNone(),
        email: optionNone(),
        firstName: optionNone(),
        lastName: optionNone(),
        password: optionNone(),
        metadata: optionNone(),
        settings: optionNone(),
        role: optionNone(),
        emailVerified: optionNone(),
        verificationToken: optionNone(),
        verificationExpires: optionNone(),
        passwordResetToken: optionNone(),
        passwordResetExpires: optionNone(),
        permissions: optionNone(),
        createdAt: optionNone(),
        lastLoginAt: optionNone()
    } as UserTainted);
    const fields = {
        id: {
            path: [
                "id"
            ] as const,
            name: "id",
            constraints: {
                required: true
            },
            get: ()=>data.id,
            set: (value: string)=>{
                data.id = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.id,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.id = value;
            },
            getTainted: ()=>tainted.id,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.id = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("id", data.id);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        email: {
            path: [
                "email"
            ] as const,
            name: "email",
            constraints: {
                required: true
            },
            get: ()=>data.email,
            set: (value: Option<string>)=>{
                data.email = value;
            },
            transform: (value: Option<string>): Option<string> =>value,
            getError: ()=>errors.email,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.email = value;
            },
            getTainted: ()=>tainted.email,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.email = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("email", data.email);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        firstName: {
            path: [
                "firstName"
            ] as const,
            name: "firstName",
            constraints: {
                required: true
            },
            get: ()=>data.firstName,
            set: (value: string)=>{
                data.firstName = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.firstName,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.firstName = value;
            },
            getTainted: ()=>tainted.firstName,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.firstName = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("firstName", data.firstName);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        lastName: {
            path: [
                "lastName"
            ] as const,
            name: "lastName",
            constraints: {
                required: true
            },
            get: ()=>data.lastName,
            set: (value: string)=>{
                data.lastName = value;
            },
            transform: (value: string): string =>value,
            getError: ()=>errors.lastName,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.lastName = value;
            },
            getTainted: ()=>tainted.lastName,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.lastName = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("lastName", data.lastName);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        password: {
            path: [
                "password"
            ] as const,
            name: "password",
            constraints: {
                required: true
            },
            get: ()=>data.password,
            set: (value: Option<string>)=>{
                data.password = value;
            },
            transform: (value: Option<string>): Option<string> =>value,
            getError: ()=>errors.password,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.password = value;
            },
            getTainted: ()=>tainted.password,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.password = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("password", data.password);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        metadata: {
            path: [
                "metadata"
            ] as const,
            name: "metadata",
            constraints: {
                required: true
            },
            get: ()=>data.metadata,
            set: (value: Option<Metadata>)=>{
                data.metadata = value;
            },
            transform: (value: Option<Metadata>): Option<Metadata> =>value,
            getError: ()=>errors.metadata,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.metadata = value;
            },
            getTainted: ()=>tainted.metadata,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.metadata = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("metadata", data.metadata);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        settings: {
            path: [
                "settings"
            ] as const,
            name: "settings",
            constraints: {
                required: true
            },
            get: ()=>data.settings,
            set: (value: Settings)=>{
                data.settings = value;
            },
            transform: (value: Settings): Settings =>value,
            getError: ()=>errors.settings,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.settings = value;
            },
            getTainted: ()=>tainted.settings,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.settings = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("settings", data.settings);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        role: {
            path: [
                "role"
            ] as const,
            name: "role",
            constraints: {
                required: true
            },
            get: ()=>data.role,
            set: (value: UserRole)=>{
                data.role = value;
            },
            transform: (value: UserRole): UserRole =>value,
            getError: ()=>errors.role,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.role = value;
            },
            getTainted: ()=>tainted.role,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.role = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("role", data.role);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        emailVerified: {
            path: [
                "emailVerified"
            ] as const,
            name: "emailVerified",
            constraints: {
                required: true
            },
            get: ()=>data.emailVerified,
            set: (value: boolean)=>{
                data.emailVerified = value;
            },
            transform: (value: boolean): boolean =>value,
            getError: ()=>errors.emailVerified,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.emailVerified = value;
            },
            getTainted: ()=>tainted.emailVerified,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.emailVerified = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("emailVerified", data.emailVerified);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        verificationToken: {
            path: [
                "verificationToken"
            ] as const,
            name: "verificationToken",
            constraints: {
                required: true
            },
            get: ()=>data.verificationToken,
            set: (value: Option<string>)=>{
                data.verificationToken = value;
            },
            transform: (value: Option<string>): Option<string> =>value,
            getError: ()=>errors.verificationToken,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.verificationToken = value;
            },
            getTainted: ()=>tainted.verificationToken,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.verificationToken = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("verificationToken", data.verificationToken);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        verificationExpires: {
            path: [
                "verificationExpires"
            ] as const,
            name: "verificationExpires",
            constraints: {
                required: true
            },
            get: ()=>data.verificationExpires,
            set: (value: Option<DateTime.DateTime>)=>{
                data.verificationExpires = value;
            },
            transform: (value: Option<DateTime.DateTime>): Option<DateTime.DateTime> =>value,
            getError: ()=>errors.verificationExpires,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.verificationExpires = value;
            },
            getTainted: ()=>tainted.verificationExpires,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.verificationExpires = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("verificationExpires", data.verificationExpires);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        passwordResetToken: {
            path: [
                "passwordResetToken"
            ] as const,
            name: "passwordResetToken",
            constraints: {
                required: true
            },
            get: ()=>data.passwordResetToken,
            set: (value: Option<string>)=>{
                data.passwordResetToken = value;
            },
            transform: (value: Option<string>): Option<string> =>value,
            getError: ()=>errors.passwordResetToken,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.passwordResetToken = value;
            },
            getTainted: ()=>tainted.passwordResetToken,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.passwordResetToken = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("passwordResetToken", data.passwordResetToken);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        passwordResetExpires: {
            path: [
                "passwordResetExpires"
            ] as const,
            name: "passwordResetExpires",
            constraints: {
                required: true
            },
            get: ()=>data.passwordResetExpires,
            set: (value: Option<DateTime.DateTime>)=>{
                data.passwordResetExpires = value;
            },
            transform: (value: Option<DateTime.DateTime>): Option<DateTime.DateTime> =>value,
            getError: ()=>errors.passwordResetExpires,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.passwordResetExpires = value;
            },
            getTainted: ()=>tainted.passwordResetExpires,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.passwordResetExpires = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("passwordResetExpires", data.passwordResetExpires);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        permissions: {
            path: [
                "permissions"
            ] as const,
            name: "permissions",
            constraints: {
                required: true
            },
            get: ()=>data.permissions,
            set: (value: AppPermissions)=>{
                data.permissions = value;
            },
            transform: (value: AppPermissions): AppPermissions =>value,
            getError: ()=>errors.permissions,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.permissions = value;
            },
            getTainted: ()=>tainted.permissions,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.permissions = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("permissions", data.permissions);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        createdAt: {
            path: [
                "createdAt"
            ] as const,
            name: "createdAt",
            constraints: {
                required: true
            },
            get: ()=>data.createdAt,
            set: (value: DateTime.DateTime)=>{
                data.createdAt = value;
            },
            transform: (value: DateTime.DateTime): DateTime.DateTime =>value,
            getError: ()=>errors.createdAt,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.createdAt = value;
            },
            getTainted: ()=>tainted.createdAt,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.createdAt = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("createdAt", data.createdAt);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        },
        lastLoginAt: {
            path: [
                "lastLoginAt"
            ] as const,
            name: "lastLoginAt",
            constraints: {
                required: true
            },
            get: ()=>data.lastLoginAt,
            set: (value: Option<DateTime.DateTime>)=>{
                data.lastLoginAt = value;
            },
            transform: (value: Option<DateTime.DateTime>): Option<DateTime.DateTime> =>value,
            getError: ()=>errors.lastLoginAt,
            setError: (value: __gf_Option<Array<string>>)=>{
                errors.lastLoginAt = value;
            },
            getTainted: ()=>tainted.lastLoginAt,
            setTainted: (value: __gf_Option<boolean>)=>{
                tainted.lastLoginAt = value;
            },
            validate: (): Array<string> =>{
                const fieldErrors = userValidateField("lastLoginAt", data.lastLoginAt);
                return fieldErrors.map((e: {
                    field: string;
                    message: string;
                })=>e.message);
            }
        }
    } as UserFieldControllers;
    const __gf_getter_hint = "get data() set data(v) get errors() set errors(v) get tainted() set tainted(v)";
    const __gf_validate_hint = ".map((e: { field: string; message: string }) => e.message)";
    function validate(): Exit<User, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(userDeserialize(data));
    }
    function reset(newOverrides?: Partial<User>): void {
        data = {
            ...userDefaultValue(),
            ...newOverrides
        };
        errors = {
            _errors: optionNone(),
            id: optionNone(),
            email: optionNone(),
            firstName: optionNone(),
            lastName: optionNone(),
            password: optionNone(),
            metadata: optionNone(),
            settings: optionNone(),
            role: optionNone(),
            emailVerified: optionNone(),
            verificationToken: optionNone(),
            verificationExpires: optionNone(),
            passwordResetToken: optionNone(),
            passwordResetExpires: optionNone(),
            permissions: optionNone(),
            createdAt: optionNone(),
            lastLoginAt: optionNone()
        };
        tainted = {
            id: optionNone(),
            email: optionNone(),
            firstName: optionNone(),
            lastName: optionNone(),
            password: optionNone(),
            metadata: optionNone(),
            settings: optionNone(),
            role: optionNone(),
            emailVerified: optionNone(),
            verificationToken: optionNone(),
            verificationExpires: optionNone(),
            passwordResetToken: optionNone(),
            passwordResetExpires: optionNone(),
            permissions: optionNone(),
            createdAt: optionNone(),
            lastLoginAt: optionNone()
        };
    }
    return {
        get data () {
            return data;
        },
        set data (v){
            data = v;
        },
        get errors () {
            return errors;
        },
        set errors (v){
            errors = v;
        },
        get tainted () {
            return tainted;
        },
        set tainted (v){
            tainted = v;
        },
        fields,
        validate,
        reset
    };
}
export function userFromFormData(formData: FormData): Exit<User, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    const __gf_exit_hint = "Exit<User, Array<{ field: string; message: string }>>";
    obj.id = formData.get(`${"id"}`) ?? "";
    {
        const emailObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"email"}.`)) {
                const fieldName = key.slice(`${"email"}.`.length);
                const parts = fieldName.split(".");
                let current = emailObj;
                for(let i = 0; i < parts.length - 1; i++){
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
    obj.firstName = formData.get(`${"firstName"}`) ?? "";
    obj.lastName = formData.get(`${"lastName"}`) ?? "";
    {
        const passwordObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"password"}.`)) {
                const fieldName = key.slice(`${"password"}.`.length);
                const parts = fieldName.split(".");
                let current = passwordObj;
                for(let i = 0; i < parts.length - 1; i++){
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
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"metadata"}.`)) {
                const fieldName = key.slice(`${"metadata"}.`.length);
                const parts = fieldName.split(".");
                let current = metadataObj;
                for(let i = 0; i < parts.length - 1; i++){
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
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"settings"}.`)) {
                const fieldName = key.slice(`${"settings"}.`.length);
                const parts = fieldName.split(".");
                let current = settingsObj;
                for(let i = 0; i < parts.length - 1; i++){
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
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"role"}.`)) {
                const fieldName = key.slice(`${"role"}.`.length);
                const parts = fieldName.split(".");
                let current = roleObj;
                for(let i = 0; i < parts.length - 1; i++){
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
        const emailVerifiedVal = formData.get(`${"emailVerified"}`);
        obj.emailVerified = emailVerifiedVal === "true" || emailVerifiedVal === "on" || emailVerifiedVal === "1";
    }
    {
        const verificationTokenObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"verificationToken"}.`)) {
                const fieldName = key.slice(`${"verificationToken"}.`.length);
                const parts = fieldName.split(".");
                let current = verificationTokenObj;
                for(let i = 0; i < parts.length - 1; i++){
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
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"verificationExpires"}.`)) {
                const fieldName = key.slice(`${"verificationExpires"}.`.length);
                const parts = fieldName.split(".");
                let current = verificationExpiresObj;
                for(let i = 0; i < parts.length - 1; i++){
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
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"passwordResetToken"}.`)) {
                const fieldName = key.slice(`${"passwordResetToken"}.`.length);
                const parts = fieldName.split(".");
                let current = passwordResetTokenObj;
                for(let i = 0; i < parts.length - 1; i++){
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
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"passwordResetExpires"}.`)) {
                const fieldName = key.slice(`${"passwordResetExpires"}.`.length);
                const parts = fieldName.split(".");
                let current = passwordResetExpiresObj;
                for(let i = 0; i < parts.length - 1; i++){
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
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"permissions"}.`)) {
                const fieldName = key.slice(`${"permissions"}.`.length);
                const parts = fieldName.split(".");
                let current = permissionsObj;
                for(let i = 0; i < parts.length - 1; i++){
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
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"createdAt"}.`)) {
                const fieldName = key.slice(`${"createdAt"}.`.length);
                const parts = fieldName.split(".");
                let current = createdAtObj;
                for(let i = 0; i < parts.length - 1; i++){
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
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"lastLoginAt"}.`)) {
                const fieldName = key.slice(`${"lastLoginAt"}.`.length);
                const parts = fieldName.split(".");
                let current = lastLoginAtObj;
                for(let i = 0; i < parts.length - 1; i++){
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
    return toExit(userDeserialize(obj));
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