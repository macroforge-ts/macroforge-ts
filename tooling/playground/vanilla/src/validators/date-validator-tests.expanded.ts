import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
/**
 * Date validator test classes for comprehensive deserializer validation testing.
 */

// ValidDate validator

export class ValidDateValidator {
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
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
              value: ValidDateValidator;
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
            const resultOrRef = ValidDateValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'ValidDateValidator.deserialize: root cannot be a forward reference'
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
    ): ValidDateValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'ValidDateValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(ValidDateValidator.prototype) as ValidDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'] as Date;
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (__dateVal == null || isNaN(__dateVal.getTime())) {
                    errors.push({
                        field: 'date',
                        message: 'must be a valid date'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof ValidDateValidator>(
        _field: K,
        _value: ValidDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'date': {
                const __val = _value as Date;
                if (__val == null || isNaN(__val.getTime())) {
                    errors.push({
                        field: 'date',
                        message: 'must be a valid date'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<ValidDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in _partial && _partial.date !== undefined) {
            const __val = _partial.date as Date;
            if (__val == null || isNaN(__val.getTime())) {
                errors.push({
                    field: 'date',
                    message: 'must be a valid date'
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
        return 'date' in o;
    }

    static is(obj: unknown): obj is ValidDateValidator {
        if (obj instanceof ValidDateValidator) {
            return true;
        }
        if (!ValidDateValidator.hasShape(obj)) {
            return false;
        }
        const result = ValidDateValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function validDateValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: ValidDateValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return ValidDateValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function validDateValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): ValidDateValidator | __mf_PendingRef {
    return ValidDateValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function validDateValidatorIs(value: unknown): value is ValidDateValidator {
    return ValidDateValidator.is(value);
}

// GreaterThanDate validator

export class GreaterThanDateValidator {
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
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
              value: GreaterThanDateValidator;
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
            const resultOrRef = GreaterThanDateValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'GreaterThanDateValidator.deserialize: root cannot be a forward reference'
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
    ): GreaterThanDateValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'GreaterThanDateValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            GreaterThanDateValidator.prototype
        ) as GreaterThanDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'] as Date;
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (__dateVal == null || __dateVal.getTime() <= new Date('2020-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be after 2020-01-01'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof GreaterThanDateValidator>(
        _field: K,
        _value: GreaterThanDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'date': {
                const __val = _value as Date;
                if (__val == null || __val.getTime() <= new Date('2020-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be after 2020-01-01'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<GreaterThanDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in _partial && _partial.date !== undefined) {
            const __val = _partial.date as Date;
            if (__val == null || __val.getTime() <= new Date('2020-01-01').getTime()) {
                errors.push({
                    field: 'date',
                    message: 'must be after 2020-01-01'
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
        return 'date' in o;
    }

    static is(obj: unknown): obj is GreaterThanDateValidator {
        if (obj instanceof GreaterThanDateValidator) {
            return true;
        }
        if (!GreaterThanDateValidator.hasShape(obj)) {
            return false;
        }
        const result = GreaterThanDateValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function greaterThanDateValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: GreaterThanDateValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return GreaterThanDateValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function greaterThanDateValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): GreaterThanDateValidator | __mf_PendingRef {
    return GreaterThanDateValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function greaterThanDateValidatorIs(value: unknown): value is GreaterThanDateValidator {
    return GreaterThanDateValidator.is(value);
}

// GreaterThanOrEqualToDate validator

export class GreaterThanOrEqualToDateValidator {
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
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
              value: GreaterThanOrEqualToDateValidator;
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
            const resultOrRef = GreaterThanOrEqualToDateValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'GreaterThanOrEqualToDateValidator.deserialize: root cannot be a forward reference'
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
    ): GreaterThanOrEqualToDateValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message:
                        'GreaterThanOrEqualToDateValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            GreaterThanOrEqualToDateValidator.prototype
        ) as GreaterThanOrEqualToDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'] as Date;
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (__dateVal == null || __dateVal.getTime() < new Date('2020-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be on or after 2020-01-01'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof GreaterThanOrEqualToDateValidator>(
        _field: K,
        _value: GreaterThanOrEqualToDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'date': {
                const __val = _value as Date;
                if (__val == null || __val.getTime() < new Date('2020-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be on or after 2020-01-01'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<GreaterThanOrEqualToDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in _partial && _partial.date !== undefined) {
            const __val = _partial.date as Date;
            if (__val == null || __val.getTime() < new Date('2020-01-01').getTime()) {
                errors.push({
                    field: 'date',
                    message: 'must be on or after 2020-01-01'
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
        return 'date' in o;
    }

    static is(obj: unknown): obj is GreaterThanOrEqualToDateValidator {
        if (obj instanceof GreaterThanOrEqualToDateValidator) {
            return true;
        }
        if (!GreaterThanOrEqualToDateValidator.hasShape(obj)) {
            return false;
        }
        const result = GreaterThanOrEqualToDateValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function greaterThanOrEqualToDateValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: GreaterThanOrEqualToDateValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return GreaterThanOrEqualToDateValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function greaterThanOrEqualToDateValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): GreaterThanOrEqualToDateValidator | __mf_PendingRef {
    return GreaterThanOrEqualToDateValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function greaterThanOrEqualToDateValidatorIs(
    value: unknown
): value is GreaterThanOrEqualToDateValidator {
    return GreaterThanOrEqualToDateValidator.is(value);
}

// LessThanDate validator

export class LessThanDateValidator {
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
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
              value: LessThanDateValidator;
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
            const resultOrRef = LessThanDateValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'LessThanDateValidator.deserialize: root cannot be a forward reference'
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
    ): LessThanDateValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'LessThanDateValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(LessThanDateValidator.prototype) as LessThanDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'] as Date;
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (__dateVal == null || __dateVal.getTime() >= new Date('2030-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be before 2030-01-01'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LessThanDateValidator>(
        _field: K,
        _value: LessThanDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'date': {
                const __val = _value as Date;
                if (__val == null || __val.getTime() >= new Date('2030-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be before 2030-01-01'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<LessThanDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in _partial && _partial.date !== undefined) {
            const __val = _partial.date as Date;
            if (__val == null || __val.getTime() >= new Date('2030-01-01').getTime()) {
                errors.push({
                    field: 'date',
                    message: 'must be before 2030-01-01'
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
        return 'date' in o;
    }

    static is(obj: unknown): obj is LessThanDateValidator {
        if (obj instanceof LessThanDateValidator) {
            return true;
        }
        if (!LessThanDateValidator.hasShape(obj)) {
            return false;
        }
        const result = LessThanDateValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function lessThanDateValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: LessThanDateValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return LessThanDateValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function lessThanDateValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): LessThanDateValidator | __mf_PendingRef {
    return LessThanDateValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function lessThanDateValidatorIs(value: unknown): value is LessThanDateValidator {
    return LessThanDateValidator.is(value);
}

// LessThanOrEqualToDate validator

export class LessThanOrEqualToDateValidator {
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
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
              value: LessThanOrEqualToDateValidator;
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
            const resultOrRef = LessThanOrEqualToDateValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'LessThanOrEqualToDateValidator.deserialize: root cannot be a forward reference'
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
    ): LessThanOrEqualToDateValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message:
                        'LessThanOrEqualToDateValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(
            LessThanOrEqualToDateValidator.prototype
        ) as LessThanOrEqualToDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'] as Date;
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (__dateVal == null || __dateVal.getTime() > new Date('2030-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be on or before 2030-01-01'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof LessThanOrEqualToDateValidator>(
        _field: K,
        _value: LessThanOrEqualToDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'date': {
                const __val = _value as Date;
                if (__val == null || __val.getTime() > new Date('2030-01-01').getTime()) {
                    errors.push({
                        field: 'date',
                        message: 'must be on or before 2030-01-01'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<LessThanOrEqualToDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in _partial && _partial.date !== undefined) {
            const __val = _partial.date as Date;
            if (__val == null || __val.getTime() > new Date('2030-01-01').getTime()) {
                errors.push({
                    field: 'date',
                    message: 'must be on or before 2030-01-01'
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
        return 'date' in o;
    }

    static is(obj: unknown): obj is LessThanOrEqualToDateValidator {
        if (obj instanceof LessThanOrEqualToDateValidator) {
            return true;
        }
        if (!LessThanOrEqualToDateValidator.hasShape(obj)) {
            return false;
        }
        const result = LessThanOrEqualToDateValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function lessThanOrEqualToDateValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: LessThanOrEqualToDateValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return LessThanOrEqualToDateValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function lessThanOrEqualToDateValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): LessThanOrEqualToDateValidator | __mf_PendingRef {
    return LessThanOrEqualToDateValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function lessThanOrEqualToDateValidatorIs(
    value: unknown
): value is LessThanOrEqualToDateValidator {
    return LessThanOrEqualToDateValidator.is(value);
}

// BetweenDate validator

export class BetweenDateValidator {
    date: Date;

    constructor(props: {
        date: Date;
    }) {
        this.date = props.date;
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
              value: BetweenDateValidator;
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
            const resultOrRef = BetweenDateValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'BetweenDateValidator.deserialize: root cannot be a forward reference'
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
    ): BetweenDateValidator | __mf_PendingRef {
        if (value?.__ref !== undefined) {
            return ctx.getOrDefer(value.__ref);
        }
        if (typeof value !== 'object' || value === null || Array.isArray(value)) {
            throw new __mf_DeserializeError([
                {
                    field: '_root',
                    message: 'BetweenDateValidator.deserializeWithContext: expected an object'
                }
            ]);
        }
        const obj = value as Record<string, unknown>;
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (!('date' in obj)) {
            errors.push({
                field: 'date',
                message: 'missing required field'
            });
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        const instance = Object.create(BetweenDateValidator.prototype) as BetweenDateValidator;
        if (obj.__id !== undefined) {
            ctx.register(obj.__id as number, instance);
        }
        ctx.trackForFreeze(instance);
        {
            const __raw_date = obj['date'] as Date;
            {
                const __dateVal =
                    typeof __raw_date === 'string' ? new Date(__raw_date) : (__raw_date as Date);
                if (
                    __dateVal == null ||
                    __dateVal.getTime() < new Date('2020-01-01').getTime() ||
                    __dateVal.getTime() > new Date('2030-01-01').getTime()
                ) {
                    errors.push({
                        field: 'date',
                        message: 'must be between 2020-01-01 and 2030-01-01'
                    });
                }
                instance.date = __dateVal;
            }
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof BetweenDateValidator>(
        _field: K,
        _value: BetweenDateValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        switch (_field) {
            case 'date': {
                const __val = _value as Date;
                if (
                    __val == null ||
                    __val.getTime() < new Date('2020-01-01').getTime() ||
                    __val.getTime() > new Date('2030-01-01').getTime()
                ) {
                    errors.push({
                        field: 'date',
                        message: 'must be between 2020-01-01 and 2030-01-01'
                    });
                }
                break;
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<BetweenDateValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('date' in _partial && _partial.date !== undefined) {
            const __val = _partial.date as Date;
            if (
                __val == null ||
                __val.getTime() < new Date('2020-01-01').getTime() ||
                __val.getTime() > new Date('2030-01-01').getTime()
            ) {
                errors.push({
                    field: 'date',
                    message: 'must be between 2020-01-01 and 2030-01-01'
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
        return 'date' in o;
    }

    static is(obj: unknown): obj is BetweenDateValidator {
        if (obj instanceof BetweenDateValidator) {
            return true;
        }
        if (!BetweenDateValidator.hasShape(obj)) {
            return false;
        }
        const result = BetweenDateValidator.deserialize(obj);
        return result.success;
    }
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */ export function betweenDateValidatorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: BetweenDateValidator }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    return BetweenDateValidator.deserialize(input, opts);
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function betweenDateValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): BetweenDateValidator | __mf_PendingRef {
    return BetweenDateValidator.deserializeWithContext(value, ctx);
} /** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */
export function betweenDateValidatorIs(value: unknown): value is BetweenDateValidator {
    return BetweenDateValidator.is(value);
}
