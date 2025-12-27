import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


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

export function userDefaultValue(): User {
    return {
        id: "",
        email: null,
        firstName: "",
        lastName: "",
        password: null,
        metadata: null,
        settings: settingsDefaultValue(),
        role: "Administrator",
        emailVerified: false,
        verificationToken: null,
        verificationExpires: null,
        passwordResetToken: null,
        passwordResetExpires: null,
        permissions: appPermissionsDefaultValue()
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
    result[`${"email"}`] = value.email;
    result[`${"firstName"}`] = value.firstName;
    result[`${"lastName"}`] = value.lastName;
    result[`${"password"}`] = value.password;
    if (value.metadata !== null) {
        result[`${"metadata"}`] = metadataSerializeWithContext(value.metadata, ctx);
    }
    result[`${"settings"}`] = settingsSerializeWithContext(value.settings, ctx);
    result[`${"role"}`] = userRoleSerializeWithContext(value.role, ctx);
    result[`${"emailVerified"}`] = value.emailVerified;
    result[`${"verificationToken"}`] = value.verificationToken;
    result[`${"verificationExpires"}`] = value.verificationExpires;
    result[`${"passwordResetToken"}`] = value.passwordResetToken;
    result[`${"passwordResetExpires"}`] = value.passwordResetExpires;
    result[`${"permissions"}`] = appPermissionsSerializeWithContext(value.permissions, ctx);
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
        const __raw_email = obj[`${"email"}`] as string | null;
        instance.email = __raw_email;
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
        const __raw_password = obj[`${"password"}`] as string | null;
        instance.password = __raw_password;
    }
    {
        const __raw_metadata = obj[`${"metadata"}`] as Metadata | null;
        if (__raw_metadata === null) {
            instance.metadata = null;
        } else {
            const __result = metadataDeserializeWithContext(__raw_metadata, ctx);
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
        const __raw_verificationToken = obj[`${"verificationToken"}`] as string | null;
        instance.verificationToken = __raw_verificationToken;
    }
    {
        const __raw_verificationExpires = obj[`${"verificationExpires"}`] as string | null;
        instance.verificationExpires = __raw_verificationExpires;
    }
    {
        const __raw_passwordResetToken = obj[`${"passwordResetToken"}`] as string | null;
        instance.passwordResetToken = __raw_passwordResetToken;
    }
    {
        const __raw_passwordResetExpires = obj[`${"passwordResetExpires"}`] as string | null;
        instance.passwordResetExpires = __raw_passwordResetExpires;
    }
    {
        const __raw_permissions = obj[`${"permissions"}`] as AppPermissions;
        {
            const __result = appPermissionsDeserializeWithContext(__raw_permissions, ctx);
            ctx.assignOrDefer(instance, `${"permissions"}`, __result);
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
    return '"id" in o && "email" in o && "firstName" in o && "lastName" in o && "password" in o && "metadata" in o && "settings" in o && "role" in o && "emailVerified" in o && "verificationToken" in o && "verificationExpires" in o && "passwordResetToken" in o && "passwordResetExpires" in o && "permissions" in o';
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


export interface Service {
    
    id: string;
    
    
    name: string;
    
    
    quickCode: string;
    
    group: string | null;
    
    subgroup: string | null;
    
    unit: string | null;
    
    active: boolean;
    
    commission: boolean;
    
    favorite: boolean;
    
    averageTime: string | null;
    defaults: ServiceDefaults;
}

export function serviceDefaultValue(): Service {
    return {
        id: "",
        name: "",
        quickCode: "",
        group: null,
        subgroup: null,
        unit: null,
        active: false,
        commission: false,
        favorite: false,
        averageTime: null,
        defaults: serviceDefaultsDefaultValue()
    } as Service;
}

export function serviceSerialize(value: Service): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(serviceSerializeWithContext(value, ctx));
}
export function serviceSerializeWithContext(value: Service, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Service"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"name"}`] = value.name;
    result[`${"quickCode"}`] = value.quickCode;
    result[`${"group"}`] = value.group;
    result[`${"subgroup"}`] = value.subgroup;
    result[`${"unit"}`] = value.unit;
    result[`${"active"}`] = value.active;
    result[`${"commission"}`] = value.commission;
    result[`${"favorite"}`] = value.favorite;
    result[`${"averageTime"}`] = value.averageTime;
    result[`${"defaults"}`] = serviceDefaultsSerializeWithContext(value.defaults, ctx);
    return result;
}

export function serviceDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Service } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = serviceDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Service.deserialize: root cannot be a forward reference"
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
export function serviceDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Service | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Service"}.deserializeWithContext: expected an object`
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
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
            message: "missing required field"
        });
    }
    if (!(`${"quickCode"}` in obj)) {
        errors.push({
            field: `${"quickCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"group"}` in obj)) {
        errors.push({
            field: `${"group"}`,
            message: "missing required field"
        });
    }
    if (!(`${"subgroup"}` in obj)) {
        errors.push({
            field: `${"subgroup"}`,
            message: "missing required field"
        });
    }
    if (!(`${"unit"}` in obj)) {
        errors.push({
            field: `${"unit"}`,
            message: "missing required field"
        });
    }
    if (!(`${"active"}` in obj)) {
        errors.push({
            field: `${"active"}`,
            message: "missing required field"
        });
    }
    if (!(`${"commission"}` in obj)) {
        errors.push({
            field: `${"commission"}`,
            message: "missing required field"
        });
    }
    if (!(`${"favorite"}` in obj)) {
        errors.push({
            field: `${"favorite"}`,
            message: "missing required field"
        });
    }
    if (!(`${"averageTime"}` in obj)) {
        errors.push({
            field: `${"averageTime"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaults"}` in obj)) {
        errors.push({
            field: `${"defaults"}`,
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
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Service.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_quickCode = obj[`${"quickCode"}`] as string;
        if (__raw_quickCode.trim().length === 0) {
            errors.push({
                field: "quickCode",
                message: "Service.quickCode must not be empty"
            });
        }
        instance.quickCode = __raw_quickCode;
    }
    {
        const __raw_group = obj[`${"group"}`] as string | null;
        instance.group = __raw_group;
    }
    {
        const __raw_subgroup = obj[`${"subgroup"}`] as string | null;
        instance.subgroup = __raw_subgroup;
    }
    {
        const __raw_unit = obj[`${"unit"}`] as string | null;
        instance.unit = __raw_unit;
    }
    {
        const __raw_active = obj[`${"active"}`] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_commission = obj[`${"commission"}`] as boolean;
        instance.commission = __raw_commission;
    }
    {
        const __raw_favorite = obj[`${"favorite"}`] as boolean;
        instance.favorite = __raw_favorite;
    }
    {
        const __raw_averageTime = obj[`${"averageTime"}`] as string | null;
        instance.averageTime = __raw_averageTime;
    }
    {
        const __raw_defaults = obj[`${"defaults"}`] as ServiceDefaults;
        {
            const __result = serviceDefaultsDeserializeWithContext(__raw_defaults, ctx);
            ctx.assignOrDefer(instance, `${"defaults"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Service;
}
export function serviceValidateField<K extends keyof Service>(_field: K, _value: Service[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"name"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Service.name must not be empty"
            });
        }
    }
    if (_field === `${"quickCode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "quickCode",
                message: "Service.quickCode must not be empty"
            });
        }
    }
    return errors;
}
export function serviceValidateFields(_partial: Partial<Service>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"name"}` in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Service.name must not be empty"
            });
        }
    }
    if (`${"quickCode"}` in _partial && _partial.quickCode !== undefined) {
        const __val = _partial.quickCode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "quickCode",
                message: "Service.quickCode must not be empty"
            });
        }
    }
    return errors;
}
export function serviceHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "name" in o && "quickCode" in o && "group" in o && "subgroup" in o && "unit" in o && "active" in o && "commission" in o && "favorite" in o && "averageTime" in o && "defaults" in o';
}
export function serviceIs(obj: unknown): obj is Service {
    if (!serviceHasShape(obj)) {
        return false;
    }
    const result = serviceDeserialize(obj);
    return result.success;
}

export const Service = {
  defaultValue: serviceDefaultValue,
  serialize: serviceSerialize,
  serializeWithContext: serviceSerializeWithContext,
  deserialize: serviceDeserialize,
  deserializeWithContext: serviceDeserializeWithContext,
  validateFields: serviceValidateFields,
  hasShape: serviceHasShape,
  is: serviceIs
} as const;


export interface ServiceDefaults {
    
    price: number;
    
    
    description: string;
}

export function serviceDefaultsDefaultValue(): ServiceDefaults {
    return {
        price: 0,
        description: ""
    } as ServiceDefaults;
}

export function serviceDefaultsSerialize(value: ServiceDefaults): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(serviceDefaultsSerializeWithContext(value, ctx));
}
export function serviceDefaultsSerializeWithContext(value: ServiceDefaults, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"ServiceDefaults"}`,
        __id
    };
    result[`${"price"}`] = value.price;
    result[`${"description"}`] = value.description;
    return result;
}

export function serviceDefaultsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ServiceDefaults } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = serviceDefaultsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ServiceDefaults.deserialize: root cannot be a forward reference"
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
export function serviceDefaultsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ServiceDefaults | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"ServiceDefaults"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"price"}` in obj)) {
        errors.push({
            field: `${"price"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
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
        const __raw_price = obj[`${"price"}`] as number;
        instance.price = __raw_price;
    }
    {
        const __raw_description = obj[`${"description"}`] as string;
        if (__raw_description.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ServiceDefaults.description must not be empty"
            });
        }
        instance.description = __raw_description;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as ServiceDefaults;
}
export function serviceDefaultsValidateField<K extends keyof ServiceDefaults>(_field: K, _value: ServiceDefaults[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"description"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ServiceDefaults.description must not be empty"
            });
        }
    }
    return errors;
}
export function serviceDefaultsValidateFields(_partial: Partial<ServiceDefaults>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"description"}` in _partial && _partial.description !== undefined) {
        const __val = _partial.description as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ServiceDefaults.description must not be empty"
            });
        }
    }
    return errors;
}
export function serviceDefaultsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"price" in o && "description" in o';
}
export function serviceDefaultsIs(obj: unknown): obj is ServiceDefaults {
    if (!serviceDefaultsHasShape(obj)) {
        return false;
    }
    const result = serviceDefaultsDeserialize(obj);
    return result.success;
}

export const ServiceDefaults = {
  defaultValue: serviceDefaultsDefaultValue,
  serialize: serviceDefaultsSerialize,
  serializeWithContext: serviceDefaultsSerializeWithContext,
  deserialize: serviceDefaultsDeserialize,
  deserializeWithContext: serviceDefaultsDeserializeWithContext,
  validateFields: serviceDefaultsValidateFields,
  hasShape: serviceDefaultsHasShape,
  is: serviceDefaultsIs
} as const;


export interface Did {
    
    in: string | Actor;
    
    out: string | Target;
    id: string;
    activityType: ActivityType;
    createdAt: string;
    metadata: string | null;
}

export function didDefaultValue(): Did {
    return {
        in: "",
        out: "",
        id: "",
        activityType: activityTypeDefaultValue(),
        createdAt: "",
        metadata: null
    } as Did;
}

export function didSerialize(value: Did): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(didSerializeWithContext(value, ctx));
}
export function didSerializeWithContext(value: Did, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Did"}`,
        __id
    };
    result[`${"in"}`] = value.in;
    result[`${"out"}`] = value.out;
    result[`${"id"}`] = value.id;
    result[`${"activityType"}`] = activityTypeSerializeWithContext(value.activityType, ctx);
    result[`${"createdAt"}`] = value.createdAt;
    result[`${"metadata"}`] = value.metadata;
    return result;
}

export function didDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Did } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = didDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Did.deserialize: root cannot be a forward reference"
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
export function didDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Did | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Did"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"in"}` in obj)) {
        errors.push({
            field: `${"in"}`,
            message: "missing required field"
        });
    }
    if (!(`${"out"}` in obj)) {
        errors.push({
            field: `${"out"}`,
            message: "missing required field"
        });
    }
    if (!(`${"id"}` in obj)) {
        errors.push({
            field: `${"id"}`,
            message: "missing required field"
        });
    }
    if (!(`${"activityType"}` in obj)) {
        errors.push({
            field: `${"activityType"}`,
            message: "missing required field"
        });
    }
    if (!(`${"createdAt"}` in obj)) {
        errors.push({
            field: `${"createdAt"}`,
            message: "missing required field"
        });
    }
    if (!(`${"metadata"}` in obj)) {
        errors.push({
            field: `${"metadata"}`,
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
        const __raw_in = obj[`${"in"}`] as string | Actor;
        instance.in = __raw_in;
    }
    {
        const __raw_out = obj[`${"out"}`] as string | Target;
        instance.out = __raw_out;
    }
    {
        const __raw_id = obj[`${"id"}`] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_activityType = obj[`${"activityType"}`] as ActivityType;
        {
            const __result = activityTypeDeserializeWithContext(__raw_activityType, ctx);
            ctx.assignOrDefer(instance, `${"activityType"}`, __result);
        }
    }
    {
        const __raw_createdAt = obj[`${"createdAt"}`] as string;
        instance.createdAt = __raw_createdAt;
    }
    {
        const __raw_metadata = obj[`${"metadata"}`] as string | null;
        instance.metadata = __raw_metadata;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Did;
}
export function didValidateField<K extends keyof Did>(_field: K, _value: Did[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function didValidateFields(_partial: Partial<Did>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function didHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"in" in o && "out" in o && "id" in o && "activityType" in o && "createdAt" in o && "metadata" in o';
}
export function didIs(obj: unknown): obj is Did {
    if (!didHasShape(obj)) {
        return false;
    }
    const result = didDeserialize(obj);
    return result.success;
}

export const Did = {
  defaultValue: didDefaultValue,
  serialize: didSerialize,
  serializeWithContext: didSerializeWithContext,
  deserialize: didDeserialize,
  deserializeWithContext: didDeserializeWithContext,
  validateFields: didValidateFields,
  hasShape: didHasShape,
  is: didIs
} as const;


export interface PersonName {
    
    
    firstName: string;
    
    
    lastName: string;
}

export function personNameDefaultValue(): PersonName {
    return {
        firstName: "",
        lastName: ""
    } as PersonName;
}

export function personNameSerialize(value: PersonName): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(personNameSerializeWithContext(value, ctx));
}
export function personNameSerializeWithContext(value: PersonName, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"PersonName"}`,
        __id
    };
    result[`${"firstName"}`] = value.firstName;
    result[`${"lastName"}`] = value.lastName;
    return result;
}

export function personNameDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: PersonName } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = personNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "PersonName.deserialize: root cannot be a forward reference"
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
export function personNameDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): PersonName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"PersonName"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
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
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_firstName = obj[`${"firstName"}`] as string;
        if (__raw_firstName.trim().length === 0) {
            errors.push({
                field: "firstName",
                message: "PersonName.firstName must not be empty"
            });
        }
        instance.firstName = __raw_firstName;
    }
    {
        const __raw_lastName = obj[`${"lastName"}`] as string;
        if (__raw_lastName.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "PersonName.lastName must not be empty"
            });
        }
        instance.lastName = __raw_lastName;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as PersonName;
}
export function personNameValidateField<K extends keyof PersonName>(_field: K, _value: PersonName[K]): Array<{
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
                message: "PersonName.firstName must not be empty"
            });
        }
    }
    if (_field === `${"lastName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "PersonName.lastName must not be empty"
            });
        }
    }
    return errors;
}
export function personNameValidateFields(_partial: Partial<PersonName>): Array<{
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
                message: "PersonName.firstName must not be empty"
            });
        }
    }
    if (`${"lastName"}` in _partial && _partial.lastName !== undefined) {
        const __val = _partial.lastName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lastName",
                message: "PersonName.lastName must not be empty"
            });
        }
    }
    return errors;
}
export function personNameHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"firstName" in o && "lastName" in o';
}
export function personNameIs(obj: unknown): obj is PersonName {
    if (!personNameHasShape(obj)) {
        return false;
    }
    const result = personNameDeserialize(obj);
    return result.success;
}

export const PersonName = {
  defaultValue: personNameDefaultValue,
  serialize: personNameSerialize,
  serializeWithContext: personNameSerializeWithContext,
  deserialize: personNameDeserialize,
  deserializeWithContext: personNameDeserializeWithContext,
  validateFields: personNameValidateFields,
  hasShape: personNameHasShape,
  is: personNameIs
} as const;


export interface Promotion {
    id: string;
    date: string;
}

export function promotionDefaultValue(): Promotion {
    return {
        id: "",
        date: ""
    } as Promotion;
}

export function promotionSerialize(value: Promotion): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(promotionSerializeWithContext(value, ctx));
}
export function promotionSerializeWithContext(value: Promotion, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Promotion"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"date"}`] = value.date;
    return result;
}

export function promotionDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Promotion } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = promotionDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Promotion.deserialize: root cannot be a forward reference"
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
export function promotionDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Promotion | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Promotion"}.deserializeWithContext: expected an object`
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
    if (!(`${"date"}` in obj)) {
        errors.push({
            field: `${"date"}`,
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
        const __raw_date = obj[`${"date"}`] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Promotion;
}
export function promotionValidateField<K extends keyof Promotion>(_field: K, _value: Promotion[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function promotionValidateFields(_partial: Partial<Promotion>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function promotionHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "date" in o';
}
export function promotionIs(obj: unknown): obj is Promotion {
    if (!promotionHasShape(obj)) {
        return false;
    }
    const result = promotionDeserialize(obj);
    return result.success;
}

export const Promotion = {
  defaultValue: promotionDefaultValue,
  serialize: promotionSerialize,
  serializeWithContext: promotionSerializeWithContext,
  deserialize: promotionDeserialize,
  deserializeWithContext: promotionDeserializeWithContext,
  validateFields: promotionValidateFields,
  hasShape: promotionHasShape,
  is: promotionIs
} as const;


export interface Site {
    id: string;
    
    addressLine1: string;
    addressLine2: string | null;
    sublocalityLevel1: string | null;
    
    locality: string;
    administrativeAreaLevel3: string | null;
    administrativeAreaLevel2: string | null;
    
    administrativeAreaLevel1: string;
    
    country: string;
    
    postalCode: string;
    postalCodeSuffix: string | null;
    coordinates: Coordinates;
}

export function siteDefaultValue(): Site {
    return {
        id: "",
        addressLine1: "",
        addressLine2: null,
        sublocalityLevel1: null,
        locality: "",
        administrativeAreaLevel3: null,
        administrativeAreaLevel2: null,
        administrativeAreaLevel1: "",
        country: "",
        postalCode: "",
        postalCodeSuffix: null,
        coordinates: coordinatesDefaultValue()
    } as Site;
}

export function siteSerialize(value: Site): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(siteSerializeWithContext(value, ctx));
}
export function siteSerializeWithContext(value: Site, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Site"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"addressLine1"}`] = value.addressLine1;
    result[`${"addressLine2"}`] = value.addressLine2;
    result[`${"sublocalityLevel1"}`] = value.sublocalityLevel1;
    result[`${"locality"}`] = value.locality;
    result[`${"administrativeAreaLevel3"}`] = value.administrativeAreaLevel3;
    result[`${"administrativeAreaLevel2"}`] = value.administrativeAreaLevel2;
    result[`${"administrativeAreaLevel1"}`] = value.administrativeAreaLevel1;
    result[`${"country"}`] = value.country;
    result[`${"postalCode"}`] = value.postalCode;
    result[`${"postalCodeSuffix"}`] = value.postalCodeSuffix;
    result[`${"coordinates"}`] = coordinatesSerializeWithContext(value.coordinates, ctx);
    return result;
}

export function siteDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Site } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = siteDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Site.deserialize: root cannot be a forward reference"
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
export function siteDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Site | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Site"}.deserializeWithContext: expected an object`
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
    if (!(`${"addressLine1"}` in obj)) {
        errors.push({
            field: `${"addressLine1"}`,
            message: "missing required field"
        });
    }
    if (!(`${"addressLine2"}` in obj)) {
        errors.push({
            field: `${"addressLine2"}`,
            message: "missing required field"
        });
    }
    if (!(`${"sublocalityLevel1"}` in obj)) {
        errors.push({
            field: `${"sublocalityLevel1"}`,
            message: "missing required field"
        });
    }
    if (!(`${"locality"}` in obj)) {
        errors.push({
            field: `${"locality"}`,
            message: "missing required field"
        });
    }
    if (!(`${"administrativeAreaLevel3"}` in obj)) {
        errors.push({
            field: `${"administrativeAreaLevel3"}`,
            message: "missing required field"
        });
    }
    if (!(`${"administrativeAreaLevel2"}` in obj)) {
        errors.push({
            field: `${"administrativeAreaLevel2"}`,
            message: "missing required field"
        });
    }
    if (!(`${"administrativeAreaLevel1"}` in obj)) {
        errors.push({
            field: `${"administrativeAreaLevel1"}`,
            message: "missing required field"
        });
    }
    if (!(`${"country"}` in obj)) {
        errors.push({
            field: `${"country"}`,
            message: "missing required field"
        });
    }
    if (!(`${"postalCode"}` in obj)) {
        errors.push({
            field: `${"postalCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"postalCodeSuffix"}` in obj)) {
        errors.push({
            field: `${"postalCodeSuffix"}`,
            message: "missing required field"
        });
    }
    if (!(`${"coordinates"}` in obj)) {
        errors.push({
            field: `${"coordinates"}`,
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
        const __raw_addressLine1 = obj[`${"addressLine1"}`] as string;
        if (__raw_addressLine1.trim().length === 0) {
            errors.push({
                field: "addressLine1",
                message: "Site.addressLine1 must not be empty"
            });
        }
        instance.addressLine1 = __raw_addressLine1;
    }
    {
        const __raw_addressLine2 = obj[`${"addressLine2"}`] as string | null;
        instance.addressLine2 = __raw_addressLine2;
    }
    {
        const __raw_sublocalityLevel1 = obj[`${"sublocalityLevel1"}`] as string | null;
        instance.sublocalityLevel1 = __raw_sublocalityLevel1;
    }
    {
        const __raw_locality = obj[`${"locality"}`] as string;
        if (__raw_locality.trim().length === 0) {
            errors.push({
                field: "locality",
                message: "Site.locality must not be empty"
            });
        }
        instance.locality = __raw_locality;
    }
    {
        const __raw_administrativeAreaLevel3 = obj[`${"administrativeAreaLevel3"}`] as string | null;
        instance.administrativeAreaLevel3 = __raw_administrativeAreaLevel3;
    }
    {
        const __raw_administrativeAreaLevel2 = obj[`${"administrativeAreaLevel2"}`] as string | null;
        instance.administrativeAreaLevel2 = __raw_administrativeAreaLevel2;
    }
    {
        const __raw_administrativeAreaLevel1 = obj[`${"administrativeAreaLevel1"}`] as string;
        if (__raw_administrativeAreaLevel1.trim().length === 0) {
            errors.push({
                field: "administrativeAreaLevel1",
                message: "Site.administrativeAreaLevel1 must not be empty"
            });
        }
        instance.administrativeAreaLevel1 = __raw_administrativeAreaLevel1;
    }
    {
        const __raw_country = obj[`${"country"}`] as string;
        if (__raw_country.trim().length === 0) {
            errors.push({
                field: "country",
                message: "Site.country must not be empty"
            });
        }
        instance.country = __raw_country;
    }
    {
        const __raw_postalCode = obj[`${"postalCode"}`] as string;
        if (__raw_postalCode.trim().length === 0) {
            errors.push({
                field: "postalCode",
                message: "Site.postalCode must not be empty"
            });
        }
        instance.postalCode = __raw_postalCode;
    }
    {
        const __raw_postalCodeSuffix = obj[`${"postalCodeSuffix"}`] as string | null;
        instance.postalCodeSuffix = __raw_postalCodeSuffix;
    }
    {
        const __raw_coordinates = obj[`${"coordinates"}`] as Coordinates;
        {
            const __result = coordinatesDeserializeWithContext(__raw_coordinates, ctx);
            ctx.assignOrDefer(instance, `${"coordinates"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Site;
}
export function siteValidateField<K extends keyof Site>(_field: K, _value: Site[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"addressLine1"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "addressLine1",
                message: "Site.addressLine1 must not be empty"
            });
        }
    }
    if (_field === `${"locality"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "locality",
                message: "Site.locality must not be empty"
            });
        }
    }
    if (_field === `${"administrativeAreaLevel1"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "administrativeAreaLevel1",
                message: "Site.administrativeAreaLevel1 must not be empty"
            });
        }
    }
    if (_field === `${"country"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "country",
                message: "Site.country must not be empty"
            });
        }
    }
    if (_field === `${"postalCode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "postalCode",
                message: "Site.postalCode must not be empty"
            });
        }
    }
    return errors;
}
export function siteValidateFields(_partial: Partial<Site>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"addressLine1"}` in _partial && _partial.addressLine1 !== undefined) {
        const __val = _partial.addressLine1 as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "addressLine1",
                message: "Site.addressLine1 must not be empty"
            });
        }
    }
    if (`${"locality"}` in _partial && _partial.locality !== undefined) {
        const __val = _partial.locality as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "locality",
                message: "Site.locality must not be empty"
            });
        }
    }
    if (`${"administrativeAreaLevel1"}` in _partial && _partial.administrativeAreaLevel1 !== undefined) {
        const __val = _partial.administrativeAreaLevel1 as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "administrativeAreaLevel1",
                message: "Site.administrativeAreaLevel1 must not be empty"
            });
        }
    }
    if (`${"country"}` in _partial && _partial.country !== undefined) {
        const __val = _partial.country as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "country",
                message: "Site.country must not be empty"
            });
        }
    }
    if (`${"postalCode"}` in _partial && _partial.postalCode !== undefined) {
        const __val = _partial.postalCode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "postalCode",
                message: "Site.postalCode must not be empty"
            });
        }
    }
    return errors;
}
export function siteHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "addressLine1" in o && "addressLine2" in o && "sublocalityLevel1" in o && "locality" in o && "administrativeAreaLevel3" in o && "administrativeAreaLevel2" in o && "administrativeAreaLevel1" in o && "country" in o && "postalCode" in o && "postalCodeSuffix" in o && "coordinates" in o';
}
export function siteIs(obj: unknown): obj is Site {
    if (!siteHasShape(obj)) {
        return false;
    }
    const result = siteDeserialize(obj);
    return result.success;
}

export const Site = {
  defaultValue: siteDefaultValue,
  serialize: siteSerialize,
  serializeWithContext: siteSerializeWithContext,
  deserialize: siteDeserialize,
  deserializeWithContext: siteDeserializeWithContext,
  validateFields: siteValidateFields,
  hasShape: siteHasShape,
  is: siteIs
} as const;


export interface Metadata {
    createdAt: string;
    lastLogin: string | null;
    isActive: boolean;
    roles: Array<string>;
}

export function metadataDefaultValue(): Metadata {
    return {
        createdAt: "",
        lastLogin: null,
        isActive: false,
        roles: []
    } as Metadata;
}

export function metadataSerialize(value: Metadata): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(metadataSerializeWithContext(value, ctx));
}
export function metadataSerializeWithContext(value: Metadata, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Metadata"}`,
        __id
    };
    result[`${"createdAt"}`] = value.createdAt;
    result[`${"lastLogin"}`] = value.lastLogin;
    result[`${"isActive"}`] = value.isActive;
    result[`${"roles"}`] = value.roles;
    return result;
}

export function metadataDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Metadata } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = metadataDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Metadata.deserialize: root cannot be a forward reference"
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
export function metadataDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Metadata | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Metadata"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"createdAt"}` in obj)) {
        errors.push({
            field: `${"createdAt"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lastLogin"}` in obj)) {
        errors.push({
            field: `${"lastLogin"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isActive"}` in obj)) {
        errors.push({
            field: `${"isActive"}`,
            message: "missing required field"
        });
    }
    if (!(`${"roles"}` in obj)) {
        errors.push({
            field: `${"roles"}`,
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
        const __raw_createdAt = obj[`${"createdAt"}`] as string;
        instance.createdAt = __raw_createdAt;
    }
    {
        const __raw_lastLogin = obj[`${"lastLogin"}`] as string | null;
        instance.lastLogin = __raw_lastLogin;
    }
    {
        const __raw_isActive = obj[`${"isActive"}`] as boolean;
        instance.isActive = __raw_isActive;
    }
    {
        const __raw_roles = obj[`${"roles"}`] as Array<string>;
        if (Array.isArray(__raw_roles)) {
            instance.roles = __raw_roles as string[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Metadata;
}
export function metadataValidateField<K extends keyof Metadata>(_field: K, _value: Metadata[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function metadataValidateFields(_partial: Partial<Metadata>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function metadataHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"createdAt" in o && "lastLogin" in o && "isActive" in o && "roles" in o';
}
export function metadataIs(obj: unknown): obj is Metadata {
    if (!metadataHasShape(obj)) {
        return false;
    }
    const result = metadataDeserialize(obj);
    return result.success;
}

export const Metadata = {
  defaultValue: metadataDefaultValue,
  serialize: metadataSerialize,
  serializeWithContext: metadataSerializeWithContext,
  deserialize: metadataDeserialize,
  deserializeWithContext: metadataDeserializeWithContext,
  validateFields: metadataValidateFields,
  hasShape: metadataHasShape,
  is: metadataIs
} as const;


export interface ColumnConfig {
    
    heading: string;
    dataPath: DataPath;
}

export function columnConfigDefaultValue(): ColumnConfig {
    return {
        heading: "",
        dataPath: dataPathDefaultValue()
    } as ColumnConfig;
}

export function columnConfigSerialize(value: ColumnConfig): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(columnConfigSerializeWithContext(value, ctx));
}
export function columnConfigSerializeWithContext(value: ColumnConfig, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"ColumnConfig"}`,
        __id
    };
    result[`${"heading"}`] = value.heading;
    result[`${"dataPath"}`] = dataPathSerializeWithContext(value.dataPath, ctx);
    return result;
}

export function columnConfigDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ColumnConfig } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = columnConfigDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ColumnConfig.deserialize: root cannot be a forward reference"
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
export function columnConfigDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ColumnConfig | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"ColumnConfig"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"heading"}` in obj)) {
        errors.push({
            field: `${"heading"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dataPath"}` in obj)) {
        errors.push({
            field: `${"dataPath"}`,
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
        const __raw_heading = obj[`${"heading"}`] as string;
        if (__raw_heading.trim().length === 0) {
            errors.push({
                field: "heading",
                message: "ColumnConfig.heading must not be empty"
            });
        }
        instance.heading = __raw_heading;
    }
    {
        const __raw_dataPath = obj[`${"dataPath"}`] as DataPath;
        {
            const __result = dataPathDeserializeWithContext(__raw_dataPath, ctx);
            ctx.assignOrDefer(instance, `${"dataPath"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as ColumnConfig;
}
export function columnConfigValidateField<K extends keyof ColumnConfig>(_field: K, _value: ColumnConfig[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"heading"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "heading",
                message: "ColumnConfig.heading must not be empty"
            });
        }
    }
    return errors;
}
export function columnConfigValidateFields(_partial: Partial<ColumnConfig>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"heading"}` in _partial && _partial.heading !== undefined) {
        const __val = _partial.heading as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "heading",
                message: "ColumnConfig.heading must not be empty"
            });
        }
    }
    return errors;
}
export function columnConfigHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"heading" in o && "dataPath" in o';
}
export function columnConfigIs(obj: unknown): obj is ColumnConfig {
    if (!columnConfigHasShape(obj)) {
        return false;
    }
    const result = columnConfigDeserialize(obj);
    return result.success;
}

export const ColumnConfig = {
  defaultValue: columnConfigDefaultValue,
  serialize: columnConfigSerialize,
  serializeWithContext: columnConfigSerializeWithContext,
  deserialize: columnConfigDeserialize,
  deserializeWithContext: columnConfigDeserializeWithContext,
  validateFields: columnConfigValidateFields,
  hasShape: columnConfigHasShape,
  is: columnConfigIs
} as const;


export interface PhoneNumber {
    
    main: boolean;
    
    
    phoneType: string;
    
    
    number: string;
    
    canText: boolean;
    
    canCall: boolean;
}

export function phoneNumberDefaultValue(): PhoneNumber {
    return {
        main: false,
        phoneType: "",
        number: "",
        canText: false,
        canCall: false
    } as PhoneNumber;
}

export function phoneNumberSerialize(value: PhoneNumber): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(phoneNumberSerializeWithContext(value, ctx));
}
export function phoneNumberSerializeWithContext(value: PhoneNumber, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"PhoneNumber"}`,
        __id
    };
    result[`${"main"}`] = value.main;
    result[`${"phoneType"}`] = value.phoneType;
    result[`${"number"}`] = value.number;
    result[`${"canText"}`] = value.canText;
    result[`${"canCall"}`] = value.canCall;
    return result;
}

export function phoneNumberDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: PhoneNumber } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = phoneNumberDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "PhoneNumber.deserialize: root cannot be a forward reference"
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
export function phoneNumberDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): PhoneNumber | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"PhoneNumber"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"main"}` in obj)) {
        errors.push({
            field: `${"main"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phoneType"}` in obj)) {
        errors.push({
            field: `${"phoneType"}`,
            message: "missing required field"
        });
    }
    if (!(`${"number"}` in obj)) {
        errors.push({
            field: `${"number"}`,
            message: "missing required field"
        });
    }
    if (!(`${"canText"}` in obj)) {
        errors.push({
            field: `${"canText"}`,
            message: "missing required field"
        });
    }
    if (!(`${"canCall"}` in obj)) {
        errors.push({
            field: `${"canCall"}`,
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
        const __raw_main = obj[`${"main"}`] as boolean;
        instance.main = __raw_main;
    }
    {
        const __raw_phoneType = obj[`${"phoneType"}`] as string;
        if (__raw_phoneType.trim().length === 0) {
            errors.push({
                field: "phoneType",
                message: "PhoneNumber.phoneType must not be empty"
            });
        }
        instance.phoneType = __raw_phoneType;
    }
    {
        const __raw_number = obj[`${"number"}`] as string;
        if (__raw_number.trim().length === 0) {
            errors.push({
                field: "number",
                message: "PhoneNumber.number must not be empty"
            });
        }
        instance.number = __raw_number;
    }
    {
        const __raw_canText = obj[`${"canText"}`] as boolean;
        instance.canText = __raw_canText;
    }
    {
        const __raw_canCall = obj[`${"canCall"}`] as boolean;
        instance.canCall = __raw_canCall;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as PhoneNumber;
}
export function phoneNumberValidateField<K extends keyof PhoneNumber>(_field: K, _value: PhoneNumber[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"phoneType"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "phoneType",
                message: "PhoneNumber.phoneType must not be empty"
            });
        }
    }
    if (_field === `${"number"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "number",
                message: "PhoneNumber.number must not be empty"
            });
        }
    }
    return errors;
}
export function phoneNumberValidateFields(_partial: Partial<PhoneNumber>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"phoneType"}` in _partial && _partial.phoneType !== undefined) {
        const __val = _partial.phoneType as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "phoneType",
                message: "PhoneNumber.phoneType must not be empty"
            });
        }
    }
    if (`${"number"}` in _partial && _partial.number !== undefined) {
        const __val = _partial.number as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "number",
                message: "PhoneNumber.number must not be empty"
            });
        }
    }
    return errors;
}
export function phoneNumberHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"main" in o && "phoneType" in o && "number" in o && "canText" in o && "canCall" in o';
}
export function phoneNumberIs(obj: unknown): obj is PhoneNumber {
    if (!phoneNumberHasShape(obj)) {
        return false;
    }
    const result = phoneNumberDeserialize(obj);
    return result.success;
}

