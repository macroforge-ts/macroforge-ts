import { ok as __mf_resultOk } from 'macroforge/reexports';
import { error as __mf_resultErr } from 'macroforge/reexports';
import { isOk as __mf_resultIsOk } from 'macroforge/reexports';
import type { Result as __mf_Result } from 'macroforge/reexports';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
/**
 * Edge case test classes for comprehensive deserializer validation testing.
 */

// Multiple validators on single field

export class MultipleValidatorsTest {
    text: string;

    constructor(props: {
        text: string;
    }) {
        this.text = props.text;
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
        MultipleValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = MultipleValidatorsTest.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'MultipleValidatorsTest.deserialize: root cannot be a forward reference'
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
    ): MultipleValidatorsTest | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'MultipleValidatorsTest.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('text' in obj)) {
            errors.push({
                field: 'text',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(MultipleValidatorsTest.prototype) as MultipleValidatorsTest;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_text = obj['text'] as string;
            if (__raw_text.length === 0) {
                errors.push({
                    field: 'text',
                    message: 'must not be empty'
                });
            }
            if (__raw_text.length > 100) {
                errors.push({
                    field: 'text',
                    message: 'must have at most 100 characters'
                });
            }
            if (__raw_text !== __raw_text.trim()) {
                errors.push({
                    field: 'text',
                    message: 'must be trimmed (no leading/trailing whitespace)'
                });
            }
            instance.text = __raw_text;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof MultipleValidatorsTest>(
        field: K,
        value: MultipleValidatorsTest[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'text': {
                const __val = value as string;
                if (__val.length === 0) {
                    errors.push({
                        field: 'text',
                        message: 'must not be empty'
                    });
                }
                if (__val.length > 100) {
                    errors.push({
                        field: 'text',
                        message: 'must have at most 100 characters'
                    });
                }
                if (__val !== __val.trim()) {
                    errors.push({
                        field: 'text',
                        message: 'must be trimmed (no leading/trailing whitespace)'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<MultipleValidatorsTest>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('text' in partial && partial.text !== undefined) {
            const __val = partial.text as string;
            if (__val.length === 0) {
                errors.push({
                    field: 'text',
                    message: 'must not be empty'
                });
            }
            if (__val.length > 100) {
                errors.push({
                    field: 'text',
                    message: 'must have at most 100 characters'
                });
            }
            if (__val !== __val.trim()) {
                errors.push({
                    field: 'text',
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
        return 'text' in o;
    }

    static is(obj: unknown): obj is MultipleValidatorsTest {
        if (obj instanceof MultipleValidatorsTest) {
            return true;
        }
        if (!MultipleValidatorsTest.hasShape(obj)) {
            return false;
        }
        const result = MultipleValidatorsTest.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function multipleValidatorsTestDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<MultipleValidatorsTest, Array<{ field: string; message: string }>> {
    return MultipleValidatorsTest.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function multipleValidatorsTestDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): MultipleValidatorsTest | __mf_PendingRef {
    return MultipleValidatorsTest.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function multipleValidatorsTestIs(value: unknown): value is MultipleValidatorsTest {
    return MultipleValidatorsTest.is(value);
}

// Custom error message

export class CustomMessageTest {
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
        CustomMessageTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = CustomMessageTest.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message: 'CustomMessageTest.deserialize: root cannot be a forward reference'
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
    ): CustomMessageTest | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'CustomMessageTest.deserializeWithContext: expected an object'
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
        const instance = Object.create(CustomMessageTest.prototype) as CustomMessageTest;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_email = obj['email'] as string;
            if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_email)) {
                errors.push({
                    field: 'email',
                    message: 'Please enter a valid email address'
                });
            }
            instance.email = __raw_email;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CustomMessageTest>(
        field: K,
        value: CustomMessageTest[K]
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
                        message: 'Please enter a valid email address'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<CustomMessageTest>): Array<{
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
                    message: 'Please enter a valid email address'
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

    static is(obj: unknown): obj is CustomMessageTest {
        if (obj instanceof CustomMessageTest) {
            return true;
        }
        if (!CustomMessageTest.hasShape(obj)) {
            return false;
        }
        const result = CustomMessageTest.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function customMessageTestDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<CustomMessageTest, Array<{ field: string; message: string }>> {
    return CustomMessageTest.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function customMessageTestDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CustomMessageTest | __mf_PendingRef {
    return CustomMessageTest.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function customMessageTestIs(value: unknown): value is CustomMessageTest {
    return CustomMessageTest.is(value);
}

// Mixed validators with custom message

export class MixedValidatorsTest {
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
        MixedValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = MixedValidatorsTest.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'MixedValidatorsTest.deserialize: root cannot be a forward reference'
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
    ): MixedValidatorsTest | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'MixedValidatorsTest.deserializeWithContext: expected an object'
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
        const instance = Object.create(MixedValidatorsTest.prototype) as MixedValidatorsTest;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_email = obj['email'] as string;
            if (__raw_email.length === 0) {
                errors.push({
                    field: 'email',
                    message: 'must not be empty'
                });
            }
            if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_email)) {
                errors.push({
                    field: 'email',
                    message: 'Invalid email format'
                });
            }
            instance.email = __raw_email;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof MixedValidatorsTest>(
        field: K,
        value: MixedValidatorsTest[K]
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
                if (__val.length === 0) {
                    errors.push({
                        field: 'email',
                        message: 'must not be empty'
                    });
                }
                if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
                    errors.push({
                        field: 'email',
                        message: 'Invalid email format'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<MixedValidatorsTest>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('email' in partial && partial.email !== undefined) {
            const __val = partial.email as string;
            if (__val.length === 0) {
                errors.push({
                    field: 'email',
                    message: 'must not be empty'
                });
            }
            if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
                errors.push({
                    field: 'email',
                    message: 'Invalid email format'
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

    static is(obj: unknown): obj is MixedValidatorsTest {
        if (obj instanceof MixedValidatorsTest) {
            return true;
        }
        if (!MixedValidatorsTest.hasShape(obj)) {
            return false;
        }
        const result = MixedValidatorsTest.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function mixedValidatorsTestDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<MixedValidatorsTest, Array<{ field: string; message: string }>> {
    return MixedValidatorsTest.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function mixedValidatorsTestDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): MixedValidatorsTest | __mf_PendingRef {
    return MixedValidatorsTest.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function mixedValidatorsTestIs(value: unknown): value is MixedValidatorsTest {
    return MixedValidatorsTest.is(value);
}

// Combined string validators

export class CombinedStringValidatorsTest {
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
        CombinedStringValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = CombinedStringValidatorsTest.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'CombinedStringValidatorsTest.deserialize: root cannot be a forward reference'
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
    ): CombinedStringValidatorsTest | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message:
                        'CombinedStringValidatorsTest.deserializeWithContext: expected an object'
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
        const instance = Object.create(
            CombinedStringValidatorsTest.prototype
        ) as CombinedStringValidatorsTest;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
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
            instance.username = __raw_username;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CombinedStringValidatorsTest>(
        field: K,
        value: CombinedStringValidatorsTest[K]
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
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<CombinedStringValidatorsTest>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
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

    static is(obj: unknown): obj is CombinedStringValidatorsTest {
        if (obj instanceof CombinedStringValidatorsTest) {
            return true;
        }
        if (!CombinedStringValidatorsTest.hasShape(obj)) {
            return false;
        }
        const result = CombinedStringValidatorsTest.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function combinedStringValidatorsTestDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<CombinedStringValidatorsTest, Array<{ field: string; message: string }>> {
    return CombinedStringValidatorsTest.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function combinedStringValidatorsTestDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CombinedStringValidatorsTest | __mf_PendingRef {
    return CombinedStringValidatorsTest.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function combinedStringValidatorsTestIs(
    value: unknown
): value is CombinedStringValidatorsTest {
    return CombinedStringValidatorsTest.is(value);
}

// Combined number validators

export class CombinedNumberValidatorsTest {
    score: number;

    constructor(props: {
        score: number;
    }) {
        this.score = props.score;
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
        CombinedNumberValidatorsTest,
        Array<{
            field: string;
            message: string;
        }>
    > {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = CombinedNumberValidatorsTest.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return __mf_resultErr([
                    {
                        field: '_root',
                        message:
                            'CombinedNumberValidatorsTest.deserialize: root cannot be a forward reference'
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
    ): CombinedNumberValidatorsTest | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message:
                        'CombinedNumberValidatorsTest.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('score' in obj)) {
            errors.push({
                field: 'score',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            CombinedNumberValidatorsTest.prototype
        ) as CombinedNumberValidatorsTest;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_score = obj['score'] as number;
            if (!Number.isInteger(__raw_score)) {
                errors.push({
                    field: 'score',
                    message: 'must be an integer'
                });
            }
            if (__raw_score <= 0) {
                errors.push({
                    field: 'score',
                    message: 'must be positive'
                });
            }
            if (__raw_score >= 1000) {
                errors.push({
                    field: 'score',
                    message: 'must be less than 1000'
                });
            }
            instance.score = __raw_score;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CombinedNumberValidatorsTest>(
        field: K,
        value: CombinedNumberValidatorsTest[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (field) {
            case 'score': {
                const __val = value as number;
                if (!Number.isInteger(__val)) {
                    errors.push({
                        field: 'score',
                        message: 'must be an integer'
                    });
                }
                if (__val <= 0) {
                    errors.push({
                        field: 'score',
                        message: 'must be positive'
                    });
                }
                if (__val >= 1000) {
                    errors.push({
                        field: 'score',
                        message: 'must be less than 1000'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(partial: Partial<CombinedNumberValidatorsTest>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('score' in partial && partial.score !== undefined) {
            const __val = partial.score as number;
            if (!Number.isInteger(__val)) {
                errors.push({
                    field: 'score',
                    message: 'must be an integer'
                });
            }
            if (__val <= 0) {
                errors.push({
                    field: 'score',
                    message: 'must be positive'
                });
            }
            if (__val >= 1000) {
                errors.push({
                    field: 'score',
                    message: 'must be less than 1000'
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
        return 'score' in o;
    }

    static is(obj: unknown): obj is CombinedNumberValidatorsTest {
        if (obj instanceof CombinedNumberValidatorsTest) {
            return true;
        }
        if (!CombinedNumberValidatorsTest.hasShape(obj)) {
            return false;
        }
        const result = CombinedNumberValidatorsTest.deserialize(obj);
        return __mf_resultIsOk(result);
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function combinedNumberValidatorsTestDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Result<CombinedNumberValidatorsTest, Array<{ field: string; message: string }>> {
    return CombinedNumberValidatorsTest.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function combinedNumberValidatorsTestDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CombinedNumberValidatorsTest | __mf_PendingRef {
    return CombinedNumberValidatorsTest.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function combinedNumberValidatorsTestIs(
    value: unknown
): value is CombinedNumberValidatorsTest {
    return CombinedNumberValidatorsTest.is(value);
}
