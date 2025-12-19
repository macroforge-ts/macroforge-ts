import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
/**
 * BigInt validator test classes for comprehensive deserializer validation testing.
 */

// GreaterThanBigInt validator

export class GreaterThanBigIntValidator {
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: GreaterThanBigIntValidator;
          }
        | {
              success: false;
              errors: Array<{
                  field: string;
                  message: string;
              }>;
          } {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = GreaterThanBigIntValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'GreaterThanBigIntValidator.deserialize: root cannot be a forward reference'
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
                        field: '_root',
                        message
                    }
                ]
            };
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): GreaterThanBigIntValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'GreaterThanBigIntValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            GreaterThanBigIntValidator.prototype
        ) as GreaterThanBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'] as bigint;
            if (__raw_value <= BigInt(0)) {
                errors.push({
                    field: 'value',
                    message: 'must be greater than 0'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof GreaterThanBigIntValidator>(
        _field: K,
        _value: GreaterThanBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'value': {
                const __val = _value as bigint;
                if (__val <= BigInt(0)) {
                    errors.push({
                        field: 'value',
                        message: 'must be greater than 0'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<GreaterThanBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in _partial && _partial.value !== undefined) {
            const __val = _partial.value as bigint;
            if (__val <= BigInt(0)) {
                errors.push({
                    field: 'value',
                    message: 'must be greater than 0'
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
        return 'value' in o;
    }

    static is(obj: unknown): obj is GreaterThanBigIntValidator {
        if (obj instanceof GreaterThanBigIntValidator) {
            return true;
        }
        if (!GreaterThanBigIntValidator.hasShape(obj)) {
            return false;
        }
        const result = GreaterThanBigIntValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function greaterThanBigIntValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: GreaterThanBigIntValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return GreaterThanBigIntValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function greaterThanBigIntValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): GreaterThanBigIntValidator | __mf_PendingRef {
    return GreaterThanBigIntValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function greaterThanBigIntValidatorIs(value: unknown): value is GreaterThanBigIntValidator {
    return GreaterThanBigIntValidator.is(value);
}

// GreaterThanOrEqualToBigInt validator

export class GreaterThanOrEqualToBigIntValidator {
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: GreaterThanOrEqualToBigIntValidator;
          }
        | {
              success: false;
              errors: Array<{
                  field: string;
                  message: string;
              }>;
          } {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = GreaterThanOrEqualToBigIntValidator.deserializeWithContext(
                data,
                ctx
            );
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'GreaterThanOrEqualToBigIntValidator.deserialize: root cannot be a forward reference'
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
                        field: '_root',
                        message
                    }
                ]
            };
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): GreaterThanOrEqualToBigIntValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message:
                        'GreaterThanOrEqualToBigIntValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            GreaterThanOrEqualToBigIntValidator.prototype
        ) as GreaterThanOrEqualToBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'] as bigint;
            if (__raw_value < BigInt(0)) {
                errors.push({
                    field: 'value',
                    message: 'must be greater than or equal to 0'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof GreaterThanOrEqualToBigIntValidator>(
        _field: K,
        _value: GreaterThanOrEqualToBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'value': {
                const __val = _value as bigint;
                if (__val < BigInt(0)) {
                    errors.push({
                        field: 'value',
                        message: 'must be greater than or equal to 0'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<GreaterThanOrEqualToBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in _partial && _partial.value !== undefined) {
            const __val = _partial.value as bigint;
            if (__val < BigInt(0)) {
                errors.push({
                    field: 'value',
                    message: 'must be greater than or equal to 0'
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
        return 'value' in o;
    }

    static is(obj: unknown): obj is GreaterThanOrEqualToBigIntValidator {
        if (obj instanceof GreaterThanOrEqualToBigIntValidator) {
            return true;
        }
        if (!GreaterThanOrEqualToBigIntValidator.hasShape(obj)) {
            return false;
        }
        const result = GreaterThanOrEqualToBigIntValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function greaterThanOrEqualToBigIntValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: GreaterThanOrEqualToBigIntValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return GreaterThanOrEqualToBigIntValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function greaterThanOrEqualToBigIntValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): GreaterThanOrEqualToBigIntValidator | __mf_PendingRef {
    return GreaterThanOrEqualToBigIntValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function greaterThanOrEqualToBigIntValidatorIs(
    value: unknown
): value is GreaterThanOrEqualToBigIntValidator {
    return GreaterThanOrEqualToBigIntValidator.is(value);
}

// LessThanBigInt validator

export class LessThanBigIntValidator {
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: LessThanBigIntValidator;
          }
        | {
              success: false;
              errors: Array<{
                  field: string;
                  message: string;
              }>;
          } {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = LessThanBigIntValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'LessThanBigIntValidator.deserialize: root cannot be a forward reference'
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
                        field: '_root',
                        message
                    }
                ]
            };
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): LessThanBigIntValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'LessThanBigIntValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            LessThanBigIntValidator.prototype
        ) as LessThanBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'] as bigint;
            if (__raw_value >= BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be less than 1000'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LessThanBigIntValidator>(
        _field: K,
        _value: LessThanBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'value': {
                const __val = _value as bigint;
                if (__val >= BigInt(1000)) {
                    errors.push({
                        field: 'value',
                        message: 'must be less than 1000'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<LessThanBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in _partial && _partial.value !== undefined) {
            const __val = _partial.value as bigint;
            if (__val >= BigInt(1000)) {
                errors.push({
                    field: 'value',
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
        return 'value' in o;
    }

    static is(obj: unknown): obj is LessThanBigIntValidator {
        if (obj instanceof LessThanBigIntValidator) {
            return true;
        }
        if (!LessThanBigIntValidator.hasShape(obj)) {
            return false;
        }
        const result = LessThanBigIntValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function lessThanBigIntValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: LessThanBigIntValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return LessThanBigIntValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function lessThanBigIntValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): LessThanBigIntValidator | __mf_PendingRef {
    return LessThanBigIntValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function lessThanBigIntValidatorIs(value: unknown): value is LessThanBigIntValidator {
    return LessThanBigIntValidator.is(value);
}

// LessThanOrEqualToBigInt validator

export class LessThanOrEqualToBigIntValidator {
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: LessThanOrEqualToBigIntValidator;
          }
        | {
              success: false;
              errors: Array<{
                  field: string;
                  message: string;
              }>;
          } {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = LessThanOrEqualToBigIntValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'LessThanOrEqualToBigIntValidator.deserialize: root cannot be a forward reference'
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
                        field: '_root',
                        message
                    }
                ]
            };
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): LessThanOrEqualToBigIntValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message:
                        'LessThanOrEqualToBigIntValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            LessThanOrEqualToBigIntValidator.prototype
        ) as LessThanOrEqualToBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'] as bigint;
            if (__raw_value > BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be less than or equal to 1000'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LessThanOrEqualToBigIntValidator>(
        _field: K,
        _value: LessThanOrEqualToBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'value': {
                const __val = _value as bigint;
                if (__val > BigInt(1000)) {
                    errors.push({
                        field: 'value',
                        message: 'must be less than or equal to 1000'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<LessThanOrEqualToBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in _partial && _partial.value !== undefined) {
            const __val = _partial.value as bigint;
            if (__val > BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be less than or equal to 1000'
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
        return 'value' in o;
    }

    static is(obj: unknown): obj is LessThanOrEqualToBigIntValidator {
        if (obj instanceof LessThanOrEqualToBigIntValidator) {
            return true;
        }
        if (!LessThanOrEqualToBigIntValidator.hasShape(obj)) {
            return false;
        }
        const result = LessThanOrEqualToBigIntValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function lessThanOrEqualToBigIntValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: LessThanOrEqualToBigIntValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return LessThanOrEqualToBigIntValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function lessThanOrEqualToBigIntValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): LessThanOrEqualToBigIntValidator | __mf_PendingRef {
    return LessThanOrEqualToBigIntValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function lessThanOrEqualToBigIntValidatorIs(
    value: unknown
): value is LessThanOrEqualToBigIntValidator {
    return LessThanOrEqualToBigIntValidator.is(value);
}

// BetweenBigInt validator

export class BetweenBigIntValidator {
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: BetweenBigIntValidator;
          }
        | {
              success: false;
              errors: Array<{
                  field: string;
                  message: string;
              }>;
          } {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = BetweenBigIntValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'BetweenBigIntValidator.deserialize: root cannot be a forward reference'
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
                        field: '_root',
                        message
                    }
                ]
            };
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): BetweenBigIntValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'BetweenBigIntValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(BetweenBigIntValidator.prototype) as BetweenBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'] as bigint;
            if (__raw_value < BigInt(0) || __raw_value > BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be between 0 and 1000'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof BetweenBigIntValidator>(
        _field: K,
        _value: BetweenBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'value': {
                const __val = _value as bigint;
                if (__val < BigInt(0) || __val > BigInt(1000)) {
                    errors.push({
                        field: 'value',
                        message: 'must be between 0 and 1000'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<BetweenBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in _partial && _partial.value !== undefined) {
            const __val = _partial.value as bigint;
            if (__val < BigInt(0) || __val > BigInt(1000)) {
                errors.push({
                    field: 'value',
                    message: 'must be between 0 and 1000'
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
        return 'value' in o;
    }

    static is(obj: unknown): obj is BetweenBigIntValidator {
        if (obj instanceof BetweenBigIntValidator) {
            return true;
        }
        if (!BetweenBigIntValidator.hasShape(obj)) {
            return false;
        }
        const result = BetweenBigIntValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function betweenBigIntValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: BetweenBigIntValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return BetweenBigIntValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function betweenBigIntValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): BetweenBigIntValidator | __mf_PendingRef {
    return BetweenBigIntValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function betweenBigIntValidatorIs(value: unknown): value is BetweenBigIntValidator {
    return BetweenBigIntValidator.is(value);
}

// PositiveBigInt validator

export class PositiveBigIntValidator {
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: PositiveBigIntValidator;
          }
        | {
              success: false;
              errors: Array<{
                  field: string;
                  message: string;
              }>;
          } {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = PositiveBigIntValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'PositiveBigIntValidator.deserialize: root cannot be a forward reference'
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
                        field: '_root',
                        message
                    }
                ]
            };
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): PositiveBigIntValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'PositiveBigIntValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            PositiveBigIntValidator.prototype
        ) as PositiveBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'] as bigint;
            if (__raw_value <= 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be positive'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof PositiveBigIntValidator>(
        _field: K,
        _value: PositiveBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'value': {
                const __val = _value as bigint;
                if (__val <= 0n) {
                    errors.push({
                        field: 'value',
                        message: 'must be positive'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<PositiveBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in _partial && _partial.value !== undefined) {
            const __val = _partial.value as bigint;
            if (__val <= 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be positive'
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
        return 'value' in o;
    }

    static is(obj: unknown): obj is PositiveBigIntValidator {
        if (obj instanceof PositiveBigIntValidator) {
            return true;
        }
        if (!PositiveBigIntValidator.hasShape(obj)) {
            return false;
        }
        const result = PositiveBigIntValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function positiveBigIntValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: PositiveBigIntValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return PositiveBigIntValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function positiveBigIntValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): PositiveBigIntValidator | __mf_PendingRef {
    return PositiveBigIntValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function positiveBigIntValidatorIs(value: unknown): value is PositiveBigIntValidator {
    return PositiveBigIntValidator.is(value);
}

// NonNegativeBigInt validator

export class NonNegativeBigIntValidator {
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: NonNegativeBigIntValidator;
          }
        | {
              success: false;
              errors: Array<{
                  field: string;
                  message: string;
              }>;
          } {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = NonNegativeBigIntValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'NonNegativeBigIntValidator.deserialize: root cannot be a forward reference'
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
                        field: '_root',
                        message
                    }
                ]
            };
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): NonNegativeBigIntValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'NonNegativeBigIntValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            NonNegativeBigIntValidator.prototype
        ) as NonNegativeBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'] as bigint;
            if (__raw_value < 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be non-negative'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof NonNegativeBigIntValidator>(
        _field: K,
        _value: NonNegativeBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'value': {
                const __val = _value as bigint;
                if (__val < 0n) {
                    errors.push({
                        field: 'value',
                        message: 'must be non-negative'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<NonNegativeBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in _partial && _partial.value !== undefined) {
            const __val = _partial.value as bigint;
            if (__val < 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be non-negative'
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
        return 'value' in o;
    }

    static is(obj: unknown): obj is NonNegativeBigIntValidator {
        if (obj instanceof NonNegativeBigIntValidator) {
            return true;
        }
        if (!NonNegativeBigIntValidator.hasShape(obj)) {
            return false;
        }
        const result = NonNegativeBigIntValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function nonNegativeBigIntValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: NonNegativeBigIntValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return NonNegativeBigIntValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function nonNegativeBigIntValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): NonNegativeBigIntValidator | __mf_PendingRef {
    return NonNegativeBigIntValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function nonNegativeBigIntValidatorIs(value: unknown): value is NonNegativeBigIntValidator {
    return NonNegativeBigIntValidator.is(value);
}

// NegativeBigInt validator

export class NegativeBigIntValidator {
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: NegativeBigIntValidator;
          }
        | {
              success: false;
              errors: Array<{
                  field: string;
                  message: string;
              }>;
          } {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = NegativeBigIntValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'NegativeBigIntValidator.deserialize: root cannot be a forward reference'
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
                        field: '_root',
                        message
                    }
                ]
            };
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): NegativeBigIntValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'NegativeBigIntValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            NegativeBigIntValidator.prototype
        ) as NegativeBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'] as bigint;
            if (__raw_value >= 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be negative'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof NegativeBigIntValidator>(
        _field: K,
        _value: NegativeBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'value': {
                const __val = _value as bigint;
                if (__val >= 0n) {
                    errors.push({
                        field: 'value',
                        message: 'must be negative'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<NegativeBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in _partial && _partial.value !== undefined) {
            const __val = _partial.value as bigint;
            if (__val >= 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be negative'
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
        return 'value' in o;
    }

    static is(obj: unknown): obj is NegativeBigIntValidator {
        if (obj instanceof NegativeBigIntValidator) {
            return true;
        }
        if (!NegativeBigIntValidator.hasShape(obj)) {
            return false;
        }
        const result = NegativeBigIntValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function negativeBigIntValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: NegativeBigIntValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return NegativeBigIntValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function negativeBigIntValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): NegativeBigIntValidator | __mf_PendingRef {
    return NegativeBigIntValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function negativeBigIntValidatorIs(value: unknown): value is NegativeBigIntValidator {
    return NegativeBigIntValidator.is(value);
}

// NonPositiveBigInt validator

export class NonPositiveBigIntValidator {
    value: bigint;

    constructor(props: {
        value: bigint;
    }) {
        this.value = props.value;
    }
    /** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(
        input: unknown,
        opts?: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: NonPositiveBigIntValidator;
          }
        | {
              success: false;
              errors: Array<{
                  field: string;
                  message: string;
              }>;
          } {
        try {
            const data = typeof input === 'string' ? JSON.parse(input) : input;
            const ctx = __mf_DeserializeContext.create();
            const resultOrRef = NonPositiveBigIntValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'NonPositiveBigIntValidator.deserialize: root cannot be a forward reference'
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
                        field: '_root',
                        message
                    }
                ]
            };
        }
    }
    /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(
        value: any,
        ctx: __mf_DeserializeContext
    ): NonPositiveBigIntValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'NonPositiveBigIntValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('value' in obj)) {
            errors.push({
                field: 'value',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            NonPositiveBigIntValidator.prototype
        ) as NonPositiveBigIntValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_value = obj['value'] as bigint;
            if (__raw_value > 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be non-positive'
                });
            }
            instance.value = __raw_value;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof NonPositiveBigIntValidator>(
        _field: K,
        _value: NonPositiveBigIntValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'value': {
                const __val = _value as bigint;
                if (__val > 0n) {
                    errors.push({
                        field: 'value',
                        message: 'must be non-positive'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<NonPositiveBigIntValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('value' in _partial && _partial.value !== undefined) {
            const __val = _partial.value as bigint;
            if (__val > 0n) {
                errors.push({
                    field: 'value',
                    message: 'must be non-positive'
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
        return 'value' in o;
    }

    static is(obj: unknown): obj is NonPositiveBigIntValidator {
        if (obj instanceof NonPositiveBigIntValidator) {
            return true;
        }
        if (!NonPositiveBigIntValidator.hasShape(obj)) {
            return false;
        }
        const result = NonPositiveBigIntValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function nonPositiveBigIntValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: NonPositiveBigIntValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return NonPositiveBigIntValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function nonPositiveBigIntValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): NonPositiveBigIntValidator | __mf_PendingRef {
    return NonPositiveBigIntValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function nonPositiveBigIntValidatorIs(value: unknown): value is NonPositiveBigIntValidator {
    return NonPositiveBigIntValidator.is(value);
}