export const PhoneNumber = {
  defaultValue: phoneNumberDefaultValue,
  serialize: phoneNumberSerialize,
  serializeWithContext: phoneNumberSerializeWithContext,
  deserialize: phoneNumberDeserialize,
  deserializeWithContext: phoneNumberDeserializeWithContext,
  validateFields: phoneNumberValidateFields,
  hasShape: phoneNumberHasShape,
  is: phoneNumberIs
} as const;


export interface Gradient {
    startHue: number;
}

export function gradientDefaultValue(): Gradient {
    return {
        startHue: 0
    } as Gradient;
}

export function gradientSerialize(value: Gradient): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(gradientSerializeWithContext(value, ctx));
}
export function gradientSerializeWithContext(value: Gradient, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Gradient"}`,
        __id
    };
    result[`${"startHue"}`] = value.startHue;
    return result;
}

export function gradientDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Gradient } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = gradientDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Gradient.deserialize: root cannot be a forward reference"
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
export function gradientDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Gradient | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Gradient"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"startHue"}` in obj)) {
        errors.push({
            field: `${"startHue"}`,
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
        const __raw_startHue = obj[`${"startHue"}`] as number;
        instance.startHue = __raw_startHue;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Gradient;
}
export function gradientValidateField<K extends keyof Gradient>(_field: K, _value: Gradient[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function gradientValidateFields(_partial: Partial<Gradient>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function gradientHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"startHue" in o';
}
export function gradientIs(obj: unknown): obj is Gradient {
    if (!gradientHasShape(obj)) {
        return false;
    }
    const result = gradientDeserialize(obj);
    return result.success;
}

export const Gradient = {
  defaultValue: gradientDefaultValue,
  serialize: gradientSerialize,
  serializeWithContext: gradientSerializeWithContext,
  deserialize: gradientDeserialize,
  deserializeWithContext: gradientDeserializeWithContext,
  validateFields: gradientValidateFields,
  hasShape: gradientHasShape,
  is: gradientIs
} as const;


export interface Product {
    
    id: string;
    
    
    name: string;
    
    
    quickCode: string;
    
    group: string | null;
    
    subgroup: string | null;
    
    unit: string | null;
    
    active: boolean;
    
    commission: boolean;
    
    favorite: boolean;
    defaults: ProductDefaults;
}

export function productDefaultValue(): Product {
    return {
        id: "",
        name: "",
        quickCode: "",
        group: null,
        subgroup: null,
        unit: null,
        active: false,
        commission: false,
        favorite: false,
        defaults: productDefaultsDefaultValue()
    } as Product;
}

export function productSerialize(value: Product): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(productSerializeWithContext(value, ctx));
}
export function productSerializeWithContext(value: Product, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Product"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"name"}`] = value.name;
    result[`${"quickCode"}`] = value.quickCode;
    result[`${"group"}`] = value.group;
    result[`${"subgroup"}`] = value.subgroup;
    result[`${"unit"}`] = value.unit;
    result[`${"active"}`] = value.active;
    result[`${"commission"}`] = value.commission;
    result[`${"favorite"}`] = value.favorite;
    result[`${"defaults"}`] = productDefaultsSerializeWithContext(value.defaults, ctx);
    return result;
}

export function productDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Product } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = productDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Product.deserialize: root cannot be a forward reference"
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
export function productDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Product | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Product"}.deserializeWithContext: expected an object`
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
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
            message: "missing required field"
        });
    }
    if (!(`${"quickCode"}` in obj)) {
        errors.push({
            field: `${"quickCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"group"}` in obj)) {
        errors.push({
            field: `${"group"}`,
            message: "missing required field"
        });
    }
    if (!(`${"subgroup"}` in obj)) {
        errors.push({
            field: `${"subgroup"}`,
            message: "missing required field"
        });
    }
    if (!(`${"unit"}` in obj)) {
        errors.push({
            field: `${"unit"}`,
            message: "missing required field"
        });
    }
    if (!(`${"active"}` in obj)) {
        errors.push({
            field: `${"active"}`,
            message: "missing required field"
        });
    }
    if (!(`${"commission"}` in obj)) {
        errors.push({
            field: `${"commission"}`,
            message: "missing required field"
        });
    }
    if (!(`${"favorite"}` in obj)) {
        errors.push({
            field: `${"favorite"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaults"}` in obj)) {
        errors.push({
            field: `${"defaults"}`,
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
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Product.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_quickCode = obj[`${"quickCode"}`] as string;
        if (__raw_quickCode.trim().length === 0) {
            errors.push({
                field: "quickCode",
                message: "Product.quickCode must not be empty"
            });
        }
        instance.quickCode = __raw_quickCode;
    }
    {
        const __raw_group = obj[`${"group"}`] as string | null;
        instance.group = __raw_group;
    }
    {
        const __raw_subgroup = obj[`${"subgroup"}`] as string | null;
        instance.subgroup = __raw_subgroup;
    }
    {
        const __raw_unit = obj[`${"unit"}`] as string | null;
        instance.unit = __raw_unit;
    }
    {
        const __raw_active = obj[`${"active"}`] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_commission = obj[`${"commission"}`] as boolean;
        instance.commission = __raw_commission;
    }
    {
        const __raw_favorite = obj[`${"favorite"}`] as boolean;
        instance.favorite = __raw_favorite;
    }
    {
        const __raw_defaults = obj[`${"defaults"}`] as ProductDefaults;
        {
            const __result = productDefaultsDeserializeWithContext(__raw_defaults, ctx);
            ctx.assignOrDefer(instance, `${"defaults"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Product;
}
export function productValidateField<K extends keyof Product>(_field: K, _value: Product[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"name"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Product.name must not be empty"
            });
        }
    }
    if (_field === `${"quickCode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "quickCode",
                message: "Product.quickCode must not be empty"
            });
        }
    }
    return errors;
}
export function productValidateFields(_partial: Partial<Product>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"name"}` in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Product.name must not be empty"
            });
        }
    }
    if (`${"quickCode"}` in _partial && _partial.quickCode !== undefined) {
        const __val = _partial.quickCode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "quickCode",
                message: "Product.quickCode must not be empty"
            });
        }
    }
    return errors;
}
export function productHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "name" in o && "quickCode" in o && "group" in o && "subgroup" in o && "unit" in o && "active" in o && "commission" in o && "favorite" in o && "defaults" in o';
}
export function productIs(obj: unknown): obj is Product {
    if (!productHasShape(obj)) {
        return false;
    }
    const result = productDeserialize(obj);
    return result.success;
}

export const Product = {
  defaultValue: productDefaultValue,
  serialize: productSerialize,
  serializeWithContext: productSerializeWithContext,
  deserialize: productDeserialize,
  deserializeWithContext: productDeserializeWithContext,
  validateFields: productValidateFields,
  hasShape: productHasShape,
  is: productIs
} as const;


export interface YearlyRecurrenceRule {
    quantityOfYears: number;
}

export function yearlyRecurrenceRuleDefaultValue(): YearlyRecurrenceRule {
    return {
        quantityOfYears: 0
    } as YearlyRecurrenceRule;
}

export function yearlyRecurrenceRuleSerialize(value: YearlyRecurrenceRule): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(yearlyRecurrenceRuleSerializeWithContext(value, ctx));
}
export function yearlyRecurrenceRuleSerializeWithContext(value: YearlyRecurrenceRule, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"YearlyRecurrenceRule"}`,
        __id
    };
    result[`${"quantityOfYears"}`] = value.quantityOfYears;
    return result;
}

export function yearlyRecurrenceRuleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: YearlyRecurrenceRule } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = yearlyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "YearlyRecurrenceRule.deserialize: root cannot be a forward reference"
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
export function yearlyRecurrenceRuleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): YearlyRecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"YearlyRecurrenceRule"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"quantityOfYears"}` in obj)) {
        errors.push({
            field: `${"quantityOfYears"}`,
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
        const __raw_quantityOfYears = obj[`${"quantityOfYears"}`] as number;
        instance.quantityOfYears = __raw_quantityOfYears;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as YearlyRecurrenceRule;
}
export function yearlyRecurrenceRuleValidateField<K extends keyof YearlyRecurrenceRule>(_field: K, _value: YearlyRecurrenceRule[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function yearlyRecurrenceRuleValidateFields(_partial: Partial<YearlyRecurrenceRule>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function yearlyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"quantityOfYears" in o';
}
export function yearlyRecurrenceRuleIs(obj: unknown): obj is YearlyRecurrenceRule {
    if (!yearlyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = yearlyRecurrenceRuleDeserialize(obj);
    return result.success;
}

export const YearlyRecurrenceRule = {
  defaultValue: yearlyRecurrenceRuleDefaultValue,
  serialize: yearlyRecurrenceRuleSerialize,
  serializeWithContext: yearlyRecurrenceRuleSerializeWithContext,
  deserialize: yearlyRecurrenceRuleDeserialize,
  deserializeWithContext: yearlyRecurrenceRuleDeserializeWithContext,
  validateFields: yearlyRecurrenceRuleValidateFields,
  hasShape: yearlyRecurrenceRuleHasShape,
  is: yearlyRecurrenceRuleIs
} as const;


export interface AppointmentNotifications {
    
    personalScheduleChangeNotifications: string;
    
    allScheduleChangeNotifications: string;
}

export function appointmentNotificationsDefaultValue(): AppointmentNotifications {
    return {
        personalScheduleChangeNotifications: "",
        allScheduleChangeNotifications: ""
    } as AppointmentNotifications;
}

export function appointmentNotificationsSerialize(value: AppointmentNotifications): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(appointmentNotificationsSerializeWithContext(value, ctx));
}
export function appointmentNotificationsSerializeWithContext(value: AppointmentNotifications, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"AppointmentNotifications"}`,
        __id
    };
    result[`${"personalScheduleChangeNotifications"}`] = value.personalScheduleChangeNotifications;
    result[`${"allScheduleChangeNotifications"}`] = value.allScheduleChangeNotifications;
    return result;
}

export function appointmentNotificationsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: AppointmentNotifications } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = appointmentNotificationsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "AppointmentNotifications.deserialize: root cannot be a forward reference"
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
export function appointmentNotificationsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): AppointmentNotifications | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"AppointmentNotifications"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"personalScheduleChangeNotifications"}` in obj)) {
        errors.push({
            field: `${"personalScheduleChangeNotifications"}`,
            message: "missing required field"
        });
    }
    if (!(`${"allScheduleChangeNotifications"}` in obj)) {
        errors.push({
            field: `${"allScheduleChangeNotifications"}`,
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
        const __raw_personalScheduleChangeNotifications = obj[`${"personalScheduleChangeNotifications"}`] as string;
        if (__raw_personalScheduleChangeNotifications.trim().length === 0) {
            errors.push({
                field: "personalScheduleChangeNotifications",
                message: "AppointmentNotifications.personalScheduleChangeNotifications must not be empty"
            });
        }
        instance.personalScheduleChangeNotifications = __raw_personalScheduleChangeNotifications;
    }
    {
        const __raw_allScheduleChangeNotifications = obj[`${"allScheduleChangeNotifications"}`] as string;
        if (__raw_allScheduleChangeNotifications.trim().length === 0) {
            errors.push({
                field: "allScheduleChangeNotifications",
                message: "AppointmentNotifications.allScheduleChangeNotifications must not be empty"
            });
        }
        instance.allScheduleChangeNotifications = __raw_allScheduleChangeNotifications;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as AppointmentNotifications;
}
export function appointmentNotificationsValidateField<K extends keyof AppointmentNotifications>(_field: K, _value: AppointmentNotifications[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"personalScheduleChangeNotifications"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "personalScheduleChangeNotifications",
                message: "AppointmentNotifications.personalScheduleChangeNotifications must not be empty"
            });
        }
    }
    if (_field === `${"allScheduleChangeNotifications"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "allScheduleChangeNotifications",
                message: "AppointmentNotifications.allScheduleChangeNotifications must not be empty"
            });
        }
    }
    return errors;
}
export function appointmentNotificationsValidateFields(_partial: Partial<AppointmentNotifications>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"personalScheduleChangeNotifications"}` in _partial && _partial.personalScheduleChangeNotifications !== undefined) {
        const __val = _partial.personalScheduleChangeNotifications as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "personalScheduleChangeNotifications",
                message: "AppointmentNotifications.personalScheduleChangeNotifications must not be empty"
            });
        }
    }
    if (`${"allScheduleChangeNotifications"}` in _partial && _partial.allScheduleChangeNotifications !== undefined) {
        const __val = _partial.allScheduleChangeNotifications as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "allScheduleChangeNotifications",
                message: "AppointmentNotifications.allScheduleChangeNotifications must not be empty"
            });
        }
    }
    return errors;
}
export function appointmentNotificationsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"personalScheduleChangeNotifications" in o && "allScheduleChangeNotifications" in o';
}
export function appointmentNotificationsIs(obj: unknown): obj is AppointmentNotifications {
    if (!appointmentNotificationsHasShape(obj)) {
        return false;
    }
    const result = appointmentNotificationsDeserialize(obj);
    return result.success;
}

export const AppointmentNotifications = {
  defaultValue: appointmentNotificationsDefaultValue,
  serialize: appointmentNotificationsSerialize,
  serializeWithContext: appointmentNotificationsSerializeWithContext,
  deserialize: appointmentNotificationsDeserialize,
  deserializeWithContext: appointmentNotificationsDeserializeWithContext,
  validateFields: appointmentNotificationsValidateFields,
  hasShape: appointmentNotificationsHasShape,
  is: appointmentNotificationsIs
} as const;


export interface DirectionHue {
    bearing: number;
    hue: number;
}

export function directionHueDefaultValue(): DirectionHue {
    return {
        bearing: 0,
        hue: 0
    } as DirectionHue;
}

export function directionHueSerialize(value: DirectionHue): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(directionHueSerializeWithContext(value, ctx));
}
export function directionHueSerializeWithContext(value: DirectionHue, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"DirectionHue"}`,
        __id
    };
    result[`${"bearing"}`] = value.bearing;
    result[`${"hue"}`] = value.hue;
    return result;
}

export function directionHueDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: DirectionHue } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = directionHueDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "DirectionHue.deserialize: root cannot be a forward reference"
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
export function directionHueDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): DirectionHue | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"DirectionHue"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"bearing"}` in obj)) {
        errors.push({
            field: `${"bearing"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hue"}` in obj)) {
        errors.push({
            field: `${"hue"}`,
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
        const __raw_bearing = obj[`${"bearing"}`] as number;
        instance.bearing = __raw_bearing;
    }
    {
        const __raw_hue = obj[`${"hue"}`] as number;
        instance.hue = __raw_hue;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as DirectionHue;
}
export function directionHueValidateField<K extends keyof DirectionHue>(_field: K, _value: DirectionHue[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function directionHueValidateFields(_partial: Partial<DirectionHue>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function directionHueHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"bearing" in o && "hue" in o';
}
export function directionHueIs(obj: unknown): obj is DirectionHue {
    if (!directionHueHasShape(obj)) {
        return false;
    }
    const result = directionHueDeserialize(obj);
    return result.success;
}

export const DirectionHue = {
  defaultValue: directionHueDefaultValue,
  serialize: directionHueSerialize,
  serializeWithContext: directionHueSerializeWithContext,
  deserialize: directionHueDeserialize,
  deserializeWithContext: directionHueDeserializeWithContext,
  validateFields: directionHueValidateFields,
  hasShape: directionHueHasShape,
  is: directionHueIs
} as const;


export interface MonthlyRecurrenceRule {
    quantityOfMonths: number;
    day: number;
    
    name: string;
}

export function monthlyRecurrenceRuleDefaultValue(): MonthlyRecurrenceRule {
    return {
        quantityOfMonths: 0,
        day: 0,
        name: ""
    } as MonthlyRecurrenceRule;
}

export function monthlyRecurrenceRuleSerialize(value: MonthlyRecurrenceRule): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(monthlyRecurrenceRuleSerializeWithContext(value, ctx));
}
export function monthlyRecurrenceRuleSerializeWithContext(value: MonthlyRecurrenceRule, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"MonthlyRecurrenceRule"}`,
        __id
    };
    result[`${"quantityOfMonths"}`] = value.quantityOfMonths;
    result[`${"day"}`] = value.day;
    result[`${"name"}`] = value.name;
    return result;
}

export function monthlyRecurrenceRuleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: MonthlyRecurrenceRule } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = monthlyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "MonthlyRecurrenceRule.deserialize: root cannot be a forward reference"
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
export function monthlyRecurrenceRuleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): MonthlyRecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"MonthlyRecurrenceRule"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"quantityOfMonths"}` in obj)) {
        errors.push({
            field: `${"quantityOfMonths"}`,
            message: "missing required field"
        });
    }
    if (!(`${"day"}` in obj)) {
        errors.push({
            field: `${"day"}`,
            message: "missing required field"
        });
    }
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
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
        const __raw_quantityOfMonths = obj[`${"quantityOfMonths"}`] as number;
        instance.quantityOfMonths = __raw_quantityOfMonths;
    }
    {
        const __raw_day = obj[`${"day"}`] as number;
        instance.day = __raw_day;
    }
    {
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "MonthlyRecurrenceRule.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as MonthlyRecurrenceRule;
}
export function monthlyRecurrenceRuleValidateField<K extends keyof MonthlyRecurrenceRule>(_field: K, _value: MonthlyRecurrenceRule[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"name"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "MonthlyRecurrenceRule.name must not be empty"
            });
        }
    }
    return errors;
}
export function monthlyRecurrenceRuleValidateFields(_partial: Partial<MonthlyRecurrenceRule>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"name"}` in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "MonthlyRecurrenceRule.name must not be empty"
            });
        }
    }
    return errors;
}
export function monthlyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"quantityOfMonths" in o && "day" in o && "name" in o';
}
export function monthlyRecurrenceRuleIs(obj: unknown): obj is MonthlyRecurrenceRule {
    if (!monthlyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = monthlyRecurrenceRuleDeserialize(obj);
    return result.success;
}

export const MonthlyRecurrenceRule = {
  defaultValue: monthlyRecurrenceRuleDefaultValue,
  serialize: monthlyRecurrenceRuleSerialize,
  serializeWithContext: monthlyRecurrenceRuleSerializeWithContext,
  deserialize: monthlyRecurrenceRuleDeserialize,
  deserializeWithContext: monthlyRecurrenceRuleDeserializeWithContext,
  validateFields: monthlyRecurrenceRuleValidateFields,
  hasShape: monthlyRecurrenceRuleHasShape,
  is: monthlyRecurrenceRuleIs
} as const;


export interface Represents {
    
    in: string | Employee;
    
    out: string | Account;
    id: string;
    dateStarted: string;
}

export function representsDefaultValue(): Represents {
    return {
        in: "",
        out: "",
        id: "",
        dateStarted: ""
    } as Represents;
}

export function representsSerialize(value: Represents): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(representsSerializeWithContext(value, ctx));
}
export function representsSerializeWithContext(value: Represents, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Represents"}`,
        __id
    };
    result[`${"in"}`] = value.in;
    result[`${"out"}`] = value.out;
    result[`${"id"}`] = value.id;
    result[`${"dateStarted"}`] = value.dateStarted;
    return result;
}

