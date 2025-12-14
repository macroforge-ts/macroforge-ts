import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
/**
 * Validator form model for E2E testing in Svelte.
 * Tests string, number, array, and date validators with real form validation.
 */

export class UserRegistrationForm {
    email: string;

    password: string;

    username: string;

    age: number;

    website: string;

    constructor(props: {
        email: string;
        password: string;
        username: string;
        age: number;
        website: string;
    }) {
        this.email = props.email;
        this.password = props.password;
        this.username = props.username;
        this.age = props.age;
        this.website = props.website;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        UserRegistrationForm,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return UserRegistrationForm.fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<
        UserRegistrationForm,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = UserRegistrationForm.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message:
                            'UserRegistrationForm.fromObject: root cannot be a forward reference'
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
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static __deserialize(value: any, ctx: DeserializeContext): UserRegistrationForm | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'UserRegistrationForm.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('email' in obj)) {
            errors.push({
                field: 'email',
                message: 'missing required field'
            });
        }
        if (!('password' in obj)) {
            errors.push({
                field: 'password',
                message: 'missing required field'
            });
        }
        if (!('username' in obj)) {
            errors.push({
                field: 'username',
                message: 'missing required field'
            });
        }
        if (!('age' in obj)) {
            errors.push({
                field: 'age',
                message: 'missing required field'
            });
        }
        if (!('website' in obj)) {
            errors.push({
                field: 'website',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(UserRegistrationForm.prototype) as UserRegistrationForm;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_email = obj['email'] as string;
            if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_email)) {
                errors.push({
                    field: 'email',
                    message: 'must be a valid email'
                });
            }
            instance.email = __raw_email;
        }
        {
            const __raw_password = obj['password'] as string;
            if (__raw_password.length < 8) {
                errors.push({
                    field: 'password',
                    message: 'must have at least 8 characters'
                });
            }
            if (__raw_password.length > 50) {
                errors.push({
                    field: 'password',
                    message: 'must have at most 50 characters'
                });
            }
            instance.password = __raw_password;
        }
        {
            const __raw_username = obj['username'] as string;
            if (__raw_username.length < 3) {
                errors.push({
                    field: 'username',
                    message: 'must have at least 3 characters'
                });
            }
            if (__raw_username.length > 20) {
                errors.push({
                    field: 'username',
                    message: 'must have at most 20 characters'
                });
            }
            if (__raw_username !== __raw_username.toLowerCase()) {
                errors.push({
                    field: 'username',
                    message: 'must be lowercase'
                });
            }
            if (!/^[a-z][a-z0-9_]+$/.test(__raw_username)) {
                errors.push({
                    field: 'username',
                    message: 'must match the required pattern'
                });
            }
            instance.username = __raw_username;
        }
        {
            const __raw_age = obj['age'] as number;
            if (!Number.isInteger(__raw_age)) {
                errors.push({
                    field: 'age',
                    message: 'must be an integer'
                });
            }
            if (__raw_age < 18 || __raw_age > 120) {
                errors.push({
                    field: 'age',
                    message: 'must be between 18 and 120'
                });
            }
            instance.age = __raw_age;
        }
        {
            const __raw_website = obj['website'] as string;
            if (
                (() => {
                    try {
                        new URL(__raw_website);
                        return false;
                    } catch {
                        return true;
                    }
                })()
            ) {
                errors.push({
                    field: 'website',
                    message: 'must be a valid URL'
                });
            }
            instance.website = __raw_website;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof UserRegistrationForm>(
        field: K,
        value: UserRegistrationForm[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'email': {
                const __val = value as string;
                if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
                    errors.push({
                        field: 'email',
                        message: 'must be a valid email'
                    });
                }
                break;
            }
            case 'password': {
                const __val = value as string;
                if (__val.length < 8) {
                    errors.push({
                        field: 'password',
                        message: 'must have at least 8 characters'
                    });
                }
                if (__val.length > 50) {
                    errors.push({
                        field: 'password',
                        message: 'must have at most 50 characters'
                    });
                }
                break;
            }
            case 'username': {
                const __val = value as string;
                if (__val.length < 3) {
                    errors.push({
                        field: 'username',
                        message: 'must have at least 3 characters'
                    });
                }
                if (__val.length > 20) {
                    errors.push({
                        field: 'username',
                        message: 'must have at most 20 characters'
                    });
                }
                if (__val !== __val.toLowerCase()) {
                    errors.push({
                        field: 'username',
                        message: 'must be lowercase'
                    });
                }
                if (!/^[a-z][a-z0-9_]+$/.test(__val)) {
                    errors.push({
                        field: 'username',
                        message: 'must match the required pattern'
                    });
                }
                break;
            }
            case 'age': {
                const __val = value as number;
                if (!Number.isInteger(__val)) {
                    errors.push({
                        field: 'age',
                        message: 'must be an integer'
                    });
                }
                if (__val < 18 || __val > 120) {
                    errors.push({
                        field: 'age',
                        message: 'must be between 18 and 120'
                    });
                }
                break;
            }
            case 'website': {
                const __val = value as string;
                if (
                    (() => {
                        try {
                            new URL(__val);
                            return false;
                        } catch {
                            return true;
                        }
                    })()
                ) {
                    errors.push({
                        field: 'website',
                        message: 'must be a valid URL'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<UserRegistrationForm>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('email' in partial && partial.email !== undefined) {
            const __val = partial.email as string;
            if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
                errors.push({
                    field: 'email',
                    message: 'must be a valid email'
                });
            }
        }
        if ('password' in partial && partial.password !== undefined) {
            const __val = partial.password as string;
            if (__val.length < 8) {
                errors.push({
                    field: 'password',
                    message: 'must have at least 8 characters'
                });
            }
            if (__val.length > 50) {
                errors.push({
                    field: 'password',
                    message: 'must have at most 50 characters'
                });
            }
        }
        if ('username' in partial && partial.username !== undefined) {
            const __val = partial.username as string;
            if (__val.length < 3) {
                errors.push({
                    field: 'username',
                    message: 'must have at least 3 characters'
                });
            }
            if (__val.length > 20) {
                errors.push({
                    field: 'username',
                    message: 'must have at most 20 characters'
                });
            }
            if (__val !== __val.toLowerCase()) {
                errors.push({
                    field: 'username',
                    message: 'must be lowercase'
                });
            }
            if (!/^[a-z][a-z0-9_]+$/.test(__val)) {
                errors.push({
                    field: 'username',
                    message: 'must match the required pattern'
                });
            }
        }
        if ('age' in partial && partial.age !== undefined) {
            const __val = partial.age as number;
            if (!Number.isInteger(__val)) {
                errors.push({
                    field: 'age',
                    message: 'must be an integer'
                });
            }
            if (__val < 18 || __val > 120) {
                errors.push({
                    field: 'age',
                    message: 'must be between 18 and 120'
                });
            }
        }
        if ('website' in partial && partial.website !== undefined) {
            const __val = partial.website as string;
            if (
                (() => {
                    try {
                        new URL(__val);
                        return false;
                    } catch {
                        return true;
                    }
                })()
            ) {
                errors.push({
                    field: 'website',
                    message: 'must be a valid URL'
                });
            }
        }
        return errors;
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'email' in o && 'password' in o && 'username' in o && 'age' in o && 'website' in o;
    }

    static is(obj: unknown): obj is UserRegistrationForm {
        if (obj instanceof UserRegistrationForm) {
            return true;
        }
        if (!UserRegistrationForm.hasShape(obj)) {
            return false;
        }
        const result = UserRegistrationForm.fromObject(obj);
        return Result.isOk(result);
    }
}

export class ProductForm {
    name: string;

    price: number;

    quantity: number;

    tags: string[];

    sku: string;

    constructor(props: {
        name: string;
        price: number;
        quantity: number;
        tags: string[];
        sku: string;
    }) {
        this.name = props.name;
        this.price = props.price;
        this.quantity = props.quantity;
        this.tags = props.tags;
        this.sku = props.sku;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        ProductForm,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return ProductForm.fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<
        ProductForm,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = ProductForm.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'ProductForm.fromObject: root cannot be a forward reference'
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
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static __deserialize(value: any, ctx: DeserializeContext): ProductForm | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'ProductForm.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('name' in obj)) {
            errors.push({
                field: 'name',
                message: 'missing required field'
            });
        }
        if (!('price' in obj)) {
            errors.push({
                field: 'price',
                message: 'missing required field'
            });
        }
        if (!('quantity' in obj)) {
            errors.push({
                field: 'quantity',
                message: 'missing required field'
            });
        }
        if (!('tags' in obj)) {
            errors.push({
                field: 'tags',
                message: 'missing required field'
            });
        }
        if (!('sku' in obj)) {
            errors.push({
                field: 'sku',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(ProductForm.prototype) as ProductForm;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_name = obj['name'] as string;
            if (__raw_name.length === 0) {
                errors.push({
                    field: 'name',
                    message: 'must not be empty'
                });
            }
            if (__raw_name.length > 100) {
                errors.push({
                    field: 'name',
                    message: 'must have at most 100 characters'
                });
            }
            instance.name = __raw_name;
        }
        {
            const __raw_price = obj['price'] as number;
            if (__raw_price <= 0) {
                errors.push({
                    field: 'price',
                    message: 'must be positive'
                });
            }
            if (__raw_price >= 1000000) {
                errors.push({
                    field: 'price',
                    message: 'must be less than 1000000'
                });
            }
            instance.price = __raw_price;
        }
        {
            const __raw_quantity = obj['quantity'] as number;
            if (!Number.isInteger(__raw_quantity)) {
                errors.push({
                    field: 'quantity',
                    message: 'must be an integer'
                });
            }
            if (__raw_quantity < 0) {
                errors.push({
                    field: 'quantity',
                    message: 'must be non-negative'
                });
            }
            instance.quantity = __raw_quantity;
        }
        {
            const __raw_tags = obj['tags'] as string[];
            if (Array.isArray(__raw_tags)) {
                if (__raw_tags.length < 1) {
                    errors.push({
                        field: 'tags',
                        message: 'must have at least 1 items'
                    });
                }
                if (__raw_tags.length > 5) {
                    errors.push({
                        field: 'tags',
                        message: 'must have at most 5 items'
                    });
                }
                instance.tags = __raw_tags as string[];
            }
        }
        {
            const __raw_sku = obj['sku'] as string;
            if (
                !/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(
                    __raw_sku
                )
            ) {
                errors.push({
                    field: 'sku',
                    message: 'must be a valid UUID'
                });
            }
            instance.sku = __raw_sku;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof ProductForm>(
        field: K,
        value: ProductForm[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'name': {
                const __val = value as string;
                if (__val.length === 0) {
                    errors.push({
                        field: 'name',
                        message: 'must not be empty'
                    });
                }
                if (__val.length > 100) {
                    errors.push({
                        field: 'name',
                        message: 'must have at most 100 characters'
                    });
                }
                break;
            }
            case 'price': {
                const __val = value as number;
                if (__val <= 0) {
                    errors.push({
                        field: 'price',
                        message: 'must be positive'
                    });
                }
                if (__val >= 1000000) {
                    errors.push({
                        field: 'price',
                        message: 'must be less than 1000000'
                    });
                }
                break;
            }
            case 'quantity': {
                const __val = value as number;
                if (!Number.isInteger(__val)) {
                    errors.push({
                        field: 'quantity',
                        message: 'must be an integer'
                    });
                }
                if (__val < 0) {
                    errors.push({
                        field: 'quantity',
                        message: 'must be non-negative'
                    });
                }
                break;
            }
            case 'tags': {
                const __val = value as string[];
                if (__val.length < 1) {
                    errors.push({
                        field: 'tags',
                        message: 'must have at least 1 items'
                    });
                }
                if (__val.length > 5) {
                    errors.push({
                        field: 'tags',
                        message: 'must have at most 5 items'
                    });
                }
                break;
            }
            case 'sku': {
                const __val = value as string;
                if (
                    !/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(
                        __val
                    )
                ) {
                    errors.push({
                        field: 'sku',
                        message: 'must be a valid UUID'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<ProductForm>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('name' in partial && partial.name !== undefined) {
            const __val = partial.name as string;
            if (__val.length === 0) {
                errors.push({
                    field: 'name',
                    message: 'must not be empty'
                });
            }
            if (__val.length > 100) {
                errors.push({
                    field: 'name',
                    message: 'must have at most 100 characters'
                });
            }
        }
        if ('price' in partial && partial.price !== undefined) {
            const __val = partial.price as number;
            if (__val <= 0) {
                errors.push({
                    field: 'price',
                    message: 'must be positive'
                });
            }
            if (__val >= 1000000) {
                errors.push({
                    field: 'price',
                    message: 'must be less than 1000000'
                });
            }
        }
        if ('quantity' in partial && partial.quantity !== undefined) {
            const __val = partial.quantity as number;
            if (!Number.isInteger(__val)) {
                errors.push({
                    field: 'quantity',
                    message: 'must be an integer'
                });
            }
            if (__val < 0) {
                errors.push({
                    field: 'quantity',
                    message: 'must be non-negative'
                });
            }
        }
        if ('tags' in partial && partial.tags !== undefined) {
            const __val = partial.tags as string[];
            if (__val.length < 1) {
                errors.push({
                    field: 'tags',
                    message: 'must have at least 1 items'
                });
            }
            if (__val.length > 5) {
                errors.push({
                    field: 'tags',
                    message: 'must have at most 5 items'
                });
            }
        }
        if ('sku' in partial && partial.sku !== undefined) {
            const __val = partial.sku as string;
            if (
                !/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(
                    __val
                )
            ) {
                errors.push({
                    field: 'sku',
                    message: 'must be a valid UUID'
                });
            }
        }
        return errors;
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'name' in o && 'price' in o && 'quantity' in o && 'tags' in o && 'sku' in o;
    }

    static is(obj: unknown): obj is ProductForm {
        if (obj instanceof ProductForm) {
            return true;
        }
        if (!ProductForm.hasShape(obj)) {
            return false;
        }
        const result = ProductForm.fromObject(obj);
        return Result.isOk(result);
    }
}

export class EventForm {
    title: string;

    startDate: Date;

    endDate: Date;

    maxAttendees: number;

    constructor(props: {
        title: string;
        startDate: Date;
        endDate: Date;
        maxAttendees: number;
    }) {
        this.title = props.title;
        this.startDate = props.startDate;
        this.endDate = props.endDate;
        this.maxAttendees = props.maxAttendees;
    }

    static fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<
        EventForm,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const raw = JSON.parse(json);
            return EventForm.fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<
        EventForm,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = EventForm.__deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'EventForm.fromObject: root cannot be a forward reference'
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
            return Result.err([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }

    static __deserialize(value: any, ctx: DeserializeContext): EventForm | PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new DeserializeError([
                {
                    field: '_root',
                    message: 'EventForm.__deserialize: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('title' in obj)) {
            errors.push({
                field: 'title',
                message: 'missing required field'
            });
        }
        if (!('startDate' in obj)) {
            errors.push({
                field: 'startDate',
                message: 'missing required field'
            });
        }
        if (!('endDate' in obj)) {
            errors.push({
                field: 'endDate',
                message: 'missing required field'
            });
        }
        if (!('maxAttendees' in obj)) {
            errors.push({
                field: 'maxAttendees',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        const instance = Object.create(EventForm.prototype) as EventForm;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_title = obj['title'] as string;
            if (__raw_title.length === 0) {
                errors.push({
                    field: 'title',
                    message: 'must not be empty'
                });
            }
            if (__raw_title !== __raw_title.trim()) {
                errors.push({
                    field: 'title',
                    message: 'must be trimmed (no leading/trailing whitespace)'
                });
            }
            instance.title = __raw_title;
        }
        {
            const __raw_startDate = obj['startDate'] as Date;
            {
                const __dateVal =
                    typeof __raw_startDate === 'string'
                        ? new Date(__raw_startDate)
                        : (__raw_startDate as Date);
                if (__dateVal == null || isNaN(__dateVal.getTime())) {
                    errors.push({
                        field: 'startDate',
                        message: 'must be a valid date'
                    });
                }
                if (__dateVal == null || __dateVal.getTime() <= new Date('2020-01-01').getTime()) {
                    errors.push({
                        field: 'startDate',
                        message: 'must be after 2020-01-01'
                    });
                }
                instance.startDate = __dateVal;
            }
        }
        {
            const __raw_endDate = obj['endDate'] as Date;
            {
                const __dateVal =
                    typeof __raw_endDate === 'string'
                        ? new Date(__raw_endDate)
                        : (__raw_endDate as Date);
                if (__dateVal == null || isNaN(__dateVal.getTime())) {
                    errors.push({
                        field: 'endDate',
                        message: 'must be a valid date'
                    });
                }
                instance.endDate = __dateVal;
            }
        }
        {
            const __raw_maxAttendees = obj['maxAttendees'] as number;
            if (!Number.isInteger(__raw_maxAttendees)) {
                errors.push({
                    field: 'maxAttendees',
                    message: 'must be an integer'
                });
            }
            if (__raw_maxAttendees < 1 || __raw_maxAttendees > 1000) {
                errors.push({
                    field: 'maxAttendees',
                    message: 'must be between 1 and 1000'
                });
            }
            instance.maxAttendees = __raw_maxAttendees;
        }
        if (errors.length > 0) {
            throw new DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof EventForm>(
        field: K,
        value: EventForm[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'title': {
                const __val = value as string;
                if (__val.length === 0) {
                    errors.push({
                        field: 'title',
                        message: 'must not be empty'
                    });
                }
                if (__val !== __val.trim()) {
                    errors.push({
                        field: 'title',
                        message: 'must be trimmed (no leading/trailing whitespace)'
                    });
                }
                break;
            }
            case 'startDate': {
                const __val = value as Date;
                if (__val == null || isNaN(__val.getTime())) {
                    errors.push({
                        field: 'startDate',
                        message: 'must be a valid date'
                    });
                }
                if (__val == null || __val.getTime() <= new Date('2020-01-01').getTime()) {
                    errors.push({
                        field: 'startDate',
                        message: 'must be after 2020-01-01'
                    });
                }
                break;
            }
            case 'endDate': {
                const __val = value as Date;
                if (__val == null || isNaN(__val.getTime())) {
                    errors.push({
                        field: 'endDate',
                        message: 'must be a valid date'
                    });
                }
                break;
            }
            case 'maxAttendees': {
                const __val = value as number;
                if (!Number.isInteger(__val)) {
                    errors.push({
                        field: 'maxAttendees',
                        message: 'must be an integer'
                    });
                }
                if (__val < 1 || __val > 1000) {
                    errors.push({
                        field: 'maxAttendees',
                        message: 'must be between 1 and 1000'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<EventForm>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('title' in partial && partial.title !== undefined) {
            const __val = partial.title as string;
            if (__val.length === 0) {
                errors.push({
                    field: 'title',
                    message: 'must not be empty'
                });
            }
            if (__val !== __val.trim()) {
                errors.push({
                    field: 'title',
                    message: 'must be trimmed (no leading/trailing whitespace)'
                });
            }
        }
        if ('startDate' in partial && partial.startDate !== undefined) {
            const __val = partial.startDate as Date;
            if (__val == null || isNaN(__val.getTime())) {
                errors.push({
                    field: 'startDate',
                    message: 'must be a valid date'
                });
            }
            if (__val == null || __val.getTime() <= new Date('2020-01-01').getTime()) {
                errors.push({
                    field: 'startDate',
                    message: 'must be after 2020-01-01'
                });
            }
        }
        if ('endDate' in partial && partial.endDate !== undefined) {
            const __val = partial.endDate as Date;
            if (__val == null || isNaN(__val.getTime())) {
                errors.push({
                    field: 'endDate',
                    message: 'must be a valid date'
                });
            }
        }
        if ('maxAttendees' in partial && partial.maxAttendees !== undefined) {
            const __val = partial.maxAttendees as number;
            if (!Number.isInteger(__val)) {
                errors.push({
                    field: 'maxAttendees',
                    message: 'must be an integer'
                });
            }
            if (__val < 1 || __val > 1000) {
                errors.push({
                    field: 'maxAttendees',
                    message: 'must be between 1 and 1000'
                });
            }
        }
        return errors;
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'title' in o && 'startDate' in o && 'endDate' in o && 'maxAttendees' in o;
    }

    static is(obj: unknown): obj is EventForm {
        if (obj instanceof EventForm) {
            return true;
        }
        if (!EventForm.hasShape(obj)) {
            return false;
        }
        const result = EventForm.fromObject(obj);
        return Result.isOk(result);
    }
}

// Type for validation result
export type ValidationResult<T> = {
    success: boolean;
    data?: T;
    errors?: string[];
};

// Helper to convert Result to ValidationResult
// The Result type is provided by the macro expansion
export function toValidationResult<T>(result: any): ValidationResult<T> {
    if (result.isOk()) {
        return { success: true, data: result.unwrap() };
    } else {
        return { success: false, errors: result.unwrapErr() };
    }
}

// Form validation functions
export function validateUserRegistration(data: unknown): ValidationResult<UserRegistrationForm> {
    const result = UserRegistrationForm.fromStringifiedJSON(JSON.stringify(data));
    return toValidationResult(result);
}

export function validateProduct(data: unknown): ValidationResult<ProductForm> {
    const result = ProductForm.fromStringifiedJSON(JSON.stringify(data));
    return toValidationResult(result);
}

export function validateEvent(data: unknown): ValidationResult<EventForm> {
    const result = EventForm.fromStringifiedJSON(JSON.stringify(data));
    return toValidationResult(result);
}
