import { ok as __mf_resultOk } from 'macroforge/reexports';
import { err as __mf_resultErr } from 'macroforge/reexports';
import { isOk as __mf_resultIsOk } from 'macroforge/reexports';
import type { Result as __mf_Result } from 'macroforge/reexports';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
/**
 * Custom validator test classes for comprehensive deserializer validation testing.
 */

// Custom validator function for even numbers
export function isEven(value: number): boolean {
    return value % 2 === 0;
}

// Custom validator function for valid usernames
export function isValidUsername(value: string): boolean {
    return /^[a-z][a-z0-9_]{2,15}$/.test(value);
}

// Custom number validator

export class CustomNumberValidator {
    evenNumber: number;

    constructor(props: {
        evenNumber: number;
    }) {
        this.evenNumber = props.evenNumber;
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
        CustomNumberValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = CustomNumberValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'CustomNumberValidator.deserialize: root cannot be a forward reference'
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
    ): CustomNumberValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'CustomNumberValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('evenNumber' in obj)) {
            errors.push({
                field: 'evenNumber',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(CustomNumberValidator.prototype) as CustomNumberValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_evenNumber = obj['evenNumber'] as number;
            {
                const __customResult = isEven(__raw_evenNumber);
                if (__customResult === false) {
                    errors.push({
                        field: 'evenNumber',
                        message: 'failed custom validation'
                    });
                }
            }
            instance.evenNumber = __raw_evenNumber;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CustomNumberValidator>(
        field: K,
        value: CustomNumberValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'evenNumber': {
                const __val = value as number;
                {
                    const __customResult = isEven(__val);
                    if (__customResult === false) {
                        errors.push({
                            field: 'evenNumber',
                            message: 'failed custom validation'
                        });
                    }
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<CustomNumberValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('evenNumber' in partial && partial.evenNumber !== undefined) {
            const __val = partial.evenNumber as number;
            {
                const __customResult = isEven(__val);
                if (__customResult === false) {
                    errors.push({
                        field: 'evenNumber',
                        message: 'failed custom validation'
                    });
                }
            }
        }
        return errors;
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'evenNumber' in o;
    }

    static is(obj: unknown): obj is CustomNumberValidator {
        if (obj instanceof CustomNumberValidator) {
            return true;
        }
        if (!CustomNumberValidator.hasShape(obj)) {
            return false;
        }
        const result = CustomNumberValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function customNumberValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<CustomNumberValidator, Array<{ field: string; message: string }>> {
    return CustomNumberValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function customNumberValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CustomNumberValidator | __mf_PendingRef {
    return CustomNumberValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function customNumberValidatorIs(value: unknown): value is CustomNumberValidator {
    return CustomNumberValidator.is(value);
}

// Custom string validator

export class CustomStringValidator {
    username: string;

    constructor(props: {
        username: string;
    }) {
        this.username = props.username;
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
        CustomStringValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = CustomStringValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'CustomStringValidator.deserialize: root cannot be a forward reference'
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
    ): CustomStringValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'CustomStringValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('username' in obj)) {
            errors.push({
                field: 'username',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(CustomStringValidator.prototype) as CustomStringValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_username = obj['username'] as string;
            {
                const __customResult = isValidUsername(__raw_username);
                if (__customResult === false) {
                    errors.push({
                        field: 'username',
                        message: 'failed custom validation'
                    });
                }
            }
            instance.username = __raw_username;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CustomStringValidator>(
        field: K,
        value: CustomStringValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'username': {
                const __val = value as string;
                {
                    const __customResult = isValidUsername(__val);
                    if (__customResult === false) {
                        errors.push({
                            field: 'username',
                            message: 'failed custom validation'
                        });
                    }
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<CustomStringValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('username' in partial && partial.username !== undefined) {
            const __val = partial.username as string;
            {
                const __customResult = isValidUsername(__val);
                if (__customResult === false) {
                    errors.push({
                        field: 'username',
                        message: 'failed custom validation'
                    });
                }
            }
        }
        return errors;
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'username' in o;
    }

    static is(obj: unknown): obj is CustomStringValidator {
        if (obj instanceof CustomStringValidator) {
            return true;
        }
        if (!CustomStringValidator.hasShape(obj)) {
            return false;
        }
        const result = CustomStringValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function customStringValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<CustomStringValidator, Array<{ field: string; message: string }>> {
    return CustomStringValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function customStringValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CustomStringValidator | __mf_PendingRef {
    return CustomStringValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function customStringValidatorIs(value: unknown): value is CustomStringValidator {
    return CustomStringValidator.is(value);
}

// Custom validator with custom message

export class CustomWithMessageValidator {
    evenNumber: number;

    constructor(props: {
        evenNumber: number;
    }) {
        this.evenNumber = props.evenNumber;
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
        CustomWithMessageValidator,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = CustomWithMessageValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'CustomWithMessageValidator.deserialize: root cannot be a forward reference'
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
    ): CustomWithMessageValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'CustomWithMessageValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('evenNumber' in obj)) {
            errors.push({
                field: 'evenNumber',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            CustomWithMessageValidator.prototype
        ) as CustomWithMessageValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_evenNumber = obj['evenNumber'] as number;
            {
                const __customResult = isEven(__raw_evenNumber);
                if (__customResult === false) {
                    errors.push({
                        field: 'evenNumber',
                        message: 'Number must be even'
                    });
                }
            }
            instance.evenNumber = __raw_evenNumber;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CustomWithMessageValidator>(
        field: K,
        value: CustomWithMessageValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'evenNumber': {
                const __val = value as number;
                {
                    const __customResult = isEven(__val);
                    if (__customResult === false) {
                        errors.push({
                            field: 'evenNumber',
                            message: 'Number must be even'
                        });
                    }
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<CustomWithMessageValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('evenNumber' in partial && partial.evenNumber !== undefined) {
            const __val = partial.evenNumber as number;
            {
                const __customResult = isEven(__val);
                if (__customResult === false) {
                    errors.push({
                        field: 'evenNumber',
                        message: 'Number must be even'
                    });
                }
            }
        }
        return errors;
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const o = obj as Record<string, unknown>;
        return 'evenNumber' in o;
    }

    static is(obj: unknown): obj is CustomWithMessageValidator {
        if (obj instanceof CustomWithMessageValidator) {
            return true;
        }
        if (!CustomWithMessageValidator.hasShape(obj)) {
            return false;
        }
        const result = CustomWithMessageValidator.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function customWithMessageValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<CustomWithMessageValidator, Array<{ field: string; message: string }>> {
    return CustomWithMessageValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function customWithMessageValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CustomWithMessageValidator | __mf_PendingRef {
    return CustomWithMessageValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function customWithMessageValidatorIs(value: unknown): value is CustomWithMessageValidator {
    return CustomWithMessageValidator.is(value);
}