export function representsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Represents } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = representsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Represents.deserialize: root cannot be a forward reference"
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
export function representsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Represents | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Represents"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"in"}` in obj)) {
        errors.push({
            field: `${"in"}`,
            message: "missing required field"
        });
    }
    if (!(`${"out"}` in obj)) {
        errors.push({
            field: `${"out"}`,
            message: "missing required field"
        });
    }
    if (!(`${"id"}` in obj)) {
        errors.push({
            field: `${"id"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dateStarted"}` in obj)) {
        errors.push({
            field: `${"dateStarted"}`,
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
        const __raw_in = obj[`${"in"}`] as string | Employee;
        instance.in = __raw_in;
    }
    {
        const __raw_out = obj[`${"out"}`] as string | Account;
        instance.out = __raw_out;
    }
    {
        const __raw_id = obj[`${"id"}`] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_dateStarted = obj[`${"dateStarted"}`] as string;
        instance.dateStarted = __raw_dateStarted;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Represents;
}
export function representsValidateField<K extends keyof Represents>(_field: K, _value: Represents[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function representsValidateFields(_partial: Partial<Represents>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function representsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"in" in o && "out" in o && "id" in o && "dateStarted" in o';
}
export function representsIs(obj: unknown): obj is Represents {
    if (!representsHasShape(obj)) {
        return false;
    }
    const result = representsDeserialize(obj);
    return result.success;
}

export const Represents = {
  defaultValue: representsDefaultValue,
  serialize: representsSerialize,
  serializeWithContext: representsSerializeWithContext,
  deserialize: representsDeserialize,
  deserializeWithContext: representsDeserializeWithContext,
  validateFields: representsValidateFields,
  hasShape: representsHasShape,
  is: representsIs
} as const;


export interface Payment {
    id: string;
    date: string;
}

export function paymentDefaultValue(): Payment {
    return {
        id: "",
        date: ""
    } as Payment;
}

export function paymentSerialize(value: Payment): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(paymentSerializeWithContext(value, ctx));
}
export function paymentSerializeWithContext(value: Payment, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Payment"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"date"}`] = value.date;
    return result;
}

export function paymentDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Payment } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = paymentDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Payment.deserialize: root cannot be a forward reference"
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
export function paymentDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Payment | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Payment"}.deserializeWithContext: expected an object`
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
    if (!(`${"date"}` in obj)) {
        errors.push({
            field: `${"date"}`,
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
        const __raw_date = obj[`${"date"}`] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Payment;
}
export function paymentValidateField<K extends keyof Payment>(_field: K, _value: Payment[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function paymentValidateFields(_partial: Partial<Payment>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function paymentHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "date" in o';
}
export function paymentIs(obj: unknown): obj is Payment {
    if (!paymentHasShape(obj)) {
        return false;
    }
    const result = paymentDeserialize(obj);
    return result.success;
}

export const Payment = {
  defaultValue: paymentDefaultValue,
  serialize: paymentSerialize,
  serializeWithContext: paymentSerializeWithContext,
  deserialize: paymentDeserialize,
  deserializeWithContext: paymentDeserializeWithContext,
  validateFields: paymentValidateFields,
  hasShape: paymentHasShape,
  is: paymentIs
} as const;


export interface Settings {
    appointmentNotifications: AppointmentNotifications | null;
    commissions: Commissions | null;
    scheduleSettings: ScheduleSettings;
    accountOverviewSettings: OverviewSettings;
    serviceOverviewSettings: OverviewSettings;
    appointmentOverviewSettings: OverviewSettings;
    leadOverviewSettings: OverviewSettings;
    packageOverviewSettings: OverviewSettings;
    productOverviewSettings: OverviewSettings;
    orderOverviewSettings: OverviewSettings;
    taxRateOverviewSettings: OverviewSettings;
    
    homePage: Page;
}

export function settingsDefaultValue(): Settings {
    return {
        appointmentNotifications: null,
        commissions: null,
        scheduleSettings: scheduleSettingsDefaultValue(),
        accountOverviewSettings: overviewSettingsDefaultValue(),
        serviceOverviewSettings: overviewSettingsDefaultValue(),
        appointmentOverviewSettings: overviewSettingsDefaultValue(),
        leadOverviewSettings: overviewSettingsDefaultValue(),
        packageOverviewSettings: overviewSettingsDefaultValue(),
        productOverviewSettings: overviewSettingsDefaultValue(),
        orderOverviewSettings: overviewSettingsDefaultValue(),
        taxRateOverviewSettings: overviewSettingsDefaultValue(),
        homePage: "UserHome"
    } as Settings;
}

export function settingsSerialize(value: Settings): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(settingsSerializeWithContext(value, ctx));
}
export function settingsSerializeWithContext(value: Settings, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Settings"}`,
        __id
    };
    if (value.appointmentNotifications !== null) {
        result[`${"appointmentNotifications"}`] = appointmentNotificationsSerializeWithContext(value.appointmentNotifications, ctx);
    }
    if (value.commissions !== null) {
        result[`${"commissions"}`] = commissionsSerializeWithContext(value.commissions, ctx);
    }
    result[`${"scheduleSettings"}`] = scheduleSettingsSerializeWithContext(value.scheduleSettings, ctx);
    result[`${"accountOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.accountOverviewSettings, ctx);
    result[`${"serviceOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.serviceOverviewSettings, ctx);
    result[`${"appointmentOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.appointmentOverviewSettings, ctx);
    result[`${"leadOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.leadOverviewSettings, ctx);
    result[`${"packageOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.packageOverviewSettings, ctx);
    result[`${"productOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.productOverviewSettings, ctx);
    result[`${"orderOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.orderOverviewSettings, ctx);
    result[`${"taxRateOverviewSettings"}`] = overviewSettingsSerializeWithContext(value.taxRateOverviewSettings, ctx);
    result[`${"homePage"}`] = pageSerializeWithContext(value.homePage, ctx);
    return result;
}

export function settingsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Settings } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = settingsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Settings.deserialize: root cannot be a forward reference"
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
export function settingsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Settings | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Settings"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"appointmentNotifications"}` in obj)) {
        errors.push({
            field: `${"appointmentNotifications"}`,
            message: "missing required field"
        });
    }
    if (!(`${"commissions"}` in obj)) {
        errors.push({
            field: `${"commissions"}`,
            message: "missing required field"
        });
    }
    if (!(`${"scheduleSettings"}` in obj)) {
        errors.push({
            field: `${"scheduleSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accountOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"accountOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"serviceOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"serviceOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"appointmentOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"appointmentOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"leadOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"leadOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"packageOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"packageOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"productOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"productOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"orderOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"orderOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxRateOverviewSettings"}` in obj)) {
        errors.push({
            field: `${"taxRateOverviewSettings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"homePage"}` in obj)) {
        errors.push({
            field: `${"homePage"}`,
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
        const __raw_appointmentNotifications = obj[`${"appointmentNotifications"}`] as AppointmentNotifications | null;
        if (__raw_appointmentNotifications === null) {
            instance.appointmentNotifications = null;
        } else {
            const __result = appointmentNotificationsDeserializeWithContext(__raw_appointmentNotifications, ctx);
            ctx.assignOrDefer(instance, `${"appointmentNotifications"}`, __result);
        }
    }
    {
        const __raw_commissions = obj[`${"commissions"}`] as Commissions | null;
        if (__raw_commissions === null) {
            instance.commissions = null;
        } else {
            const __result = commissionsDeserializeWithContext(__raw_commissions, ctx);
            ctx.assignOrDefer(instance, `${"commissions"}`, __result);
        }
    }
    {
        const __raw_scheduleSettings = obj[`${"scheduleSettings"}`] as ScheduleSettings;
        {
            const __result = scheduleSettingsDeserializeWithContext(__raw_scheduleSettings, ctx);
            ctx.assignOrDefer(instance, `${"scheduleSettings"}`, __result);
        }
    }
    {
        const __raw_accountOverviewSettings = obj[`${"accountOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_accountOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"accountOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_serviceOverviewSettings = obj[`${"serviceOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_serviceOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"serviceOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_appointmentOverviewSettings = obj[`${"appointmentOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_appointmentOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"appointmentOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_leadOverviewSettings = obj[`${"leadOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_leadOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"leadOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_packageOverviewSettings = obj[`${"packageOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_packageOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"packageOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_productOverviewSettings = obj[`${"productOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_productOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"productOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_orderOverviewSettings = obj[`${"orderOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_orderOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"orderOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_taxRateOverviewSettings = obj[`${"taxRateOverviewSettings"}`] as OverviewSettings;
        {
            const __result = overviewSettingsDeserializeWithContext(__raw_taxRateOverviewSettings, ctx);
            ctx.assignOrDefer(instance, `${"taxRateOverviewSettings"}`, __result);
        }
    }
    {
        const __raw_homePage = obj[`${"homePage"}`] as Page;
        {
            const __result = pageDeserializeWithContext(__raw_homePage, ctx);
            ctx.assignOrDefer(instance, `${"homePage"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Settings;
}
export function settingsValidateField<K extends keyof Settings>(_field: K, _value: Settings[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function settingsValidateFields(_partial: Partial<Settings>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function settingsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"appointmentNotifications" in o && "commissions" in o && "scheduleSettings" in o && "accountOverviewSettings" in o && "serviceOverviewSettings" in o && "appointmentOverviewSettings" in o && "leadOverviewSettings" in o && "packageOverviewSettings" in o && "productOverviewSettings" in o && "orderOverviewSettings" in o && "taxRateOverviewSettings" in o && "homePage" in o';
}
export function settingsIs(obj: unknown): obj is Settings {
    if (!settingsHasShape(obj)) {
        return false;
    }
    const result = settingsDeserialize(obj);
    return result.success;
}

export const Settings = {
  defaultValue: settingsDefaultValue,
  serialize: settingsSerialize,
  serializeWithContext: settingsSerializeWithContext,
  deserialize: settingsDeserialize,
  deserializeWithContext: settingsDeserializeWithContext,
  validateFields: settingsValidateFields,
  hasShape: settingsHasShape,
  is: settingsIs
} as const;


export interface Color {
    red: number;
    green: number;
    blue: number;
}

export function colorDefaultValue(): Color {
    return {
        red: 0,
        green: 0,
        blue: 0
    } as Color;
}

export function colorSerialize(value: Color): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(colorSerializeWithContext(value, ctx));
}
export function colorSerializeWithContext(value: Color, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Color"}`,
        __id
    };
    result[`${"red"}`] = value.red;
    result[`${"green"}`] = value.green;
    result[`${"blue"}`] = value.blue;
    return result;
}

export function colorDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Color } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = colorDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Color.deserialize: root cannot be a forward reference"
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
export function colorDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Color | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Color"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"red"}` in obj)) {
        errors.push({
            field: `${"red"}`,
            message: "missing required field"
        });
    }
    if (!(`${"green"}` in obj)) {
        errors.push({
            field: `${"green"}`,
            message: "missing required field"
        });
    }
    if (!(`${"blue"}` in obj)) {
        errors.push({
            field: `${"blue"}`,
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
        const __raw_red = obj[`${"red"}`] as number;
        instance.red = __raw_red;
    }
    {
        const __raw_green = obj[`${"green"}`] as number;
        instance.green = __raw_green;
    }
    {
        const __raw_blue = obj[`${"blue"}`] as number;
        instance.blue = __raw_blue;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Color;
}
export function colorValidateField<K extends keyof Color>(_field: K, _value: Color[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function colorValidateFields(_partial: Partial<Color>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function colorHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"red" in o && "green" in o && "blue" in o';
}
export function colorIs(obj: unknown): obj is Color {
    if (!colorHasShape(obj)) {
        return false;
    }
    const result = colorDeserialize(obj);
    return result.success;
}

export const Color = {
  defaultValue: colorDefaultValue,
  serialize: colorSerialize,
  serializeWithContext: colorSerializeWithContext,
  deserialize: colorDeserialize,
  deserializeWithContext: colorDeserializeWithContext,
  validateFields: colorValidateFields,
  hasShape: colorHasShape,
  is: colorIs
} as const;


export interface CompanyName {
    
    
    companyName: string;
}

export function companyNameDefaultValue(): CompanyName {
    return {
        companyName: ""
    } as CompanyName;
}

export function companyNameSerialize(value: CompanyName): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(companyNameSerializeWithContext(value, ctx));
}
export function companyNameSerializeWithContext(value: CompanyName, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"CompanyName"}`,
        __id
    };
    result[`${"companyName"}`] = value.companyName;
    return result;
}

export function companyNameDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: CompanyName } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = companyNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "CompanyName.deserialize: root cannot be a forward reference"
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
export function companyNameDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): CompanyName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"CompanyName"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"companyName"}` in obj)) {
        errors.push({
            field: `${"companyName"}`,
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
        const __raw_companyName = obj[`${"companyName"}`] as string;
        if (__raw_companyName.trim().length === 0) {
            errors.push({
                field: "companyName",
                message: "CompanyName.companyName must not be empty"
            });
        }
        instance.companyName = __raw_companyName;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as CompanyName;
}
export function companyNameValidateField<K extends keyof CompanyName>(_field: K, _value: CompanyName[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"companyName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "companyName",
                message: "CompanyName.companyName must not be empty"
            });
        }
    }
    return errors;
}
export function companyNameValidateFields(_partial: Partial<CompanyName>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"companyName"}` in _partial && _partial.companyName !== undefined) {
        const __val = _partial.companyName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "companyName",
                message: "CompanyName.companyName must not be empty"
            });
        }
    }
    return errors;
}
export function companyNameHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"companyName" in o';
}
export function companyNameIs(obj: unknown): obj is CompanyName {
    if (!companyNameHasShape(obj)) {
        return false;
    }
    const result = companyNameDeserialize(obj);
    return result.success;
}

export const CompanyName = {
  defaultValue: companyNameDefaultValue,
  serialize: companyNameSerialize,
  serializeWithContext: companyNameSerializeWithContext,
  deserialize: companyNameDeserialize,
  deserializeWithContext: companyNameDeserializeWithContext,
  validateFields: companyNameValidateFields,
  hasShape: companyNameHasShape,
  is: companyNameIs
} as const;


export interface Appointment {
    
    id: string;
    
    
    title: string;
    
    
    status: Status;
    
    begins: string;
    
    duration: number;
    
    timeZone: string;
    
    offsetMs: number;
    
    allDay: boolean;
    
    multiDay: boolean;
    
    employees: Array<string | Employee>;
    
    
    location: string | Site;
    
    description: string | null;
    
    
    colors: Colors;
    
    recurrenceRule: RecurrenceRule | null;
}

export function appointmentDefaultValue(): Appointment {
    return {
        id: "",
        title: "",
        status: "Scheduled",
        begins: "",
        duration: 0,
        timeZone: "",
        offsetMs: 0,
        allDay: false,
        multiDay: false,
        employees: [],
        location: "",
        description: null,
        colors: {
            main: "#000000",
            hover: "#333333",
            active: "#666666"
        },
        recurrenceRule: null
    } as Appointment;
}

export function appointmentSerialize(value: Appointment): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(appointmentSerializeWithContext(value, ctx));
}
export function appointmentSerializeWithContext(value: Appointment, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Appointment"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"title"}`] = value.title;
    result[`${"status"}`] = statusSerializeWithContext(value.status, ctx);
    result[`${"begins"}`] = value.begins;
    result[`${"duration"}`] = value.duration;
    result[`${"timeZone"}`] = value.timeZone;
    result[`${"offsetMs"}`] = value.offsetMs;
    result[`${"allDay"}`] = value.allDay;
    result[`${"multiDay"}`] = value.multiDay;
    result[`${"employees"}`] = value.employees;
    result[`${"location"}`] = value.location;
    result[`${"description"}`] = value.description;
    result[`${"colors"}`] = colorsSerializeWithContext(value.colors, ctx);
    if (value.recurrenceRule !== null) {
        result[`${"recurrenceRule"}`] = recurrenceRuleSerializeWithContext(value.recurrenceRule, ctx);
    }
    return result;
}

export function appointmentDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Appointment } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = appointmentDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Appointment.deserialize: root cannot be a forward reference"
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
export function appointmentDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Appointment | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Appointment"}.deserializeWithContext: expected an object`
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
    if (!(`${"title"}` in obj)) {
        errors.push({
            field: `${"title"}`,
            message: "missing required field"
        });
    }
    if (!(`${"status"}` in obj)) {
        errors.push({
            field: `${"status"}`,
            message: "missing required field"
        });
    }
    if (!(`${"begins"}` in obj)) {
        errors.push({
            field: `${"begins"}`,
            message: "missing required field"
        });
    }
    if (!(`${"duration"}` in obj)) {
        errors.push({
            field: `${"duration"}`,
            message: "missing required field"
        });
    }
    if (!(`${"timeZone"}` in obj)) {
        errors.push({
            field: `${"timeZone"}`,
            message: "missing required field"
        });
    }
    if (!(`${"offsetMs"}` in obj)) {
        errors.push({
            field: `${"offsetMs"}`,
            message: "missing required field"
        });
    }
    if (!(`${"allDay"}` in obj)) {
        errors.push({
            field: `${"allDay"}`,
            message: "missing required field"
        });
    }
    if (!(`${"multiDay"}` in obj)) {
        errors.push({
            field: `${"multiDay"}`,
            message: "missing required field"
        });
    }
    if (!(`${"employees"}` in obj)) {
        errors.push({
            field: `${"employees"}`,
            message: "missing required field"
        });
    }
    if (!(`${"location"}` in obj)) {
        errors.push({
            field: `${"location"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
            message: "missing required field"
        });
    }
    if (!(`${"colors"}` in obj)) {
        errors.push({
            field: `${"colors"}`,
            message: "missing required field"
        });
    }
    if (!(`${"recurrenceRule"}` in obj)) {
        errors.push({
            field: `${"recurrenceRule"}`,
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
        const __raw_title = obj[`${"title"}`] as string;
        if (__raw_title.trim().length === 0) {
            errors.push({
                field: "title",
                message: "Appointment.title must not be empty"
            });
        }
        instance.title = __raw_title;
    }
    {
        const __raw_status = obj[`${"status"}`] as Status;
        {
            const __result = statusDeserializeWithContext(__raw_status, ctx);
            ctx.assignOrDefer(instance, `${"status"}`, __result);
        }
    }
    {
        const __raw_begins = obj[`${"begins"}`] as string;
        instance.begins = __raw_begins;
    }
    {
        const __raw_duration = obj[`${"duration"}`] as number;
        instance.duration = __raw_duration;
    }
    {
        const __raw_timeZone = obj[`${"timeZone"}`] as string;
        instance.timeZone = __raw_timeZone;
    }
    {
        const __raw_offsetMs = obj[`${"offsetMs"}`] as number;
        instance.offsetMs = __raw_offsetMs;
    }
    {
        const __raw_allDay = obj[`${"allDay"}`] as boolean;
        instance.allDay = __raw_allDay;
    }
    {
        const __raw_multiDay = obj[`${"multiDay"}`] as boolean;
        instance.multiDay = __raw_multiDay;
    }
    {
        const __raw_employees = obj[`${"employees"}`] as Array<string | Employee>;
        if (Array.isArray(__raw_employees)) {
            instance.employees = __raw_employees as string | Employee[];
        }
    }
    {
        const __raw_location = obj[`${"location"}`] as string | Site;
        instance.location = __raw_location;
    }
    {
        const __raw_description = obj[`${"description"}`] as string | null;
        instance.description = __raw_description;
    }
    {
        const __raw_colors = obj[`${"colors"}`] as Colors;
        {
            const __result = colorsDeserializeWithContext(__raw_colors, ctx);
            ctx.assignOrDefer(instance, `${"colors"}`, __result);
        }
    }
    {
        const __raw_recurrenceRule = obj[`${"recurrenceRule"}`] as RecurrenceRule | null;
        if (__raw_recurrenceRule === null) {
            instance.recurrenceRule = null;
        } else {
            const __result = recurrenceRuleDeserializeWithContext(__raw_recurrenceRule, ctx);
            ctx.assignOrDefer(instance, `${"recurrenceRule"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Appointment;
}
export function appointmentValidateField<K extends keyof Appointment>(_field: K, _value: Appointment[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"title"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "title",
                message: "Appointment.title must not be empty"
            });
        }
    }
    return errors;
}
export function appointmentValidateFields(_partial: Partial<Appointment>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"title"}` in _partial && _partial.title !== undefined) {
        const __val = _partial.title as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "title",
                message: "Appointment.title must not be empty"
            });
        }
    }
    return errors;
}
export function appointmentHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "title" in o && "status" in o && "begins" in o && "duration" in o && "timeZone" in o && "offsetMs" in o && "allDay" in o && "multiDay" in o && "employees" in o && "location" in o && "description" in o && "colors" in o && "recurrenceRule" in o';
}
export function appointmentIs(obj: unknown): obj is Appointment {
    if (!appointmentHasShape(obj)) {
        return false;
    }
    const result = appointmentDeserialize(obj);
    return result.success;
}

export const Appointment = {
  defaultValue: appointmentDefaultValue,
  serialize: appointmentSerialize,
  serializeWithContext: appointmentSerializeWithContext,
  deserialize: appointmentDeserialize,
  deserializeWithContext: appointmentDeserializeWithContext,
  validateFields: appointmentValidateFields,
  hasShape: appointmentHasShape,
  is: appointmentIs
} as const;


export interface Package {
    
    id: string;
    
    date: string;
}

export function packageDefaultValue(): Package {
    return {
        id: "",
        date: ""
    } as Package;
}

export function packageSerialize(value: Package): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(packageSerializeWithContext(value, ctx));
}
export function packageSerializeWithContext(value: Package, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Package"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"date"}`] = value.date;
    return result;
}

export function packageDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Package } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = packageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Package.deserialize: root cannot be a forward reference"
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
export function packageDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Package | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Package"}.deserializeWithContext: expected an object`
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
    if (!(`${"date"}` in obj)) {
        errors.push({
            field: `${"date"}`,
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
        const __raw_date = obj[`${"date"}`] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Package;
}
export function packageValidateField<K extends keyof Package>(_field: K, _value: Package[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function packageValidateFields(_partial: Partial<Package>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function packageHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "date" in o';
}
export function packageIs(obj: unknown): obj is Package {
    if (!packageHasShape(obj)) {
        return false;
    }
    const result = packageDeserialize(obj);
    return result.success;
}

export const Package = {
  defaultValue: packageDefaultValue,
  serialize: packageSerialize,
  serializeWithContext: packageSerializeWithContext,
  deserialize: packageDeserialize,
  deserializeWithContext: packageDeserializeWithContext,
  validateFields: packageValidateFields,
  hasShape: packageHasShape,
  is: packageIs
} as const;


export interface ScheduleSettings {
    daysPerWeek: number;
    
    rowHeight: RowHeight;
    visibleRoutes: Array<string>;
    detailedCards: boolean;
}

export function scheduleSettingsDefaultValue(): ScheduleSettings {
    return {
        daysPerWeek: 0,
        rowHeight: "Medium",
        visibleRoutes: [],
        detailedCards: false
    } as ScheduleSettings;
}

export function scheduleSettingsSerialize(value: ScheduleSettings): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(scheduleSettingsSerializeWithContext(value, ctx));
}
export function scheduleSettingsSerializeWithContext(value: ScheduleSettings, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"ScheduleSettings"}`,
        __id
    };
    result[`${"daysPerWeek"}`] = value.daysPerWeek;
    result[`${"rowHeight"}`] = rowHeightSerializeWithContext(value.rowHeight, ctx);
    result[`${"visibleRoutes"}`] = value.visibleRoutes;
    result[`${"detailedCards"}`] = value.detailedCards;
    return result;
}

export function scheduleSettingsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ScheduleSettings } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = scheduleSettingsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ScheduleSettings.deserialize: root cannot be a forward reference"
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
export function scheduleSettingsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ScheduleSettings | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"ScheduleSettings"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"daysPerWeek"}` in obj)) {
        errors.push({
            field: `${"daysPerWeek"}`,
            message: "missing required field"
        });
    }
    if (!(`${"rowHeight"}` in obj)) {
        errors.push({
            field: `${"rowHeight"}`,
            message: "missing required field"
        });
    }
    if (!(`${"visibleRoutes"}` in obj)) {
        errors.push({
            field: `${"visibleRoutes"}`,
            message: "missing required field"
        });
    }
    if (!(`${"detailedCards"}` in obj)) {
        errors.push({
            field: `${"detailedCards"}`,
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
        const __raw_daysPerWeek = obj[`${"daysPerWeek"}`] as number;
        instance.daysPerWeek = __raw_daysPerWeek;
    }
    {
        const __raw_rowHeight = obj[`${"rowHeight"}`] as RowHeight;
        {
            const __result = rowHeightDeserializeWithContext(__raw_rowHeight, ctx);
            ctx.assignOrDefer(instance, `${"rowHeight"}`, __result);
        }
    }
    {
        const __raw_visibleRoutes = obj[`${"visibleRoutes"}`] as Array<string>;
        if (Array.isArray(__raw_visibleRoutes)) {
            instance.visibleRoutes = __raw_visibleRoutes as string[];
        }
    }
    {
        const __raw_detailedCards = obj[`${"detailedCards"}`] as boolean;
        instance.detailedCards = __raw_detailedCards;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as ScheduleSettings;
}
export function scheduleSettingsValidateField<K extends keyof ScheduleSettings>(_field: K, _value: ScheduleSettings[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function scheduleSettingsValidateFields(_partial: Partial<ScheduleSettings>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function scheduleSettingsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"daysPerWeek" in o && "rowHeight" in o && "visibleRoutes" in o && "detailedCards" in o';
}
export function scheduleSettingsIs(obj: unknown): obj is ScheduleSettings {
    if (!scheduleSettingsHasShape(obj)) {
        return false;
    }
    const result = scheduleSettingsDeserialize(obj);
    return result.success;
}

export const ScheduleSettings = {
  defaultValue: scheduleSettingsDefaultValue,
  serialize: scheduleSettingsSerialize,
  serializeWithContext: scheduleSettingsSerializeWithContext,
  deserialize: scheduleSettingsDeserialize,
  deserializeWithContext: scheduleSettingsDeserializeWithContext,
  validateFields: scheduleSettingsValidateFields,
  hasShape: scheduleSettingsHasShape,
  is: scheduleSettingsIs
} as const;


export interface DailyRecurrenceRule {
    quantityOfDays: number;
}

export function dailyRecurrenceRuleDefaultValue(): DailyRecurrenceRule {
    return {
        quantityOfDays: 0
    } as DailyRecurrenceRule;
}

export function dailyRecurrenceRuleSerialize(value: DailyRecurrenceRule): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(dailyRecurrenceRuleSerializeWithContext(value, ctx));
}
export function dailyRecurrenceRuleSerializeWithContext(value: DailyRecurrenceRule, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"DailyRecurrenceRule"}`,
        __id
    };
    result[`${"quantityOfDays"}`] = value.quantityOfDays;
    return result;
}

export function dailyRecurrenceRuleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: DailyRecurrenceRule } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = dailyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "DailyRecurrenceRule.deserialize: root cannot be a forward reference"
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
export function dailyRecurrenceRuleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): DailyRecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"DailyRecurrenceRule"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"quantityOfDays"}` in obj)) {
        errors.push({
            field: `${"quantityOfDays"}`,
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
        const __raw_quantityOfDays = obj[`${"quantityOfDays"}`] as number;
        instance.quantityOfDays = __raw_quantityOfDays;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as DailyRecurrenceRule;
}
export function dailyRecurrenceRuleValidateField<K extends keyof DailyRecurrenceRule>(_field: K, _value: DailyRecurrenceRule[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function dailyRecurrenceRuleValidateFields(_partial: Partial<DailyRecurrenceRule>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function dailyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"quantityOfDays" in o';
}
export function dailyRecurrenceRuleIs(obj: unknown): obj is DailyRecurrenceRule {
    if (!dailyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = dailyRecurrenceRuleDeserialize(obj);
    return result.success;
}

export const DailyRecurrenceRule = {
  defaultValue: dailyRecurrenceRuleDefaultValue,
  serialize: dailyRecurrenceRuleSerialize,
  serializeWithContext: dailyRecurrenceRuleSerializeWithContext,
  deserialize: dailyRecurrenceRuleDeserialize,
  deserializeWithContext: dailyRecurrenceRuleDeserializeWithContext,
  validateFields: dailyRecurrenceRuleValidateFields,
  hasShape: dailyRecurrenceRuleHasShape,
  is: dailyRecurrenceRuleIs
} as const;


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

export function signUpCredentialsSerialize(value: SignUpCredentials): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(signUpCredentialsSerializeWithContext(value, ctx));
}
export function signUpCredentialsSerializeWithContext(value: SignUpCredentials, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"SignUpCredentials"}`,
        __id
    };
    result[`${"firstName"}`] = firstNameSerializeWithContext(value.firstName, ctx);
    result[`${"lastName"}`] = lastNameSerializeWithContext(value.lastName, ctx);
    result[`${"email"}`] = emailPartsSerializeWithContext(value.email, ctx);
    result[`${"password"}`] = passwordSerializeWithContext(value.password, ctx);
    result[`${"rememberMe"}`] = value.rememberMe;
    return result;
}

export function signUpCredentialsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: SignUpCredentials } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = signUpCredentialsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "SignUpCredentials.deserialize: root cannot be a forward reference"
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
export function signUpCredentialsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): SignUpCredentials | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"SignUpCredentials"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
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
    if (!(`${"email"}` in obj)) {
        errors.push({
            field: `${"email"}`,
            message: "missing required field"
        });
    }
    if (!(`${"password"}` in obj)) {
        errors.push({
            field: `${"password"}`,
            message: "missing required field"
        });
    }
    if (!(`${"rememberMe"}` in obj)) {
        errors.push({
            field: `${"rememberMe"}`,
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
        const __raw_firstName = obj[`${"firstName"}`] as FirstName;
        {
            const __result = firstNameDeserializeWithContext(__raw_firstName, ctx);
            ctx.assignOrDefer(instance, `${"firstName"}`, __result);
        }
    }
    {
        const __raw_lastName = obj[`${"lastName"}`] as LastName;
        {
            const __result = lastNameDeserializeWithContext(__raw_lastName, ctx);
            ctx.assignOrDefer(instance, `${"lastName"}`, __result);
        }
    }
    {
        const __raw_email = obj[`${"email"}`] as EmailParts;
        {
            const __result = emailPartsDeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, `${"email"}`, __result);
        }
    }
    {
        const __raw_password = obj[`${"password"}`] as Password;
        {
            const __result = passwordDeserializeWithContext(__raw_password, ctx);
            ctx.assignOrDefer(instance, `${"password"}`, __result);
        }
    }
    {
        const __raw_rememberMe = obj[`${"rememberMe"}`] as boolean;
        instance.rememberMe = __raw_rememberMe;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as SignUpCredentials;
}
export function signUpCredentialsValidateField<K extends keyof SignUpCredentials>(_field: K, _value: SignUpCredentials[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function signUpCredentialsValidateFields(_partial: Partial<SignUpCredentials>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function signUpCredentialsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"firstName" in o && "lastName" in o && "email" in o && "password" in o && "rememberMe" in o';
}
export function signUpCredentialsIs(obj: unknown): obj is SignUpCredentials {
    if (!signUpCredentialsHasShape(obj)) {
        return false;
    }
    const result = signUpCredentialsDeserialize(obj);
    return result.success;
}

export const SignUpCredentials = {
  defaultValue: signUpCredentialsDefaultValue,
  serialize: signUpCredentialsSerialize,
  serializeWithContext: signUpCredentialsSerializeWithContext,
  deserialize: signUpCredentialsDeserialize,
  deserializeWithContext: signUpCredentialsDeserializeWithContext,
  validateFields: signUpCredentialsValidateFields,
  hasShape: signUpCredentialsHasShape,
  is: signUpCredentialsIs
} as const;


export interface OverviewSettings {
    
    rowHeight: RowHeight;
    
    cardOrRow: OverviewDisplay;
    perPage: number;
    columnConfigs: Array<ColumnConfig>;
}

export function overviewSettingsDefaultValue(): OverviewSettings {
    return {
        rowHeight: "Medium",
        cardOrRow: "Table",
        perPage: 0,
        columnConfigs: []
    } as OverviewSettings;
}

export function overviewSettingsSerialize(value: OverviewSettings): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(overviewSettingsSerializeWithContext(value, ctx));
}
export function overviewSettingsSerializeWithContext(value: OverviewSettings, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"OverviewSettings"}`,
        __id
    };
    result[`${"rowHeight"}`] = rowHeightSerializeWithContext(value.rowHeight, ctx);
    result[`${"cardOrRow"}`] = overviewDisplaySerializeWithContext(value.cardOrRow, ctx);
    result[`${"perPage"}`] = value.perPage;
    result[`${"columnConfigs"}`] = value.columnConfigs.map((item)=>columnConfigSerializeWithContext(item, ctx));
    return result;
}

export function overviewSettingsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: OverviewSettings } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = overviewSettingsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "OverviewSettings.deserialize: root cannot be a forward reference"
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
export function overviewSettingsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): OverviewSettings | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"OverviewSettings"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"rowHeight"}` in obj)) {
        errors.push({
            field: `${"rowHeight"}`,
            message: "missing required field"
        });
    }
    if (!(`${"cardOrRow"}` in obj)) {
        errors.push({
            field: `${"cardOrRow"}`,
            message: "missing required field"
        });
    }
    if (!(`${"perPage"}` in obj)) {
        errors.push({
            field: `${"perPage"}`,
            message: "missing required field"
        });
    }
    if (!(`${"columnConfigs"}` in obj)) {
        errors.push({
            field: `${"columnConfigs"}`,
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
        const __raw_rowHeight = obj[`${"rowHeight"}`] as RowHeight;
        {
            const __result = rowHeightDeserializeWithContext(__raw_rowHeight, ctx);
            ctx.assignOrDefer(instance, `${"rowHeight"}`, __result);
        }
    }
    {
        const __raw_cardOrRow = obj[`${"cardOrRow"}`] as OverviewDisplay;
        {
            const __result = overviewDisplayDeserializeWithContext(__raw_cardOrRow, ctx);
            ctx.assignOrDefer(instance, `${"cardOrRow"}`, __result);
        }
    }
    {
        const __raw_perPage = obj[`${"perPage"}`] as number;
        instance.perPage = __raw_perPage;
    }
    {
        const __raw_columnConfigs = obj[`${"columnConfigs"}`] as Array<ColumnConfig>;
        if (Array.isArray(__raw_columnConfigs)) {
            instance.columnConfigs = __raw_columnConfigs as ColumnConfig[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as OverviewSettings;
}
export function overviewSettingsValidateField<K extends keyof OverviewSettings>(_field: K, _value: OverviewSettings[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function overviewSettingsValidateFields(_partial: Partial<OverviewSettings>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function overviewSettingsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"rowHeight" in o && "cardOrRow" in o && "perPage" in o && "columnConfigs" in o';
}
export function overviewSettingsIs(obj: unknown): obj is OverviewSettings {
    if (!overviewSettingsHasShape(obj)) {
        return false;
    }
    const result = overviewSettingsDeserialize(obj);
    return result.success;
}

export const OverviewSettings = {
  defaultValue: overviewSettingsDefaultValue,
  serialize: overviewSettingsSerialize,
  serializeWithContext: overviewSettingsSerializeWithContext,
  deserialize: overviewSettingsDeserialize,
  deserializeWithContext: overviewSettingsDeserializeWithContext,
  validateFields: overviewSettingsValidateFields,
  hasShape: overviewSettingsHasShape,
  is: overviewSettingsIs
} as const;


export interface FirstName {
    
    name: string;
}

export function firstNameDefaultValue(): FirstName {
    return {
        name: ""
    } as FirstName;
}

export function firstNameSerialize(value: FirstName): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(firstNameSerializeWithContext(value, ctx));
}
export function firstNameSerializeWithContext(value: FirstName, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"FirstName"}`,
        __id
    };
    result[`${"name"}`] = value.name;
    return result;
}

export function firstNameDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: FirstName } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = firstNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "FirstName.deserialize: root cannot be a forward reference"
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
export function firstNameDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): FirstName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"FirstName"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
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
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "FirstName.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as FirstName;
}
export function firstNameValidateField<K extends keyof FirstName>(_field: K, _value: FirstName[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"name"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "FirstName.name must not be empty"
            });
        }
    }
    return errors;
}
export function firstNameValidateFields(_partial: Partial<FirstName>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"name"}` in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "FirstName.name must not be empty"
            });
        }
    }
    return errors;
}
export function firstNameHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"name" in o';
}
export function firstNameIs(obj: unknown): obj is FirstName {
    if (!firstNameHasShape(obj)) {
        return false;
    }
    const result = firstNameDeserialize(obj);
    return result.success;
}

export const FirstName = {
  defaultValue: firstNameDefaultValue,
  serialize: firstNameSerialize,
  serializeWithContext: firstNameSerializeWithContext,
  deserialize: firstNameDeserialize,
  deserializeWithContext: firstNameDeserializeWithContext,
  validateFields: firstNameValidateFields,
  hasShape: firstNameHasShape,
  is: firstNameIs
} as const;


export interface Account {
    
    id: string;
    
    
    taxRate: string | TaxRate;
    
    
    site: string | Site;
    
    salesRep: Array<Represents> | null;
    
    orders: Array<Ordered>;
    
    activity: Array<Did>;
    
    customFields: Array<[string, string]>;
    
    accountName: AccountName;
    
    
    sector: Sector;
    
    memo: string | null;
    
    phones: Array<PhoneNumber>;
    
    email: Email;
    
    
    leadSource: string;
    
    colors: Colors;
    
    needsReview: boolean;
    
    hasAlert: boolean;
    
    
    accountType: string;
    
    
    subtype: string;
    
    isTaxExempt: boolean;
    
    
    paymentTerms: string;
    
    tags: Array<string>;
    
    dateAdded: string;
}

export function accountDefaultValue(): Account {
    return {
        id: "",
        taxRate: "",
        site: "",
        salesRep: null,
        orders: [],
        activity: [],
        customFields: [],
        accountName: accountNameDefaultValue(),
        sector: "Residential",
        memo: null,
        phones: [],
        email: emailDefaultValue(),
        leadSource: "",
        colors: colorsDefaultValue(),
        needsReview: false,
        hasAlert: false,
        accountType: "",
        subtype: "",
        isTaxExempt: false,
        paymentTerms: "",
        tags: [],
        dateAdded: ""
    } as Account;
}

export function accountSerialize(value: Account): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(accountSerializeWithContext(value, ctx));
}
export function accountSerializeWithContext(value: Account, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Account"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"taxRate"}`] = value.taxRate;
    result[`${"site"}`] = value.site;
    if (value.salesRep !== null) {
        result[`${"salesRep"}`] = value.salesRep;
    }
    result[`${"orders"}`] = value.orders.map((item)=>orderedSerializeWithContext(item, ctx));
    result[`${"activity"}`] = value.activity.map((item)=>didSerializeWithContext(item, ctx));
    result[`${"customFields"}`] = value.customFields;
    result[`${"accountName"}`] = accountNameSerializeWithContext(value.accountName, ctx);
    result[`${"sector"}`] = sectorSerializeWithContext(value.sector, ctx);
    result[`${"memo"}`] = value.memo;
    result[`${"phones"}`] = value.phones.map((item)=>phoneNumberSerializeWithContext(item, ctx));
    result[`${"email"}`] = emailSerializeWithContext(value.email, ctx);
    result[`${"leadSource"}`] = value.leadSource;
    result[`${"colors"}`] = colorsSerializeWithContext(value.colors, ctx);
    result[`${"needsReview"}`] = value.needsReview;
    result[`${"hasAlert"}`] = value.hasAlert;
    result[`${"accountType"}`] = value.accountType;
    result[`${"subtype"}`] = value.subtype;
    result[`${"isTaxExempt"}`] = value.isTaxExempt;
    result[`${"paymentTerms"}`] = value.paymentTerms;
    result[`${"tags"}`] = value.tags;
    result[`${"dateAdded"}`] = value.dateAdded;
    return result;
}

export function accountDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Account } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = accountDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Account.deserialize: root cannot be a forward reference"
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
export function accountDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Account | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Account"}.deserializeWithContext: expected an object`
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
    if (!(`${"taxRate"}` in obj)) {
        errors.push({
            field: `${"taxRate"}`,
            message: "missing required field"
        });
    }
    if (!(`${"site"}` in obj)) {
        errors.push({
            field: `${"site"}`,
            message: "missing required field"
        });
    }
    if (!(`${"salesRep"}` in obj)) {
        errors.push({
            field: `${"salesRep"}`,
            message: "missing required field"
        });
    }
    if (!(`${"orders"}` in obj)) {
        errors.push({
            field: `${"orders"}`,
            message: "missing required field"
        });
    }
    if (!(`${"activity"}` in obj)) {
        errors.push({
            field: `${"activity"}`,
            message: "missing required field"
        });
    }
    if (!(`${"customFields"}` in obj)) {
        errors.push({
            field: `${"customFields"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accountName"}` in obj)) {
        errors.push({
            field: `${"accountName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"sector"}` in obj)) {
        errors.push({
            field: `${"sector"}`,
            message: "missing required field"
        });
    }
    if (!(`${"memo"}` in obj)) {
        errors.push({
            field: `${"memo"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phones"}` in obj)) {
        errors.push({
            field: `${"phones"}`,
            message: "missing required field"
        });
    }
    if (!(`${"email"}` in obj)) {
        errors.push({
            field: `${"email"}`,
            message: "missing required field"
        });
    }
    if (!(`${"leadSource"}` in obj)) {
        errors.push({
            field: `${"leadSource"}`,
            message: "missing required field"
        });
    }
    if (!(`${"colors"}` in obj)) {
        errors.push({
            field: `${"colors"}`,
            message: "missing required field"
        });
    }
    if (!(`${"needsReview"}` in obj)) {
        errors.push({
            field: `${"needsReview"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAlert"}` in obj)) {
        errors.push({
            field: `${"hasAlert"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accountType"}` in obj)) {
        errors.push({
            field: `${"accountType"}`,
            message: "missing required field"
        });
    }
    if (!(`${"subtype"}` in obj)) {
        errors.push({
            field: `${"subtype"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isTaxExempt"}` in obj)) {
        errors.push({
            field: `${"isTaxExempt"}`,
            message: "missing required field"
        });
    }
    if (!(`${"paymentTerms"}` in obj)) {
        errors.push({
            field: `${"paymentTerms"}`,
            message: "missing required field"
        });
    }
    if (!(`${"tags"}` in obj)) {
        errors.push({
            field: `${"tags"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dateAdded"}` in obj)) {
        errors.push({
            field: `${"dateAdded"}`,
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
        const __raw_taxRate = obj[`${"taxRate"}`] as string | TaxRate;
        instance.taxRate = __raw_taxRate;
    }
    {
        const __raw_site = obj[`${"site"}`] as string | Site;
        instance.site = __raw_site;
    }
    {
        const __raw_salesRep = obj[`${"salesRep"}`] as Array<Represents> | null;
        if (__raw_salesRep === null) {
            instance.salesRep = null;
        } else {
            instance.salesRep = __raw_salesRep;
        }
    }
    {
        const __raw_orders = obj[`${"orders"}`] as Array<Ordered>;
        if (Array.isArray(__raw_orders)) {
            instance.orders = __raw_orders as Ordered[];
        }
    }
    {
        const __raw_activity = obj[`${"activity"}`] as Array<Did>;
        if (Array.isArray(__raw_activity)) {
            instance.activity = __raw_activity as Did[];
        }
    }
    {
        const __raw_customFields = obj[`${"customFields"}`] as Array<[string, string]>;
        if (Array.isArray(__raw_customFields)) {
            instance.customFields = __raw_customFields as [string, string][];
        }
    }
    {
        const __raw_accountName = obj[`${"accountName"}`] as AccountName;
        {
            const __result = accountNameDeserializeWithContext(__raw_accountName, ctx);
            ctx.assignOrDefer(instance, `${"accountName"}`, __result);
        }
    }
    {
        const __raw_sector = obj[`${"sector"}`] as Sector;
        {
            const __result = sectorDeserializeWithContext(__raw_sector, ctx);
            ctx.assignOrDefer(instance, `${"sector"}`, __result);
        }
    }
    {
        const __raw_memo = obj[`${"memo"}`] as string | null;
        instance.memo = __raw_memo;
    }
    {
        const __raw_phones = obj[`${"phones"}`] as Array<PhoneNumber>;
        if (Array.isArray(__raw_phones)) {
            instance.phones = __raw_phones as PhoneNumber[];
        }
    }
    {
        const __raw_email = obj[`${"email"}`] as Email;
        {
            const __result = emailDeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, `${"email"}`, __result);
        }
    }
    {
        const __raw_leadSource = obj[`${"leadSource"}`] as string;
        if (__raw_leadSource.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Account.leadSource must not be empty"
            });
        }
        instance.leadSource = __raw_leadSource;
    }
    {
        const __raw_colors = obj[`${"colors"}`] as Colors;
        {
            const __result = colorsDeserializeWithContext(__raw_colors, ctx);
            ctx.assignOrDefer(instance, `${"colors"}`, __result);
        }
    }
    {
        const __raw_needsReview = obj[`${"needsReview"}`] as boolean;
        instance.needsReview = __raw_needsReview;
    }
    {
        const __raw_hasAlert = obj[`${"hasAlert"}`] as boolean;
        instance.hasAlert = __raw_hasAlert;
    }
    {
        const __raw_accountType = obj[`${"accountType"}`] as string;
        if (__raw_accountType.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Account.accountType must not be empty"
            });
        }
        instance.accountType = __raw_accountType;
    }
    {
        const __raw_subtype = obj[`${"subtype"}`] as string;
        if (__raw_subtype.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Account.subtype must not be empty"
            });
        }
        instance.subtype = __raw_subtype;
    }
    {
        const __raw_isTaxExempt = obj[`${"isTaxExempt"}`] as boolean;
        instance.isTaxExempt = __raw_isTaxExempt;
    }
    {
        const __raw_paymentTerms = obj[`${"paymentTerms"}`] as string;
        if (__raw_paymentTerms.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Account.paymentTerms must not be empty"
            });
        }
        instance.paymentTerms = __raw_paymentTerms;
    }
    {
        const __raw_tags = obj[`${"tags"}`] as Array<string>;
        if (Array.isArray(__raw_tags)) {
            instance.tags = __raw_tags as string[];
        }
    }
    {
        const __raw_dateAdded = obj[`${"dateAdded"}`] as string;
        instance.dateAdded = __raw_dateAdded;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Account;
}
export function accountValidateField<K extends keyof Account>(_field: K, _value: Account[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"leadSource"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Account.leadSource must not be empty"
            });
        }
    }
    if (_field === `${"accountType"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Account.accountType must not be empty"
            });
        }
    }
    if (_field === `${"subtype"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Account.subtype must not be empty"
            });
        }
    }
    if (_field === `${"paymentTerms"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Account.paymentTerms must not be empty"
            });
        }
    }
    return errors;
}
export function accountValidateFields(_partial: Partial<Account>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"leadSource"}` in _partial && _partial.leadSource !== undefined) {
        const __val = _partial.leadSource as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Account.leadSource must not be empty"
            });
        }
    }
    if (`${"accountType"}` in _partial && _partial.accountType !== undefined) {
        const __val = _partial.accountType as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Account.accountType must not be empty"
            });
        }
    }
    if (`${"subtype"}` in _partial && _partial.subtype !== undefined) {
        const __val = _partial.subtype as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Account.subtype must not be empty"
            });
        }
    }
    if (`${"paymentTerms"}` in _partial && _partial.paymentTerms !== undefined) {
        const __val = _partial.paymentTerms as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Account.paymentTerms must not be empty"
            });
        }
    }
    return errors;
}
export function accountHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "taxRate" in o && "site" in o && "salesRep" in o && "orders" in o && "activity" in o && "customFields" in o && "accountName" in o && "sector" in o && "memo" in o && "phones" in o && "email" in o && "leadSource" in o && "colors" in o && "needsReview" in o && "hasAlert" in o && "accountType" in o && "subtype" in o && "isTaxExempt" in o && "paymentTerms" in o && "tags" in o && "dateAdded" in o';
}
export function accountIs(obj: unknown): obj is Account {
    if (!accountHasShape(obj)) {
        return false;
    }
    const result = accountDeserialize(obj);
    return result.success;
}

