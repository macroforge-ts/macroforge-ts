import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
/**
 * Validator form model for E2E testing.
 * Tests string, number, array, and date validators with real form validation.
 */


export class UserRegistrationForm {
    
    email: string;

    
    password: string;

    
    username: string;

    
    age: number;

    
    website: string;

    constructor(props: Record<string, unknown>){}

    static deserialize(input: unknown, opts: __mf_DeserializeOptions): {
    success: true;
    value: UserRegistrationForm;
} | {
    success: false;
    errors: Array<{
        field: string;
        message: string;
    }>;
} {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = UserRegistrationForm.deserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "UserRegistrationForm.deserialize: root cannot be a forward reference"
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

    static deserializeWithContext(value: any, ctx: __mf_DeserializeContext): UserRegistrationForm | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"UserRegistrationForm"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
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
    if (!(`${"username"}` in obj)) {
        errors.push({
            field: `${"username"}`,
            message: "missing required field"
        });
    }
    if (!(`${"age"}` in obj)) {
        errors.push({
            field: `${"age"}`,
            message: "missing required field"
        });
    }
    if (!(`${"website"}` in obj)) {
        errors.push({
            field: `${"website"}`,
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance = Object.create(UserRegistrationForm.prototype) as UserRegistrationForm;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_email = obj[`${"email"}`] as string;
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_email)) {
            errors.push({
                field: "email",
                message: "UserRegistrationForm.email must be a valid email"
            });
        }
        instance.email = __raw_email;
    }
    {
        const __raw_password = obj[`${"password"}`] as string;
        if (__raw_password.length < 8) {
            errors.push({
                field: "password",
                message: "UserRegistrationForm.password must have at least 8 characters"
            });
        }
        if (__raw_password.length > 50) {
            errors.push({
                field: "password",
                message: "UserRegistrationForm.password must have at most 50 characters"
            });
        }
        instance.password = __raw_password;
    }
    {
        const __raw_username = obj[`${"username"}`] as string;
        if (__raw_username.length < 3) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must have at least 3 characters"
            });
        }
        if (__raw_username.length > 20) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must have at most 20 characters"
            });
        }
        if (__raw_username !== __raw_username.toLowerCase()) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must be lowercase"
            });
        }
        if (!/^[a-z][a-z0-9_]+$/.test(__raw_username)) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must match pattern ^[a-z][a-z0-9_]+$"
            });
        }
        instance.username = __raw_username;
    }
    {
        const __raw_age = obj[`${"age"}`] as number;
        if (!Number.isInteger(__raw_age)) {
            errors.push({
                field: "age",
                message: "UserRegistrationForm.age must be an integer"
            });
        }
        if (__raw_age < 18 || __raw_age > 120) {
            errors.push({
                field: "age",
                message: "UserRegistrationForm.age must be between 18 and 120"
            });
        }
        instance.age = __raw_age;
    }
    {
        const __raw_website = obj[`${"website"}`] as string;
        if (!/^https?:\/\/.+/.test(__raw_website)) {
            errors.push({
                field: "website",
                message: "UserRegistrationForm.website must be a valid URL"
            });
        }
        instance.website = __raw_website;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof UserRegistrationForm>(_field: K, _value: UserRegistrationForm[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"email"}`) {
        const __val = _value as string;
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
            errors.push({
                field: "email",
                message: "UserRegistrationForm.email must be a valid email"
            });
        }
    }
    if (_field === `${"password"}`) {
        const __val = _value as string;
        if (__val.length < 8) {
            errors.push({
                field: "password",
                message: "UserRegistrationForm.password must have at least 8 characters"
            });
        }
        if (__val.length > 50) {
            errors.push({
                field: "password",
                message: "UserRegistrationForm.password must have at most 50 characters"
            });
        }
    }
    if (_field === `${"username"}`) {
        const __val = _value as string;
        if (__val.length < 3) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must have at least 3 characters"
            });
        }
        if (__val.length > 20) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must have at most 20 characters"
            });
        }
        if (__val !== __val.toLowerCase()) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must be lowercase"
            });
        }
        if (!/^[a-z][a-z0-9_]+$/.test(__val)) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must match pattern ^[a-z][a-z0-9_]+$"
            });
        }
    }
    if (_field === `${"age"}`) {
        const __val = _value as number;
        if (!Number.isInteger(__val)) {
            errors.push({
                field: "age",
                message: "UserRegistrationForm.age must be an integer"
            });
        }
        if (__val < 18 || __val > 120) {
            errors.push({
                field: "age",
                message: "UserRegistrationForm.age must be between 18 and 120"
            });
        }
    }
    if (_field === `${"website"}`) {
        const __val = _value as string;
        if (!/^https?:\/\/.+/.test(__val)) {
            errors.push({
                field: "website",
                message: "UserRegistrationForm.website must be a valid URL"
            });
        }
    }
    return errors;
}

    static validateFields(_partial: Partial<UserRegistrationForm>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"email"}` in _partial && _partial.email !== undefined) {
        const __val = _partial.email as string;
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
            errors.push({
                field: "email",
                message: "UserRegistrationForm.email must be a valid email"
            });
        }
    }
    if (`${"password"}` in _partial && _partial.password !== undefined) {
        const __val = _partial.password as string;
        if (__val.length < 8) {
            errors.push({
                field: "password",
                message: "UserRegistrationForm.password must have at least 8 characters"
            });
        }
        if (__val.length > 50) {
            errors.push({
                field: "password",
                message: "UserRegistrationForm.password must have at most 50 characters"
            });
        }
    }
    if (`${"username"}` in _partial && _partial.username !== undefined) {
        const __val = _partial.username as string;
        if (__val.length < 3) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must have at least 3 characters"
            });
        }
        if (__val.length > 20) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must have at most 20 characters"
            });
        }
        if (__val !== __val.toLowerCase()) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must be lowercase"
            });
        }
        if (!/^[a-z][a-z0-9_]+$/.test(__val)) {
            errors.push({
                field: "username",
                message: "UserRegistrationForm.username must match pattern ^[a-z][a-z0-9_]+$"
            });
        }
    }
    if (`${"age"}` in _partial && _partial.age !== undefined) {
        const __val = _partial.age as number;
        if (!Number.isInteger(__val)) {
            errors.push({
                field: "age",
                message: "UserRegistrationForm.age must be an integer"
            });
        }
        if (__val < 18 || __val > 120) {
            errors.push({
                field: "age",
                message: "UserRegistrationForm.age must be between 18 and 120"
            });
        }
    }
    if (`${"website"}` in _partial && _partial.website !== undefined) {
        const __val = _partial.website as string;
        if (!/^https?:\/\/.+/.test(__val)) {
            errors.push({
                field: "website",
                message: "UserRegistrationForm.website must be a valid URL"
            });
        }
    }
    return errors;
}

    static hasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"email" in o && "password" in o && "username" in o && "age" in o && "website" in o';
}

    static is(obj: unknown): obj is UserRegistrationForm {
    if (obj instanceof UserRegistrationForm) {
        return true;
    }
    if (!UserRegistrationForm.hasShape(obj)) {
        return false;
    }
    const result = UserRegistrationForm.deserialize(obj);
    return result.success;
}
}

