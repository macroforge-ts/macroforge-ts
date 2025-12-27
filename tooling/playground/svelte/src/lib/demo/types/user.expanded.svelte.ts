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
        __type: `${"User"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"email"}`] = option<string>SerializeWithContext(value.email, ctx);
    result[`${"firstName"}`] = value.firstName;
    result[`${"lastName"}`] = value.lastName;
    result[`${"password"}`] = option<string>SerializeWithContext(value.password, ctx);
    result[`${"metadata"}`] = option<metadata>SerializeWithContext(value.metadata, ctx);
    result[`${"settings"}`] = settingsSerializeWithContext(value.settings, ctx);
    result[`${"role"}`] = userRoleSerializeWithContext(value.role, ctx);
    result[`${"emailVerified"}`] = value.emailVerified;
    result[`${"verificationToken"}`] = option<string>SerializeWithContext(value.verificationToken, ctx);
    result[`${"verificationExpires"}`] = option<dateTime.dateTime>SerializeWithContext(value.verificationExpires, ctx);
    result[`${"passwordResetToken"}`] = option<string>SerializeWithContext(value.passwordResetToken, ctx);
    result[`${"passwordResetExpires"}`] = option<dateTime.dateTime>SerializeWithContext(value.passwordResetExpires, ctx);
    result[`${"permissions"}`] = appPermissionsSerializeWithContext(value.permissions, ctx);
    result[`${"createdAt"}`] = dateTime.dateTimeSerializeWithContext(value.createdAt, ctx);
    result[`${"lastLoginAt"}`] = option<dateTime.dateTime>SerializeWithContext(value.lastLoginAt, ctx);
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
    if (!(`${"id"}` in obj)) {
        errors.push({
            field: `${"id"}`,
            message: "missing required field"
        });
    }
    if (!(`${"email"}` in obj)) {
        errors.push({
            field: `${"email"}`,
            message: "missing required field"
        });
    }
    if (!(`${"firstName"}` in obj)) {
        errors.push({
            field: `${"firstName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lastName"}` in obj)) {
        errors.push({
            field: `${"lastName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"password"}` in obj)) {
        errors.push({
            field: `${"password"}`,
            message: "missing required field"
        });
    }
    if (!(`${"metadata"}` in obj)) {
        errors.push({
            field: `${"metadata"}`,
            message: "missing required field"
        });
    }
    if (!(`${"settings"}` in obj)) {
        errors.push({
            field: `${"settings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"role"}` in obj)) {
        errors.push({
            field: `${"role"}`,
            message: "missing required field"
        });
    }
    if (!(`${"emailVerified"}` in obj)) {
        errors.push({
            field: `${"emailVerified"}`,
            message: "missing required field"
        });
    }
    if (!(`${"verificationToken"}` in obj)) {
        errors.push({
            field: `${"verificationToken"}`,
            message: "missing required field"
        });
    }
    if (!(`${"verificationExpires"}` in obj)) {
        errors.push({
            field: `${"verificationExpires"}`,
            message: "missing required field"
        });
    }
    if (!(`${"passwordResetToken"}` in obj)) {
        errors.push({
            field: `${"passwordResetToken"}`,
            message: "missing required field"
        });
    }
    if (!(`${"passwordResetExpires"}` in obj)) {
        errors.push({
            field: `${"passwordResetExpires"}`,
            message: "missing required field"
        });
    }
    if (!(`${"permissions"}` in obj)) {
        errors.push({
            field: `${"permissions"}`,
            message: "missing required field"
        });
    }
    if (!(`${"createdAt"}` in obj)) {
        errors.push({
            field: `${"createdAt"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lastLoginAt"}` in obj)) {
        errors.push({
            field: `${"lastLoginAt"}`,
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
        const __raw_id = obj[`${"id"}`] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_email = obj[`${"email"}`] as Option<string>;
        {
            const __result = option<string>DeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, `${"email"}`, __result);
        }
    }
    {
        const __raw_firstName = obj[`${"firstName"}`] as string;
        if (__raw_firstName.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "User.firstName must not be empty"
            });
        }
        instance.firstName = __raw_firstName;
    }
    {
        const __raw_lastName = obj[`${"lastName"}`] as string;
        if (__raw_lastName.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "User.lastName must not be empty"
            });
        }
        instance.lastName = __raw_lastName;
    }
    {
        const __raw_password = obj[`${"password"}`] as Option<string>;
        {
            const __result = option<string>DeserializeWithContext(__raw_password, ctx);
            ctx.assignOrDefer(instance, `${"password"}`, __result);
        }
    }
    {
        const __raw_metadata = obj[`${"metadata"}`] as Option<Metadata>;
        {
            const __result = option<metadata>DeserializeWithContext(__raw_metadata, ctx);
            ctx.assignOrDefer(instance, `${"metadata"}`, __result);
        }
    }
    {
        const __raw_settings = obj[`${"settings"}`] as Settings;
        {
            const __result = settingsDeserializeWithContext(__raw_settings, ctx);
            ctx.assignOrDefer(instance, `${"settings"}`, __result);
        }
    }
    {
        const __raw_role = obj[`${"role"}`] as UserRole;
        {
            const __result = userRoleDeserializeWithContext(__raw_role, ctx);
            ctx.assignOrDefer(instance, `${"role"}`, __result);
        }
    }
    {
        const __raw_emailVerified = obj[`${"emailVerified"}`] as boolean;
        instance.emailVerified = __raw_emailVerified;
    }
    {
        const __raw_verificationToken = obj[`${"verificationToken"}`] as Option<string>;
        {
            const __result = option<string>DeserializeWithContext(__raw_verificationToken, ctx);
            ctx.assignOrDefer(instance, `${"verificationToken"}`, __result);
        }
    }
    {
        const __raw_verificationExpires = obj[`${"verificationExpires"}`] as Option<DateTime.DateTime>;
        {
            const __result = option<dateTime.dateTime>DeserializeWithContext(__raw_verificationExpires, ctx);
            ctx.assignOrDefer(instance, `${"verificationExpires"}`, __result);
        }
    }
    {
        const __raw_passwordResetToken = obj[`${"passwordResetToken"}`] as Option<string>;
        {
            const __result = option<string>DeserializeWithContext(__raw_passwordResetToken, ctx);
            ctx.assignOrDefer(instance, `${"passwordResetToken"}`, __result);
        }
    }
    {
        const __raw_passwordResetExpires = obj[`${"passwordResetExpires"}`] as Option<DateTime.DateTime>;
        {
            const __result = option<dateTime.dateTime>DeserializeWithContext(__raw_passwordResetExpires, ctx);
            ctx.assignOrDefer(instance, `${"passwordResetExpires"}`, __result);
        }
    }
    {
        const __raw_permissions = obj[`${"permissions"}`] as AppPermissions;
        {
            const __result = appPermissionsDeserializeWithContext(__raw_permissions, ctx);
            ctx.assignOrDefer(instance, `${"permissions"}`, __result);
        }
    }
    {
        const __raw_createdAt = obj[`${"createdAt"}`] as DateTime.DateTime;
        {
            const __result = dateTime.dateTimeDeserializeWithContext(__raw_createdAt, ctx);
            ctx.assignOrDefer(instance, `${"createdAt"}`, __result);
        }
    }
    {
        const __raw_lastLoginAt = obj[`${"lastLoginAt"}`] as Option<DateTime.DateTime>;
        {
            const __result = option<dateTime.dateTime>DeserializeWithContext(__raw_lastLoginAt, ctx);
            ctx.assignOrDefer(instance, `${"lastLoginAt"}`, __result);
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
    if (_field === `${"firstName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "User.firstName must not be empty"
            });
        }
    }
    if (_field === `${"lastName"}`) {
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
    if (`${"firstName"}` in _partial && _partial.firstName !== undefined) {
        const __val = _partial.firstName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "User.firstName must not be empty"
            });
        }
    }
    if (`${"lastName"}` in _partial && _partial.lastName !== undefined) {
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

export const User = {
  defaultValue: userDefaultValue,
  serialize: userSerialize,
  serializeWithContext: userSerializeWithContext,
  deserialize: userDeserialize,
  deserializeWithContext: userDeserializeWithContext,
  validateFields: userValidateFields,
  hasShape: userHasShape,
  is: userIs
} as const;