export const Account = {
  defaultValue: accountDefaultValue,
  serialize: accountSerialize,
  serializeWithContext: accountSerializeWithContext,
  deserialize: accountDeserialize,
  deserializeWithContext: accountDeserializeWithContext,
  validateFields: accountValidateFields,
  hasShape: accountHasShape,
  is: accountIs
} as const;


export interface Edited {
    
    fieldName: string;
    oldValue: string | null;
    newValue: string | null;
}

export function editedDefaultValue(): Edited {
    return {
        fieldName: "",
        oldValue: null,
        newValue: null
    } as Edited;
}

export function editedSerialize(value: Edited): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(editedSerializeWithContext(value, ctx));
}
export function editedSerializeWithContext(value: Edited, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Edited"}`,
        __id
    };
    result[`${"fieldName"}`] = value.fieldName;
    result[`${"oldValue"}`] = value.oldValue;
    result[`${"newValue"}`] = value.newValue;
    return result;
}

export function editedDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Edited } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = editedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Edited.deserialize: root cannot be a forward reference"
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
export function editedDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Edited | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Edited"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"fieldName"}` in obj)) {
        errors.push({
            field: `${"fieldName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"oldValue"}` in obj)) {
        errors.push({
            field: `${"oldValue"}`,
            message: "missing required field"
        });
    }
    if (!(`${"newValue"}` in obj)) {
        errors.push({
            field: `${"newValue"}`,
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
        const __raw_fieldName = obj[`${"fieldName"}`] as string;
        if (__raw_fieldName.trim().length === 0) {
            errors.push({
                field: "fieldName",
                message: "Edited.fieldName must not be empty"
            });
        }
        instance.fieldName = __raw_fieldName;
    }
    {
        const __raw_oldValue = obj[`${"oldValue"}`] as string | null;
        instance.oldValue = __raw_oldValue;
    }
    {
        const __raw_newValue = obj[`${"newValue"}`] as string | null;
        instance.newValue = __raw_newValue;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Edited;
}
export function editedValidateField<K extends keyof Edited>(_field: K, _value: Edited[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"fieldName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "fieldName",
                message: "Edited.fieldName must not be empty"
            });
        }
    }
    return errors;
}
export function editedValidateFields(_partial: Partial<Edited>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"fieldName"}` in _partial && _partial.fieldName !== undefined) {
        const __val = _partial.fieldName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "fieldName",
                message: "Edited.fieldName must not be empty"
            });
        }
    }
    return errors;
}
export function editedHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"fieldName" in o && "oldValue" in o && "newValue" in o';
}
export function editedIs(obj: unknown): obj is Edited {
    if (!editedHasShape(obj)) {
        return false;
    }
    const result = editedDeserialize(obj);
    return result.success;
}

export const Edited = {
  defaultValue: editedDefaultValue,
  serialize: editedSerialize,
  serializeWithContext: editedSerializeWithContext,
  deserialize: editedDeserialize,
  deserializeWithContext: editedDeserializeWithContext,
  validateFields: editedValidateFields,
  hasShape: editedHasShape,
  is: editedIs
} as const;


export interface Order {
    
    id: string;
    
    
    account: string | Account;
    
    
    stage: OrderStage;
    
    number: number;
    
    payments: Array<string | Payment>;
    
    
    opportunity: string;
    
    
    reference: string;
    
    
    leadSource: string;
    
    
    salesRep: string | Employee;
    
    
    group: string;
    
    
    subgroup: string;
    
    isPosted: boolean;
    
    needsReview: boolean;
    
    
    actionItem: string;
    
    upsale: number;
    
    dateCreated: string;
    
    
    appointment: string | Appointment;
    
    lastTechs: Array<string | Employee>;
    
    package: Array<string | Package> | null;
    
    promotion: Array<string | Promotion> | null;
    
    balance: number;
    
    due: string;
    
    total: number;
    
    
    site: string | Site;
    
    billedItems: Array<BilledItem>;
    
    
    memo: string;
    
    discount: number;
    
    tip: number;
    
    commissions: Array<number>;
}

export function orderDefaultValue(): Order {
    return {
        id: "",
        account: "",
        stage: "Estimate",
        number: 0,
        payments: [],
        opportunity: "",
        reference: "",
        leadSource: "",
        salesRep: "",
        group: "",
        subgroup: "",
        isPosted: false,
        needsReview: false,
        actionItem: "",
        upsale: 0,
        dateCreated: "",
        appointment: "",
        lastTechs: [],
        package: null,
        promotion: null,
        balance: 0,
        due: "",
        total: 0,
        site: "",
        billedItems: [],
        memo: "",
        discount: 0,
        tip: 0,
        commissions: []
    } as Order;
}

export function orderSerialize(value: Order): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(orderSerializeWithContext(value, ctx));
}
export function orderSerializeWithContext(value: Order, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Order"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"account"}`] = value.account;
    result[`${"stage"}`] = orderStageSerializeWithContext(value.stage, ctx);
    result[`${"number"}`] = value.number;
    result[`${"payments"}`] = value.payments;
    result[`${"opportunity"}`] = value.opportunity;
    result[`${"reference"}`] = value.reference;
    result[`${"leadSource"}`] = value.leadSource;
    result[`${"salesRep"}`] = value.salesRep;
    result[`${"group"}`] = value.group;
    result[`${"subgroup"}`] = value.subgroup;
    result[`${"isPosted"}`] = value.isPosted;
    result[`${"needsReview"}`] = value.needsReview;
    result[`${"actionItem"}`] = value.actionItem;
    result[`${"upsale"}`] = value.upsale;
    result[`${"dateCreated"}`] = value.dateCreated;
    result[`${"appointment"}`] = value.appointment;
    result[`${"lastTechs"}`] = value.lastTechs;
    if (value.package !== null) {
        result[`${"package"}`] = value.package;
    }
    if (value.promotion !== null) {
        result[`${"promotion"}`] = value.promotion;
    }
    result[`${"balance"}`] = value.balance;
    result[`${"due"}`] = value.due;
    result[`${"total"}`] = value.total;
    result[`${"site"}`] = value.site;
    result[`${"billedItems"}`] = value.billedItems.map((item)=>billedItemSerializeWithContext(item, ctx));
    result[`${"memo"}`] = value.memo;
    result[`${"discount"}`] = value.discount;
    result[`${"tip"}`] = value.tip;
    result[`${"commissions"}`] = value.commissions;
    return result;
}

export function orderDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Order } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = orderDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Order.deserialize: root cannot be a forward reference"
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
export function orderDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Order | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Order"}.deserializeWithContext: expected an object`
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
    if (!(`${"account"}` in obj)) {
        errors.push({
            field: `${"account"}`,
            message: "missing required field"
        });
    }
    if (!(`${"stage"}` in obj)) {
        errors.push({
            field: `${"stage"}`,
            message: "missing required field"
        });
    }
    if (!(`${"number"}` in obj)) {
        errors.push({
            field: `${"number"}`,
            message: "missing required field"
        });
    }
    if (!(`${"payments"}` in obj)) {
        errors.push({
            field: `${"payments"}`,
            message: "missing required field"
        });
    }
    if (!(`${"opportunity"}` in obj)) {
        errors.push({
            field: `${"opportunity"}`,
            message: "missing required field"
        });
    }
    if (!(`${"reference"}` in obj)) {
        errors.push({
            field: `${"reference"}`,
            message: "missing required field"
        });
    }
    if (!(`${"leadSource"}` in obj)) {
        errors.push({
            field: `${"leadSource"}`,
            message: "missing required field"
        });
    }
    if (!(`${"salesRep"}` in obj)) {
        errors.push({
            field: `${"salesRep"}`,
            message: "missing required field"
        });
    }
    if (!(`${"group"}` in obj)) {
        errors.push({
            field: `${"group"}`,
            message: "missing required field"
        });
    }
    if (!(`${"subgroup"}` in obj)) {
        errors.push({
            field: `${"subgroup"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isPosted"}` in obj)) {
        errors.push({
            field: `${"isPosted"}`,
            message: "missing required field"
        });
    }
    if (!(`${"needsReview"}` in obj)) {
        errors.push({
            field: `${"needsReview"}`,
            message: "missing required field"
        });
    }
    if (!(`${"actionItem"}` in obj)) {
        errors.push({
            field: `${"actionItem"}`,
            message: "missing required field"
        });
    }
    if (!(`${"upsale"}` in obj)) {
        errors.push({
            field: `${"upsale"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dateCreated"}` in obj)) {
        errors.push({
            field: `${"dateCreated"}`,
            message: "missing required field"
        });
    }
    if (!(`${"appointment"}` in obj)) {
        errors.push({
            field: `${"appointment"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lastTechs"}` in obj)) {
        errors.push({
            field: `${"lastTechs"}`,
            message: "missing required field"
        });
    }
    if (!(`${"package"}` in obj)) {
        errors.push({
            field: `${"package"}`,
            message: "missing required field"
        });
    }
    if (!(`${"promotion"}` in obj)) {
        errors.push({
            field: `${"promotion"}`,
            message: "missing required field"
        });
    }
    if (!(`${"balance"}` in obj)) {
        errors.push({
            field: `${"balance"}`,
            message: "missing required field"
        });
    }
    if (!(`${"due"}` in obj)) {
        errors.push({
            field: `${"due"}`,
            message: "missing required field"
        });
    }
    if (!(`${"total"}` in obj)) {
        errors.push({
            field: `${"total"}`,
            message: "missing required field"
        });
    }
    if (!(`${"site"}` in obj)) {
        errors.push({
            field: `${"site"}`,
            message: "missing required field"
        });
    }
    if (!(`${"billedItems"}` in obj)) {
        errors.push({
            field: `${"billedItems"}`,
            message: "missing required field"
        });
    }
    if (!(`${"memo"}` in obj)) {
        errors.push({
            field: `${"memo"}`,
            message: "missing required field"
        });
    }
    if (!(`${"discount"}` in obj)) {
        errors.push({
            field: `${"discount"}`,
            message: "missing required field"
        });
    }
    if (!(`${"tip"}` in obj)) {
        errors.push({
            field: `${"tip"}`,
            message: "missing required field"
        });
    }
    if (!(`${"commissions"}` in obj)) {
        errors.push({
            field: `${"commissions"}`,
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
        const __raw_account = obj[`${"account"}`] as string | Account;
        instance.account = __raw_account;
    }
    {
        const __raw_stage = obj[`${"stage"}`] as OrderStage;
        {
            const __result = orderStageDeserializeWithContext(__raw_stage, ctx);
            ctx.assignOrDefer(instance, `${"stage"}`, __result);
        }
    }
    {
        const __raw_number = obj[`${"number"}`] as number;
        instance.number = __raw_number;
    }
    {
        const __raw_payments = obj[`${"payments"}`] as Array<string | Payment>;
        if (Array.isArray(__raw_payments)) {
            instance.payments = __raw_payments as string | Payment[];
        }
    }
    {
        const __raw_opportunity = obj[`${"opportunity"}`] as string;
        if (__raw_opportunity.trim().length === 0) {
            errors.push({
                field: "opportunity",
                message: "Order.opportunity must not be empty"
            });
        }
        instance.opportunity = __raw_opportunity;
    }
    {
        const __raw_reference = obj[`${"reference"}`] as string;
        if (__raw_reference.trim().length === 0) {
            errors.push({
                field: "reference",
                message: "Order.reference must not be empty"
            });
        }
        instance.reference = __raw_reference;
    }
    {
        const __raw_leadSource = obj[`${"leadSource"}`] as string;
        if (__raw_leadSource.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Order.leadSource must not be empty"
            });
        }
        instance.leadSource = __raw_leadSource;
    }
    {
        const __raw_salesRep = obj[`${"salesRep"}`] as string | Employee;
        instance.salesRep = __raw_salesRep;
    }
    {
        const __raw_group = obj[`${"group"}`] as string;
        if (__raw_group.trim().length === 0) {
            errors.push({
                field: "group",
                message: "Order.group must not be empty"
            });
        }
        instance.group = __raw_group;
    }
    {
        const __raw_subgroup = obj[`${"subgroup"}`] as string;
        if (__raw_subgroup.trim().length === 0) {
            errors.push({
                field: "subgroup",
                message: "Order.subgroup must not be empty"
            });
        }
        instance.subgroup = __raw_subgroup;
    }
    {
        const __raw_isPosted = obj[`${"isPosted"}`] as boolean;
        instance.isPosted = __raw_isPosted;
    }
    {
        const __raw_needsReview = obj[`${"needsReview"}`] as boolean;
        instance.needsReview = __raw_needsReview;
    }
    {
        const __raw_actionItem = obj[`${"actionItem"}`] as string;
        if (__raw_actionItem.trim().length === 0) {
            errors.push({
                field: "actionItem",
                message: "Order.actionItem must not be empty"
            });
        }
        instance.actionItem = __raw_actionItem;
    }
    {
        const __raw_upsale = obj[`${"upsale"}`] as number;
        instance.upsale = __raw_upsale;
    }
    {
        const __raw_dateCreated = obj[`${"dateCreated"}`] as string;
        instance.dateCreated = __raw_dateCreated;
    }
    {
        const __raw_appointment = obj[`${"appointment"}`] as string | Appointment;
        instance.appointment = __raw_appointment;
    }
    {
        const __raw_lastTechs = obj[`${"lastTechs"}`] as Array<string | Employee>;
        if (Array.isArray(__raw_lastTechs)) {
            instance.lastTechs = __raw_lastTechs as string | Employee[];
        }
    }
    {
        const __raw_package = obj[`${"package"}`] as Array<string | Package> | null;
        if (__raw_package === null) {
            instance.package = null;
        } else {
            instance.package = __raw_package;
        }
    }
    {
        const __raw_promotion = obj[`${"promotion"}`] as Array<string | Promotion> | null;
        if (__raw_promotion === null) {
            instance.promotion = null;
        } else {
            instance.promotion = __raw_promotion;
        }
    }
    {
        const __raw_balance = obj[`${"balance"}`] as number;
        instance.balance = __raw_balance;
    }
    {
        const __raw_due = obj[`${"due"}`] as string;
        instance.due = __raw_due;
    }
    {
        const __raw_total = obj[`${"total"}`] as number;
        instance.total = __raw_total;
    }
    {
        const __raw_site = obj[`${"site"}`] as string | Site;
        instance.site = __raw_site;
    }
    {
        const __raw_billedItems = obj[`${"billedItems"}`] as Array<BilledItem>;
        if (Array.isArray(__raw_billedItems)) {
            instance.billedItems = __raw_billedItems as BilledItem[];
        }
    }
    {
        const __raw_memo = obj[`${"memo"}`] as string;
        if (__raw_memo.trim().length === 0) {
            errors.push({
                field: "memo",
                message: "Order.memo must not be empty"
            });
        }
        instance.memo = __raw_memo;
    }
    {
        const __raw_discount = obj[`${"discount"}`] as number;
        instance.discount = __raw_discount;
    }
    {
        const __raw_tip = obj[`${"tip"}`] as number;
        instance.tip = __raw_tip;
    }
    {
        const __raw_commissions = obj[`${"commissions"}`] as Array<number>;
        if (Array.isArray(__raw_commissions)) {
            instance.commissions = __raw_commissions as number[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Order;
}
export function orderValidateField<K extends keyof Order>(_field: K, _value: Order[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"opportunity"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "opportunity",
                message: "Order.opportunity must not be empty"
            });
        }
    }
    if (_field === `${"reference"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "reference",
                message: "Order.reference must not be empty"
            });
        }
    }
    if (_field === `${"leadSource"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Order.leadSource must not be empty"
            });
        }
    }
    if (_field === `${"group"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "group",
                message: "Order.group must not be empty"
            });
        }
    }
    if (_field === `${"subgroup"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subgroup",
                message: "Order.subgroup must not be empty"
            });
        }
    }
    if (_field === `${"actionItem"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "actionItem",
                message: "Order.actionItem must not be empty"
            });
        }
    }
    if (_field === `${"memo"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "memo",
                message: "Order.memo must not be empty"
            });
        }
    }
    return errors;
}
export function orderValidateFields(_partial: Partial<Order>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"opportunity"}` in _partial && _partial.opportunity !== undefined) {
        const __val = _partial.opportunity as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "opportunity",
                message: "Order.opportunity must not be empty"
            });
        }
    }
    if (`${"reference"}` in _partial && _partial.reference !== undefined) {
        const __val = _partial.reference as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "reference",
                message: "Order.reference must not be empty"
            });
        }
    }
    if (`${"leadSource"}` in _partial && _partial.leadSource !== undefined) {
        const __val = _partial.leadSource as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "leadSource",
                message: "Order.leadSource must not be empty"
            });
        }
    }
    if (`${"group"}` in _partial && _partial.group !== undefined) {
        const __val = _partial.group as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "group",
                message: "Order.group must not be empty"
            });
        }
    }
    if (`${"subgroup"}` in _partial && _partial.subgroup !== undefined) {
        const __val = _partial.subgroup as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subgroup",
                message: "Order.subgroup must not be empty"
            });
        }
    }
    if (`${"actionItem"}` in _partial && _partial.actionItem !== undefined) {
        const __val = _partial.actionItem as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "actionItem",
                message: "Order.actionItem must not be empty"
            });
        }
    }
    if (`${"memo"}` in _partial && _partial.memo !== undefined) {
        const __val = _partial.memo as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "memo",
                message: "Order.memo must not be empty"
            });
        }
    }
    return errors;
}
export function orderHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "account" in o && "stage" in o && "number" in o && "payments" in o && "opportunity" in o && "reference" in o && "leadSource" in o && "salesRep" in o && "group" in o && "subgroup" in o && "isPosted" in o && "needsReview" in o && "actionItem" in o && "upsale" in o && "dateCreated" in o && "appointment" in o && "lastTechs" in o && "package" in o && "promotion" in o && "balance" in o && "due" in o && "total" in o && "site" in o && "billedItems" in o && "memo" in o && "discount" in o && "tip" in o && "commissions" in o';
}
export function orderIs(obj: unknown): obj is Order {
    if (!orderHasShape(obj)) {
        return false;
    }
    const result = orderDeserialize(obj);
    return result.success;
}

export const Order = {
  defaultValue: orderDefaultValue,
  serialize: orderSerialize,
  serializeWithContext: orderSerializeWithContext,
  deserialize: orderDeserialize,
  deserializeWithContext: orderDeserializeWithContext,
  validateFields: orderValidateFields,
  hasShape: orderHasShape,
  is: orderIs
} as const;


export interface Commented {
    
    comment: string;
    replyTo: string | null;
}

export function commentedDefaultValue(): Commented {
    return {
        comment: "",
        replyTo: null
    } as Commented;
}

export function commentedSerialize(value: Commented): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(commentedSerializeWithContext(value, ctx));
}
export function commentedSerializeWithContext(value: Commented, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Commented"}`,
        __id
    };
    result[`${"comment"}`] = value.comment;
    result[`${"replyTo"}`] = value.replyTo;
    return result;
}

export function commentedDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Commented } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = commentedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Commented.deserialize: root cannot be a forward reference"
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
export function commentedDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Commented | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Commented"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"comment"}` in obj)) {
        errors.push({
            field: `${"comment"}`,
            message: "missing required field"
        });
    }
    if (!(`${"replyTo"}` in obj)) {
        errors.push({
            field: `${"replyTo"}`,
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
        const __raw_comment = obj[`${"comment"}`] as string;
        if (__raw_comment.trim().length === 0) {
            errors.push({
                field: "comment",
                message: "Commented.comment must not be empty"
            });
        }
        instance.comment = __raw_comment;
    }
    {
        const __raw_replyTo = obj[`${"replyTo"}`] as string | null;
        instance.replyTo = __raw_replyTo;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Commented;
}
export function commentedValidateField<K extends keyof Commented>(_field: K, _value: Commented[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"comment"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "comment",
                message: "Commented.comment must not be empty"
            });
        }
    }
    return errors;
}
export function commentedValidateFields(_partial: Partial<Commented>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"comment"}` in _partial && _partial.comment !== undefined) {
        const __val = _partial.comment as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "comment",
                message: "Commented.comment must not be empty"
            });
        }
    }
    return errors;
}
export function commentedHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"comment" in o && "replyTo" in o';
}
export function commentedIs(obj: unknown): obj is Commented {
    if (!commentedHasShape(obj)) {
        return false;
    }
    const result = commentedDeserialize(obj);
    return result.success;
}

export const Commented = {
  defaultValue: commentedDefaultValue,
  serialize: commentedSerialize,
  serializeWithContext: commentedSerializeWithContext,
  deserialize: commentedDeserialize,
  deserializeWithContext: commentedDeserializeWithContext,
  validateFields: commentedValidateFields,
  hasShape: commentedHasShape,
  is: commentedIs
} as const;


export interface Custom {
    mappings: Array<DirectionHue>;
}

export function customDefaultValue(): Custom {
    return {
        mappings: []
    } as Custom;
}

export function customSerialize(value: Custom): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(customSerializeWithContext(value, ctx));
}
export function customSerializeWithContext(value: Custom, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Custom"}`,
        __id
    };
    result[`${"mappings"}`] = value.mappings.map((item)=>directionHueSerializeWithContext(item, ctx));
    return result;
}

export function customDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Custom } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = customDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Custom.deserialize: root cannot be a forward reference"
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
export function customDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Custom | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Custom"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"mappings"}` in obj)) {
        errors.push({
            field: `${"mappings"}`,
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
        const __raw_mappings = obj[`${"mappings"}`] as Array<DirectionHue>;
        if (Array.isArray(__raw_mappings)) {
            instance.mappings = __raw_mappings as DirectionHue[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Custom;
}
export function customValidateField<K extends keyof Custom>(_field: K, _value: Custom[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function customValidateFields(_partial: Partial<Custom>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function customHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"mappings" in o';
}
export function customIs(obj: unknown): obj is Custom {
    if (!customHasShape(obj)) {
        return false;
    }
    const result = customDeserialize(obj);
    return result.success;
}

export const Custom = {
  defaultValue: customDefaultValue,
  serialize: customSerialize,
  serializeWithContext: customSerializeWithContext,
  deserialize: customDeserialize,
  deserializeWithContext: customDeserializeWithContext,
  validateFields: customValidateFields,
  hasShape: customHasShape,
  is: customIs
} as const;


export interface Colors {
    
    main: string;
    
    hover: string;
    
    active: string;
}

export function colorsDefaultValue(): Colors {
    return {
        main: "",
        hover: "",
        active: ""
    } as Colors;
}

export function colorsSerialize(value: Colors): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(colorsSerializeWithContext(value, ctx));
}
export function colorsSerializeWithContext(value: Colors, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Colors"}`,
        __id
    };
    result[`${"main"}`] = value.main;
    result[`${"hover"}`] = value.hover;
    result[`${"active"}`] = value.active;
    return result;
}

export function colorsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Colors } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = colorsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Colors.deserialize: root cannot be a forward reference"
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
export function colorsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Colors | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Colors"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"main"}` in obj)) {
        errors.push({
            field: `${"main"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hover"}` in obj)) {
        errors.push({
            field: `${"hover"}`,
            message: "missing required field"
        });
    }
    if (!(`${"active"}` in obj)) {
        errors.push({
            field: `${"active"}`,
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
        const __raw_main = obj[`${"main"}`] as string;
        if (__raw_main.trim().length === 0) {
            errors.push({
                field: "main",
                message: "Colors.main must not be empty"
            });
        }
        instance.main = __raw_main;
    }
    {
        const __raw_hover = obj[`${"hover"}`] as string;
        if (__raw_hover.trim().length === 0) {
            errors.push({
                field: "hover",
                message: "Colors.hover must not be empty"
            });
        }
        instance.hover = __raw_hover;
    }
    {
        const __raw_active = obj[`${"active"}`] as string;
        if (__raw_active.trim().length === 0) {
            errors.push({
                field: "active",
                message: "Colors.active must not be empty"
            });
        }
        instance.active = __raw_active;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Colors;
}
export function colorsValidateField<K extends keyof Colors>(_field: K, _value: Colors[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"main"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "main",
                message: "Colors.main must not be empty"
            });
        }
    }
    if (_field === `${"hover"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "hover",
                message: "Colors.hover must not be empty"
            });
        }
    }
    if (_field === `${"active"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "active",
                message: "Colors.active must not be empty"
            });
        }
    }
    return errors;
}
export function colorsValidateFields(_partial: Partial<Colors>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"main"}` in _partial && _partial.main !== undefined) {
        const __val = _partial.main as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "main",
                message: "Colors.main must not be empty"
            });
        }
    }
    if (`${"hover"}` in _partial && _partial.hover !== undefined) {
        const __val = _partial.hover as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "hover",
                message: "Colors.hover must not be empty"
            });
        }
    }
    if (`${"active"}` in _partial && _partial.active !== undefined) {
        const __val = _partial.active as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "active",
                message: "Colors.active must not be empty"
            });
        }
    }
    return errors;
}
export function colorsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"main" in o && "hover" in o && "active" in o';
}
export function colorsIs(obj: unknown): obj is Colors {
    if (!colorsHasShape(obj)) {
        return false;
    }
    const result = colorsDeserialize(obj);
    return result.success;
}

export const Colors = {
  defaultValue: colorsDefaultValue,
  serialize: colorsSerialize,
  serializeWithContext: colorsSerializeWithContext,
  deserialize: colorsDeserialize,
  deserializeWithContext: colorsDeserializeWithContext,
  validateFields: colorsValidateFields,
  hasShape: colorsHasShape,
  is: colorsIs
} as const;


export interface ProductDefaults {
    
    price: number;
    
    
    description: string;
}

export function productDefaultsDefaultValue(): ProductDefaults {
    return {
        price: 0,
        description: ""
    } as ProductDefaults;
}

export function productDefaultsSerialize(value: ProductDefaults): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(productDefaultsSerializeWithContext(value, ctx));
}
export function productDefaultsSerializeWithContext(value: ProductDefaults, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"ProductDefaults"}`,
        __id
    };
    result[`${"price"}`] = value.price;
    result[`${"description"}`] = value.description;
    return result;
}

export function productDefaultsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ProductDefaults } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = productDefaultsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ProductDefaults.deserialize: root cannot be a forward reference"
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
export function productDefaultsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ProductDefaults | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"ProductDefaults"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"price"}` in obj)) {
        errors.push({
            field: `${"price"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
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
        const __raw_price = obj[`${"price"}`] as number;
        instance.price = __raw_price;
    }
    {
        const __raw_description = obj[`${"description"}`] as string;
        if (__raw_description.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ProductDefaults.description must not be empty"
            });
        }
        instance.description = __raw_description;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as ProductDefaults;
}
export function productDefaultsValidateField<K extends keyof ProductDefaults>(_field: K, _value: ProductDefaults[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"description"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ProductDefaults.description must not be empty"
            });
        }
    }
    return errors;
}
export function productDefaultsValidateFields(_partial: Partial<ProductDefaults>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"description"}` in _partial && _partial.description !== undefined) {
        const __val = _partial.description as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ProductDefaults.description must not be empty"
            });
        }
    }
    return errors;
}
export function productDefaultsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"price" in o && "description" in o';
}
export function productDefaultsIs(obj: unknown): obj is ProductDefaults {
    if (!productDefaultsHasShape(obj)) {
        return false;
    }
    const result = productDefaultsDeserialize(obj);
    return result.success;
}

export const ProductDefaults = {
  defaultValue: productDefaultsDefaultValue,
  serialize: productDefaultsSerialize,
  serializeWithContext: productDefaultsSerializeWithContext,
  deserialize: productDefaultsDeserialize,
  deserializeWithContext: productDefaultsDeserializeWithContext,
  validateFields: productDefaultsValidateFields,
  hasShape: productDefaultsHasShape,
  is: productDefaultsIs
} as const;


export interface Viewed {
    durationSeconds: number | null;
    source: string | null;
}

export function viewedDefaultValue(): Viewed {
    return {
        durationSeconds: null,
        source: null
    } as Viewed;
}

export function viewedSerialize(value: Viewed): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(viewedSerializeWithContext(value, ctx));
}
export function viewedSerializeWithContext(value: Viewed, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Viewed"}`,
        __id
    };
    result[`${"durationSeconds"}`] = value.durationSeconds;
    result[`${"source"}`] = value.source;
    return result;
}

