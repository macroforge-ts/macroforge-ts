import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import {
    DeserializeContext as __mf_DeserializeContext,
    DeserializeError as __mf_DeserializeError,
    PendingRef as __mf_PendingRef
} from 'macroforge/serde';
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

    constructor(props: Record<string, unknown>) {
        this.evenNumber = props.evenNumber;
    }

    static deserialize(
        input: unknown,
        opts: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: CustomNumberValidator;
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
            const resultOrRef = CustomNumberValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'CustomNumberValidator.deserialize: root cannot be a forward reference'
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
                    message: `${'CustomNumberValidator'}.deserializeWithContext: expected an object`
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
            const __raw_evenNumber = obj.evenNumber as number;
            if (!isEven(__raw_evenNumber)) {
                errors.push({
                    field: 'evenNumber',
                    message: 'CustomNumberValidator.evenNumber failed custom validation (isEven)'
                });
            }
            instance.evenNumber = __raw_evenNumber;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CustomNumberValidator>(
        _field: K,
        _value: CustomNumberValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (_field === 'evenNumber') {
            const __val = _value as number;
            if (!isEven(__val)) {
                errors.push({
                    field: 'evenNumber',
                    message: 'CustomNumberValidator.evenNumber failed custom validation (isEven)'
                });
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<CustomNumberValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('evenNumber' in _partial && _partial.evenNumber !== undefined) {
            const __val = _partial.evenNumber as number;
            if (!isEven(__val)) {
                errors.push({
                    field: 'evenNumber',
                    message: 'CustomNumberValidator.evenNumber failed custom validation (isEven)'
                });
            }
        }
        return errors;
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const _o = obj as Record<string, unknown>;
        return '"evenNumber" in o';
    }

    static is(obj: unknown): obj is CustomNumberValidator {
        if (obj instanceof CustomNumberValidator) {
            return true;
        }
        if (!CustomNumberValidator.hasShape(obj)) {
            return false;
        }
        const result = CustomNumberValidator.deserialize(obj);
        return result.success;
    }
}

export function customNumberValidatorDeserialize(
    input: unknown,
    opts: __mf_DeserializeOptions
):
    | {
          success: true;
          value: CustomNumberValidator;
      }
    | {
          success: false;
          errors: Array<{
              field: string;
              message: string;
          }>;
      } {
    return CustomNumberValidator.deserialize(input, opts);
}
export function customNumberValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CustomNumberValidator | __mf_PendingRef {
    return CustomNumberValidator.deserializeWithContext(value, ctx);
}
export function customNumberValidatorIs(value: unknown): value is CustomNumberValidator {
    return CustomNumberValidator.is(value);
}

// Custom string validator

export class CustomStringValidator {
    username: string;

    constructor(props: Record<string, unknown>) {
        this.username = props.username;
    }

    static deserialize(
        input: unknown,
        opts: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: CustomStringValidator;
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
            const resultOrRef = CustomStringValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'CustomStringValidator.deserialize: root cannot be a forward reference'
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
                    message: `${'CustomStringValidator'}.deserializeWithContext: expected an object`
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
            const __raw_username = obj.username as string;
            if (!isValidUsername(__raw_username)) {
                errors.push({
                    field: 'username',
                    message:
                        'CustomStringValidator.username failed custom validation (isValidUsername)'
                });
            }
            instance.username = __raw_username;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CustomStringValidator>(
        _field: K,
        _value: CustomStringValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (_field === 'username') {
            const __val = _value as string;
            if (!isValidUsername(__val)) {
                errors.push({
                    field: 'username',
                    message:
                        'CustomStringValidator.username failed custom validation (isValidUsername)'
                });
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<CustomStringValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('username' in _partial && _partial.username !== undefined) {
            const __val = _partial.username as string;
            if (!isValidUsername(__val)) {
                errors.push({
                    field: 'username',
                    message:
                        'CustomStringValidator.username failed custom validation (isValidUsername)'
                });
            }
        }
        return errors;
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const _o = obj as Record<string, unknown>;
        return '"username" in o';
    }

    static is(obj: unknown): obj is CustomStringValidator {
        if (obj instanceof CustomStringValidator) {
            return true;
        }
        if (!CustomStringValidator.hasShape(obj)) {
            return false;
        }
        const result = CustomStringValidator.deserialize(obj);
        return result.success;
    }
}

export function customStringValidatorDeserialize(
    input: unknown,
    opts: __mf_DeserializeOptions
):
    | {
          success: true;
          value: CustomStringValidator;
      }
    | {
          success: false;
          errors: Array<{
              field: string;
              message: string;
          }>;
      } {
    return CustomStringValidator.deserialize(input, opts);
}
export function customStringValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CustomStringValidator | __mf_PendingRef {
    return CustomStringValidator.deserializeWithContext(value, ctx);
}
export function customStringValidatorIs(value: unknown): value is CustomStringValidator {
    return CustomStringValidator.is(value);
}

// Custom validator with custom message

export class CustomWithMessageValidator {
    evenNumber: number;

    constructor(props: Record<string, unknown>) {
        this.evenNumber = props.evenNumber;
    }

    static deserialize(
        input: unknown,
        opts: __mf_DeserializeOptions
    ):
        | {
              success: true;
              value: CustomWithMessageValidator;
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
            const resultOrRef = CustomWithMessageValidator.deserializeWithContext(data, ctx);
            if (__mf_PendingRef.is(resultOrRef)) {
                return {
                    success: false,
                    errors: [
                        {
                            field: '_root',
                            message:
                                'CustomWithMessageValidator.deserialize: root cannot be a forward reference'
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
                    message: `${'CustomWithMessageValidator'}.deserializeWithContext: expected an object`
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
            const __raw_evenNumber = obj.evenNumber as number;
            if (!isEven(__raw_evenNumber)) {
                errors.push({
                    field: 'evenNumber',
                    message: 'Number must be even'
                });
            }
            instance.evenNumber = __raw_evenNumber;
        }
        if (errors.length > 0) {
            throw new __mf_DeserializeError(errors);
        }
        return instance;
    }

    static validateField<K extends keyof CustomWithMessageValidator>(
        _field: K,
        _value: CustomWithMessageValidator[K]
    ): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if (_field === 'evenNumber') {
            const __val = _value as number;
            if (!isEven(__val)) {
                errors.push({
                    field: 'evenNumber',
                    message: 'Number must be even'
                });
            }
        }
        return errors;
    }

    static validateFields(_partial: Partial<CustomWithMessageValidator>): Array<{
        field: string;
        message: string;
    }> {
        const errors: Array<{
            field: string;
            message: string;
        }> = [];
        if ('evenNumber' in _partial && _partial.evenNumber !== undefined) {
            const __val = _partial.evenNumber as number;
            if (!isEven(__val)) {
                errors.push({
                    field: 'evenNumber',
                    message: 'Number must be even'
                });
            }
        }
        return errors;
    }

    static hasShape(obj: unknown): boolean {
        if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
            return false;
        }
        const _o = obj as Record<string, unknown>;
        return '"evenNumber" in o';
    }

    static is(obj: unknown): obj is CustomWithMessageValidator {
        if (obj instanceof CustomWithMessageValidator) {
            return true;
        }
        if (!CustomWithMessageValidator.hasShape(obj)) {
            return false;
        }
        const result = CustomWithMessageValidator.deserialize(obj);
        return result.success;
    }
}

export function customWithMessageValidatorDeserialize(
    input: unknown,
    opts: __mf_DeserializeOptions
):
    | {
          success: true;
          value: CustomWithMessageValidator;
      }
    | {
          success: false;
          errors: Array<{
              field: string;
              message: string;
          }>;
      } {
    return CustomWithMessageValidator.deserialize(input, opts);
}
export function customWithMessageValidatorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CustomWithMessageValidator | __mf_PendingRef {
    return CustomWithMessageValidator.deserializeWithContext(value, ctx);
}
export function customWithMessageValidatorIs(value: unknown): value is CustomWithMessageValidator {
    return CustomWithMessageValidator.is(value);
}