export function userRegistrationFormDeserialize(input: unknown, opts: __mf_DeserializeOptions): {
    success: true;
    value: UserRegistrationForm;
} | {
    success: false;
    errors: Array<{
        field: string;
        message: string;
    }>;
} {
    return UserRegistrationForm.deserialize(input, opts);
}
export function userRegistrationFormDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): UserRegistrationForm | __mf_PendingRef {
    return UserRegistrationForm.deserializeWithContext(value, ctx);
}
export function userRegistrationFormIs(value: unknown): value is UserRegistrationForm {
    return UserRegistrationForm.is(value);
}


export class ProductForm {
    
    name: string;

    
    price: number;

    
    quantity: number;

    
    tags: Array<string>;

    
    sku: string;

    constructor(props: Record<string, unknown>){}

    static deserialize(input: unknown, opts: __mf_DeserializeOptions): {
    success: true;
    value: ProductForm;
} | {
    success: false;
    errors: Array<{
        field: string;
        message: string;
    }>;
} {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = ProductForm.deserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ProductForm.deserialize: root cannot be a forward reference"
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

    static deserializeWithContext(value: any, ctx: __mf_DeserializeContext): ProductForm | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"ProductForm"}.deserializeWithContext: expected an object`
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
    if (!(`${"price"}` in obj)) {
        errors.push({
            field: `${"price"}`,
            message: "missing required field"
        });
    }
    if (!(`${"quantity"}` in obj)) {
        errors.push({
            field: `${"quantity"}`,
            message: "missing required field"
        });
    }
    if (!(`${"tags"}` in obj)) {
        errors.push({
            field: `${"tags"}`,
            message: "missing required field"
        });
    }
    if (!(`${"sku"}` in obj)) {
        errors.push({
            field: `${"sku"}`,
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance = Object.create(ProductForm.prototype) as ProductForm;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "ProductForm.name must not be empty"
            });
        }
        if (__raw_name.length > 100) {
            errors.push({
                field: "name",
                message: "ProductForm.name must have at most 100 characters"
            });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_price = obj[`${"price"}`] as number;
        if (__raw_price <= 0) {
            errors.push({
                field: "price",
                message: "ProductForm.price must be positive"
            });
        }
        if (__raw_price >= 1000000) {
            errors.push({
                field: "price",
                message: "ProductForm.price must be less than 1000000"
            });
        }
        instance.price = __raw_price;
    }
    {
        const __raw_quantity = obj[`${"quantity"}`] as number;
        if (!Number.isInteger(__raw_quantity)) {
            errors.push({
                field: "quantity",
                message: "ProductForm.quantity must be an integer"
            });
        }
        if (__raw_quantity < 0) {
            errors.push({
                field: "quantity",
                message: "ProductForm.quantity must be non-negative"
            });
        }
        instance.quantity = __raw_quantity;
    }
    {
        const __raw_tags = obj[`${"tags"}`] as Array<string>;
        if (Array.isArray(__raw_tags)) {
            if (__raw_tags.length < 1) {
                errors.push({
                    field: "tags",
                    message: "ProductForm.tags must have at least 1 items"
                });
            }
            if (__raw_tags.length > 5) {
                errors.push({
                    field: "tags",
                    message: "ProductForm.tags must have at most 5 items"
                });
            }
            instance.tags = __raw_tags as string[];
        }
    }
    {
        const __raw_sku = obj[`${"sku"}`] as string;
        if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(__raw_sku)) {
            errors.push({
                field: "sku",
                message: "ProductForm.sku must be a valid UUID"
            });
        }
        instance.sku = __raw_sku;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof ProductForm>(_field: K, _value: ProductForm[K]): Array<{
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
                message: "ProductForm.name must not be empty"
            });
        }
        if (__val.length > 100) {
            errors.push({
                field: "name",
                message: "ProductForm.name must have at most 100 characters"
            });
        }
    }
    if (_field === `${"price"}`) {
        const __val = _value as number;
        if (__val <= 0) {
            errors.push({
                field: "price",
                message: "ProductForm.price must be positive"
            });
        }
        if (__val >= 1000000) {
            errors.push({
                field: "price",
                message: "ProductForm.price must be less than 1000000"
            });
        }
    }
    if (_field === `${"quantity"}`) {
        const __val = _value as number;
        if (!Number.isInteger(__val)) {
            errors.push({
                field: "quantity",
                message: "ProductForm.quantity must be an integer"
            });
        }
        if (__val < 0) {
            errors.push({
                field: "quantity",
                message: "ProductForm.quantity must be non-negative"
            });
        }
    }
    if (_field === `${"tags"}`) {
        const __val = _value as Array<string>;
        if (__val.length < 1) {
            errors.push({
                field: "tags",
                message: "ProductForm.tags must have at least 1 items"
            });
        }
        if (__val.length > 5) {
            errors.push({
                field: "tags",
                message: "ProductForm.tags must have at most 5 items"
            });
        }
    }
    if (_field === `${"sku"}`) {
        const __val = _value as string;
        if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(__val)) {
            errors.push({
                field: "sku",
                message: "ProductForm.sku must be a valid UUID"
            });
        }
    }
    return errors;
}

    static validateFields(_partial: Partial<ProductForm>): Array<{
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
                message: "ProductForm.name must not be empty"
            });
        }
        if (__val.length > 100) {
            errors.push({
                field: "name",
                message: "ProductForm.name must have at most 100 characters"
            });
        }
    }
    if (`${"price"}` in _partial && _partial.price !== undefined) {
        const __val = _partial.price as number;
        if (__val <= 0) {
            errors.push({
                field: "price",
                message: "ProductForm.price must be positive"
            });
        }
        if (__val >= 1000000) {
            errors.push({
                field: "price",
                message: "ProductForm.price must be less than 1000000"
            });
        }
    }
    if (`${"quantity"}` in _partial && _partial.quantity !== undefined) {
        const __val = _partial.quantity as number;
        if (!Number.isInteger(__val)) {
            errors.push({
                field: "quantity",
                message: "ProductForm.quantity must be an integer"
            });
        }
        if (__val < 0) {
            errors.push({
                field: "quantity",
                message: "ProductForm.quantity must be non-negative"
            });
        }
    }
    if (`${"tags"}` in _partial && _partial.tags !== undefined) {
        const __val = _partial.tags as Array<string>;
        if (__val.length < 1) {
            errors.push({
                field: "tags",
                message: "ProductForm.tags must have at least 1 items"
            });
        }
        if (__val.length > 5) {
            errors.push({
                field: "tags",
                message: "ProductForm.tags must have at most 5 items"
            });
        }
    }
    if (`${"sku"}` in _partial && _partial.sku !== undefined) {
        const __val = _partial.sku as string;
        if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(__val)) {
            errors.push({
                field: "sku",
                message: "ProductForm.sku must be a valid UUID"
            });
        }
    }
    return errors;
}

    static hasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"name" in o && "price" in o && "quantity" in o && "tags" in o && "sku" in o';
}

    static is(obj: unknown): obj is ProductForm {
    if (obj instanceof ProductForm) {
        return true;
    }
    if (!ProductForm.hasShape(obj)) {
        return false;
    }
    const result = ProductForm.deserialize(obj);
    return result.success;
}
}

export function productFormDeserialize(input: unknown, opts: __mf_DeserializeOptions): {
    success: true;
    value: ProductForm;
} | {
    success: false;
    errors: Array<{
        field: string;
        message: string;
    }>;
} {
    return ProductForm.deserialize(input, opts);
}
export function productFormDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ProductForm | __mf_PendingRef {
    return ProductForm.deserializeWithContext(value, ctx);
}
export function productFormIs(value: unknown): value is ProductForm {
    return ProductForm.is(value);
}


export class EventForm {
    
    title: string;

    
    startDate: Date;

    
    endDate: Date;

    
    maxAttendees: number;

    constructor(props: Record<string, unknown>){}

    static deserialize(input: unknown, opts: __mf_DeserializeOptions): {
    success: true;
    value: EventForm;
} | {
    success: false;
    errors: Array<{
        field: string;
        message: string;
    }>;
} {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = EventForm.deserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "EventForm.deserialize: root cannot be a forward reference"
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

    static deserializeWithContext(value: any, ctx: __mf_DeserializeContext): EventForm | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"EventForm"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"title"}` in obj)) {
        errors.push({
            field: `${"title"}`,
            message: "missing required field"
        });
    }
    if (!(`${"startDate"}` in obj)) {
        errors.push({
            field: `${"startDate"}`,
            message: "missing required field"
        });
    }
    if (!(`${"endDate"}` in obj)) {
        errors.push({
            field: `${"endDate"}`,
            message: "missing required field"
        });
    }
    if (!(`${"maxAttendees"}` in obj)) {
        errors.push({
            field: `${"maxAttendees"}`,
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance = Object.create(EventForm.prototype) as EventForm;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_title = obj[`${"title"}`] as string;
        if (__raw_title.trim().length === 0) {
            errors.push({
                field: "title",
                message: "EventForm.title must not be empty"
            });
        }
        if (__raw_title !== __raw_title.trim()) {
            errors.push({
                field: "title",
                message: "EventForm.title must be trimmed"
            });
        }
        instance.title = __raw_title;
    }
    {
        const __raw_startDate = obj[`${"startDate"}`] as Date;
        {
            const __dateVal = typeof __raw_startDate === "string" ? new Date(__raw_startDate) : $MfPh116 as Date;
            if (isNaN(__dateVal.getTime())) {
                errors.push({
                    field: "startDate",
                    message: "EventForm.startDate must be a valid date"
                });
            }
            if (__dateVal.getTime() <= new Date("2020-01-01").getTime()) {
                errors.push({
                    field: "startDate",
                    message: "EventForm.startDate must be after 2020-01-01"
                });
            }
            instance.startDate = __dateVal;
        }
    }
    {
        const __raw_endDate = obj[`${"endDate"}`] as Date;
        {
            const __dateVal = typeof __raw_endDate === "string" ? new Date(__raw_endDate) : $MfPh116 as Date;
            if (isNaN(__dateVal.getTime())) {
                errors.push({
                    field: "endDate",
                    message: "EventForm.endDate must be a valid date"
                });
            }
            instance.endDate = __dateVal;
        }
    }
    {
        const __raw_maxAttendees = obj[`${"maxAttendees"}`] as number;
        if (!Number.isInteger(__raw_maxAttendees)) {
            errors.push({
                field: "maxAttendees",
                message: "EventForm.maxAttendees must be an integer"
            });
        }
        if (__raw_maxAttendees < 1 || __raw_maxAttendees > 1000) {
            errors.push({
                field: "maxAttendees",
                message: "EventForm.maxAttendees must be between 1 and 1000"
            });
        }
        instance.maxAttendees = __raw_maxAttendees;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof EventForm>(_field: K, _value: EventForm[K]): Array<{
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
                message: "EventForm.title must not be empty"
            });
        }
        if (__val !== __val.trim()) {
            errors.push({
                field: "title",
                message: "EventForm.title must be trimmed"
            });
        }
    }
    if (_field === `${"startDate"}`) {
        const __val = _value as Date;
        if (isNaN(__val.getTime())) {
            errors.push({
                field: "startDate",
                message: "EventForm.startDate must be a valid date"
            });
        }
        if (__val.getTime() <= new Date("2020-01-01").getTime()) {
            errors.push({
                field: "startDate",
                message: "EventForm.startDate must be after 2020-01-01"
            });
        }
    }
    if (_field === `${"endDate"}`) {
        const __val = _value as Date;
        if (isNaN(__val.getTime())) {
            errors.push({
                field: "endDate",
                message: "EventForm.endDate must be a valid date"
            });
        }
    }
    if (_field === `${"maxAttendees"}`) {
        const __val = _value as number;
        if (!Number.isInteger(__val)) {
            errors.push({
                field: "maxAttendees",
                message: "EventForm.maxAttendees must be an integer"
            });
        }
        if (__val < 1 || __val > 1000) {
            errors.push({
                field: "maxAttendees",
                message: "EventForm.maxAttendees must be between 1 and 1000"
            });
        }
    }
    return errors;
}

    static validateFields(_partial: Partial<EventForm>): Array<{
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
                message: "EventForm.title must not be empty"
            });
        }
        if (__val !== __val.trim()) {
            errors.push({
                field: "title",
                message: "EventForm.title must be trimmed"
            });
        }
    }
    if (`${"startDate"}` in _partial && _partial.startDate !== undefined) {
        const __val = _partial.startDate as Date;
        if (isNaN(__val.getTime())) {
            errors.push({
                field: "startDate",
                message: "EventForm.startDate must be a valid date"
            });
        }
        if (__val.getTime() <= new Date("2020-01-01").getTime()) {
            errors.push({
                field: "startDate",
                message: "EventForm.startDate must be after 2020-01-01"
            });
        }
    }
    if (`${"endDate"}` in _partial && _partial.endDate !== undefined) {
        const __val = _partial.endDate as Date;
        if (isNaN(__val.getTime())) {
            errors.push({
                field: "endDate",
                message: "EventForm.endDate must be a valid date"
            });
        }
    }
    if (`${"maxAttendees"}` in _partial && _partial.maxAttendees !== undefined) {
        const __val = _partial.maxAttendees as number;
        if (!Number.isInteger(__val)) {
            errors.push({
                field: "maxAttendees",
                message: "EventForm.maxAttendees must be an integer"
            });
        }
        if (__val < 1 || __val > 1000) {
            errors.push({
                field: "maxAttendees",
                message: "EventForm.maxAttendees must be between 1 and 1000"
            });
        }
    }
    return errors;
}

    static hasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"title" in o && "startDate" in o && "endDate" in o && "maxAttendees" in o';
}

    static is(obj: unknown): obj is EventForm {
    if (obj instanceof EventForm) {
        return true;
    }
    if (!EventForm.hasShape(obj)) {
        return false;
    }
    const result = EventForm.deserialize(obj);
    return result.success;
}
}

export function eventFormDeserialize(input: unknown, opts: __mf_DeserializeOptions): {
    success: true;
    value: EventForm;
} | {
    success: false;
    errors: Array<{
        field: string;
        message: string;
    }>;
} {
    return EventForm.deserialize(input, opts);
}
export function eventFormDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): EventForm | __mf_PendingRef {
    return EventForm.deserializeWithContext(value, ctx);
}
export function eventFormIs(value: unknown): value is EventForm {
    return EventForm.is(value);
}

// Type for validation result (matches macroforge's vanilla return type)
export type ValidationResult<T> =
    | {
          success: true;
          value: T;
      }
    | { success: false; errors: Array<{ field: string; message: string }> };

// Form validation functions
export function validateUserRegistration(data: unknown): ValidationResult<UserRegistrationForm> {
    const result = UserRegistrationForm.deserialize(JSON.stringify(data));
    return result;
}

export function validateProduct(data: unknown): ValidationResult<ProductForm> {
    const result = ProductForm.deserialize(JSON.stringify(data));
    return result;
}

export function validateEvent(data: unknown): ValidationResult<EventForm> {
    const result = EventForm.deserialize(JSON.stringify(data));
    return result;
}