export function viewedDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Viewed } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = viewedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Viewed.deserialize: root cannot be a forward reference"
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
export function viewedDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Viewed | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Viewed"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"durationSeconds"}` in obj)) {
        errors.push({
            field: `${"durationSeconds"}`,
            message: "missing required field"
        });
    }
    if (!(`${"source"}` in obj)) {
        errors.push({
            field: `${"source"}`,
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
        const __raw_durationSeconds = obj[`${"durationSeconds"}`] as number | null;
        instance.durationSeconds = __raw_durationSeconds;
    }
    {
        const __raw_source = obj[`${"source"}`] as string | null;
        instance.source = __raw_source;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Viewed;
}
export function viewedValidateField<K extends keyof Viewed>(_field: K, _value: Viewed[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function viewedValidateFields(_partial: Partial<Viewed>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function viewedHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"durationSeconds" in o && "source" in o';
}
export function viewedIs(obj: unknown): obj is Viewed {
    if (!viewedHasShape(obj)) {
        return false;
    }
    const result = viewedDeserialize(obj);
    return result.success;
}

export const Viewed = {
  defaultValue: viewedDefaultValue,
  serialize: viewedSerialize,
  serializeWithContext: viewedSerializeWithContext,
  deserialize: viewedDeserialize,
  deserializeWithContext: viewedDeserializeWithContext,
  validateFields: viewedValidateFields,
  hasShape: viewedHasShape,
  is: viewedIs
} as const;


export interface WeeklyRecurrenceRule {
    quantityOfWeeks: number;
    weekdays: Array<Weekday>;
}

export function weeklyRecurrenceRuleDefaultValue(): WeeklyRecurrenceRule {
    return {
        quantityOfWeeks: 0,
        weekdays: []
    } as WeeklyRecurrenceRule;
}

export function weeklyRecurrenceRuleSerialize(value: WeeklyRecurrenceRule): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(weeklyRecurrenceRuleSerializeWithContext(value, ctx));
}
export function weeklyRecurrenceRuleSerializeWithContext(value: WeeklyRecurrenceRule, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"WeeklyRecurrenceRule"}`,
        __id
    };
    result[`${"quantityOfWeeks"}`] = value.quantityOfWeeks;
    result[`${"weekdays"}`] = value.weekdays.map((item)=>weekdaySerializeWithContext(item, ctx));
    return result;
}

export function weeklyRecurrenceRuleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: WeeklyRecurrenceRule } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = weeklyRecurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "WeeklyRecurrenceRule.deserialize: root cannot be a forward reference"
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
export function weeklyRecurrenceRuleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): WeeklyRecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"WeeklyRecurrenceRule"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"quantityOfWeeks"}` in obj)) {
        errors.push({
            field: `${"quantityOfWeeks"}`,
            message: "missing required field"
        });
    }
    if (!(`${"weekdays"}` in obj)) {
        errors.push({
            field: `${"weekdays"}`,
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
        const __raw_quantityOfWeeks = obj[`${"quantityOfWeeks"}`] as number;
        instance.quantityOfWeeks = __raw_quantityOfWeeks;
    }
    {
        const __raw_weekdays = obj[`${"weekdays"}`] as Array<Weekday>;
        if (Array.isArray(__raw_weekdays)) {
            instance.weekdays = __raw_weekdays as Weekday[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as WeeklyRecurrenceRule;
}
export function weeklyRecurrenceRuleValidateField<K extends keyof WeeklyRecurrenceRule>(_field: K, _value: WeeklyRecurrenceRule[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function weeklyRecurrenceRuleValidateFields(_partial: Partial<WeeklyRecurrenceRule>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function weeklyRecurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"quantityOfWeeks" in o && "weekdays" in o';
}
export function weeklyRecurrenceRuleIs(obj: unknown): obj is WeeklyRecurrenceRule {
    if (!weeklyRecurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = weeklyRecurrenceRuleDeserialize(obj);
    return result.success;
}

export const WeeklyRecurrenceRule = {
  defaultValue: weeklyRecurrenceRuleDefaultValue,
  serialize: weeklyRecurrenceRuleSerialize,
  serializeWithContext: weeklyRecurrenceRuleSerializeWithContext,
  deserialize: weeklyRecurrenceRuleDeserialize,
  deserializeWithContext: weeklyRecurrenceRuleDeserializeWithContext,
  validateFields: weeklyRecurrenceRuleValidateFields,
  hasShape: weeklyRecurrenceRuleHasShape,
  is: weeklyRecurrenceRuleIs
} as const;


export interface Paid {
    amount: number | null;
    currency: string | null;
    paymentMethod: string | null;
}

export function paidDefaultValue(): Paid {
    return {
        amount: null,
        currency: null,
        paymentMethod: null
    } as Paid;
}

export function paidSerialize(value: Paid): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(paidSerializeWithContext(value, ctx));
}
export function paidSerializeWithContext(value: Paid, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Paid"}`,
        __id
    };
    result[`${"amount"}`] = value.amount;
    result[`${"currency"}`] = value.currency;
    result[`${"paymentMethod"}`] = value.paymentMethod;
    return result;
}

export function paidDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Paid } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = paidDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Paid.deserialize: root cannot be a forward reference"
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
export function paidDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Paid | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Paid"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"amount"}` in obj)) {
        errors.push({
            field: `${"amount"}`,
            message: "missing required field"
        });
    }
    if (!(`${"currency"}` in obj)) {
        errors.push({
            field: `${"currency"}`,
            message: "missing required field"
        });
    }
    if (!(`${"paymentMethod"}` in obj)) {
        errors.push({
            field: `${"paymentMethod"}`,
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
        const __raw_amount = obj[`${"amount"}`] as number | null;
        instance.amount = __raw_amount;
    }
    {
        const __raw_currency = obj[`${"currency"}`] as string | null;
        instance.currency = __raw_currency;
    }
    {
        const __raw_paymentMethod = obj[`${"paymentMethod"}`] as string | null;
        instance.paymentMethod = __raw_paymentMethod;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Paid;
}
export function paidValidateField<K extends keyof Paid>(_field: K, _value: Paid[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function paidValidateFields(_partial: Partial<Paid>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function paidHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"amount" in o && "currency" in o && "paymentMethod" in o';
}
export function paidIs(obj: unknown): obj is Paid {
    if (!paidHasShape(obj)) {
        return false;
    }
    const result = paidDeserialize(obj);
    return result.success;
}

export const Paid = {
  defaultValue: paidDefaultValue,
  serialize: paidSerialize,
  serializeWithContext: paidSerializeWithContext,
  deserialize: paidDeserialize,
  deserializeWithContext: paidDeserializeWithContext,
  validateFields: paidValidateFields,
  hasShape: paidHasShape,
  is: paidIs
} as const;


export interface TaxRate {
    
    id: string;
    
    
    name: string;
    
    
    taxAgency: string;
    
    zip: number;
    
    
    city: string;
    
    
    county: string;
    
    
    state: string;
    
    isActive: boolean;
    
    
    description: string;
    
    
    taxComponents: { [key: string]: number };
}

export function taxRateDefaultValue(): TaxRate {
    return {
        id: "",
        name: "",
        taxAgency: "",
        zip: 0,
        city: "",
        county: "",
        state: "",
        isActive: false,
        description: "",
        taxComponents: {}
    } as TaxRate;
}

export function taxRateSerialize(value: TaxRate): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(taxRateSerializeWithContext(value, ctx));
}
export function taxRateSerializeWithContext(value: TaxRate, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"TaxRate"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"name"}`] = value.name;
    result[`${"taxAgency"}`] = value.taxAgency;
    result[`${"zip"}`] = value.zip;
    result[`${"city"}`] = value.city;
    result[`${"county"}`] = value.county;
    result[`${"state"}`] = value.state;
    result[`${"isActive"}`] = value.isActive;
    result[`${"description"}`] = value.description;
    result[`${"taxComponents"}`] = value.taxComponents;
    return result;
}

export function taxRateDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: TaxRate } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = taxRateDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "TaxRate.deserialize: root cannot be a forward reference"
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
export function taxRateDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): TaxRate | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"TaxRate"}.deserializeWithContext: expected an object`
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
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxAgency"}` in obj)) {
        errors.push({
            field: `${"taxAgency"}`,
            message: "missing required field"
        });
    }
    if (!(`${"zip"}` in obj)) {
        errors.push({
            field: `${"zip"}`,
            message: "missing required field"
        });
    }
    if (!(`${"city"}` in obj)) {
        errors.push({
            field: `${"city"}`,
            message: "missing required field"
        });
    }
    if (!(`${"county"}` in obj)) {
        errors.push({
            field: `${"county"}`,
            message: "missing required field"
        });
    }
    if (!(`${"state"}` in obj)) {
        errors.push({
            field: `${"state"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isActive"}` in obj)) {
        errors.push({
            field: `${"isActive"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxComponents"}` in obj)) {
        errors.push({
            field: `${"taxComponents"}`,
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
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "TaxRate.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_taxAgency = obj[`${"taxAgency"}`] as string;
        if (__raw_taxAgency.trim().length === 0) {
            errors.push({
                field: "taxAgency",
                message: "TaxRate.taxAgency must not be empty"
            });
        }
        instance.taxAgency = __raw_taxAgency;
    }
    {
        const __raw_zip = obj[`${"zip"}`] as number;
        instance.zip = __raw_zip;
    }
    {
        const __raw_city = obj[`${"city"}`] as string;
        if (__raw_city.trim().length === 0) {
            errors.push({
                field: "city",
                message: "TaxRate.city must not be empty"
            });
        }
        instance.city = __raw_city;
    }
    {
        const __raw_county = obj[`${"county"}`] as string;
        if (__raw_county.trim().length === 0) {
            errors.push({
                field: "county",
                message: "TaxRate.county must not be empty"
            });
        }
        instance.county = __raw_county;
    }
    {
        const __raw_state = obj[`${"state"}`] as string;
        if (__raw_state.trim().length === 0) {
            errors.push({
                field: "state",
                message: "TaxRate.state must not be empty"
            });
        }
        instance.state = __raw_state;
    }
    {
        const __raw_isActive = obj[`${"isActive"}`] as boolean;
        instance.isActive = __raw_isActive;
    }
    {
        const __raw_description = obj[`${"description"}`] as string;
        if (__raw_description.trim().length === 0) {
            errors.push({
                field: "description",
                message: "TaxRate.description must not be empty"
            });
        }
        instance.description = __raw_description;
    }
    {
        const __raw_taxComponents = obj[`${"taxComponents"}`] as { [key: string]: number };
        instance.taxComponents = __raw_taxComponents;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as TaxRate;
}
export function taxRateValidateField<K extends keyof TaxRate>(_field: K, _value: TaxRate[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"name"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "TaxRate.name must not be empty"
            });
        }
    }
    if (_field === `${"taxAgency"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "taxAgency",
                message: "TaxRate.taxAgency must not be empty"
            });
        }
    }
    if (_field === `${"city"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "city",
                message: "TaxRate.city must not be empty"
            });
        }
    }
    if (_field === `${"county"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "county",
                message: "TaxRate.county must not be empty"
            });
        }
    }
    if (_field === `${"state"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "state",
                message: "TaxRate.state must not be empty"
            });
        }
    }
    if (_field === `${"description"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "TaxRate.description must not be empty"
            });
        }
    }
    return errors;
}
export function taxRateValidateFields(_partial: Partial<TaxRate>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"name"}` in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "TaxRate.name must not be empty"
            });
        }
    }
    if (`${"taxAgency"}` in _partial && _partial.taxAgency !== undefined) {
        const __val = _partial.taxAgency as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "taxAgency",
                message: "TaxRate.taxAgency must not be empty"
            });
        }
    }
    if (`${"city"}` in _partial && _partial.city !== undefined) {
        const __val = _partial.city as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "city",
                message: "TaxRate.city must not be empty"
            });
        }
    }
    if (`${"county"}` in _partial && _partial.county !== undefined) {
        const __val = _partial.county as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "county",
                message: "TaxRate.county must not be empty"
            });
        }
    }
    if (`${"state"}` in _partial && _partial.state !== undefined) {
        const __val = _partial.state as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "state",
                message: "TaxRate.state must not be empty"
            });
        }
    }
    if (`${"description"}` in _partial && _partial.description !== undefined) {
        const __val = _partial.description as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "TaxRate.description must not be empty"
            });
        }
    }
    return errors;
}
export function taxRateHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "name" in o && "taxAgency" in o && "zip" in o && "city" in o && "county" in o && "state" in o && "isActive" in o && "description" in o && "taxComponents" in o';
}
export function taxRateIs(obj: unknown): obj is TaxRate {
    if (!taxRateHasShape(obj)) {
        return false;
    }
    const result = taxRateDeserialize(obj);
    return result.success;
}

export const TaxRate = {
  defaultValue: taxRateDefaultValue,
  serialize: taxRateSerialize,
  serializeWithContext: taxRateSerializeWithContext,
  deserialize: taxRateDeserialize,
  deserializeWithContext: taxRateDeserializeWithContext,
  validateFields: taxRateValidateFields,
  hasShape: taxRateHasShape,
  is: taxRateIs
} as const;


export interface Address {
    
    street: string;
    
    city: string;
    
    state: string;
    
    zipcode: string;
}

export function addressDefaultValue(): Address {
    return {
        street: "",
        city: "",
        state: "",
        zipcode: ""
    } as Address;
}

export function addressSerialize(value: Address): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(addressSerializeWithContext(value, ctx));
}
export function addressSerializeWithContext(value: Address, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Address"}`,
        __id
    };
    result[`${"street"}`] = value.street;
    result[`${"city"}`] = value.city;
    result[`${"state"}`] = value.state;
    result[`${"zipcode"}`] = value.zipcode;
    return result;
}

export function addressDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Address } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = addressDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Address.deserialize: root cannot be a forward reference"
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
export function addressDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Address | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Address"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"street"}` in obj)) {
        errors.push({
            field: `${"street"}`,
            message: "missing required field"
        });
    }
    if (!(`${"city"}` in obj)) {
        errors.push({
            field: `${"city"}`,
            message: "missing required field"
        });
    }
    if (!(`${"state"}` in obj)) {
        errors.push({
            field: `${"state"}`,
            message: "missing required field"
        });
    }
    if (!(`${"zipcode"}` in obj)) {
        errors.push({
            field: `${"zipcode"}`,
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
        const __raw_street = obj[`${"street"}`] as string;
        if (__raw_street.trim().length === 0) {
            errors.push({
                field: "street",
                message: "Address.street must not be empty"
            });
        }
        instance.street = __raw_street;
    }
    {
        const __raw_city = obj[`${"city"}`] as string;
        if (__raw_city.trim().length === 0) {
            errors.push({
                field: "city",
                message: "Address.city must not be empty"
            });
        }
        instance.city = __raw_city;
    }
    {
        const __raw_state = obj[`${"state"}`] as string;
        if (__raw_state.trim().length === 0) {
            errors.push({
                field: "state",
                message: "Address.state must not be empty"
            });
        }
        instance.state = __raw_state;
    }
    {
        const __raw_zipcode = obj[`${"zipcode"}`] as string;
        if (__raw_zipcode.trim().length === 0) {
            errors.push({
                field: "zipcode",
                message: "Address.zipcode must not be empty"
            });
        }
        instance.zipcode = __raw_zipcode;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Address;
}
export function addressValidateField<K extends keyof Address>(_field: K, _value: Address[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"street"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "street",
                message: "Address.street must not be empty"
            });
        }
    }
    if (_field === `${"city"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "city",
                message: "Address.city must not be empty"
            });
        }
    }
    if (_field === `${"state"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "state",
                message: "Address.state must not be empty"
            });
        }
    }
    if (_field === `${"zipcode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "zipcode",
                message: "Address.zipcode must not be empty"
            });
        }
    }
    return errors;
}
export function addressValidateFields(_partial: Partial<Address>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"street"}` in _partial && _partial.street !== undefined) {
        const __val = _partial.street as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "street",
                message: "Address.street must not be empty"
            });
        }
    }
    if (`${"city"}` in _partial && _partial.city !== undefined) {
        const __val = _partial.city as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "city",
                message: "Address.city must not be empty"
            });
        }
    }
    if (`${"state"}` in _partial && _partial.state !== undefined) {
        const __val = _partial.state as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "state",
                message: "Address.state must not be empty"
            });
        }
    }
    if (`${"zipcode"}` in _partial && _partial.zipcode !== undefined) {
        const __val = _partial.zipcode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "zipcode",
                message: "Address.zipcode must not be empty"
            });
        }
    }
    return errors;
}
export function addressHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"street" in o && "city" in o && "state" in o && "zipcode" in o';
}
export function addressIs(obj: unknown): obj is Address {
    if (!addressHasShape(obj)) {
        return false;
    }
    const result = addressDeserialize(obj);
    return result.success;
}

export const Address = {
  defaultValue: addressDefaultValue,
  serialize: addressSerialize,
  serializeWithContext: addressSerializeWithContext,
  deserialize: addressDeserialize,
  deserializeWithContext: addressDeserializeWithContext,
  validateFields: addressValidateFields,
  hasShape: addressHasShape,
  is: addressIs
} as const;


export interface Lead {
    
    id: string;
    
    number: number | null;
    
    accepted: boolean;
    
    probability: number;
    
    
    priority: Priority;
    
    dueDate: string | null;
    
    closeDate: string | null;
    
    value: number;
    
    
    stage: LeadStage;
    
    
    status: string;
    
    description: string | null;
    
    
    nextStep: NextStep;
    
    favorite: boolean;
    
    dateAdded: string | null;
    
    taxRate: (string | TaxRate) | null;
    
    
    sector: Sector;
    
    leadName: AccountName;
    
    phones: Array<PhoneNumber>;
    
    email: Email;
    
    leadSource: string | null;
    
    
    site: string | Site;
    
    
    memo: string;
    
    needsReview: boolean;
    
    hasAlert: boolean;
    
    salesRep: Array<Represents> | null;
    
    color: string | null;
    
    
    accountType: string;
    
    
    subtype: string;
    
    isTaxExempt: boolean;
    
    
    paymentTerms: string;
    
    tags: Array<string>;
    
    customFields: Array<[string, string]>;
}

export function leadDefaultValue(): Lead {
    return {
        id: "",
        number: null,
        accepted: false,
        probability: 0,
        priority: "Medium",
        dueDate: null,
        closeDate: null,
        value: 0,
        stage: "Open",
        status: "",
        description: null,
        nextStep: "InitialContact",
        favorite: false,
        dateAdded: null,
        taxRate: null,
        sector: "Residential",
        leadName: accountNameDefaultValue(),
        phones: [],
        email: emailDefaultValue(),
        leadSource: null,
        site: "",
        memo: "",
        needsReview: false,
        hasAlert: false,
        salesRep: null,
        color: null,
        accountType: "",
        subtype: "",
        isTaxExempt: false,
        paymentTerms: "",
        tags: [],
        customFields: []
    } as Lead;
}

export function leadSerialize(value: Lead): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(leadSerializeWithContext(value, ctx));
}
export function leadSerializeWithContext(value: Lead, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Lead"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"number"}`] = value.number;
    result[`${"accepted"}`] = value.accepted;
    result[`${"probability"}`] = value.probability;
    result[`${"priority"}`] = prioritySerializeWithContext(value.priority, ctx);
    result[`${"dueDate"}`] = value.dueDate;
    result[`${"closeDate"}`] = value.closeDate;
    result[`${"value"}`] = value.value;
    result[`${"stage"}`] = leadStageSerializeWithContext(value.stage, ctx);
    result[`${"status"}`] = value.status;
    result[`${"description"}`] = value.description;
    result[`${"nextStep"}`] = nextStepSerializeWithContext(value.nextStep, ctx);
    result[`${"favorite"}`] = value.favorite;
    result[`${"dateAdded"}`] = value.dateAdded;
    if (value.taxRate !== null) {
        result[`${"taxRate"}`] = value.taxRate;
    }
    result[`${"sector"}`] = sectorSerializeWithContext(value.sector, ctx);
    result[`${"leadName"}`] = accountNameSerializeWithContext(value.leadName, ctx);
    result[`${"phones"}`] = value.phones.map((item)=>phoneNumberSerializeWithContext(item, ctx));
    result[`${"email"}`] = emailSerializeWithContext(value.email, ctx);
    result[`${"leadSource"}`] = value.leadSource;
    result[`${"site"}`] = value.site;
    result[`${"memo"}`] = value.memo;
    result[`${"needsReview"}`] = value.needsReview;
    result[`${"hasAlert"}`] = value.hasAlert;
    if (value.salesRep !== null) {
        result[`${"salesRep"}`] = value.salesRep;
    }
    result[`${"color"}`] = value.color;
    result[`${"accountType"}`] = value.accountType;
    result[`${"subtype"}`] = value.subtype;
    result[`${"isTaxExempt"}`] = value.isTaxExempt;
    result[`${"paymentTerms"}`] = value.paymentTerms;
    result[`${"tags"}`] = value.tags;
    result[`${"customFields"}`] = value.customFields;
    return result;
}

export function leadDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Lead } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = leadDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Lead.deserialize: root cannot be a forward reference"
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
export function leadDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Lead | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Lead"}.deserializeWithContext: expected an object`
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
    if (!(`${"number"}` in obj)) {
        errors.push({
            field: `${"number"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accepted"}` in obj)) {
        errors.push({
            field: `${"accepted"}`,
            message: "missing required field"
        });
    }
    if (!(`${"probability"}` in obj)) {
        errors.push({
            field: `${"probability"}`,
            message: "missing required field"
        });
    }
    if (!(`${"priority"}` in obj)) {
        errors.push({
            field: `${"priority"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dueDate"}` in obj)) {
        errors.push({
            field: `${"dueDate"}`,
            message: "missing required field"
        });
    }
    if (!(`${"closeDate"}` in obj)) {
        errors.push({
            field: `${"closeDate"}`,
            message: "missing required field"
        });
    }
    if (!(`${"value"}` in obj)) {
        errors.push({
            field: `${"value"}`,
            message: "missing required field"
        });
    }
    if (!(`${"stage"}` in obj)) {
        errors.push({
            field: `${"stage"}`,
            message: "missing required field"
        });
    }
    if (!(`${"status"}` in obj)) {
        errors.push({
            field: `${"status"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
            message: "missing required field"
        });
    }
    if (!(`${"nextStep"}` in obj)) {
        errors.push({
            field: `${"nextStep"}`,
            message: "missing required field"
        });
    }
    if (!(`${"favorite"}` in obj)) {
        errors.push({
            field: `${"favorite"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dateAdded"}` in obj)) {
        errors.push({
            field: `${"dateAdded"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxRate"}` in obj)) {
        errors.push({
            field: `${"taxRate"}`,
            message: "missing required field"
        });
    }
    if (!(`${"sector"}` in obj)) {
        errors.push({
            field: `${"sector"}`,
            message: "missing required field"
        });
    }
    if (!(`${"leadName"}` in obj)) {
        errors.push({
            field: `${"leadName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phones"}` in obj)) {
        errors.push({
            field: `${"phones"}`,
            message: "missing required field"
        });
    }
    if (!(`${"email"}` in obj)) {
        errors.push({
            field: `${"email"}`,
            message: "missing required field"
        });
    }
    if (!(`${"leadSource"}` in obj)) {
        errors.push({
            field: `${"leadSource"}`,
            message: "missing required field"
        });
    }
    if (!(`${"site"}` in obj)) {
        errors.push({
            field: `${"site"}`,
            message: "missing required field"
        });
    }
    if (!(`${"memo"}` in obj)) {
        errors.push({
            field: `${"memo"}`,
            message: "missing required field"
        });
    }
    if (!(`${"needsReview"}` in obj)) {
        errors.push({
            field: `${"needsReview"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAlert"}` in obj)) {
        errors.push({
            field: `${"hasAlert"}`,
            message: "missing required field"
        });
    }
    if (!(`${"salesRep"}` in obj)) {
        errors.push({
            field: `${"salesRep"}`,
            message: "missing required field"
        });
    }
    if (!(`${"color"}` in obj)) {
        errors.push({
            field: `${"color"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accountType"}` in obj)) {
        errors.push({
            field: `${"accountType"}`,
            message: "missing required field"
        });
    }
    if (!(`${"subtype"}` in obj)) {
        errors.push({
            field: `${"subtype"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isTaxExempt"}` in obj)) {
        errors.push({
            field: `${"isTaxExempt"}`,
            message: "missing required field"
        });
    }
    if (!(`${"paymentTerms"}` in obj)) {
        errors.push({
            field: `${"paymentTerms"}`,
            message: "missing required field"
        });
    }
    if (!(`${"tags"}` in obj)) {
        errors.push({
            field: `${"tags"}`,
            message: "missing required field"
        });
    }
    if (!(`${"customFields"}` in obj)) {
        errors.push({
            field: `${"customFields"}`,
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
        const __raw_number = obj[`${"number"}`] as number | null;
        instance.number = __raw_number;
    }
    {
        const __raw_accepted = obj[`${"accepted"}`] as boolean;
        instance.accepted = __raw_accepted;
    }
    {
        const __raw_probability = obj[`${"probability"}`] as number;
        instance.probability = __raw_probability;
    }
    {
        const __raw_priority = obj[`${"priority"}`] as Priority;
        {
            const __result = priorityDeserializeWithContext(__raw_priority, ctx);
            ctx.assignOrDefer(instance, `${"priority"}`, __result);
        }
    }
    {
        const __raw_dueDate = obj[`${"dueDate"}`] as string | null;
        instance.dueDate = __raw_dueDate;
    }
    {
        const __raw_closeDate = obj[`${"closeDate"}`] as string | null;
        instance.closeDate = __raw_closeDate;
    }
    {
        const __raw_value = obj[`${"value"}`] as number;
        instance.value = __raw_value;
    }
    {
        const __raw_stage = obj[`${"stage"}`] as LeadStage;
        {
            const __result = leadStageDeserializeWithContext(__raw_stage, ctx);
            ctx.assignOrDefer(instance, `${"stage"}`, __result);
        }
    }
    {
        const __raw_status = obj[`${"status"}`] as string;
        if (__raw_status.trim().length === 0) {
            errors.push({
                field: "status",
                message: "Lead.status must not be empty"
            });
        }
        instance.status = __raw_status;
    }
    {
        const __raw_description = obj[`${"description"}`] as string | null;
        instance.description = __raw_description;
    }
    {
        const __raw_nextStep = obj[`${"nextStep"}`] as NextStep;
        {
            const __result = nextStepDeserializeWithContext(__raw_nextStep, ctx);
            ctx.assignOrDefer(instance, `${"nextStep"}`, __result);
        }
    }
    {
        const __raw_favorite = obj[`${"favorite"}`] as boolean;
        instance.favorite = __raw_favorite;
    }
    {
        const __raw_dateAdded = obj[`${"dateAdded"}`] as string | null;
        instance.dateAdded = __raw_dateAdded;
    }
    {
        const __raw_taxRate = obj[`${"taxRate"}`] as (string | TaxRate) | null;
        if (__raw_taxRate === null) {
            instance.taxRate = null;
        } else {
            instance.taxRate = __raw_taxRate;
        }
    }
    {
        const __raw_sector = obj[`${"sector"}`] as Sector;
        {
            const __result = sectorDeserializeWithContext(__raw_sector, ctx);
            ctx.assignOrDefer(instance, `${"sector"}`, __result);
        }
    }
    {
        const __raw_leadName = obj[`${"leadName"}`] as AccountName;
        {
            const __result = accountNameDeserializeWithContext(__raw_leadName, ctx);
            ctx.assignOrDefer(instance, `${"leadName"}`, __result);
        }
    }
    {
        const __raw_phones = obj[`${"phones"}`] as Array<PhoneNumber>;
        if (Array.isArray(__raw_phones)) {
            instance.phones = __raw_phones as PhoneNumber[];
        }
    }
    {
        const __raw_email = obj[`${"email"}`] as Email;
        {
            const __result = emailDeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, `${"email"}`, __result);
        }
    }
    {
        const __raw_leadSource = obj[`${"leadSource"}`] as string | null;
        instance.leadSource = __raw_leadSource;
    }
    {
        const __raw_site = obj[`${"site"}`] as string | Site;
        instance.site = __raw_site;
    }
    {
        const __raw_memo = obj[`${"memo"}`] as string;
        if (__raw_memo.trim().length === 0) {
            errors.push({
                field: "memo",
                message: "Lead.memo must not be empty"
            });
        }
        instance.memo = __raw_memo;
    }
    {
        const __raw_needsReview = obj[`${"needsReview"}`] as boolean;
        instance.needsReview = __raw_needsReview;
    }
    {
        const __raw_hasAlert = obj[`${"hasAlert"}`] as boolean;
        instance.hasAlert = __raw_hasAlert;
    }
    {
        const __raw_salesRep = obj[`${"salesRep"}`] as Array<Represents> | null;
        if (__raw_salesRep === null) {
            instance.salesRep = null;
        } else {
            instance.salesRep = __raw_salesRep;
        }
    }
    {
        const __raw_color = obj[`${"color"}`] as string | null;
        instance.color = __raw_color;
    }
    {
        const __raw_accountType = obj[`${"accountType"}`] as string;
        if (__raw_accountType.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Lead.accountType must not be empty"
            });
        }
        instance.accountType = __raw_accountType;
    }
    {
        const __raw_subtype = obj[`${"subtype"}`] as string;
        if (__raw_subtype.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Lead.subtype must not be empty"
            });
        }
        instance.subtype = __raw_subtype;
    }
    {
        const __raw_isTaxExempt = obj[`${"isTaxExempt"}`] as boolean;
        instance.isTaxExempt = __raw_isTaxExempt;
    }
    {
        const __raw_paymentTerms = obj[`${"paymentTerms"}`] as string;
        if (__raw_paymentTerms.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Lead.paymentTerms must not be empty"
            });
        }
        instance.paymentTerms = __raw_paymentTerms;
    }
    {
        const __raw_tags = obj[`${"tags"}`] as Array<string>;
        if (Array.isArray(__raw_tags)) {
            instance.tags = __raw_tags as string[];
        }
    }
    {
        const __raw_customFields = obj[`${"customFields"}`] as Array<[string, string]>;
        if (Array.isArray(__raw_customFields)) {
            instance.customFields = __raw_customFields as [string, string][];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Lead;
}
export function leadValidateField<K extends keyof Lead>(_field: K, _value: Lead[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"status"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "status",
                message: "Lead.status must not be empty"
            });
        }
    }
    if (_field === `${"memo"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "memo",
                message: "Lead.memo must not be empty"
            });
        }
    }
    if (_field === `${"accountType"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Lead.accountType must not be empty"
            });
        }
    }
    if (_field === `${"subtype"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Lead.subtype must not be empty"
            });
        }
    }
    if (_field === `${"paymentTerms"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Lead.paymentTerms must not be empty"
            });
        }
    }
    return errors;
}
export function leadValidateFields(_partial: Partial<Lead>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"status"}` in _partial && _partial.status !== undefined) {
        const __val = _partial.status as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "status",
                message: "Lead.status must not be empty"
            });
        }
    }
    if (`${"memo"}` in _partial && _partial.memo !== undefined) {
        const __val = _partial.memo as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "memo",
                message: "Lead.memo must not be empty"
            });
        }
    }
    if (`${"accountType"}` in _partial && _partial.accountType !== undefined) {
        const __val = _partial.accountType as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountType",
                message: "Lead.accountType must not be empty"
            });
        }
    }
    if (`${"subtype"}` in _partial && _partial.subtype !== undefined) {
        const __val = _partial.subtype as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "subtype",
                message: "Lead.subtype must not be empty"
            });
        }
    }
    if (`${"paymentTerms"}` in _partial && _partial.paymentTerms !== undefined) {
        const __val = _partial.paymentTerms as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "paymentTerms",
                message: "Lead.paymentTerms must not be empty"
            });
        }
    }
    return errors;
}
export function leadHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "number" in o && "accepted" in o && "probability" in o && "priority" in o && "dueDate" in o && "closeDate" in o && "value" in o && "stage" in o && "status" in o && "description" in o && "nextStep" in o && "favorite" in o && "dateAdded" in o && "taxRate" in o && "sector" in o && "leadName" in o && "phones" in o && "email" in o && "leadSource" in o && "site" in o && "memo" in o && "needsReview" in o && "hasAlert" in o && "salesRep" in o && "color" in o && "accountType" in o && "subtype" in o && "isTaxExempt" in o && "paymentTerms" in o && "tags" in o && "customFields" in o';
}
export function leadIs(obj: unknown): obj is Lead {
    if (!leadHasShape(obj)) {
        return false;
    }
    const result = leadDeserialize(obj);
    return result.success;
}

export const Lead = {
  defaultValue: leadDefaultValue,
  serialize: leadSerialize,
  serializeWithContext: leadSerializeWithContext,
  deserialize: leadDeserialize,
  deserializeWithContext: leadDeserializeWithContext,
  validateFields: leadValidateFields,
  hasShape: leadHasShape,
  is: leadIs
} as const;


export interface AppPermissions {
    applications: Array<Applications>;
    pages: Array<Page>;
    data: Array<Table>;
}

export function appPermissionsDefaultValue(): AppPermissions {
    return {
        applications: [],
        pages: [],
        data: []
    } as AppPermissions;
}

export function appPermissionsSerialize(value: AppPermissions): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(appPermissionsSerializeWithContext(value, ctx));
}
export function appPermissionsSerializeWithContext(value: AppPermissions, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"AppPermissions"}`,
        __id
    };
    result[`${"applications"}`] = value.applications.map((item)=>applicationsSerializeWithContext(item, ctx));
    result[`${"pages"}`] = value.pages.map((item)=>pageSerializeWithContext(item, ctx));
    result[`${"data"}`] = value.data.map((item)=>tableSerializeWithContext(item, ctx));
    return result;
}

export function appPermissionsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: AppPermissions } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = appPermissionsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "AppPermissions.deserialize: root cannot be a forward reference"
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
export function appPermissionsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): AppPermissions | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"AppPermissions"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"applications"}` in obj)) {
        errors.push({
            field: `${"applications"}`,
            message: "missing required field"
        });
    }
    if (!(`${"pages"}` in obj)) {
        errors.push({
            field: `${"pages"}`,
            message: "missing required field"
        });
    }
    if (!(`${"data"}` in obj)) {
        errors.push({
            field: `${"data"}`,
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
        const __raw_applications = obj[`${"applications"}`] as Array<Applications>;
        if (Array.isArray(__raw_applications)) {
            instance.applications = __raw_applications as Applications[];
        }
    }
    {
        const __raw_pages = obj[`${"pages"}`] as Array<Page>;
        if (Array.isArray(__raw_pages)) {
            instance.pages = __raw_pages as Page[];
        }
    }
    {
        const __raw_data = obj[`${"data"}`] as Array<Table>;
        if (Array.isArray(__raw_data)) {
            instance.data = __raw_data as Table[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as AppPermissions;
}
export function appPermissionsValidateField<K extends keyof AppPermissions>(_field: K, _value: AppPermissions[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function appPermissionsValidateFields(_partial: Partial<AppPermissions>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function appPermissionsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"applications" in o && "pages" in o && "data" in o';
}
export function appPermissionsIs(obj: unknown): obj is AppPermissions {
    if (!appPermissionsHasShape(obj)) {
        return false;
    }
    const result = appPermissionsDeserialize(obj);
    return result.success;
}

export const AppPermissions = {
  defaultValue: appPermissionsDefaultValue,
  serialize: appPermissionsSerialize,
  serializeWithContext: appPermissionsSerializeWithContext,
  deserialize: appPermissionsDeserialize,
  deserializeWithContext: appPermissionsDeserializeWithContext,
  validateFields: appPermissionsValidateFields,
  hasShape: appPermissionsHasShape,
  is: appPermissionsIs
} as const;


export interface Company {
    id: string;
    
    legalName: string;
    
    headquarters: string | Site;
    phones: Array<PhoneNumber>;
    
    fax: string;
    
    email: string;
    
    website: string;
    
    taxId: string;
    referenceNumber: number;
    
    postalCodeLookup: string;
    timeZone: string;
    
    defaultTax: string | TaxRate;
    
    defaultTaxLocation: string;
    defaultAreaCode: number;
    
    defaultAccountType: string;
    
    lookupFormatting: string;
    
    accountNameFormat: string;
    merchantServiceProvider: string | null;
    
    dateDisplayStyle: string;
    hasAutoCommission: boolean;
    hasAutoDaylightSavings: boolean;
    hasAutoFmsTracking: boolean;
    hasNotifications: boolean;
    hasRequiredLeadSource: boolean;
    hasRequiredEmail: boolean;
    hasSortServiceItemsAlphabetically: boolean;
    hasAttachOrderToAppointmentEmails: boolean;
    scheduleInterval: number;
    colorsConfig: ColorsConfig;
}

export function companyDefaultValue(): Company {
    return {
        id: "",
        legalName: "",
        headquarters: "",
        phones: [],
        fax: "",
        email: "",
        website: "",
        taxId: "",
        referenceNumber: 0,
        postalCodeLookup: "",
        timeZone: "",
        defaultTax: "",
        defaultTaxLocation: "",
        defaultAreaCode: 0,
        defaultAccountType: "",
        lookupFormatting: "",
        accountNameFormat: "",
        merchantServiceProvider: null,
        dateDisplayStyle: "",
        hasAutoCommission: false,
        hasAutoDaylightSavings: false,
        hasAutoFmsTracking: false,
        hasNotifications: false,
        hasRequiredLeadSource: false,
        hasRequiredEmail: false,
        hasSortServiceItemsAlphabetically: false,
        hasAttachOrderToAppointmentEmails: false,
        scheduleInterval: 0,
        colorsConfig: colorsConfigDefaultValue()
    } as Company;
}

export function companySerialize(value: Company): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(companySerializeWithContext(value, ctx));
}
export function companySerializeWithContext(value: Company, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Company"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"legalName"}`] = value.legalName;
    result[`${"headquarters"}`] = value.headquarters;
    result[`${"phones"}`] = value.phones.map((item)=>phoneNumberSerializeWithContext(item, ctx));
    result[`${"fax"}`] = value.fax;
    result[`${"email"}`] = value.email;
    result[`${"website"}`] = value.website;
    result[`${"taxId"}`] = value.taxId;
    result[`${"referenceNumber"}`] = value.referenceNumber;
    result[`${"postalCodeLookup"}`] = value.postalCodeLookup;
    result[`${"timeZone"}`] = value.timeZone;
    result[`${"defaultTax"}`] = value.defaultTax;
    result[`${"defaultTaxLocation"}`] = value.defaultTaxLocation;
    result[`${"defaultAreaCode"}`] = value.defaultAreaCode;
    result[`${"defaultAccountType"}`] = value.defaultAccountType;
    result[`${"lookupFormatting"}`] = value.lookupFormatting;
    result[`${"accountNameFormat"}`] = value.accountNameFormat;
    result[`${"merchantServiceProvider"}`] = value.merchantServiceProvider;
    result[`${"dateDisplayStyle"}`] = value.dateDisplayStyle;
    result[`${"hasAutoCommission"}`] = value.hasAutoCommission;
    result[`${"hasAutoDaylightSavings"}`] = value.hasAutoDaylightSavings;
    result[`${"hasAutoFmsTracking"}`] = value.hasAutoFmsTracking;
    result[`${"hasNotifications"}`] = value.hasNotifications;
    result[`${"hasRequiredLeadSource"}`] = value.hasRequiredLeadSource;
    result[`${"hasRequiredEmail"}`] = value.hasRequiredEmail;
    result[`${"hasSortServiceItemsAlphabetically"}`] = value.hasSortServiceItemsAlphabetically;
    result[`${"hasAttachOrderToAppointmentEmails"}`] = value.hasAttachOrderToAppointmentEmails;
    result[`${"scheduleInterval"}`] = value.scheduleInterval;
    result[`${"colorsConfig"}`] = colorsConfigSerializeWithContext(value.colorsConfig, ctx);
    return result;
}

