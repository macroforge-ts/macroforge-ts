import { ok as __mf_resultOk } from 'macroforge/reexports';
import { error as __mf_resultErr } from 'macroforge/reexports';
import { isOk as __mf_resultIsOk } from 'macroforge/reexports';
import type { Result as __mf_Result } from 'macroforge/reexports';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
/**
 * String validator test classes for comprehensive deserializer validation testing.
 * Each class tests a single validator for isolation.
 */

// Email validator

export class EmailValidator {
    email: string;

    constructor(props: {
        email: string;
    }) {
        this.email = props.email;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        EmailValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = EmailValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message: 'EmailValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): EmailValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'EmailValidator.deserializeWithContext: expected an object'
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
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(EmailValidator.prototype) as EmailValidator;
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
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof EmailValidator>(
        field: K,
        value: EmailValidator[K]
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
        }
        return errors;
    }

    static validateFields(partial: Partial<EmailValidator>): Array<{
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
        return errors;
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'email' in o;
    }

    static is(obj: unknown): obj is EmailValidator {
        if (obj instanceof EmailValidator) {
            return true;
        }
        if (!EmailValidator.hasShape(obj)) {
            return false;
        }
        const result = EmailValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function emailValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<EmailValidator, Array<{ field: string; message: string }>> {
    return EmailValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function emailValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): EmailValidator | __mf_PendingRef {
    return EmailValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function emailValidatorIs(value: unknown): value is EmailValidator {
    return EmailValidator.is(value);
}

// URL validator

export class UrlValidator {
    url: string;

    constructor(props: {
        url: string;
    }) {
        this.url = props.url;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        UrlValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = UrlValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message: 'UrlValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): UrlValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'UrlValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('url' in obj)) {
            errors.push({
                field: 'url',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(UrlValidator.prototype) as UrlValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_url = obj['url'] as string;
            if (
                (() => {
                    try {
                        new URL(__raw_url);
                        return false;
                    } catch {
                        return true;
                    }
                })()
            ) {
                errors.push({
                    field: 'url',
                    message: 'must be a valid URL'
                });
            }
            instance.url = __raw_url;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof UrlValidator>(
        field: K,
        value: UrlValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'url': {
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
                        field: 'url',
                        message: 'must be a valid URL'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<UrlValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('url' in partial && partial.url !== undefined) {
            const __val = partial.url as string;
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
                    field: 'url',
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
        return 'url' in o;
    }

    static is(obj: unknown): obj is UrlValidator {
        if (obj instanceof UrlValidator) {
            return true;
        }
        if (!UrlValidator.hasShape(obj)) {
            return false;
        }
        const result = UrlValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function urlValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<UrlValidator, Array<{ field: string; message: string }>> {
    return UrlValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function urlValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): UrlValidator | __mf_PendingRef {
    return UrlValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function urlValidatorIs(value: unknown): value is UrlValidator {
    return UrlValidator.is(value);
}

// UUID validator

export class UuidValidator {
    id: string;

    constructor(props: {
        id: string;
    }) {
        this.id = props.id;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        UuidValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = UuidValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message: 'UuidValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): UuidValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'UuidValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('id' in obj)) {
            errors.push({
                field: 'id',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(UuidValidator.prototype) as UuidValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_id = obj['id'] as string;
            if (
                !/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(
                    __raw_id
                )
            ) {
                errors.push({
                    field: 'id',
                    message: 'must be a valid UUID'
                });
            }
            instance.id = __raw_id;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof UuidValidator>(
        field: K,
        value: UuidValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'id': {
                const __val = value as string;
                if (
                    !/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(
                        __val
                    )
                ) {
                    errors.push({
                        field: 'id',
                        message: 'must be a valid UUID'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<UuidValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('id' in partial && partial.id !== undefined) {
            const __val = partial.id as string;
            if (
                !/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(
                    __val
                )
            ) {
                errors.push({
                    field: 'id',
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
        return 'id' in o;
    }

    static is(obj: unknown): obj is UuidValidator {
        if (obj instanceof UuidValidator) {
            return true;
        }
        if (!UuidValidator.hasShape(obj)) {
            return false;
        }
        const result = UuidValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function uuidValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<UuidValidator, Array<{ field: string; message: string }>> {
    return UuidValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function uuidValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): UuidValidator | __mf_PendingRef {
    return UuidValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function uuidValidatorIs(value: unknown): value is UuidValidator {
    return UuidValidator.is(value);
}

// MaxLength validator

export class MaxLengthValidator {
    shortText: string;

    constructor(props: {
        shortText: string;
    }) {
        this.shortText = props.shortText;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        MaxLengthValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = MaxLengthValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'MaxLengthValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): MaxLengthValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'MaxLengthValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('shortText' in obj)) {
            errors.push({
                field: 'shortText',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(MaxLengthValidator.prototype) as MaxLengthValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_shortText = obj['shortText'] as string;
            if (__raw_shortText.length > 10) {
                errors.push({
                    field: 'shortText',
                    message: 'must have at most 10 characters'
                });
            }
            instance.shortText = __raw_shortText;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof MaxLengthValidator>(
        field: K,
        value: MaxLengthValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'shortText': {
                const __val = value as string;
                if (__val.length > 10) {
                    errors.push({
                        field: 'shortText',
                        message: 'must have at most 10 characters'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<MaxLengthValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('shortText' in partial && partial.shortText !== undefined) {
            const __val = partial.shortText as string;
            if (__val.length > 10) {
                errors.push({
                    field: 'shortText',
                    message: 'must have at most 10 characters'
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
        return 'shortText' in o;
    }

    static is(obj: unknown): obj is MaxLengthValidator {
        if (obj instanceof MaxLengthValidator) {
            return true;
        }
        if (!MaxLengthValidator.hasShape(obj)) {
            return false;
        }
        const result = MaxLengthValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function maxLengthValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<MaxLengthValidator, Array<{ field: string; message: string }>> {
    return MaxLengthValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function maxLengthValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): MaxLengthValidator | __mf_PendingRef {
    return MaxLengthValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function maxLengthValidatorIs(value: unknown): value is MaxLengthValidator {
    return MaxLengthValidator.is(value);
}

// MinLength validator

export class MinLengthValidator {
    longText: string;

    constructor(props: {
        longText: string;
    }) {
        this.longText = props.longText;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        MinLengthValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = MinLengthValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'MinLengthValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): MinLengthValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'MinLengthValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('longText' in obj)) {
            errors.push({
                field: 'longText',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(MinLengthValidator.prototype) as MinLengthValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_longText = obj['longText'] as string;
            if (__raw_longText.length < 5) {
                errors.push({
                    field: 'longText',
                    message: 'must have at least 5 characters'
                });
            }
            instance.longText = __raw_longText;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof MinLengthValidator>(
        field: K,
        value: MinLengthValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'longText': {
                const __val = value as string;
                if (__val.length < 5) {
                    errors.push({
                        field: 'longText',
                        message: 'must have at least 5 characters'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<MinLengthValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('longText' in partial && partial.longText !== undefined) {
            const __val = partial.longText as string;
            if (__val.length < 5) {
                errors.push({
                    field: 'longText',
                    message: 'must have at least 5 characters'
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
        return 'longText' in o;
    }

    static is(obj: unknown): obj is MinLengthValidator {
        if (obj instanceof MinLengthValidator) {
            return true;
        }
        if (!MinLengthValidator.hasShape(obj)) {
            return false;
        }
        const result = MinLengthValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function minLengthValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<MinLengthValidator, Array<{ field: string; message: string }>> {
    return MinLengthValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function minLengthValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): MinLengthValidator | __mf_PendingRef {
    return MinLengthValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function minLengthValidatorIs(value: unknown): value is MinLengthValidator {
    return MinLengthValidator.is(value);
}

// Length validator (exact)

export class LengthValidator {
    fixedText: string;

    constructor(props: {
        fixedText: string;
    }) {
        this.fixedText = props.fixedText;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        LengthValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = LengthValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message: 'LengthValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): LengthValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'LengthValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('fixedText' in obj)) {
            errors.push({
                field: 'fixedText',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(LengthValidator.prototype) as LengthValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_fixedText = obj['fixedText'] as string;
            if (__raw_fixedText.length !== 8) {
                errors.push({
                    field: 'fixedText',
                    message: 'must have exactly 8 characters'
                });
            }
            instance.fixedText = __raw_fixedText;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LengthValidator>(
        field: K,
        value: LengthValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'fixedText': {
                const __val = value as string;
                if (__val.length !== 8) {
                    errors.push({
                        field: 'fixedText',
                        message: 'must have exactly 8 characters'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<LengthValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('fixedText' in partial && partial.fixedText !== undefined) {
            const __val = partial.fixedText as string;
            if (__val.length !== 8) {
                errors.push({
                    field: 'fixedText',
                    message: 'must have exactly 8 characters'
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
        return 'fixedText' in o;
    }

    static is(obj: unknown): obj is LengthValidator {
        if (obj instanceof LengthValidator) {
            return true;
        }
        if (!LengthValidator.hasShape(obj)) {
            return false;
        }
        const result = LengthValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function lengthValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<LengthValidator, Array<{ field: string; message: string }>> {
    return LengthValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function lengthValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): LengthValidator | __mf_PendingRef {
    return LengthValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function lengthValidatorIs(value: unknown): value is LengthValidator {
    return LengthValidator.is(value);
}

// LengthRange validator (use length with 2 args)

export class LengthRangeValidator {
    rangedText: string;

    constructor(props: {
        rangedText: string;
    }) {
        this.rangedText = props.rangedText;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        LengthRangeValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = LengthRangeValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'LengthRangeValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): LengthRangeValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'LengthRangeValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('rangedText' in obj)) {
            errors.push({
                field: 'rangedText',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(LengthRangeValidator.prototype) as LengthRangeValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_rangedText = obj['rangedText'] as string;
            if (__raw_rangedText.length < 5 || __raw_rangedText.length > 10) {
                errors.push({
                    field: 'rangedText',
                    message: 'must have between 5 and 10 characters'
                });
            }
            instance.rangedText = __raw_rangedText;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LengthRangeValidator>(
        field: K,
        value: LengthRangeValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'rangedText': {
                const __val = value as string;
                if (__val.length < 5 || __val.length > 10) {
                    errors.push({
                        field: 'rangedText',
                        message: 'must have between 5 and 10 characters'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<LengthRangeValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('rangedText' in partial && partial.rangedText !== undefined) {
            const __val = partial.rangedText as string;
            if (__val.length < 5 || __val.length > 10) {
                errors.push({
                    field: 'rangedText',
                    message: 'must have between 5 and 10 characters'
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
        return 'rangedText' in o;
    }

    static is(obj: unknown): obj is LengthRangeValidator {
        if (obj instanceof LengthRangeValidator) {
            return true;
        }
        if (!LengthRangeValidator.hasShape(obj)) {
            return false;
        }
        const result = LengthRangeValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function lengthRangeValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<LengthRangeValidator, Array<{ field: string; message: string }>> {
    return LengthRangeValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function lengthRangeValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): LengthRangeValidator | __mf_PendingRef {
    return LengthRangeValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function lengthRangeValidatorIs(value: unknown): value is LengthRangeValidator {
    return LengthRangeValidator.is(value);
}

// Pattern validator

export class PatternValidator {
    code: string;

    constructor(props: {
        code: string;
    }) {
        this.code = props.code;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        PatternValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = PatternValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message: 'PatternValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): PatternValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'PatternValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('code' in obj)) {
            errors.push({
                field: 'code',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(PatternValidator.prototype) as PatternValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_code = obj['code'] as string;
            if (!/^[A-Z]{3}$/.test(__raw_code)) {
                errors.push({
                    field: 'code',
                    message: 'must match the required pattern'
                });
            }
            instance.code = __raw_code;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof PatternValidator>(
        field: K,
        value: PatternValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'code': {
                const __val = value as string;
                if (!/^[A-Z]{3}$/.test(__val)) {
                    errors.push({
                        field: 'code',
                        message: 'must match the required pattern'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<PatternValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('code' in partial && partial.code !== undefined) {
            const __val = partial.code as string;
            if (!/^[A-Z]{3}$/.test(__val)) {
                errors.push({
                    field: 'code',
                    message: 'must match the required pattern'
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
        return 'code' in o;
    }

    static is(obj: unknown): obj is PatternValidator {
        if (obj instanceof PatternValidator) {
            return true;
        }
        if (!PatternValidator.hasShape(obj)) {
            return false;
        }
        const result = PatternValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function patternValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<PatternValidator, Array<{ field: string; message: string }>> {
    return PatternValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function patternValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): PatternValidator | __mf_PendingRef {
    return PatternValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function patternValidatorIs(value: unknown): value is PatternValidator {
    return PatternValidator.is(value);
}

// NonEmpty validator

export class NonEmptyValidator {
    required: string;

    constructor(props: {
        required: string;
    }) {
        this.required = props.required;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        NonEmptyValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = NonEmptyValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message: 'NonEmptyValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): NonEmptyValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'NonEmptyValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('required' in obj)) {
            errors.push({
                field: 'required',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(NonEmptyValidator.prototype) as NonEmptyValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_required = obj['required'] as string;
            if (__raw_required.length === 0) {
                errors.push({
                    field: 'required',
                    message: 'must not be empty'
                });
            }
            instance.required = __raw_required;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof NonEmptyValidator>(
        field: K,
        value: NonEmptyValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'required': {
                const __val = value as string;
                if (__val.length === 0) {
                    errors.push({
                        field: 'required',
                        message: 'must not be empty'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<NonEmptyValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('required' in partial && partial.required !== undefined) {
            const __val = partial.required as string;
            if (__val.length === 0) {
                errors.push({
                    field: 'required',
                    message: 'must not be empty'
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
        return 'required' in o;
    }

    static is(obj: unknown): obj is NonEmptyValidator {
        if (obj instanceof NonEmptyValidator) {
            return true;
        }
        if (!NonEmptyValidator.hasShape(obj)) {
            return false;
        }
        const result = NonEmptyValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function nonEmptyValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<NonEmptyValidator, Array<{ field: string; message: string }>> {
    return NonEmptyValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function nonEmptyValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): NonEmptyValidator | __mf_PendingRef {
    return NonEmptyValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function nonEmptyValidatorIs(value: unknown): value is NonEmptyValidator {
    return NonEmptyValidator.is(value);
}

// Trimmed validator

export class TrimmedValidator {
    trimmed: string;

    constructor(props: {
        trimmed: string;
    }) {
        this.trimmed = props.trimmed;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        TrimmedValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = TrimmedValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message: 'TrimmedValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): TrimmedValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'TrimmedValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('trimmed' in obj)) {
            errors.push({
                field: 'trimmed',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(TrimmedValidator.prototype) as TrimmedValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_trimmed = obj['trimmed'] as string;
            if (__raw_trimmed !== __raw_trimmed.trim()) {
                errors.push({
                    field: 'trimmed',
                    message: 'must be trimmed (no leading/trailing whitespace)'
                });
            }
            instance.trimmed = __raw_trimmed;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof TrimmedValidator>(
        field: K,
        value: TrimmedValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'trimmed': {
                const __val = value as string;
                if (__val !== __val.trim()) {
                    errors.push({
                        field: 'trimmed',
                        message: 'must be trimmed (no leading/trailing whitespace)'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<TrimmedValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('trimmed' in partial && partial.trimmed !== undefined) {
            const __val = partial.trimmed as string;
            if (__val !== __val.trim()) {
                errors.push({
                    field: 'trimmed',
                    message: 'must be trimmed (no leading/trailing whitespace)'
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
        return 'trimmed' in o;
    }

    static is(obj: unknown): obj is TrimmedValidator {
        if (obj instanceof TrimmedValidator) {
            return true;
        }
        if (!TrimmedValidator.hasShape(obj)) {
            return false;
        }
        const result = TrimmedValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function trimmedValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<TrimmedValidator, Array<{ field: string; message: string }>> {
    return TrimmedValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function trimmedValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): TrimmedValidator | __mf_PendingRef {
    return TrimmedValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function trimmedValidatorIs(value: unknown): value is TrimmedValidator {
    return TrimmedValidator.is(value);
}

// Lowercase validator

export class LowercaseValidator {
    lower: string;

    constructor(props: {
        lower: string;
    }) {
        this.lower = props.lower;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        LowercaseValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = LowercaseValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'LowercaseValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): LowercaseValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'LowercaseValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('lower' in obj)) {
            errors.push({
                field: 'lower',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(LowercaseValidator.prototype) as LowercaseValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_lower = obj['lower'] as string;
            if (__raw_lower !== __raw_lower.toLowerCase()) {
                errors.push({
                    field: 'lower',
                    message: 'must be lowercase'
                });
            }
            instance.lower = __raw_lower;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LowercaseValidator>(
        field: K,
        value: LowercaseValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'lower': {
                const __val = value as string;
                if (__val !== __val.toLowerCase()) {
                    errors.push({
                        field: 'lower',
                        message: 'must be lowercase'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<LowercaseValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('lower' in partial && partial.lower !== undefined) {
            const __val = partial.lower as string;
            if (__val !== __val.toLowerCase()) {
                errors.push({
                    field: 'lower',
                    message: 'must be lowercase'
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
        return 'lower' in o;
    }

    static is(obj: unknown): obj is LowercaseValidator {
        if (obj instanceof LowercaseValidator) {
            return true;
        }
        if (!LowercaseValidator.hasShape(obj)) {
            return false;
        }
        const result = LowercaseValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function lowercaseValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<LowercaseValidator, Array<{ field: string; message: string }>> {
    return LowercaseValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function lowercaseValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): LowercaseValidator | __mf_PendingRef {
    return LowercaseValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function lowercaseValidatorIs(value: unknown): value is LowercaseValidator {
    return LowercaseValidator.is(value);
}

// Uppercase validator

export class UppercaseValidator {
    upper: string;

    constructor(props: {
        upper: string;
    }) {
        this.upper = props.upper;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        UppercaseValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = UppercaseValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'UppercaseValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): UppercaseValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'UppercaseValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('upper' in obj)) {
            errors.push({
                field: 'upper',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(UppercaseValidator.prototype) as UppercaseValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_upper = obj['upper'] as string;
            if (__raw_upper !== __raw_upper.toUpperCase()) {
                errors.push({
                    field: 'upper',
                    message: 'must be uppercase'
                });
            }
            instance.upper = __raw_upper;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof UppercaseValidator>(
        field: K,
        value: UppercaseValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'upper': {
                const __val = value as string;
                if (__val !== __val.toUpperCase()) {
                    errors.push({
                        field: 'upper',
                        message: 'must be uppercase'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<UppercaseValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('upper' in partial && partial.upper !== undefined) {
            const __val = partial.upper as string;
            if (__val !== __val.toUpperCase()) {
                errors.push({
                    field: 'upper',
                    message: 'must be uppercase'
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
        return 'upper' in o;
    }

    static is(obj: unknown): obj is UppercaseValidator {
        if (obj instanceof UppercaseValidator) {
            return true;
        }
        if (!UppercaseValidator.hasShape(obj)) {
            return false;
        }
        const result = UppercaseValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function uppercaseValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<UppercaseValidator, Array<{ field: string; message: string }>> {
    return UppercaseValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function uppercaseValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): UppercaseValidator | __mf_PendingRef {
    return UppercaseValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function uppercaseValidatorIs(value: unknown): value is UppercaseValidator {
    return UppercaseValidator.is(value);
}

// Capitalized validator

export class CapitalizedValidator {
    cap: string;

    constructor(props: {
        cap: string;
    }) {
        this.cap = props.cap;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        CapitalizedValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = CapitalizedValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'CapitalizedValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): CapitalizedValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'CapitalizedValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('cap' in obj)) {
            errors.push({
                field: 'cap',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(CapitalizedValidator.prototype) as CapitalizedValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_cap = obj['cap'] as string;
            if (__raw_cap.length > 0 && __raw_cap[0] !== __raw_cap[0].toUpperCase()) {
                errors.push({
                    field: 'cap',
                    message: 'must be capitalized'
                });
            }
            instance.cap = __raw_cap;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CapitalizedValidator>(
        field: K,
        value: CapitalizedValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'cap': {
                const __val = value as string;
                if (__val.length > 0 && __val[0] !== __val[0].toUpperCase()) {
                    errors.push({
                        field: 'cap',
                        message: 'must be capitalized'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<CapitalizedValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('cap' in partial && partial.cap !== undefined) {
            const __val = partial.cap as string;
            if (__val.length > 0 && __val[0] !== __val[0].toUpperCase()) {
                errors.push({
                    field: 'cap',
                    message: 'must be capitalized'
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
        return 'cap' in o;
    }

    static is(obj: unknown): obj is CapitalizedValidator {
        if (obj instanceof CapitalizedValidator) {
            return true;
        }
        if (!CapitalizedValidator.hasShape(obj)) {
            return false;
        }
        const result = CapitalizedValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function capitalizedValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<CapitalizedValidator, Array<{ field: string; message: string }>> {
    return CapitalizedValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function capitalizedValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CapitalizedValidator | __mf_PendingRef {
    return CapitalizedValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function capitalizedValidatorIs(value: unknown): value is CapitalizedValidator {
    return CapitalizedValidator.is(value);
}

// Uncapitalized validator

export class UncapitalizedValidator {
    uncap: string;

    constructor(props: {
        uncap: string;
    }) {
        this.uncap = props.uncap;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        UncapitalizedValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = UncapitalizedValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'UncapitalizedValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): UncapitalizedValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'UncapitalizedValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('uncap' in obj)) {
            errors.push({
                field: 'uncap',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(UncapitalizedValidator.prototype) as UncapitalizedValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_uncap = obj['uncap'] as string;
            if (__raw_uncap.length > 0 && __raw_uncap[0] !== __raw_uncap[0].toLowerCase()) {
                errors.push({
                    field: 'uncap',
                    message: 'must not be capitalized'
                });
            }
            instance.uncap = __raw_uncap;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof UncapitalizedValidator>(
        field: K,
        value: UncapitalizedValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'uncap': {
                const __val = value as string;
                if (__val.length > 0 && __val[0] !== __val[0].toLowerCase()) {
                    errors.push({
                        field: 'uncap',
                        message: 'must not be capitalized'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<UncapitalizedValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('uncap' in partial && partial.uncap !== undefined) {
            const __val = partial.uncap as string;
            if (__val.length > 0 && __val[0] !== __val[0].toLowerCase()) {
                errors.push({
                    field: 'uncap',
                    message: 'must not be capitalized'
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
        return 'uncap' in o;
    }

    static is(obj: unknown): obj is UncapitalizedValidator {
        if (obj instanceof UncapitalizedValidator) {
            return true;
        }
        if (!UncapitalizedValidator.hasShape(obj)) {
            return false;
        }
        const result = UncapitalizedValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function uncapitalizedValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<UncapitalizedValidator, Array<{ field: string; message: string }>> {
    return UncapitalizedValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function uncapitalizedValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): UncapitalizedValidator | __mf_PendingRef {
    return UncapitalizedValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function uncapitalizedValidatorIs(value: unknown): value is UncapitalizedValidator {
    return UncapitalizedValidator.is(value);
}

// StartsWith validator

export class StartsWithValidator {
    secureUrl: string;

    constructor(props: {
        secureUrl: string;
    }) {
        this.secureUrl = props.secureUrl;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        StartsWithValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = StartsWithValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'StartsWithValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): StartsWithValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'StartsWithValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('secureUrl' in obj)) {
            errors.push({
                field: 'secureUrl',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(StartsWithValidator.prototype) as StartsWithValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_secureUrl = obj['secureUrl'] as string;
            if (!__raw_secureUrl.startsWith('https://')) {
                errors.push({
                    field: 'secureUrl',
                    message: "must start with 'https://'"
                });
            }
            instance.secureUrl = __raw_secureUrl;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof StartsWithValidator>(
        field: K,
        value: StartsWithValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'secureUrl': {
                const __val = value as string;
                if (!__val.startsWith('https://')) {
                    errors.push({
                        field: 'secureUrl',
                        message: "must start with 'https://'"
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<StartsWithValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('secureUrl' in partial && partial.secureUrl !== undefined) {
            const __val = partial.secureUrl as string;
            if (!__val.startsWith('https://')) {
                errors.push({
                    field: 'secureUrl',
                    message: "must start with 'https://'"
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
        return 'secureUrl' in o;
    }

    static is(obj: unknown): obj is StartsWithValidator {
        if (obj instanceof StartsWithValidator) {
            return true;
        }
        if (!StartsWithValidator.hasShape(obj)) {
            return false;
        }
        const result = StartsWithValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function startsWithValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<StartsWithValidator, Array<{ field: string; message: string }>> {
    return StartsWithValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function startsWithValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): StartsWithValidator | __mf_PendingRef {
    return StartsWithValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function startsWithValidatorIs(value: unknown): value is StartsWithValidator {
    return StartsWithValidator.is(value);
}

// EndsWith validator

export class EndsWithValidator {
    filename: string;

    constructor(props: {
        filename: string;
    }) {
        this.filename = props.filename;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        EndsWithValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = EndsWithValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message: 'EndsWithValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): EndsWithValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'EndsWithValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('filename' in obj)) {
            errors.push({
                field: 'filename',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(EndsWithValidator.prototype) as EndsWithValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_filename = obj['filename'] as string;
            if (!__raw_filename.endsWith('.json')) {
                errors.push({
                    field: 'filename',
                    message: "must end with '.json'"
                });
            }
            instance.filename = __raw_filename;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof EndsWithValidator>(
        field: K,
        value: EndsWithValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'filename': {
                const __val = value as string;
                if (!__val.endsWith('.json')) {
                    errors.push({
                        field: 'filename',
                        message: "must end with '.json'"
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<EndsWithValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('filename' in partial && partial.filename !== undefined) {
            const __val = partial.filename as string;
            if (!__val.endsWith('.json')) {
                errors.push({
                    field: 'filename',
                    message: "must end with '.json'"
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
        return 'filename' in o;
    }

    static is(obj: unknown): obj is EndsWithValidator {
        if (obj instanceof EndsWithValidator) {
            return true;
        }
        if (!EndsWithValidator.hasShape(obj)) {
            return false;
        }
        const result = EndsWithValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function endsWithValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<EndsWithValidator, Array<{ field: string; message: string }>> {
    return EndsWithValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function endsWithValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): EndsWithValidator | __mf_PendingRef {
    return EndsWithValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function endsWithValidatorIs(value: unknown): value is EndsWithValidator {
    return EndsWithValidator.is(value);
}

// Includes validator

export class IncludesValidator {
    emailLike: string;

    constructor(props: {
        emailLike: string;
    }) {
        this.emailLike = props.emailLike;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ): __mf_Result<
        IncludesValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = IncludesValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message: 'IncludesValidator.deserialize: root cannot be a forward reference'
                    }
                ]);
            }
            ctx.applyPatches();
            if (opts?.freeze) {
                ctx.freezeAll();
            }
            return __mf_resultOk(resultOrRef);
        } catch (e) {
            if (e instanceof __mf_DeserializeError) {
                return __mf_resultErr(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return __mf_resultErr([
                {
                    field: '_root',
                    message
                }
            ]);
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): IncludesValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'IncludesValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('emailLike' in obj)) {
            errors.push({
                field: 'emailLike',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(IncludesValidator.prototype) as IncludesValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_emailLike = obj['emailLike'] as string;
            if (!__raw_emailLike.includes('@')) {
                errors.push({
                    field: 'emailLike',
                    message: "must include '@'"
                });
            }
            instance.emailLike = __raw_emailLike;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof IncludesValidator>(
        field: K,
        value: IncludesValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'emailLike': {
                const __val = value as string;
                if (!__val.includes('@')) {
                    errors.push({
                        field: 'emailLike',
                        message: "must include '@'"
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<IncludesValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('emailLike' in partial && partial.emailLike !== undefined) {
            const __val = partial.emailLike as string;
            if (!__val.includes('@')) {
                errors.push({
                    field: 'emailLike',
                    message: "must include '@'"
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
        return 'emailLike' in o;
    }

    static is(obj: unknown): obj is IncludesValidator {
        if (obj instanceof IncludesValidator) {
            return true;
        }
        if (!IncludesValidator.hasShape(obj)) {
            return false;
        }
        const result = IncludesValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function includesValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<IncludesValidator, Array<{ field: string; message: string }>> {
    return IncludesValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function includesValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): IncludesValidator | __mf_PendingRef {
    return IncludesValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function includesValidatorIs(value: unknown): value is IncludesValidator {
    return IncludesValidator.is(value);
}