export function companyDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Company } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = companyDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Company.deserialize: root cannot be a forward reference"
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
export function companyDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Company | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Company"}.deserializeWithContext: expected an object`
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
    if (!(`${"legalName"}` in obj)) {
        errors.push({
            field: `${"legalName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"headquarters"}` in obj)) {
        errors.push({
            field: `${"headquarters"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phones"}` in obj)) {
        errors.push({
            field: `${"phones"}`,
            message: "missing required field"
        });
    }
    if (!(`${"fax"}` in obj)) {
        errors.push({
            field: `${"fax"}`,
            message: "missing required field"
        });
    }
    if (!(`${"email"}` in obj)) {
        errors.push({
            field: `${"email"}`,
            message: "missing required field"
        });
    }
    if (!(`${"website"}` in obj)) {
        errors.push({
            field: `${"website"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxId"}` in obj)) {
        errors.push({
            field: `${"taxId"}`,
            message: "missing required field"
        });
    }
    if (!(`${"referenceNumber"}` in obj)) {
        errors.push({
            field: `${"referenceNumber"}`,
            message: "missing required field"
        });
    }
    if (!(`${"postalCodeLookup"}` in obj)) {
        errors.push({
            field: `${"postalCodeLookup"}`,
            message: "missing required field"
        });
    }
    if (!(`${"timeZone"}` in obj)) {
        errors.push({
            field: `${"timeZone"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaultTax"}` in obj)) {
        errors.push({
            field: `${"defaultTax"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaultTaxLocation"}` in obj)) {
        errors.push({
            field: `${"defaultTaxLocation"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaultAreaCode"}` in obj)) {
        errors.push({
            field: `${"defaultAreaCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaultAccountType"}` in obj)) {
        errors.push({
            field: `${"defaultAccountType"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lookupFormatting"}` in obj)) {
        errors.push({
            field: `${"lookupFormatting"}`,
            message: "missing required field"
        });
    }
    if (!(`${"accountNameFormat"}` in obj)) {
        errors.push({
            field: `${"accountNameFormat"}`,
            message: "missing required field"
        });
    }
    if (!(`${"merchantServiceProvider"}` in obj)) {
        errors.push({
            field: `${"merchantServiceProvider"}`,
            message: "missing required field"
        });
    }
    if (!(`${"dateDisplayStyle"}` in obj)) {
        errors.push({
            field: `${"dateDisplayStyle"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAutoCommission"}` in obj)) {
        errors.push({
            field: `${"hasAutoCommission"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAutoDaylightSavings"}` in obj)) {
        errors.push({
            field: `${"hasAutoDaylightSavings"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAutoFmsTracking"}` in obj)) {
        errors.push({
            field: `${"hasAutoFmsTracking"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasNotifications"}` in obj)) {
        errors.push({
            field: `${"hasNotifications"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasRequiredLeadSource"}` in obj)) {
        errors.push({
            field: `${"hasRequiredLeadSource"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasRequiredEmail"}` in obj)) {
        errors.push({
            field: `${"hasRequiredEmail"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasSortServiceItemsAlphabetically"}` in obj)) {
        errors.push({
            field: `${"hasSortServiceItemsAlphabetically"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hasAttachOrderToAppointmentEmails"}` in obj)) {
        errors.push({
            field: `${"hasAttachOrderToAppointmentEmails"}`,
            message: "missing required field"
        });
    }
    if (!(`${"scheduleInterval"}` in obj)) {
        errors.push({
            field: `${"scheduleInterval"}`,
            message: "missing required field"
        });
    }
    if (!(`${"colorsConfig"}` in obj)) {
        errors.push({
            field: `${"colorsConfig"}`,
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
        const __raw_legalName = obj[`${"legalName"}`] as string;
        if (__raw_legalName.trim().length === 0) {
            errors.push({
                field: "legalName",
                message: "Company.legalName must not be empty"
            });
        }
        instance.legalName = __raw_legalName;
    }
    {
        const __raw_headquarters = obj[`${"headquarters"}`] as string | Site;
        instance.headquarters = __raw_headquarters;
    }
    {
        const __raw_phones = obj[`${"phones"}`] as Array<PhoneNumber>;
        if (Array.isArray(__raw_phones)) {
            instance.phones = __raw_phones as PhoneNumber[];
        }
    }
    {
        const __raw_fax = obj[`${"fax"}`] as string;
        if (__raw_fax.trim().length === 0) {
            errors.push({
                field: "fax",
                message: "Company.fax must not be empty"
            });
        }
        instance.fax = __raw_fax;
    }
    {
        const __raw_email = obj[`${"email"}`] as string;
        if (__raw_email.trim().length === 0) {
            errors.push({
                field: "email",
                message: "Company.email must not be empty"
            });
        }
        instance.email = __raw_email;
    }
    {
        const __raw_website = obj[`${"website"}`] as string;
        if (__raw_website.trim().length === 0) {
            errors.push({
                field: "website",
                message: "Company.website must not be empty"
            });
        }
        instance.website = __raw_website;
    }
    {
        const __raw_taxId = obj[`${"taxId"}`] as string;
        if (__raw_taxId.trim().length === 0) {
            errors.push({
                field: "taxId",
                message: "Company.taxId must not be empty"
            });
        }
        instance.taxId = __raw_taxId;
    }
    {
        const __raw_referenceNumber = obj[`${"referenceNumber"}`] as number;
        instance.referenceNumber = __raw_referenceNumber;
    }
    {
        const __raw_postalCodeLookup = obj[`${"postalCodeLookup"}`] as string;
        if (__raw_postalCodeLookup.trim().length === 0) {
            errors.push({
                field: "postalCodeLookup",
                message: "Company.postalCodeLookup must not be empty"
            });
        }
        instance.postalCodeLookup = __raw_postalCodeLookup;
    }
    {
        const __raw_timeZone = obj[`${"timeZone"}`] as string;
        instance.timeZone = __raw_timeZone;
    }
    {
        const __raw_defaultTax = obj[`${"defaultTax"}`] as string | TaxRate;
        instance.defaultTax = __raw_defaultTax;
    }
    {
        const __raw_defaultTaxLocation = obj[`${"defaultTaxLocation"}`] as string;
        if (__raw_defaultTaxLocation.trim().length === 0) {
            errors.push({
                field: "defaultTaxLocation",
                message: "Company.defaultTaxLocation must not be empty"
            });
        }
        instance.defaultTaxLocation = __raw_defaultTaxLocation;
    }
    {
        const __raw_defaultAreaCode = obj[`${"defaultAreaCode"}`] as number;
        instance.defaultAreaCode = __raw_defaultAreaCode;
    }
    {
        const __raw_defaultAccountType = obj[`${"defaultAccountType"}`] as string;
        if (__raw_defaultAccountType.trim().length === 0) {
            errors.push({
                field: "defaultAccountType",
                message: "Company.defaultAccountType must not be empty"
            });
        }
        instance.defaultAccountType = __raw_defaultAccountType;
    }
    {
        const __raw_lookupFormatting = obj[`${"lookupFormatting"}`] as string;
        if (__raw_lookupFormatting.trim().length === 0) {
            errors.push({
                field: "lookupFormatting",
                message: "Company.lookupFormatting must not be empty"
            });
        }
        instance.lookupFormatting = __raw_lookupFormatting;
    }
    {
        const __raw_accountNameFormat = obj[`${"accountNameFormat"}`] as string;
        if (__raw_accountNameFormat.trim().length === 0) {
            errors.push({
                field: "accountNameFormat",
                message: "Company.accountNameFormat must not be empty"
            });
        }
        instance.accountNameFormat = __raw_accountNameFormat;
    }
    {
        const __raw_merchantServiceProvider = obj[`${"merchantServiceProvider"}`] as string | null;
        instance.merchantServiceProvider = __raw_merchantServiceProvider;
    }
    {
        const __raw_dateDisplayStyle = obj[`${"dateDisplayStyle"}`] as string;
        if (__raw_dateDisplayStyle.trim().length === 0) {
            errors.push({
                field: "dateDisplayStyle",
                message: "Company.dateDisplayStyle must not be empty"
            });
        }
        instance.dateDisplayStyle = __raw_dateDisplayStyle;
    }
    {
        const __raw_hasAutoCommission = obj[`${"hasAutoCommission"}`] as boolean;
        instance.hasAutoCommission = __raw_hasAutoCommission;
    }
    {
        const __raw_hasAutoDaylightSavings = obj[`${"hasAutoDaylightSavings"}`] as boolean;
        instance.hasAutoDaylightSavings = __raw_hasAutoDaylightSavings;
    }
    {
        const __raw_hasAutoFmsTracking = obj[`${"hasAutoFmsTracking"}`] as boolean;
        instance.hasAutoFmsTracking = __raw_hasAutoFmsTracking;
    }
    {
        const __raw_hasNotifications = obj[`${"hasNotifications"}`] as boolean;
        instance.hasNotifications = __raw_hasNotifications;
    }
    {
        const __raw_hasRequiredLeadSource = obj[`${"hasRequiredLeadSource"}`] as boolean;
        instance.hasRequiredLeadSource = __raw_hasRequiredLeadSource;
    }
    {
        const __raw_hasRequiredEmail = obj[`${"hasRequiredEmail"}`] as boolean;
        instance.hasRequiredEmail = __raw_hasRequiredEmail;
    }
    {
        const __raw_hasSortServiceItemsAlphabetically = obj[`${"hasSortServiceItemsAlphabetically"}`] as boolean;
        instance.hasSortServiceItemsAlphabetically = __raw_hasSortServiceItemsAlphabetically;
    }
    {
        const __raw_hasAttachOrderToAppointmentEmails = obj[`${"hasAttachOrderToAppointmentEmails"}`] as boolean;
        instance.hasAttachOrderToAppointmentEmails = __raw_hasAttachOrderToAppointmentEmails;
    }
    {
        const __raw_scheduleInterval = obj[`${"scheduleInterval"}`] as number;
        instance.scheduleInterval = __raw_scheduleInterval;
    }
    {
        const __raw_colorsConfig = obj[`${"colorsConfig"}`] as ColorsConfig;
        {
            const __result = colorsConfigDeserializeWithContext(__raw_colorsConfig, ctx);
            ctx.assignOrDefer(instance, `${"colorsConfig"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Company;
}
export function companyValidateField<K extends keyof Company>(_field: K, _value: Company[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"legalName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "legalName",
                message: "Company.legalName must not be empty"
            });
        }
    }
    if (_field === `${"fax"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "fax",
                message: "Company.fax must not be empty"
            });
        }
    }
    if (_field === `${"email"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "email",
                message: "Company.email must not be empty"
            });
        }
    }
    if (_field === `${"website"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "website",
                message: "Company.website must not be empty"
            });
        }
    }
    if (_field === `${"taxId"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "taxId",
                message: "Company.taxId must not be empty"
            });
        }
    }
    if (_field === `${"postalCodeLookup"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "postalCodeLookup",
                message: "Company.postalCodeLookup must not be empty"
            });
        }
    }
    if (_field === `${"defaultTaxLocation"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "defaultTaxLocation",
                message: "Company.defaultTaxLocation must not be empty"
            });
        }
    }
    if (_field === `${"defaultAccountType"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "defaultAccountType",
                message: "Company.defaultAccountType must not be empty"
            });
        }
    }
    if (_field === `${"lookupFormatting"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lookupFormatting",
                message: "Company.lookupFormatting must not be empty"
            });
        }
    }
    if (_field === `${"accountNameFormat"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountNameFormat",
                message: "Company.accountNameFormat must not be empty"
            });
        }
    }
    if (_field === `${"dateDisplayStyle"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "dateDisplayStyle",
                message: "Company.dateDisplayStyle must not be empty"
            });
        }
    }
    return errors;
}
export function companyValidateFields(_partial: Partial<Company>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"legalName"}` in _partial && _partial.legalName !== undefined) {
        const __val = _partial.legalName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "legalName",
                message: "Company.legalName must not be empty"
            });
        }
    }
    if (`${"fax"}` in _partial && _partial.fax !== undefined) {
        const __val = _partial.fax as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "fax",
                message: "Company.fax must not be empty"
            });
        }
    }
    if (`${"email"}` in _partial && _partial.email !== undefined) {
        const __val = _partial.email as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "email",
                message: "Company.email must not be empty"
            });
        }
    }
    if (`${"website"}` in _partial && _partial.website !== undefined) {
        const __val = _partial.website as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "website",
                message: "Company.website must not be empty"
            });
        }
    }
    if (`${"taxId"}` in _partial && _partial.taxId !== undefined) {
        const __val = _partial.taxId as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "taxId",
                message: "Company.taxId must not be empty"
            });
        }
    }
    if (`${"postalCodeLookup"}` in _partial && _partial.postalCodeLookup !== undefined) {
        const __val = _partial.postalCodeLookup as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "postalCodeLookup",
                message: "Company.postalCodeLookup must not be empty"
            });
        }
    }
    if (`${"defaultTaxLocation"}` in _partial && _partial.defaultTaxLocation !== undefined) {
        const __val = _partial.defaultTaxLocation as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "defaultTaxLocation",
                message: "Company.defaultTaxLocation must not be empty"
            });
        }
    }
    if (`${"defaultAccountType"}` in _partial && _partial.defaultAccountType !== undefined) {
        const __val = _partial.defaultAccountType as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "defaultAccountType",
                message: "Company.defaultAccountType must not be empty"
            });
        }
    }
    if (`${"lookupFormatting"}` in _partial && _partial.lookupFormatting !== undefined) {
        const __val = _partial.lookupFormatting as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "lookupFormatting",
                message: "Company.lookupFormatting must not be empty"
            });
        }
    }
    if (`${"accountNameFormat"}` in _partial && _partial.accountNameFormat !== undefined) {
        const __val = _partial.accountNameFormat as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "accountNameFormat",
                message: "Company.accountNameFormat must not be empty"
            });
        }
    }
    if (`${"dateDisplayStyle"}` in _partial && _partial.dateDisplayStyle !== undefined) {
        const __val = _partial.dateDisplayStyle as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "dateDisplayStyle",
                message: "Company.dateDisplayStyle must not be empty"
            });
        }
    }
    return errors;
}
export function companyHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "legalName" in o && "headquarters" in o && "phones" in o && "fax" in o && "email" in o && "website" in o && "taxId" in o && "referenceNumber" in o && "postalCodeLookup" in o && "timeZone" in o && "defaultTax" in o && "defaultTaxLocation" in o && "defaultAreaCode" in o && "defaultAccountType" in o && "lookupFormatting" in o && "accountNameFormat" in o && "merchantServiceProvider" in o && "dateDisplayStyle" in o && "hasAutoCommission" in o && "hasAutoDaylightSavings" in o && "hasAutoFmsTracking" in o && "hasNotifications" in o && "hasRequiredLeadSource" in o && "hasRequiredEmail" in o && "hasSortServiceItemsAlphabetically" in o && "hasAttachOrderToAppointmentEmails" in o && "scheduleInterval" in o && "colorsConfig" in o';
}
export function companyIs(obj: unknown): obj is Company {
    if (!companyHasShape(obj)) {
        return false;
    }
    const result = companyDeserialize(obj);
    return result.success;
}

export const Company = {
  defaultValue: companyDefaultValue,
  serialize: companySerialize,
  serializeWithContext: companySerializeWithContext,
  deserialize: companyDeserialize,
  deserializeWithContext: companyDeserializeWithContext,
  validateFields: companyValidateFields,
  hasShape: companyHasShape,
  is: companyIs
} as const;


export interface Ordinal {
    north: number;
    northeast: number;
    east: number;
    southeast: number;
    south: number;
    southwest: number;
    west: number;
    northwest: number;
}

export function ordinalDefaultValue(): Ordinal {
    return {
        north: 0,
        northeast: 0,
        east: 0,
        southeast: 0,
        south: 0,
        southwest: 0,
        west: 0,
        northwest: 0
    } as Ordinal;
}

export function ordinalSerialize(value: Ordinal): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(ordinalSerializeWithContext(value, ctx));
}
export function ordinalSerializeWithContext(value: Ordinal, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Ordinal"}`,
        __id
    };
    result[`${"north"}`] = value.north;
    result[`${"northeast"}`] = value.northeast;
    result[`${"east"}`] = value.east;
    result[`${"southeast"}`] = value.southeast;
    result[`${"south"}`] = value.south;
    result[`${"southwest"}`] = value.southwest;
    result[`${"west"}`] = value.west;
    result[`${"northwest"}`] = value.northwest;
    return result;
}

export function ordinalDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Ordinal } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = ordinalDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Ordinal.deserialize: root cannot be a forward reference"
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
export function ordinalDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Ordinal | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Ordinal"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"north"}` in obj)) {
        errors.push({
            field: `${"north"}`,
            message: "missing required field"
        });
    }
    if (!(`${"northeast"}` in obj)) {
        errors.push({
            field: `${"northeast"}`,
            message: "missing required field"
        });
    }
    if (!(`${"east"}` in obj)) {
        errors.push({
            field: `${"east"}`,
            message: "missing required field"
        });
    }
    if (!(`${"southeast"}` in obj)) {
        errors.push({
            field: `${"southeast"}`,
            message: "missing required field"
        });
    }
    if (!(`${"south"}` in obj)) {
        errors.push({
            field: `${"south"}`,
            message: "missing required field"
        });
    }
    if (!(`${"southwest"}` in obj)) {
        errors.push({
            field: `${"southwest"}`,
            message: "missing required field"
        });
    }
    if (!(`${"west"}` in obj)) {
        errors.push({
            field: `${"west"}`,
            message: "missing required field"
        });
    }
    if (!(`${"northwest"}` in obj)) {
        errors.push({
            field: `${"northwest"}`,
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
        const __raw_north = obj[`${"north"}`] as number;
        instance.north = __raw_north;
    }
    {
        const __raw_northeast = obj[`${"northeast"}`] as number;
        instance.northeast = __raw_northeast;
    }
    {
        const __raw_east = obj[`${"east"}`] as number;
        instance.east = __raw_east;
    }
    {
        const __raw_southeast = obj[`${"southeast"}`] as number;
        instance.southeast = __raw_southeast;
    }
    {
        const __raw_south = obj[`${"south"}`] as number;
        instance.south = __raw_south;
    }
    {
        const __raw_southwest = obj[`${"southwest"}`] as number;
        instance.southwest = __raw_southwest;
    }
    {
        const __raw_west = obj[`${"west"}`] as number;
        instance.west = __raw_west;
    }
    {
        const __raw_northwest = obj[`${"northwest"}`] as number;
        instance.northwest = __raw_northwest;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Ordinal;
}
export function ordinalValidateField<K extends keyof Ordinal>(_field: K, _value: Ordinal[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function ordinalValidateFields(_partial: Partial<Ordinal>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function ordinalHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"north" in o && "northeast" in o && "east" in o && "southeast" in o && "south" in o && "southwest" in o && "west" in o && "northwest" in o';
}
export function ordinalIs(obj: unknown): obj is Ordinal {
    if (!ordinalHasShape(obj)) {
        return false;
    }
    const result = ordinalDeserialize(obj);
    return result.success;
}

export const Ordinal = {
  defaultValue: ordinalDefaultValue,
  serialize: ordinalSerialize,
  serializeWithContext: ordinalSerializeWithContext,
  deserialize: ordinalDeserialize,
  deserializeWithContext: ordinalDeserializeWithContext,
  validateFields: ordinalValidateFields,
  hasShape: ordinalHasShape,
  is: ordinalIs
} as const;


export interface Password {
    
    password: string;
}

export function passwordDefaultValue(): Password {
    return {
        password: ""
    } as Password;
}

export function passwordSerialize(value: Password): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(passwordSerializeWithContext(value, ctx));
}
export function passwordSerializeWithContext(value: Password, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Password"}`,
        __id
    };
    result[`${"password"}`] = value.password;
    return result;
}

export function passwordDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Password } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = passwordDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Password.deserialize: root cannot be a forward reference"
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
export function passwordDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Password | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Password"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"password"}` in obj)) {
        errors.push({
            field: `${"password"}`,
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
        const __raw_password = obj[`${"password"}`] as string;
        if (__raw_password.trim().length === 0) {
            errors.push({
                field: "password",
                message: "Password.password must not be empty"
            });
        }
        instance.password = __raw_password;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Password;
}
export function passwordValidateField<K extends keyof Password>(_field: K, _value: Password[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"password"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "password",
                message: "Password.password must not be empty"
            });
        }
    }
    return errors;
}
export function passwordValidateFields(_partial: Partial<Password>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"password"}` in _partial && _partial.password !== undefined) {
        const __val = _partial.password as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "password",
                message: "Password.password must not be empty"
            });
        }
    }
    return errors;
}
export function passwordHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"password" in o';
}
export function passwordIs(obj: unknown): obj is Password {
    if (!passwordHasShape(obj)) {
        return false;
    }
    const result = passwordDeserialize(obj);
    return result.success;
}

export const Password = {
  defaultValue: passwordDefaultValue,
  serialize: passwordSerialize,
  serializeWithContext: passwordSerializeWithContext,
  deserialize: passwordDeserialize,
  deserializeWithContext: passwordDeserializeWithContext,
  validateFields: passwordValidateFields,
  hasShape: passwordHasShape,
  is: passwordIs
} as const;


export interface Created {
    initialData: string | null;
}

export function createdDefaultValue(): Created {
    return {
        initialData: null
    } as Created;
}

export function createdSerialize(value: Created): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(createdSerializeWithContext(value, ctx));
}
export function createdSerializeWithContext(value: Created, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Created"}`,
        __id
    };
    result[`${"initialData"}`] = value.initialData;
    return result;
}

export function createdDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Created } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = createdDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Created.deserialize: root cannot be a forward reference"
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
export function createdDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Created | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Created"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"initialData"}` in obj)) {
        errors.push({
            field: `${"initialData"}`,
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
        const __raw_initialData = obj[`${"initialData"}`] as string | null;
        instance.initialData = __raw_initialData;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Created;
}
export function createdValidateField<K extends keyof Created>(_field: K, _value: Created[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function createdValidateFields(_partial: Partial<Created>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function createdHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"initialData" in o';
}
export function createdIs(obj: unknown): obj is Created {
    if (!createdHasShape(obj)) {
        return false;
    }
    const result = createdDeserialize(obj);
    return result.success;
}

export const Created = {
  defaultValue: createdDefaultValue,
  serialize: createdSerialize,
  serializeWithContext: createdSerializeWithContext,
  deserialize: createdDeserialize,
  deserializeWithContext: createdDeserializeWithContext,
  validateFields: createdValidateFields,
  hasShape: createdHasShape,
  is: createdIs
} as const;


export interface Employee {
    id: string;
    imageUrl: string | null;
    
    name: string;
    phones: Array<PhoneNumber>;
    
    role: string;
    
    title: JobTitle;
    email: Email;
    
    address: string;
    
    username: string;
    
    route: string | Route;
    ratePerHour: number;
    active: boolean;
    isTechnician: boolean;
    isSalesRep: boolean;
    description: string | null;
    linkedinUrl: string | null;
    attendance: Array<string>;
    settings: Settings;
}

export function employeeDefaultValue(): Employee {
    return {
        id: "",
        imageUrl: null,
        name: "",
        phones: [],
        role: "",
        title: "Technician",
        email: emailDefaultValue(),
        address: "",
        username: "",
        route: "",
        ratePerHour: 0,
        active: false,
        isTechnician: false,
        isSalesRep: false,
        description: null,
        linkedinUrl: null,
        attendance: [],
        settings: settingsDefaultValue()
    } as Employee;
}

export function employeeSerialize(value: Employee): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(employeeSerializeWithContext(value, ctx));
}
export function employeeSerializeWithContext(value: Employee, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Employee"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"imageUrl"}`] = value.imageUrl;
    result[`${"name"}`] = value.name;
    result[`${"phones"}`] = value.phones.map((item)=>phoneNumberSerializeWithContext(item, ctx));
    result[`${"role"}`] = value.role;
    result[`${"title"}`] = jobTitleSerializeWithContext(value.title, ctx);
    result[`${"email"}`] = emailSerializeWithContext(value.email, ctx);
    result[`${"address"}`] = value.address;
    result[`${"username"}`] = value.username;
    result[`${"route"}`] = value.route;
    result[`${"ratePerHour"}`] = value.ratePerHour;
    result[`${"active"}`] = value.active;
    result[`${"isTechnician"}`] = value.isTechnician;
    result[`${"isSalesRep"}`] = value.isSalesRep;
    result[`${"description"}`] = value.description;
    result[`${"linkedinUrl"}`] = value.linkedinUrl;
    result[`${"attendance"}`] = value.attendance;
    result[`${"settings"}`] = settingsSerializeWithContext(value.settings, ctx);
    return result;
}

export function employeeDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Employee } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = employeeDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Employee.deserialize: root cannot be a forward reference"
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
export function employeeDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Employee | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Employee"}.deserializeWithContext: expected an object`
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
    if (!(`${"imageUrl"}` in obj)) {
        errors.push({
            field: `${"imageUrl"}`,
            message: "missing required field"
        });
    }
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phones"}` in obj)) {
        errors.push({
            field: `${"phones"}`,
            message: "missing required field"
        });
    }
    if (!(`${"role"}` in obj)) {
        errors.push({
            field: `${"role"}`,
            message: "missing required field"
        });
    }
    if (!(`${"title"}` in obj)) {
        errors.push({
            field: `${"title"}`,
            message: "missing required field"
        });
    }
    if (!(`${"email"}` in obj)) {
        errors.push({
            field: `${"email"}`,
            message: "missing required field"
        });
    }
    if (!(`${"address"}` in obj)) {
        errors.push({
            field: `${"address"}`,
            message: "missing required field"
        });
    }
    if (!(`${"username"}` in obj)) {
        errors.push({
            field: `${"username"}`,
            message: "missing required field"
        });
    }
    if (!(`${"route"}` in obj)) {
        errors.push({
            field: `${"route"}`,
            message: "missing required field"
        });
    }
    if (!(`${"ratePerHour"}` in obj)) {
        errors.push({
            field: `${"ratePerHour"}`,
            message: "missing required field"
        });
    }
    if (!(`${"active"}` in obj)) {
        errors.push({
            field: `${"active"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isTechnician"}` in obj)) {
        errors.push({
            field: `${"isTechnician"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isSalesRep"}` in obj)) {
        errors.push({
            field: `${"isSalesRep"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
            message: "missing required field"
        });
    }
    if (!(`${"linkedinUrl"}` in obj)) {
        errors.push({
            field: `${"linkedinUrl"}`,
            message: "missing required field"
        });
    }
    if (!(`${"attendance"}` in obj)) {
        errors.push({
            field: `${"attendance"}`,
            message: "missing required field"
        });
    }
    if (!(`${"settings"}` in obj)) {
        errors.push({
            field: `${"settings"}`,
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
        const __raw_imageUrl = obj[`${"imageUrl"}`] as string | null;
        instance.imageUrl = __raw_imageUrl;
    }
    {
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Employee.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_phones = obj[`${"phones"}`] as Array<PhoneNumber>;
        if (Array.isArray(__raw_phones)) {
            instance.phones = __raw_phones as PhoneNumber[];
        }
    }
    {
        const __raw_role = obj[`${"role"}`] as string;
        if (__raw_role.trim().length === 0) {
            errors.push({
                field: "role",
                message: "Employee.role must not be empty"
            });
        }
        instance.role = __raw_role;
    }
    {
        const __raw_title = obj[`${"title"}`] as JobTitle;
        {
            const __result = jobTitleDeserializeWithContext(__raw_title, ctx);
            ctx.assignOrDefer(instance, `${"title"}`, __result);
        }
    }
    {
        const __raw_email = obj[`${"email"}`] as Email;
        {
            const __result = emailDeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, `${"email"}`, __result);
        }
    }
    {
        const __raw_address = obj[`${"address"}`] as string;
        if (__raw_address.trim().length === 0) {
            errors.push({
                field: "address",
                message: "Employee.address must not be empty"
            });
        }
        instance.address = __raw_address;
    }
    {
        const __raw_username = obj[`${"username"}`] as string;
        if (__raw_username.trim().length === 0) {
            errors.push({
                field: "username",
                message: "Employee.username must not be empty"
            });
        }
        instance.username = __raw_username;
    }
    {
        const __raw_route = obj[`${"route"}`] as string | Route;
        instance.route = __raw_route;
    }
    {
        const __raw_ratePerHour = obj[`${"ratePerHour"}`] as number;
        instance.ratePerHour = __raw_ratePerHour;
    }
    {
        const __raw_active = obj[`${"active"}`] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_isTechnician = obj[`${"isTechnician"}`] as boolean;
        instance.isTechnician = __raw_isTechnician;
    }
    {
        const __raw_isSalesRep = obj[`${"isSalesRep"}`] as boolean;
        instance.isSalesRep = __raw_isSalesRep;
    }
    {
        const __raw_description = obj[`${"description"}`] as string | null;
        instance.description = __raw_description;
    }
    {
        const __raw_linkedinUrl = obj[`${"linkedinUrl"}`] as string | null;
        instance.linkedinUrl = __raw_linkedinUrl;
    }
    {
        const __raw_attendance = obj[`${"attendance"}`] as Array<string>;
        if (Array.isArray(__raw_attendance)) {
            instance.attendance = __raw_attendance as string[];
        }
    }
    {
        const __raw_settings = obj[`${"settings"}`] as Settings;
        {
            const __result = settingsDeserializeWithContext(__raw_settings, ctx);
            ctx.assignOrDefer(instance, `${"settings"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Employee;
}
export function employeeValidateField<K extends keyof Employee>(_field: K, _value: Employee[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"name"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Employee.name must not be empty"
            });
        }
    }
    if (_field === `${"role"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "role",
                message: "Employee.role must not be empty"
            });
        }
    }
    if (_field === `${"address"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "address",
                message: "Employee.address must not be empty"
            });
        }
    }
    if (_field === `${"username"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "username",
                message: "Employee.username must not be empty"
            });
        }
    }
    return errors;
}
export function employeeValidateFields(_partial: Partial<Employee>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"name"}` in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Employee.name must not be empty"
            });
        }
    }
    if (`${"role"}` in _partial && _partial.role !== undefined) {
        const __val = _partial.role as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "role",
                message: "Employee.role must not be empty"
            });
        }
    }
    if (`${"address"}` in _partial && _partial.address !== undefined) {
        const __val = _partial.address as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "address",
                message: "Employee.address must not be empty"
            });
        }
    }
    if (`${"username"}` in _partial && _partial.username !== undefined) {
        const __val = _partial.username as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "username",
                message: "Employee.username must not be empty"
            });
        }
    }
    return errors;
}
export function employeeHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "imageUrl" in o && "name" in o && "phones" in o && "role" in o && "title" in o && "email" in o && "address" in o && "username" in o && "route" in o && "ratePerHour" in o && "active" in o && "isTechnician" in o && "isSalesRep" in o && "description" in o && "linkedinUrl" in o && "attendance" in o && "settings" in o';
}
export function employeeIs(obj: unknown): obj is Employee {
    if (!employeeHasShape(obj)) {
        return false;
    }
    const result = employeeDeserialize(obj);
    return result.success;
}

export const Employee = {
  defaultValue: employeeDefaultValue,
  serialize: employeeSerialize,
  serializeWithContext: employeeSerializeWithContext,
  deserialize: employeeDeserialize,
  deserializeWithContext: employeeDeserializeWithContext,
  validateFields: employeeValidateFields,
  hasShape: employeeHasShape,
  is: employeeIs
} as const;


export interface Commissions {
    
    technician: string;
    
    salesRep: string;
}

export function commissionsDefaultValue(): Commissions {
    return {
        technician: "",
        salesRep: ""
    } as Commissions;
}

export function commissionsSerialize(value: Commissions): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(commissionsSerializeWithContext(value, ctx));
}
export function commissionsSerializeWithContext(value: Commissions, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Commissions"}`,
        __id
    };
    result[`${"technician"}`] = value.technician;
    result[`${"salesRep"}`] = value.salesRep;
    return result;
}

export function commissionsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Commissions } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = commissionsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Commissions.deserialize: root cannot be a forward reference"
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
export function commissionsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Commissions | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Commissions"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"technician"}` in obj)) {
        errors.push({
            field: `${"technician"}`,
            message: "missing required field"
        });
    }
    if (!(`${"salesRep"}` in obj)) {
        errors.push({
            field: `${"salesRep"}`,
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
        const __raw_technician = obj[`${"technician"}`] as string;
        if (__raw_technician.trim().length === 0) {
            errors.push({
                field: "technician",
                message: "Commissions.technician must not be empty"
            });
        }
        instance.technician = __raw_technician;
    }
    {
        const __raw_salesRep = obj[`${"salesRep"}`] as string;
        if (__raw_salesRep.trim().length === 0) {
            errors.push({
                field: "salesRep",
                message: "Commissions.salesRep must not be empty"
            });
        }
        instance.salesRep = __raw_salesRep;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Commissions;
}
export function commissionsValidateField<K extends keyof Commissions>(_field: K, _value: Commissions[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"technician"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "technician",
                message: "Commissions.technician must not be empty"
            });
        }
    }
    if (_field === `${"salesRep"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "salesRep",
                message: "Commissions.salesRep must not be empty"
            });
        }
    }
    return errors;
}
export function commissionsValidateFields(_partial: Partial<Commissions>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"technician"}` in _partial && _partial.technician !== undefined) {
        const __val = _partial.technician as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "technician",
                message: "Commissions.technician must not be empty"
            });
        }
    }
    if (`${"salesRep"}` in _partial && _partial.salesRep !== undefined) {
        const __val = _partial.salesRep as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "salesRep",
                message: "Commissions.salesRep must not be empty"
            });
        }
    }
    return errors;
}
export function commissionsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"technician" in o && "salesRep" in o';
}
export function commissionsIs(obj: unknown): obj is Commissions {
    if (!commissionsHasShape(obj)) {
        return false;
    }
    const result = commissionsDeserialize(obj);
    return result.success;
}

export const Commissions = {
  defaultValue: commissionsDefaultValue,
  serialize: commissionsSerialize,
  serializeWithContext: commissionsSerializeWithContext,
  deserialize: commissionsDeserialize,
  deserializeWithContext: commissionsDeserializeWithContext,
  validateFields: commissionsValidateFields,
  hasShape: commissionsHasShape,
  is: commissionsIs
} as const;


export interface Number {
    
    countryCode: string;
    
    areaCode: string;
    
    localNumber: string;
}

export function numberDefaultValue(): Number {
    return {
        countryCode: "",
        areaCode: "",
        localNumber: ""
    } as Number;
}

export function numberSerialize(value: Number): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(numberSerializeWithContext(value, ctx));
}
export function numberSerializeWithContext(value: Number, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Number"}`,
        __id
    };
    result[`${"countryCode"}`] = value.countryCode;
    result[`${"areaCode"}`] = value.areaCode;
    result[`${"localNumber"}`] = value.localNumber;
    return result;
}

export function numberDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Number } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = numberDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Number.deserialize: root cannot be a forward reference"
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
export function numberDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Number | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Number"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"countryCode"}` in obj)) {
        errors.push({
            field: `${"countryCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"areaCode"}` in obj)) {
        errors.push({
            field: `${"areaCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"localNumber"}` in obj)) {
        errors.push({
            field: `${"localNumber"}`,
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
        const __raw_countryCode = obj[`${"countryCode"}`] as string;
        if (__raw_countryCode.trim().length === 0) {
            errors.push({
                field: "countryCode",
                message: "Number.countryCode must not be empty"
            });
        }
        instance.countryCode = __raw_countryCode;
    }
    {
        const __raw_areaCode = obj[`${"areaCode"}`] as string;
        if (__raw_areaCode.trim().length === 0) {
            errors.push({
                field: "areaCode",
                message: "Number.areaCode must not be empty"
            });
        }
        instance.areaCode = __raw_areaCode;
    }
    {
        const __raw_localNumber = obj[`${"localNumber"}`] as string;
        if (__raw_localNumber.trim().length === 0) {
            errors.push({
                field: "localNumber",
                message: "Number.localNumber must not be empty"
            });
        }
        instance.localNumber = __raw_localNumber;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Number;
}
export function numberValidateField<K extends keyof Number>(_field: K, _value: Number[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"countryCode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "countryCode",
                message: "Number.countryCode must not be empty"
            });
        }
    }
    if (_field === `${"areaCode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "areaCode",
                message: "Number.areaCode must not be empty"
            });
        }
    }
    if (_field === `${"localNumber"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "localNumber",
                message: "Number.localNumber must not be empty"
            });
        }
    }
    return errors;
}
export function numberValidateFields(_partial: Partial<Number>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"countryCode"}` in _partial && _partial.countryCode !== undefined) {
        const __val = _partial.countryCode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "countryCode",
                message: "Number.countryCode must not be empty"
            });
        }
    }
    if (`${"areaCode"}` in _partial && _partial.areaCode !== undefined) {
        const __val = _partial.areaCode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "areaCode",
                message: "Number.areaCode must not be empty"
            });
        }
    }
    if (`${"localNumber"}` in _partial && _partial.localNumber !== undefined) {
        const __val = _partial.localNumber as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "localNumber",
                message: "Number.localNumber must not be empty"
            });
        }
    }
    return errors;
}
export function numberHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"countryCode" in o && "areaCode" in o && "localNumber" in o';
}
export function numberIs(obj: unknown): obj is Number {
    if (!numberHasShape(obj)) {
        return false;
    }
    const result = numberDeserialize(obj);
    return result.success;
}

export const Number = {
  defaultValue: numberDefaultValue,
  serialize: numberSerialize,
  serializeWithContext: numberSerializeWithContext,
  deserialize: numberDeserialize,
  deserializeWithContext: numberDeserializeWithContext,
  validateFields: numberValidateFields,
  hasShape: numberHasShape,
  is: numberIs
} as const;


export interface DataPath {
    path: Array<string>;
    formatter: string | null;
}

export function dataPathDefaultValue(): DataPath {
    return {
        path: [],
        formatter: null
    } as DataPath;
}

export function dataPathSerialize(value: DataPath): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(dataPathSerializeWithContext(value, ctx));
}
export function dataPathSerializeWithContext(value: DataPath, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"DataPath"}`,
        __id
    };
    result[`${"path"}`] = value.path;
    result[`${"formatter"}`] = value.formatter;
    return result;
}

export function dataPathDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: DataPath } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = dataPathDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "DataPath.deserialize: root cannot be a forward reference"
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
export function dataPathDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): DataPath | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"DataPath"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"path"}` in obj)) {
        errors.push({
            field: `${"path"}`,
            message: "missing required field"
        });
    }
    if (!(`${"formatter"}` in obj)) {
        errors.push({
            field: `${"formatter"}`,
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
        const __raw_path = obj[`${"path"}`] as Array<string>;
        if (Array.isArray(__raw_path)) {
            instance.path = __raw_path as string[];
        }
    }
    {
        const __raw_formatter = obj[`${"formatter"}`] as string | null;
        instance.formatter = __raw_formatter;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as DataPath;
}
export function dataPathValidateField<K extends keyof DataPath>(_field: K, _value: DataPath[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function dataPathValidateFields(_partial: Partial<DataPath>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function dataPathHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"path" in o && "formatter" in o';
}
export function dataPathIs(obj: unknown): obj is DataPath {
    if (!dataPathHasShape(obj)) {
        return false;
    }
    const result = dataPathDeserialize(obj);
    return result.success;
}

export const DataPath = {
  defaultValue: dataPathDefaultValue,
  serialize: dataPathSerialize,
  serializeWithContext: dataPathSerializeWithContext,
  deserialize: dataPathDeserialize,
  deserializeWithContext: dataPathDeserializeWithContext,
  validateFields: dataPathValidateFields,
  hasShape: dataPathHasShape,
  is: dataPathIs
} as const;


export interface Route {
    id: string;
    techs: Array<string | Employee> | null;
    active: boolean;
    
    name: string;
    
    phone: string;
    
    position: string;
    serviceRoute: boolean;
    defaultDurationHours: number;
    tags: Array<string>;
    icon: string | null;
    color: string | null;
}

export function routeDefaultValue(): Route {
    return {
        id: "",
        techs: null,
        active: false,
        name: "",
        phone: "",
        position: "",
        serviceRoute: false,
        defaultDurationHours: 0,
        tags: [],
        icon: null,
        color: null
    } as Route;
}

export function routeSerialize(value: Route): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(routeSerializeWithContext(value, ctx));
}
export function routeSerializeWithContext(value: Route, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Route"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    if (value.techs !== null) {
        result[`${"techs"}`] = value.techs;
    }
    result[`${"active"}`] = value.active;
    result[`${"name"}`] = value.name;
    result[`${"phone"}`] = value.phone;
    result[`${"position"}`] = value.position;
    result[`${"serviceRoute"}`] = value.serviceRoute;
    result[`${"defaultDurationHours"}`] = value.defaultDurationHours;
    result[`${"tags"}`] = value.tags;
    result[`${"icon"}`] = value.icon;
    result[`${"color"}`] = value.color;
    return result;
}

export function routeDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Route } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = routeDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Route.deserialize: root cannot be a forward reference"
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
export function routeDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Route | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Route"}.deserializeWithContext: expected an object`
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
    if (!(`${"techs"}` in obj)) {
        errors.push({
            field: `${"techs"}`,
            message: "missing required field"
        });
    }
    if (!(`${"active"}` in obj)) {
        errors.push({
            field: `${"active"}`,
            message: "missing required field"
        });
    }
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phone"}` in obj)) {
        errors.push({
            field: `${"phone"}`,
            message: "missing required field"
        });
    }
    if (!(`${"position"}` in obj)) {
        errors.push({
            field: `${"position"}`,
            message: "missing required field"
        });
    }
    if (!(`${"serviceRoute"}` in obj)) {
        errors.push({
            field: `${"serviceRoute"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaultDurationHours"}` in obj)) {
        errors.push({
            field: `${"defaultDurationHours"}`,
            message: "missing required field"
        });
    }
    if (!(`${"tags"}` in obj)) {
        errors.push({
            field: `${"tags"}`,
            message: "missing required field"
        });
    }
    if (!(`${"icon"}` in obj)) {
        errors.push({
            field: `${"icon"}`,
            message: "missing required field"
        });
    }
    if (!(`${"color"}` in obj)) {
        errors.push({
            field: `${"color"}`,
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
        const __raw_techs = obj[`${"techs"}`] as Array<string | Employee> | null;
        if (__raw_techs === null) {
            instance.techs = null;
        } else {
            instance.techs = __raw_techs;
        }
    }
    {
        const __raw_active = obj[`${"active"}`] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Route.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_phone = obj[`${"phone"}`] as string;
        if (__raw_phone.trim().length === 0) {
            errors.push({
                field: "phone",
                message: "Route.phone must not be empty"
            });
        }
        instance.phone = __raw_phone;
    }
    {
        const __raw_position = obj[`${"position"}`] as string;
        if (__raw_position.trim().length === 0) {
            errors.push({
                field: "position",
                message: "Route.position must not be empty"
            });
        }
        instance.position = __raw_position;
    }
    {
        const __raw_serviceRoute = obj[`${"serviceRoute"}`] as boolean;
        instance.serviceRoute = __raw_serviceRoute;
    }
    {
        const __raw_defaultDurationHours = obj[`${"defaultDurationHours"}`] as number;
        instance.defaultDurationHours = __raw_defaultDurationHours;
    }
    {
        const __raw_tags = obj[`${"tags"}`] as Array<string>;
        if (Array.isArray(__raw_tags)) {
            instance.tags = __raw_tags as string[];
        }
    }
    {
        const __raw_icon = obj[`${"icon"}`] as string | null;
        instance.icon = __raw_icon;
    }
    {
        const __raw_color = obj[`${"color"}`] as string | null;
        instance.color = __raw_color;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Route;
}
export function routeValidateField<K extends keyof Route>(_field: K, _value: Route[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"name"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Route.name must not be empty"
            });
        }
    }
    if (_field === `${"phone"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "phone",
                message: "Route.phone must not be empty"
            });
        }
    }
    if (_field === `${"position"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "position",
                message: "Route.position must not be empty"
            });
        }
    }
    return errors;
}
export function routeValidateFields(_partial: Partial<Route>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"name"}` in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Route.name must not be empty"
            });
        }
    }
    if (`${"phone"}` in _partial && _partial.phone !== undefined) {
        const __val = _partial.phone as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "phone",
                message: "Route.phone must not be empty"
            });
        }
    }
    if (`${"position"}` in _partial && _partial.position !== undefined) {
        const __val = _partial.position as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "position",
                message: "Route.position must not be empty"
            });
        }
    }
    return errors;
}
export function routeHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "techs" in o && "active" in o && "name" in o && "phone" in o && "position" in o && "serviceRoute" in o && "defaultDurationHours" in o && "tags" in o && "icon" in o && "color" in o';
}
export function routeIs(obj: unknown): obj is Route {
    if (!routeHasShape(obj)) {
        return false;
    }
    const result = routeDeserialize(obj);
    return result.success;
}

export const Route = {
  defaultValue: routeDefaultValue,
  serialize: routeSerialize,
  serializeWithContext: routeSerializeWithContext,
  deserialize: routeDeserialize,
  deserializeWithContext: routeDeserializeWithContext,
  validateFields: routeValidateFields,
  hasShape: routeHasShape,
  is: routeIs
} as const;


export interface EmailParts {
    
    local: string;
    
    domainName: string;
    
    topLevelDomain: string;
}

export function emailPartsDefaultValue(): EmailParts {
    return {
        local: "",
        domainName: "",
        topLevelDomain: ""
    } as EmailParts;
}

export function emailPartsSerialize(value: EmailParts): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(emailPartsSerializeWithContext(value, ctx));
}
export function emailPartsSerializeWithContext(value: EmailParts, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"EmailParts"}`,
        __id
    };
    result[`${"local"}`] = value.local;
    result[`${"domainName"}`] = value.domainName;
    result[`${"topLevelDomain"}`] = value.topLevelDomain;
    return result;
}

export function emailPartsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: EmailParts } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = emailPartsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "EmailParts.deserialize: root cannot be a forward reference"
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
export function emailPartsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): EmailParts | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"EmailParts"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"local"}` in obj)) {
        errors.push({
            field: `${"local"}`,
            message: "missing required field"
        });
    }
    if (!(`${"domainName"}` in obj)) {
        errors.push({
            field: `${"domainName"}`,
            message: "missing required field"
        });
    }
    if (!(`${"topLevelDomain"}` in obj)) {
        errors.push({
            field: `${"topLevelDomain"}`,
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
        const __raw_local = obj[`${"local"}`] as string;
        if (__raw_local.trim().length === 0) {
            errors.push({
                field: "local",
                message: "EmailParts.local must not be empty"
            });
        }
        instance.local = __raw_local;
    }
    {
        const __raw_domainName = obj[`${"domainName"}`] as string;
        if (__raw_domainName.trim().length === 0) {
            errors.push({
                field: "domainName",
                message: "EmailParts.domainName must not be empty"
            });
        }
        instance.domainName = __raw_domainName;
    }
    {
        const __raw_topLevelDomain = obj[`${"topLevelDomain"}`] as string;
        if (__raw_topLevelDomain.trim().length === 0) {
            errors.push({
                field: "topLevelDomain",
                message: "EmailParts.topLevelDomain must not be empty"
            });
        }
        instance.topLevelDomain = __raw_topLevelDomain;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as EmailParts;
}
export function emailPartsValidateField<K extends keyof EmailParts>(_field: K, _value: EmailParts[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"local"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "local",
                message: "EmailParts.local must not be empty"
            });
        }
    }
    if (_field === `${"domainName"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "domainName",
                message: "EmailParts.domainName must not be empty"
            });
        }
    }
    if (_field === `${"topLevelDomain"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "topLevelDomain",
                message: "EmailParts.topLevelDomain must not be empty"
            });
        }
    }
    return errors;
}
export function emailPartsValidateFields(_partial: Partial<EmailParts>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"local"}` in _partial && _partial.local !== undefined) {
        const __val = _partial.local as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "local",
                message: "EmailParts.local must not be empty"
            });
        }
    }
    if (`${"domainName"}` in _partial && _partial.domainName !== undefined) {
        const __val = _partial.domainName as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "domainName",
                message: "EmailParts.domainName must not be empty"
            });
        }
    }
    if (`${"topLevelDomain"}` in _partial && _partial.topLevelDomain !== undefined) {
        const __val = _partial.topLevelDomain as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "topLevelDomain",
                message: "EmailParts.topLevelDomain must not be empty"
            });
        }
    }
    return errors;
}
export function emailPartsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"local" in o && "domainName" in o && "topLevelDomain" in o';
}
export function emailPartsIs(obj: unknown): obj is EmailParts {
    if (!emailPartsHasShape(obj)) {
        return false;
    }
    const result = emailPartsDeserialize(obj);
    return result.success;
}

export const EmailParts = {
  defaultValue: emailPartsDefaultValue,
  serialize: emailPartsSerialize,
  serializeWithContext: emailPartsSerializeWithContext,
  deserialize: emailPartsDeserialize,
  deserializeWithContext: emailPartsDeserializeWithContext,
  validateFields: emailPartsValidateFields,
  hasShape: emailPartsHasShape,
  is: emailPartsIs
} as const;


export interface Sent {
    recipient: string | null;
    method: string | null;
}

export function sentDefaultValue(): Sent {
    return {
        recipient: null,
        method: null
    } as Sent;
}

export function sentSerialize(value: Sent): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(sentSerializeWithContext(value, ctx));
}
export function sentSerializeWithContext(value: Sent, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Sent"}`,
        __id
    };
    result[`${"recipient"}`] = value.recipient;
    result[`${"method"}`] = value.method;
    return result;
}

export function sentDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Sent } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = sentDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Sent.deserialize: root cannot be a forward reference"
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
export function sentDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Sent | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Sent"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"recipient"}` in obj)) {
        errors.push({
            field: `${"recipient"}`,
            message: "missing required field"
        });
    }
    if (!(`${"method"}` in obj)) {
        errors.push({
            field: `${"method"}`,
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
        const __raw_recipient = obj[`${"recipient"}`] as string | null;
        instance.recipient = __raw_recipient;
    }
    {
        const __raw_method = obj[`${"method"}`] as string | null;
        instance.method = __raw_method;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Sent;
}
export function sentValidateField<K extends keyof Sent>(_field: K, _value: Sent[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function sentValidateFields(_partial: Partial<Sent>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function sentHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"recipient" in o && "method" in o';
}
export function sentIs(obj: unknown): obj is Sent {
    if (!sentHasShape(obj)) {
        return false;
    }
    const result = sentDeserialize(obj);
    return result.success;
}

export const Sent = {
  defaultValue: sentDefaultValue,
  serialize: sentSerialize,
  serializeWithContext: sentSerializeWithContext,
  deserialize: sentDeserialize,
  deserializeWithContext: sentDeserializeWithContext,
  validateFields: sentValidateFields,
  hasShape: sentHasShape,
  is: sentIs
} as const;


export interface BilledItem {
    
    
    item: Item;
    
    quantity: number;
    
    taxed: boolean;
    
    upsale: boolean;
}

export function billedItemDefaultValue(): BilledItem {
    return {
        item: "",
        quantity: 0,
        taxed: false,
        upsale: false
    } as BilledItem;
}

export function billedItemSerialize(value: BilledItem): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(billedItemSerializeWithContext(value, ctx));
}
export function billedItemSerializeWithContext(value: BilledItem, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"BilledItem"}`,
        __id
    };
    result[`${"item"}`] = itemSerializeWithContext(value.item, ctx);
    result[`${"quantity"}`] = value.quantity;
    result[`${"taxed"}`] = value.taxed;
    result[`${"upsale"}`] = value.upsale;
    return result;
}

export function billedItemDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: BilledItem } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = billedItemDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "BilledItem.deserialize: root cannot be a forward reference"
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
export function billedItemDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): BilledItem | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"BilledItem"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"item"}` in obj)) {
        errors.push({
            field: `${"item"}`,
            message: "missing required field"
        });
    }
    if (!(`${"quantity"}` in obj)) {
        errors.push({
            field: `${"quantity"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxed"}` in obj)) {
        errors.push({
            field: `${"taxed"}`,
            message: "missing required field"
        });
    }
    if (!(`${"upsale"}` in obj)) {
        errors.push({
            field: `${"upsale"}`,
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
        const __raw_item = obj[`${"item"}`] as Item;
        {
            const __result = itemDeserializeWithContext(__raw_item, ctx);
            ctx.assignOrDefer(instance, `${"item"}`, __result);
        }
    }
    {
        const __raw_quantity = obj[`${"quantity"}`] as number;
        instance.quantity = __raw_quantity;
    }
    {
        const __raw_taxed = obj[`${"taxed"}`] as boolean;
        instance.taxed = __raw_taxed;
    }
    {
        const __raw_upsale = obj[`${"upsale"}`] as boolean;
        instance.upsale = __raw_upsale;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as BilledItem;
}
export function billedItemValidateField<K extends keyof BilledItem>(_field: K, _value: BilledItem[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function billedItemValidateFields(_partial: Partial<BilledItem>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function billedItemHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"item" in o && "quantity" in o && "taxed" in o && "upsale" in o';
}
export function billedItemIs(obj: unknown): obj is BilledItem {
    if (!billedItemHasShape(obj)) {
        return false;
    }
    const result = billedItemDeserialize(obj);
    return result.success;
}

export const BilledItem = {
  defaultValue: billedItemDefaultValue,
  serialize: billedItemSerialize,
  serializeWithContext: billedItemSerializeWithContext,
  deserialize: billedItemDeserialize,
  deserializeWithContext: billedItemDeserializeWithContext,
  validateFields: billedItemValidateFields,
  hasShape: billedItemHasShape,
  is: billedItemIs
} as const;


export interface Coordinates {
    lat: number;
    lng: number;
}

export function coordinatesDefaultValue(): Coordinates {
    return {
        lat: 0,
        lng: 0
    } as Coordinates;
}

export function coordinatesSerialize(value: Coordinates): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(coordinatesSerializeWithContext(value, ctx));
}
export function coordinatesSerializeWithContext(value: Coordinates, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Coordinates"}`,
        __id
    };
    result[`${"lat"}`] = value.lat;
    result[`${"lng"}`] = value.lng;
    return result;
}

export function coordinatesDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Coordinates } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = coordinatesDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Coordinates.deserialize: root cannot be a forward reference"
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
export function coordinatesDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Coordinates | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Coordinates"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"lat"}` in obj)) {
        errors.push({
            field: `${"lat"}`,
            message: "missing required field"
        });
    }
    if (!(`${"lng"}` in obj)) {
        errors.push({
            field: `${"lng"}`,
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
        const __raw_lat = obj[`${"lat"}`] as number;
        instance.lat = __raw_lat;
    }
    {
        const __raw_lng = obj[`${"lng"}`] as number;
        instance.lng = __raw_lng;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Coordinates;
}
export function coordinatesValidateField<K extends keyof Coordinates>(_field: K, _value: Coordinates[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function coordinatesValidateFields(_partial: Partial<Coordinates>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function coordinatesHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"lat" in o && "lng" in o';
}
export function coordinatesIs(obj: unknown): obj is Coordinates {
    if (!coordinatesHasShape(obj)) {
        return false;
    }
    const result = coordinatesDeserialize(obj);
    return result.success;
}

export const Coordinates = {
  defaultValue: coordinatesDefaultValue,
  serialize: coordinatesSerialize,
  serializeWithContext: coordinatesSerializeWithContext,
  deserialize: coordinatesDeserialize,
  deserializeWithContext: coordinatesDeserializeWithContext,
  validateFields: coordinatesValidateFields,
  hasShape: coordinatesHasShape,
  is: coordinatesIs
} as const;


export interface Ordered {
    id: string;
    
    in: string | Account;
    
    out: string | Order;
    date: string;
}

export function orderedDefaultValue(): Ordered {
    return {
        id: "",
        in: "",
        out: "",
        date: ""
    } as Ordered;
}

export function orderedSerialize(value: Ordered): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(orderedSerializeWithContext(value, ctx));
}
export function orderedSerializeWithContext(value: Ordered, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Ordered"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"in"}`] = value.in;
    result[`${"out"}`] = value.out;
    result[`${"date"}`] = value.date;
    return result;
}

export function orderedDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Ordered } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = orderedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Ordered.deserialize: root cannot be a forward reference"
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
export function orderedDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Ordered | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Ordered"}.deserializeWithContext: expected an object`
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
    if (!(`${"in"}` in obj)) {
        errors.push({
            field: `${"in"}`,
            message: "missing required field"
        });
    }
    if (!(`${"out"}` in obj)) {
        errors.push({
            field: `${"out"}`,
            message: "missing required field"
        });
    }
    if (!(`${"date"}` in obj)) {
        errors.push({
            field: `${"date"}`,
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
        const __raw_in = obj[`${"in"}`] as string | Account;
        instance.in = __raw_in;
    }
    {
        const __raw_out = obj[`${"out"}`] as string | Order;
        instance.out = __raw_out;
    }
    {
        const __raw_date = obj[`${"date"}`] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Ordered;
}
export function orderedValidateField<K extends keyof Ordered>(_field: K, _value: Ordered[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function orderedValidateFields(_partial: Partial<Ordered>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function orderedHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "in" in o && "out" in o && "date" in o';
}
export function orderedIs(obj: unknown): obj is Ordered {
    if (!orderedHasShape(obj)) {
        return false;
    }
    const result = orderedDeserialize(obj);
    return result.success;
}

export const Ordered = {
  defaultValue: orderedDefaultValue,
  serialize: orderedSerialize,
  serializeWithContext: orderedSerializeWithContext,
  deserialize: orderedDeserialize,
  deserializeWithContext: orderedDeserializeWithContext,
  validateFields: orderedValidateFields,
  hasShape: orderedHasShape,
  is: orderedIs
} as const;


export interface Email {
    
    canEmail: boolean;
    
    
    emailString: string;
}

export function emailDefaultValue(): Email {
    return {
        canEmail: false,
        emailString: ""
    } as Email;
}

export function emailSerialize(value: Email): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(emailSerializeWithContext(value, ctx));
}
export function emailSerializeWithContext(value: Email, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Email"}`,
        __id
    };
    result[`${"canEmail"}`] = value.canEmail;
    result[`${"emailString"}`] = value.emailString;
    return result;
}

export function emailDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Email } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = emailDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Email.deserialize: root cannot be a forward reference"
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
export function emailDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Email | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Email"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"canEmail"}` in obj)) {
        errors.push({
            field: `${"canEmail"}`,
            message: "missing required field"
        });
    }
    if (!(`${"emailString"}` in obj)) {
        errors.push({
            field: `${"emailString"}`,
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
        const __raw_canEmail = obj[`${"canEmail"}`] as boolean;
        instance.canEmail = __raw_canEmail;
    }
    {
        const __raw_emailString = obj[`${"emailString"}`] as string;
        if (__raw_emailString.trim().length === 0) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must not be empty"
            });
        }
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_emailString)) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must be a valid email"
            });
        }
        instance.emailString = __raw_emailString;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Email;
}
export function emailValidateField<K extends keyof Email>(_field: K, _value: Email[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"emailString"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must not be empty"
            });
        }
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must be a valid email"
            });
        }
    }
    return errors;
}
export function emailValidateFields(_partial: Partial<Email>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"emailString"}` in _partial && _partial.emailString !== undefined) {
        const __val = _partial.emailString as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must not be empty"
            });
        }
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
            errors.push({
                field: "emailString",
                message: "Email.emailString must be a valid email"
            });
        }
    }
    return errors;
}
export function emailHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"canEmail" in o && "emailString" in o';
}
export function emailIs(obj: unknown): obj is Email {
    if (!emailHasShape(obj)) {
        return false;
    }
    const result = emailDeserialize(obj);
    return result.success;
}

export const Email = {
  defaultValue: emailDefaultValue,
  serialize: emailSerialize,
  serializeWithContext: emailSerializeWithContext,
  deserialize: emailDeserialize,
  deserializeWithContext: emailDeserializeWithContext,
  validateFields: emailValidateFields,
  hasShape: emailHasShape,
  is: emailIs
} as const;


export interface RecurrenceRule {
    interval: Interval;
    recurrenceBegins: string;
    recurrenceEnds: RecurrenceEnd | null;
    cancelledInstances: Array<string> | null;
    additionalInstances: Array<string> | null;
}

export function recurrenceRuleDefaultValue(): RecurrenceRule {
    return {
        interval: intervalDefaultValue(),
        recurrenceBegins: "",
        recurrenceEnds: null,
        cancelledInstances: null,
        additionalInstances: null
    } as RecurrenceRule;
}

export function recurrenceRuleSerialize(value: RecurrenceRule): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(recurrenceRuleSerializeWithContext(value, ctx));
}
export function recurrenceRuleSerializeWithContext(value: RecurrenceRule, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"RecurrenceRule"}`,
        __id
    };
    result[`${"interval"}`] = intervalSerializeWithContext(value.interval, ctx);
    result[`${"recurrenceBegins"}`] = value.recurrenceBegins;
    if (value.recurrenceEnds !== null) {
        result[`${"recurrenceEnds"}`] = recurrenceEndSerializeWithContext(value.recurrenceEnds, ctx);
    }
    if (value.cancelledInstances !== null) {
        result[`${"cancelledInstances"}`] = value.cancelledInstances;
    }
    if (value.additionalInstances !== null) {
        result[`${"additionalInstances"}`] = value.additionalInstances;
    }
    return result;
}

export function recurrenceRuleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: RecurrenceRule } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = recurrenceRuleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "RecurrenceRule.deserialize: root cannot be a forward reference"
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
export function recurrenceRuleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): RecurrenceRule | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"RecurrenceRule"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"interval"}` in obj)) {
        errors.push({
            field: `${"interval"}`,
            message: "missing required field"
        });
    }
    if (!(`${"recurrenceBegins"}` in obj)) {
        errors.push({
            field: `${"recurrenceBegins"}`,
            message: "missing required field"
        });
    }
    if (!(`${"recurrenceEnds"}` in obj)) {
        errors.push({
            field: `${"recurrenceEnds"}`,
            message: "missing required field"
        });
    }
    if (!(`${"cancelledInstances"}` in obj)) {
        errors.push({
            field: `${"cancelledInstances"}`,
            message: "missing required field"
        });
    }
    if (!(`${"additionalInstances"}` in obj)) {
        errors.push({
            field: `${"additionalInstances"}`,
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
        const __raw_interval = obj[`${"interval"}`] as Interval;
        {
            const __result = intervalDeserializeWithContext(__raw_interval, ctx);
            ctx.assignOrDefer(instance, `${"interval"}`, __result);
        }
    }
    {
        const __raw_recurrenceBegins = obj[`${"recurrenceBegins"}`] as string;
        instance.recurrenceBegins = __raw_recurrenceBegins;
    }
    {
        const __raw_recurrenceEnds = obj[`${"recurrenceEnds"}`] as RecurrenceEnd | null;
        if (__raw_recurrenceEnds === null) {
            instance.recurrenceEnds = null;
        } else {
            const __result = recurrenceEndDeserializeWithContext(__raw_recurrenceEnds, ctx);
            ctx.assignOrDefer(instance, `${"recurrenceEnds"}`, __result);
        }
    }
    {
        const __raw_cancelledInstances = obj[`${"cancelledInstances"}`] as Array<string> | null;
        if (__raw_cancelledInstances === null) {
            instance.cancelledInstances = null;
        } else {
            instance.cancelledInstances = __raw_cancelledInstances;
        }
    }
    {
        const __raw_additionalInstances = obj[`${"additionalInstances"}`] as Array<string> | null;
        if (__raw_additionalInstances === null) {
            instance.additionalInstances = null;
        } else {
            instance.additionalInstances = __raw_additionalInstances;
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as RecurrenceRule;
}
export function recurrenceRuleValidateField<K extends keyof RecurrenceRule>(_field: K, _value: RecurrenceRule[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function recurrenceRuleValidateFields(_partial: Partial<RecurrenceRule>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function recurrenceRuleHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"interval" in o && "recurrenceBegins" in o && "recurrenceEnds" in o && "cancelledInstances" in o && "additionalInstances" in o';
}
export function recurrenceRuleIs(obj: unknown): obj is RecurrenceRule {
    if (!recurrenceRuleHasShape(obj)) {
        return false;
    }
    const result = recurrenceRuleDeserialize(obj);
    return result.success;
}

export const RecurrenceRule = {
  defaultValue: recurrenceRuleDefaultValue,
  serialize: recurrenceRuleSerialize,
  serializeWithContext: recurrenceRuleSerializeWithContext,
  deserialize: recurrenceRuleDeserialize,
  deserializeWithContext: recurrenceRuleDeserializeWithContext,
  validateFields: recurrenceRuleValidateFields,
  hasShape: recurrenceRuleHasShape,
  is: recurrenceRuleIs
} as const;


export interface LastName {
    
    name: string;
}

export function lastNameDefaultValue(): LastName {
    return {
        name: ""
    } as LastName;
}

export function lastNameSerialize(value: LastName): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(lastNameSerializeWithContext(value, ctx));
}
export function lastNameSerializeWithContext(value: LastName, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"LastName"}`,
        __id
    };
    result[`${"name"}`] = value.name;
    return result;
}

export function lastNameDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: LastName } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = lastNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "LastName.deserialize: root cannot be a forward reference"
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
export function lastNameDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): LastName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"LastName"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
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
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "LastName.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as LastName;
}
export function lastNameValidateField<K extends keyof LastName>(_field: K, _value: LastName[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"name"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "LastName.name must not be empty"
            });
        }
    }
    return errors;
}
export function lastNameValidateFields(_partial: Partial<LastName>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"name"}` in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "LastName.name must not be empty"
            });
        }
    }
    return errors;
}
export function lastNameHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"name" in o';
}
export function lastNameIs(obj: unknown): obj is LastName {
    if (!lastNameHasShape(obj)) {
        return false;
    }
    const result = lastNameDeserialize(obj);
    return result.success;
}

export const LastName = {
  defaultValue: lastNameDefaultValue,
  serialize: lastNameSerialize,
  serializeWithContext: lastNameSerializeWithContext,
  deserialize: lastNameDeserialize,
  deserializeWithContext: lastNameDeserializeWithContext,
  validateFields: lastNameValidateFields,
  hasShape: lastNameHasShape,
  is: lastNameIs
} as const;


export interface Cardinal {
    north: number;
    east: number;
    south: number;
    west: number;
}

export function cardinalDefaultValue(): Cardinal {
    return {
        north: 0,
        east: 0,
        south: 0,
        west: 0
    } as Cardinal;
}

export function cardinalSerialize(value: Cardinal): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(cardinalSerializeWithContext(value, ctx));
}
export function cardinalSerializeWithContext(value: Cardinal, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Cardinal"}`,
        __id
    };
    result[`${"north"}`] = value.north;
    result[`${"east"}`] = value.east;
    result[`${"south"}`] = value.south;
    result[`${"west"}`] = value.west;
    return result;
}

export function cardinalDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Cardinal } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = cardinalDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Cardinal.deserialize: root cannot be a forward reference"
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
export function cardinalDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Cardinal | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Cardinal"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"north"}` in obj)) {
        errors.push({
            field: `${"north"}`,
            message: "missing required field"
        });
    }
    if (!(`${"east"}` in obj)) {
        errors.push({
            field: `${"east"}`,
            message: "missing required field"
        });
    }
    if (!(`${"south"}` in obj)) {
        errors.push({
            field: `${"south"}`,
            message: "missing required field"
        });
    }
    if (!(`${"west"}` in obj)) {
        errors.push({
            field: `${"west"}`,
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
        const __raw_north = obj[`${"north"}`] as number;
        instance.north = __raw_north;
    }
    {
        const __raw_east = obj[`${"east"}`] as number;
        instance.east = __raw_east;
    }
    {
        const __raw_south = obj[`${"south"}`] as number;
        instance.south = __raw_south;
    }
    {
        const __raw_west = obj[`${"west"}`] as number;
        instance.west = __raw_west;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Cardinal;
}
export function cardinalValidateField<K extends keyof Cardinal>(_field: K, _value: Cardinal[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function cardinalValidateFields(_partial: Partial<Cardinal>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function cardinalHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"north" in o && "east" in o && "south" in o && "west" in o';
}
export function cardinalIs(obj: unknown): obj is Cardinal {
    if (!cardinalHasShape(obj)) {
        return false;
    }
    const result = cardinalDeserialize(obj);
    return result.success;
}

export const Cardinal = {
  defaultValue: cardinalDefaultValue,
  serialize: cardinalSerialize,
  serializeWithContext: cardinalSerializeWithContext,
  deserialize: cardinalDeserialize,
  deserializeWithContext: cardinalDeserializeWithContext,
  validateFields: cardinalValidateFields,
  hasShape: cardinalHasShape,
  is: cardinalIs
} as const;


export type Interval =
    | /** @default */ DailyRecurrenceRule
    | WeeklyRecurrenceRule
    | MonthlyRecurrenceRule
    | YearlyRecurrenceRule;

export function intervalDefaultValue#0#0(): Interval {
    return dailyRecurrenceRuleDefaultValue();
}

export function intervalSerialize#0#0(value: Interval): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(intervalSerializeWithContext(value, ctx));
}
export function intervalSerializeWithContext#0#0(value: Interval, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function intervalDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Interval } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = intervalDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Interval.deserialize: root cannot be a forward reference"
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
export function intervalDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Interval | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Interval | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"DailyRecurrenceRule"}`) {
            return dailyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
        }
        if (__typeName === `${"WeeklyRecurrenceRule"}`) {
            return weeklyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
        }
        if (__typeName === `${"MonthlyRecurrenceRule"}`) {
            return monthlyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
        }
        if (__typeName === `${"YearlyRecurrenceRule"}`) {
            return yearlyRecurrenceRuleDeserializeWithContext(value, ctx) as Interval;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"Interval"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function intervalIs(value: unknown): value is Interval {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "DailyRecurrenceRule" || __typeName === "WeeklyRecurrenceRule" || __typeName === "MonthlyRecurrenceRule" || __typeName === "YearlyRecurrenceRule"') return true;
    }
    return false;
}
     }

export const Interval = {
  deserialize: intervalDeserialize,
  deserializeWithContext: intervalDeserializeWithContext,
  is: intervalIs
} as const;


export type Page =
    | /** @default */ 'SalesHomeDashboard'
    | 'SalesHomeProducts'
    | 'SalesHomeServices'
    | 'SalesHomePackages'
    | 'SalesHomeTaxRates'
    | 'SalesLeadsOverview'
    | 'SalesLeadsActivities'
    | 'SalesLeadsCampaigns'
    | 'SalesLeadsDripCampaigns'
    | 'SalesLeadsOpportunities'
    | 'SalesLeadsPromotions'
    | 'SalesAccountsOverview'
    | 'SalesAccountsActivities'
    | 'SalesAccountsBilling'
    | 'SalesAccountsContracts'
    | 'SalesOrdersOverview'
    | 'SalesOrdersActivities'
    | 'SalesOrdersPayments'
    | 'SalesOrdersCommissions'
    | 'SalesSchedulingSchedule'
    | 'SalesSchedulingAppointments'
    | 'SalesSchedulingRecurring'
    | 'SalesSchedulingRoutes'
    | 'SalesSchedulingReminders'
    | 'UserHome';

export function pageDefaultValue#0#0(): Page {
    return 'SalesHomeDashboard';
}

export function pageSerialize#0#0(value: Page): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(pageSerializeWithContext(value, ctx));
}
export function pageSerializeWithContext#0#0(value: Page, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function pageDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Page } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = pageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Page.deserialize: root cannot be a forward reference"
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
export function pageDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Page | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Page | __mf_PendingRef;
    }
    const allowedValues = [
        "'SalesHomeDashboard', 'SalesHomeProducts', 'SalesHomeServices', 'SalesHomePackages', 'SalesHomeTaxRates', 'SalesLeadsOverview', 'SalesLeadsActivities', 'SalesLeadsCampaigns', 'SalesLeadsDripCampaigns', 'SalesLeadsOpportunities', 'SalesLeadsPromotions', 'SalesAccountsOverview', 'SalesAccountsActivities', 'SalesAccountsBilling', 'SalesAccountsContracts', 'SalesOrdersOverview', 'SalesOrdersActivities', 'SalesOrdersPayments', 'SalesOrdersCommissions', 'SalesSchedulingSchedule', 'SalesSchedulingAppointments', 'SalesSchedulingRecurring', 'SalesSchedulingRoutes', 'SalesSchedulingReminders', 'UserHome'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Page"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Page;
}
export function pageIs(value: unknown): value is Page {
    const allowedValues = [
        "'SalesHomeDashboard', 'SalesHomeProducts', 'SalesHomeServices', 'SalesHomePackages', 'SalesHomeTaxRates', 'SalesLeadsOverview', 'SalesLeadsActivities', 'SalesLeadsCampaigns', 'SalesLeadsDripCampaigns', 'SalesLeadsOpportunities', 'SalesLeadsPromotions', 'SalesAccountsOverview', 'SalesAccountsActivities', 'SalesAccountsBilling', 'SalesAccountsContracts', 'SalesOrdersOverview', 'SalesOrdersActivities', 'SalesOrdersPayments', 'SalesOrdersCommissions', 'SalesSchedulingSchedule', 'SalesSchedulingAppointments', 'SalesSchedulingRecurring', 'SalesSchedulingRoutes', 'SalesSchedulingReminders', 'UserHome'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const Page = {
  deserialize: pageDeserialize,
  deserializeWithContext: pageDeserializeWithContext,
  is: pageIs
} as const;


export type UserRole =
    | /** @default */ 'Administrator'
    | 'SalesRepresentative'
    | 'Technician'
    | 'HumanResources'
    | 'InformationTechnology';

export function userRoleDefaultValue#0#0(): UserRole {
    return 'Administrator';
}

export function userRoleSerialize#0#0(value: UserRole): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(userRoleSerializeWithContext(value, ctx));
}
export function userRoleSerializeWithContext#0#0(value: UserRole, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function userRoleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: UserRole } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = userRoleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "UserRole.deserialize: root cannot be a forward reference"
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
export function userRoleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): UserRole | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as UserRole | __mf_PendingRef;
    }
    const allowedValues = [
        "'Administrator', 'SalesRepresentative', 'Technician', 'HumanResources', 'InformationTechnology'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"UserRole"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as UserRole;
}
export function userRoleIs(value: unknown): value is UserRole {
    const allowedValues = [
        "'Administrator', 'SalesRepresentative', 'Technician', 'HumanResources', 'InformationTechnology'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const UserRole = {
  deserialize: userRoleDeserialize,
  deserializeWithContext: userRoleDeserializeWithContext,
  is: userRoleIs
} as const;


export type Target =
    | /** @default */ Account
    | User
    | Employee
    | Appointment
    | Lead
    | TaxRate
    | Site
    | Route
    | Company
    | Product
    | Service
    | Order
    | Payment
    | Package
    | Promotion
    | Represents
    | Ordered;

export function targetDefaultValue#0#0(): Target {
    return accountDefaultValue();
}

export function targetSerialize#0#0(value: Target): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(targetSerializeWithContext(value, ctx));
}
export function targetSerializeWithContext#0#0(value: Target, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function targetDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Target } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = targetDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Target.deserialize: root cannot be a forward reference"
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
export function targetDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Target | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Target | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"Account"}`) {
            return accountDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"User"}`) {
            return userDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Employee"}`) {
            return employeeDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Appointment"}`) {
            return appointmentDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Lead"}`) {
            return leadDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"TaxRate"}`) {
            return taxRateDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Site"}`) {
            return siteDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Route"}`) {
            return routeDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Company"}`) {
            return companyDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Product"}`) {
            return productDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Service"}`) {
            return serviceDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Order"}`) {
            return orderDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Payment"}`) {
            return paymentDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Package"}`) {
            return packageDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Promotion"}`) {
            return promotionDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Represents"}`) {
            return representsDeserializeWithContext(value, ctx) as Target;
        }
        if (__typeName === `${"Ordered"}`) {
            return orderedDeserializeWithContext(value, ctx) as Target;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"Target"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function targetIs(value: unknown): value is Target {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "Account" || __typeName === "User" || __typeName === "Employee" || __typeName === "Appointment" || __typeName === "Lead" || __typeName === "TaxRate" || __typeName === "Site" || __typeName === "Route" || __typeName === "Company" || __typeName === "Product" || __typeName === "Service" || __typeName === "Order" || __typeName === "Payment" || __typeName === "Package" || __typeName === "Promotion" || __typeName === "Represents" || __typeName === "Ordered"') return true;
    }
    return false;
}
     }

export const Target = {
  deserialize: targetDeserialize,
  deserializeWithContext: targetDeserializeWithContext,
  is: targetIs
} as const;


export type RecurrenceEnd = /** @default(0) */ number | string;

export function recurrenceEndDefaultValue#0#0(): RecurrenceEnd {
    return 0;
}

export function recurrenceEndSerialize#0#0(value: RecurrenceEnd): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(recurrenceEndSerializeWithContext(value, ctx));
}
export function recurrenceEndSerializeWithContext#0#0(value: RecurrenceEnd, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function recurrenceEndDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: RecurrenceEnd } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = recurrenceEndDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "RecurrenceEnd.deserialize: root cannot be a forward reference"
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
export function recurrenceEndDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): RecurrenceEnd | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RecurrenceEnd | __mf_PendingRef;
    }
    if (typeof value === `${"number"}`) {
        return value as RecurrenceEnd;
    }
    if (typeof value === `${"string"}`) {
        return value as RecurrenceEnd;
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"RecurrenceEnd"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function recurrenceEndIs(value: unknown): value is RecurrenceEnd {
    if (typeof value === `${"number"}`) return true;
    if (typeof value === `${"string"}`) return true;
    return false;
}

export const RecurrenceEnd = {
  deserialize: recurrenceEndDeserialize,
  deserializeWithContext: recurrenceEndDeserializeWithContext,
  is: recurrenceEndIs
} as const;


export type OverviewDisplay = /** @default */ 'Card' | 'Table';

export function overviewDisplayDefaultValue#0#0(): OverviewDisplay {
    return 'Card';
}

export function overviewDisplaySerialize#0#0(value: OverviewDisplay): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(overviewDisplaySerializeWithContext(value, ctx));
}
export function overviewDisplaySerializeWithContext#0#0(value: OverviewDisplay, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function overviewDisplayDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: OverviewDisplay } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = overviewDisplayDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "OverviewDisplay.deserialize: root cannot be a forward reference"
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
export function overviewDisplayDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): OverviewDisplay | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as OverviewDisplay | __mf_PendingRef;
    }
    const allowedValues = [
        "'Card', 'Table'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"OverviewDisplay"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as OverviewDisplay;
}
export function overviewDisplayIs(value: unknown): value is OverviewDisplay {
    const allowedValues = [
        "'Card', 'Table'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const OverviewDisplay = {
  deserialize: overviewDisplayDeserialize,
  deserializeWithContext: overviewDisplayDeserializeWithContext,
  is: overviewDisplayIs
} as const;


export type IntervalUnit = /** @default */ 'Day' | 'Week' | 'Month' | 'Year';

export function intervalUnitDefaultValue#0#0(): IntervalUnit {
    return 'Day';
}

export function intervalUnitSerialize#0#0(value: IntervalUnit): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(intervalUnitSerializeWithContext(value, ctx));
}
export function intervalUnitSerializeWithContext#0#0(value: IntervalUnit, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function intervalUnitDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: IntervalUnit } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = intervalUnitDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "IntervalUnit.deserialize: root cannot be a forward reference"
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
export function intervalUnitDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): IntervalUnit | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as IntervalUnit | __mf_PendingRef;
    }
    const allowedValues = [
        "'Day', 'Week', 'Month', 'Year'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"IntervalUnit"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as IntervalUnit;
}
export function intervalUnitIs(value: unknown): value is IntervalUnit {
    const allowedValues = [
        "'Day', 'Week', 'Month', 'Year'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const IntervalUnit = {
  deserialize: intervalUnitDeserialize,
  deserializeWithContext: intervalUnitDeserializeWithContext,
  is: intervalUnitIs
} as const;


export type Sector = /** @default */ 'Residential' | 'Commercial';

export function sectorDefaultValue#0#0(): Sector {
    return 'Residential';
}

export function sectorSerialize#0#0(value: Sector): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(sectorSerializeWithContext(value, ctx));
}
export function sectorSerializeWithContext#0#0(value: Sector, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function sectorDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Sector } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = sectorDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Sector.deserialize: root cannot be a forward reference"
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
export function sectorDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Sector | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Sector | __mf_PendingRef;
    }
    const allowedValues = [
        "'Residential', 'Commercial'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Sector"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Sector;
}
export function sectorIs(value: unknown): value is Sector {
    const allowedValues = [
        "'Residential', 'Commercial'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const Sector = {
  deserialize: sectorDeserialize,
  deserializeWithContext: sectorDeserializeWithContext,
  is: sectorIs
} as const;


export type Weekday =
    | /** @default */ 'Monday'
    | 'Tuesday'
    | 'Wednesday'
    | 'Thursday'
    | 'Friday'
    | 'Saturday'
    | 'Sunday';

export function weekdayDefaultValue#0#0(): Weekday {
    return 'Monday';
}

export function weekdaySerialize#0#0(value: Weekday): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(weekdaySerializeWithContext(value, ctx));
}
export function weekdaySerializeWithContext#0#0(value: Weekday, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function weekdayDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Weekday } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = weekdayDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Weekday.deserialize: root cannot be a forward reference"
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
export function weekdayDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Weekday | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Weekday | __mf_PendingRef;
    }
    const allowedValues = [
        "'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Weekday"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Weekday;
}
export function weekdayIs(value: unknown): value is Weekday {
    const allowedValues = [
        "'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const Weekday = {
  deserialize: weekdayDeserialize,
  deserializeWithContext: weekdayDeserializeWithContext,
  is: weekdayIs
} as const;


export type Status = /** @default */ 'Scheduled' | 'OnDeck' | 'Waiting';

export function statusDefaultValue#0#0(): Status {
    return 'Scheduled';
}

export function statusSerialize#0#0(value: Status): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(statusSerializeWithContext(value, ctx));
}
export function statusSerializeWithContext#0#0(value: Status, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function statusDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Status } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = statusDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Status.deserialize: root cannot be a forward reference"
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
export function statusDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Status | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Status | __mf_PendingRef;
    }
    const allowedValues = [
        "'Scheduled', 'OnDeck', 'Waiting'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Status"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Status;
}
export function statusIs(value: unknown): value is Status {
    const allowedValues = [
        "'Scheduled', 'OnDeck', 'Waiting'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const Status = {
  deserialize: statusDeserialize,
  deserializeWithContext: statusDeserializeWithContext,
  is: statusIs
} as const;


export type NextStep = /** @default */ 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation';

export function nextStepDefaultValue#0#0(): NextStep {
    return 'InitialContact';
}

export function nextStepSerialize#0#0(value: NextStep): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(nextStepSerializeWithContext(value, ctx));
}
export function nextStepSerializeWithContext#0#0(value: NextStep, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function nextStepDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: NextStep } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = nextStepDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "NextStep.deserialize: root cannot be a forward reference"
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
export function nextStepDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): NextStep | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as NextStep | __mf_PendingRef;
    }
    const allowedValues = [
        "'InitialContact', 'Qualified', 'Estimate', 'Negotiation'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"NextStep"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as NextStep;
}
export function nextStepIs(value: unknown): value is NextStep {
    const allowedValues = [
        "'InitialContact', 'Qualified', 'Estimate', 'Negotiation'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const NextStep = {
  deserialize: nextStepDeserialize,
  deserializeWithContext: nextStepDeserializeWithContext,
  is: nextStepIs
} as const;


export type LeadStage =
    | /** @default */ 'Open'
    | 'InitialContact'
    | 'Qualified'
    | 'Estimate'
    | 'Negotiation';

export function leadStageDefaultValue#0#0(): LeadStage {
    return 'Open';
}

export function leadStageSerialize#0#0(value: LeadStage): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(leadStageSerializeWithContext(value, ctx));
}
export function leadStageSerializeWithContext#0#0(value: LeadStage, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function leadStageDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: LeadStage } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = leadStageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "LeadStage.deserialize: root cannot be a forward reference"
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
export function leadStageDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): LeadStage | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as LeadStage | __mf_PendingRef;
    }
    const allowedValues = [
        "'Open', 'InitialContact', 'Qualified', 'Estimate', 'Negotiation'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"LeadStage"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as LeadStage;
}
export function leadStageIs(value: unknown): value is LeadStage {
    const allowedValues = [
        "'Open', 'InitialContact', 'Qualified', 'Estimate', 'Negotiation'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const LeadStage = {
  deserialize: leadStageDeserialize,
  deserializeWithContext: leadStageDeserializeWithContext,
  is: leadStageIs
} as const;


export type AccountName = /** @default */ CompanyName | PersonName;

export function accountNameDefaultValue#0#0(): AccountName {
    return companyNameDefaultValue();
}

export function accountNameSerialize#0#0(value: AccountName): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(accountNameSerializeWithContext(value, ctx));
}
export function accountNameSerializeWithContext#0#0(value: AccountName, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function accountNameDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: AccountName } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = accountNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "AccountName.deserialize: root cannot be a forward reference"
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
export function accountNameDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): AccountName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as AccountName | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"CompanyName"}`) {
            return companyNameDeserializeWithContext(value, ctx) as AccountName;
        }
        if (__typeName === `${"PersonName"}`) {
            return personNameDeserializeWithContext(value, ctx) as AccountName;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"AccountName"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function accountNameIs(value: unknown): value is AccountName {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "CompanyName" || __typeName === "PersonName"') return true;
    }
    return false;
}
     }

export const AccountName = {
  deserialize: accountNameDeserialize,
  deserializeWithContext: accountNameDeserializeWithContext,
  is: accountNameIs
} as const;


export type Priority = /** @default */ 'Medium' | 'High' | 'Low';

export function priorityDefaultValue#0#0(): Priority {
    return 'Medium';
}

export function prioritySerialize#0#0(value: Priority): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(prioritySerializeWithContext(value, ctx));
}
export function prioritySerializeWithContext#0#0(value: Priority, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function priorityDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Priority } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = priorityDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Priority.deserialize: root cannot be a forward reference"
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
export function priorityDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Priority | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Priority | __mf_PendingRef;
    }
    const allowedValues = [
        "'Medium', 'High', 'Low'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Priority"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Priority;
}
export function priorityIs(value: unknown): value is Priority {
    const allowedValues = [
        "'Medium', 'High', 'Low'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const Priority = {
  deserialize: priorityDeserialize,
  deserializeWithContext: priorityDeserializeWithContext,
  is: priorityIs
} as const;


export type Applications =
    | /** @default */ 'Sales'
    | 'Accounting'
    | 'Errand'
    | 'HumanResources'
    | 'Logistics'
    | 'Marketing'
    | 'Website';

export function applicationsDefaultValue#0#0(): Applications {
    return 'Sales';
}

export function applicationsSerialize#0#0(value: Applications): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(applicationsSerializeWithContext(value, ctx));
}
export function applicationsSerializeWithContext#0#0(value: Applications, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function applicationsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Applications } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = applicationsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Applications.deserialize: root cannot be a forward reference"
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
export function applicationsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Applications | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Applications | __mf_PendingRef;
    }
    const allowedValues = [
        "'Sales', 'Accounting', 'Errand', 'HumanResources', 'Logistics', 'Marketing', 'Website'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Applications"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Applications;
}
export function applicationsIs(value: unknown): value is Applications {
    const allowedValues = [
        "'Sales', 'Accounting', 'Errand', 'HumanResources', 'Logistics', 'Marketing', 'Website'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const Applications = {
  deserialize: applicationsDeserialize,
  deserializeWithContext: applicationsDeserializeWithContext,
  is: applicationsIs
} as const;


export type JobTitle =
    | /** @default */ 'Technician'
    | 'SalesRepresentative'
    | 'HumanResources'
    | 'InformationTechnology';

export function jobTitleDefaultValue#0#0(): JobTitle {
    return 'Technician';
}

export function jobTitleSerialize#0#0(value: JobTitle): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(jobTitleSerializeWithContext(value, ctx));
}
export function jobTitleSerializeWithContext#0#0(value: JobTitle, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function jobTitleDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: JobTitle } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = jobTitleDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "JobTitle.deserialize: root cannot be a forward reference"
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
export function jobTitleDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): JobTitle | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as JobTitle | __mf_PendingRef;
    }
    const allowedValues = [
        "'Technician', 'SalesRepresentative', 'HumanResources', 'InformationTechnology'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"JobTitle"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as JobTitle;
}
export function jobTitleIs(value: unknown): value is JobTitle {
    const allowedValues = [
        "'Technician', 'SalesRepresentative', 'HumanResources', 'InformationTechnology'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const JobTitle = {
  deserialize: jobTitleDeserialize,
  deserializeWithContext: jobTitleDeserializeWithContext,
  is: jobTitleIs
} as const;


export type ColorsConfig = Cardinal | Ordinal | Custom | /** @default */ Gradient;

export function colorsConfigDefaultValue#0#0(): ColorsConfig {
    return gradientDefaultValue();
}

export function colorsConfigSerialize#0#0(value: ColorsConfig): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(colorsConfigSerializeWithContext(value, ctx));
}
export function colorsConfigSerializeWithContext#0#0(value: ColorsConfig, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function colorsConfigDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ColorsConfig } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = colorsConfigDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ColorsConfig.deserialize: root cannot be a forward reference"
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
export function colorsConfigDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ColorsConfig | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as ColorsConfig | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"Cardinal"}`) {
            return cardinalDeserializeWithContext(value, ctx) as ColorsConfig;
        }
        if (__typeName === `${"Ordinal"}`) {
            return ordinalDeserializeWithContext(value, ctx) as ColorsConfig;
        }
        if (__typeName === `${"Custom"}`) {
            return customDeserializeWithContext(value, ctx) as ColorsConfig;
        }
        if (__typeName === `${"Gradient"}`) {
            return gradientDeserializeWithContext(value, ctx) as ColorsConfig;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"ColorsConfig"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function colorsConfigIs(value: unknown): value is ColorsConfig {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "Cardinal" || __typeName === "Ordinal" || __typeName === "Custom" || __typeName === "Gradient"') return true;
    }
    return false;
}
     }

export const ColorsConfig = {
  deserialize: colorsConfigDeserialize,
  deserializeWithContext: colorsConfigDeserializeWithContext,
  is: colorsConfigIs
} as const;


export type WeekOfMonth = /** @default */ 'First' | 'Second' | 'Third' | 'Fourth' | 'Last';

export function weekOfMonthDefaultValue#0#0(): WeekOfMonth {
    return 'First';
}

export function weekOfMonthSerialize#0#0(value: WeekOfMonth): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(weekOfMonthSerializeWithContext(value, ctx));
}
export function weekOfMonthSerializeWithContext#0#0(value: WeekOfMonth, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function weekOfMonthDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: WeekOfMonth } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = weekOfMonthDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "WeekOfMonth.deserialize: root cannot be a forward reference"
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
export function weekOfMonthDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): WeekOfMonth | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as WeekOfMonth | __mf_PendingRef;
    }
    const allowedValues = [
        "'First', 'Second', 'Third', 'Fourth', 'Last'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"WeekOfMonth"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as WeekOfMonth;
}
export function weekOfMonthIs(value: unknown): value is WeekOfMonth {
    const allowedValues = [
        "'First', 'Second', 'Third', 'Fourth', 'Last'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const WeekOfMonth = {
  deserialize: weekOfMonthDeserialize,
  deserializeWithContext: weekOfMonthDeserializeWithContext,
  is: weekOfMonthIs
} as const;


export type ActivityType = /** @default */ Created | Edited | Sent | Viewed | Commented | Paid;

export function activityTypeDefaultValue#0#0(): ActivityType {
    return createdDefaultValue();
}

export function activityTypeSerialize#0#0(value: ActivityType): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(activityTypeSerializeWithContext(value, ctx));
}
export function activityTypeSerializeWithContext#0#0(value: ActivityType, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function activityTypeDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ActivityType } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = activityTypeDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ActivityType.deserialize: root cannot be a forward reference"
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
export function activityTypeDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ActivityType | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as ActivityType | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"Created"}`) {
            return createdDeserializeWithContext(value, ctx) as ActivityType;
        }
        if (__typeName === `${"Edited"}`) {
            return editedDeserializeWithContext(value, ctx) as ActivityType;
        }
        if (__typeName === `${"Sent"}`) {
            return sentDeserializeWithContext(value, ctx) as ActivityType;
        }
        if (__typeName === `${"Viewed"}`) {
            return viewedDeserializeWithContext(value, ctx) as ActivityType;
        }
        if (__typeName === `${"Commented"}`) {
            return commentedDeserializeWithContext(value, ctx) as ActivityType;
        }
        if (__typeName === `${"Paid"}`) {
            return paidDeserializeWithContext(value, ctx) as ActivityType;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"ActivityType"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function activityTypeIs(value: unknown): value is ActivityType {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "Created" || __typeName === "Edited" || __typeName === "Sent" || __typeName === "Viewed" || __typeName === "Commented" || __typeName === "Paid"') return true;
    }
    return false;
}
     }

export const ActivityType = {
  deserialize: activityTypeDeserialize,
  deserializeWithContext: activityTypeDeserializeWithContext,
  is: activityTypeIs
} as const;


export type RowHeight = 'ExtraSmall' | 'Small' | /** @default */ 'Medium' | 'Large';

export function rowHeightDefaultValue#0#0(): RowHeight {
    return 'Medium';
}

export function rowHeightSerialize#0#0(value: RowHeight): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(rowHeightSerializeWithContext(value, ctx));
}
export function rowHeightSerializeWithContext#0#0(value: RowHeight, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function rowHeightDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: RowHeight } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = rowHeightDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "RowHeight.deserialize: root cannot be a forward reference"
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
export function rowHeightDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): RowHeight | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RowHeight | __mf_PendingRef;
    }
    const allowedValues = [
        "'ExtraSmall', 'Small', 'Medium', 'Large'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"RowHeight"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as RowHeight;
}
export function rowHeightIs(value: unknown): value is RowHeight {
    const allowedValues = [
        "'ExtraSmall', 'Small', 'Medium', 'Large'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const RowHeight = {
  deserialize: rowHeightDeserialize,
  deserializeWithContext: rowHeightDeserializeWithContext,
  is: rowHeightIs
} as const;


export type OrderStage = /** @default */ 'Estimate' | 'Active' | 'Invoice';

export function orderStageDefaultValue#0#0(): OrderStage {
    return 'Estimate';
}

export function orderStageSerialize#0#0(value: OrderStage): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(orderStageSerializeWithContext(value, ctx));
}
export function orderStageSerializeWithContext#0#0(value: OrderStage, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function orderStageDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: OrderStage } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = orderStageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "OrderStage.deserialize: root cannot be a forward reference"
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
export function orderStageDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): OrderStage | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as OrderStage | __mf_PendingRef;
    }
    const allowedValues = [
        "'Estimate', 'Active', 'Invoice'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"OrderStage"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as OrderStage;
}
export function orderStageIs(value: unknown): value is OrderStage {
    const allowedValues = [
        "'Estimate', 'Active', 'Invoice'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const OrderStage = {
  deserialize: orderStageDeserialize,
  deserializeWithContext: orderStageDeserializeWithContext,
  is: orderStageIs
} as const;


export type Table =
    | /** @default */ 'Account'
    | 'Did'
    | 'Appointment'
    | 'Lead'
    | 'TaxRate'
    | 'Site'
    | 'Employee'
    | 'Route'
    | 'Company'
    | 'Product'
    | 'Service'
    | 'User'
    | 'Order'
    | 'Payment'
    | 'Package'
    | 'Promotion'
    | 'Represents'
    | 'Ordered';

export function tableDefaultValue#0#0(): Table {
    return 'Account';
}

export function tableSerialize#0#0(value: Table): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(tableSerializeWithContext(value, ctx));
}
export function tableSerializeWithContext#0#0(value: Table, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function tableDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Table } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = tableDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Table.deserialize: root cannot be a forward reference"
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
export function tableDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Table | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Table | __mf_PendingRef;
    }
    const allowedValues = [
        "'Account', 'Did', 'Appointment', 'Lead', 'TaxRate', 'Site', 'Employee', 'Route', 'Company', 'Product', 'Service', 'User', 'Order', 'Payment', 'Package', 'Promotion', 'Represents', 'Ordered'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Table"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Table;
}
export function tableIs(value: unknown): value is Table {
    const allowedValues = [
        "'Account', 'Did', 'Appointment', 'Lead', 'TaxRate', 'Site', 'Employee', 'Route', 'Company', 'Product', 'Service', 'User', 'Order', 'Payment', 'Package', 'Promotion', 'Represents', 'Ordered'"
    ] as const;
    return allowedValues.includes(value as any);
}

export const Table = {
  deserialize: tableDeserialize,
  deserializeWithContext: tableDeserializeWithContext,
  is: tableIs
} as const;


export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;

export function itemDefaultValue#0#0(): Item {
    return recordLinkDefaultValue<Service>();
}

export function itemSerialize#0#0(value: Item): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(itemSerializeWithContext(value, ctx));
}
export function itemSerializeWithContext#0#0(value: Item, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function itemDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Item } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = itemDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Item.deserialize: root cannot be a forward reference"
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
export function itemDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Item | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Item | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"RecordLink<Product>"}`) {
            return recordLinkDeserializeWithContext(value, ctx) as Item;
        }
        if (__typeName === `${"RecordLink<Service>"}`) {
            return recordLinkDeserializeWithContext(value, ctx) as Item;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"Item"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function itemIs(value: unknown): value is Item {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "RecordLink<Product>" || __typeName === "RecordLink<Service>"') return true;
    }
    return false;
}
     }

export const Item = {
  deserialize: itemDeserialize,
  deserializeWithContext: itemDeserializeWithContext,
  is: itemIs
} as const;


export type RecordLink<T> = /** @default */ string | T;

export function recordLinkDefaultValue#0<T>#0(): RecordLink<T> {
    return "";
}

export function recordLinkSerialize#0<T>#0(value: RecordLink<T>): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(recordLinkSerializeWithContext(value, ctx));
}
export function recordLinkSerializeWithContext#0<T>#0(value: RecordLink<T>, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function recordLinkDeserialize<T>(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: RecordLink<T> } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = recordLinkDeserializeWithContext<T>(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "RecordLink.deserialize: root cannot be a forward reference"
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
export function recordLinkDeserializeWithContext<T>(value: any, ctx: __mf_DeserializeContext): RecordLink<T> | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RecordLink<T> | __mf_PendingRef;
    }
    if (typeof value === `${"string"}`) {
        return value as RecordLink<T>;
    }
    return value as RecordLink<T>;
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"RecordLink"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function recordLinkIs<T>(value: unknown): value is RecordLink<T> {
    if (typeof value === `${"string"}`) return true;
    return true;
}


export type Actor = /** @default */ User | Employee | Account;

export function actorDefaultValue#0#0(): Actor {
    return userDefaultValue();
}

export function actorSerialize#0#0(value: Actor): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(actorSerializeWithContext(value, ctx));
}
export function actorSerializeWithContext#0#0(value: Actor, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function actorDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Actor } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = actorDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Actor.deserialize: root cannot be a forward reference"
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
export function actorDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Actor | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Actor | __mf_PendingRef;
    }
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if (typeof __typeName === "string") {}
        if (__typeName === `${"User"}`) {
            return userDeserializeWithContext(value, ctx) as Actor;
        }
        if (__typeName === `${"Employee"}`) {
            return employeeDeserializeWithContext(value, ctx) as Actor;
        }
        if (__typeName === `${"Account"}`) {
            return accountDeserializeWithContext(value, ctx) as Actor;
        }
    }
    throw new __mf_DeserializeError([
        {
            field: "_root",
            message: `${"Actor"}.deserializeWithContext: value does not match any union member`
        }
    ]);
}
export function actorIs(value: unknown): value is Actor {
    if (typeof value === "object" && value !== null) {
        const __typeName = (value as any).__type;
        if ('__typeName === "User" || __typeName === "Employee" || __typeName === "Account"') return true;
    }
    return false;
}
     }

export const Actor = {
  deserialize: actorDeserialize,
  deserializeWithContext: actorDeserializeWithContext,
  is: actorIs
} as const;