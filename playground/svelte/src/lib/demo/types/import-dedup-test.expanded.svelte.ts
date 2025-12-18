import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { optionSome as __mf_optionSome } from 'macroforge/reexports/effect';
import { optionNone as __mf_optionNone } from 'macroforge/reexports/effect';
import { optionIsNone as __mf_optionIsNone } from 'macroforge/reexports/effect';
import type { Option as __mf_Option } from 'macroforge/reexports/effect';
/**
 * Stress test for aliased imports and tree-shaking.
 *
 * Macroforge now imports specific functions with `__mf_` prefixed aliases
 * to avoid any risk of collision with user imports:
 *
 * - `import { succeed as __mf_exitSucceed } from "effect/Exit";`
 * - `import { some as __mf_optionSome } from "effect/Option";`
 *
 * This approach:
 * - Eliminates namespace collisions with user imports
 * - Enables better tree-shaking (only specific functions imported)
 * - Is explicit about what macroforge uses
 *
 * The user's `import { Exit, Option } from "effect"` is preserved and works
 * alongside the aliased imports without any conflict.
 */

import { Exit, Option } from 'effect';

// Test 1: Type with Deserialize (uses Exit for return type)

export interface UserWithDeserialize {
    name: string;
    age: number;
    email: string;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function userWithDeserializeDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, UserWithDeserialize> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = userWithDeserializeDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'UserWithDeserialize.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function userWithDeserializeDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): UserWithDeserialize | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'UserWithDeserialize.deserializeWithContext: expected an object'
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('name' in obj)) {
        errors.push({ field: 'name', message: 'missing required field' });
    }
    if (!('age' in obj)) {
        errors.push({ field: 'age', message: 'missing required field' });
    }
    if (!('email' in obj)) {
        errors.push({ field: 'email', message: 'missing required field' });
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
        const __raw_name = obj['name'] as string;
        instance.name = __raw_name;
    }
    {
        const __raw_age = obj['age'] as number;
        instance.age = __raw_age;
    }
    {
        const __raw_email = obj['email'] as string;
        instance.email = __raw_email;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as UserWithDeserialize;
}
export function userWithDeserializeValidateField<K extends keyof UserWithDeserialize>(
    field: K,
    value: UserWithDeserialize[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function userWithDeserializeValidateFields(
    partial: Partial<UserWithDeserialize>
): Array<{ field: string; message: string }> {
    return [];
}
export function userWithDeserializeHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'name' in o && 'age' in o && 'email' in o;
}
export function userWithDeserializeIs(obj: unknown): obj is UserWithDeserialize {
    if (!userWithDeserializeHasShape(obj)) {
        return false;
    }
    const result = userWithDeserializeDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

export const UserWithDeserialize = {
    deserialize: userWithDeserializeDeserialize,
    deserializeWithContext: userWithDeserializeDeserializeWithContext,
    validateFields: userWithDeserializeValidateFields,
    hasShape: userWithDeserializeHasShape,
    is: userWithDeserializeIs
} as const;

// Test 2: Type with PartialOrd (uses Option for return type)

export interface ComparableItem {
    priority: number;
    name: string;
}

export function comparableItemPartialCompare(
    a: ComparableItem,
    b: ComparableItem
): __mf_Option<number> {
    if (a === b) return __mf_optionSome(0);
    const cmp0 = a.priority < b.priority ? -1 : a.priority > b.priority ? 1 : 0;
    if (cmp0 === null) return __mf_optionNone();
    if (cmp0 !== 0) return __mf_optionSome(cmp0);
    const cmp1 = a.name.localeCompare(b.name);
    if (cmp1 === null) return __mf_optionNone();
    if (cmp1 !== 0) return __mf_optionSome(cmp1);
    return __mf_optionSome(0);
}

export const ComparableItem = {
    partialCompare: comparableItemPartialCompare
} as const;

// Test 3: Type with both Deserialize and PartialOrd

export interface FullFeaturedType {
    id: number;
    value: string;
    score: number;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function fullFeaturedTypeDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, FullFeaturedType> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = fullFeaturedTypeDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'FullFeaturedType.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function fullFeaturedTypeDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): FullFeaturedType | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'FullFeaturedType.deserializeWithContext: expected an object'
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('value' in obj)) {
        errors.push({ field: 'value', message: 'missing required field' });
    }
    if (!('score' in obj)) {
        errors.push({ field: 'score', message: 'missing required field' });
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
        const __raw_id = obj['id'] as number;
        instance.id = __raw_id;
    }
    {
        const __raw_value = obj['value'] as string;
        instance.value = __raw_value;
    }
    {
        const __raw_score = obj['score'] as number;
        instance.score = __raw_score;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as FullFeaturedType;
}
export function fullFeaturedTypeValidateField<K extends keyof FullFeaturedType>(
    field: K,
    value: FullFeaturedType[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function fullFeaturedTypeValidateFields(
    partial: Partial<FullFeaturedType>
): Array<{ field: string; message: string }> {
    return [];
}
export function fullFeaturedTypeHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'id' in o && 'value' in o && 'score' in o;
}
export function fullFeaturedTypeIs(obj: unknown): obj is FullFeaturedType {
    if (!fullFeaturedTypeHasShape(obj)) {
        return false;
    }
    const result = fullFeaturedTypeDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

export function fullFeaturedTypePartialCompare(
    a: FullFeaturedType,
    b: FullFeaturedType
): __mf_Option<number> {
    if (a === b) return __mf_optionSome(0);
    const cmp0 = a.id < b.id ? -1 : a.id > b.id ? 1 : 0;
    if (cmp0 === null) return __mf_optionNone();
    if (cmp0 !== 0) return __mf_optionSome(cmp0);
    const cmp1 = a.value.localeCompare(b.value);
    if (cmp1 === null) return __mf_optionNone();
    if (cmp1 !== 0) return __mf_optionSome(cmp1);
    const cmp2 = a.score < b.score ? -1 : a.score > b.score ? 1 : 0;
    if (cmp2 === null) return __mf_optionNone();
    if (cmp2 !== 0) return __mf_optionSome(cmp2);
    return __mf_optionSome(0);
}

export const FullFeaturedType = {
    deserialize: fullFeaturedTypeDeserialize,
    deserializeWithContext: fullFeaturedTypeDeserializeWithContext,
    validateFields: fullFeaturedTypeValidateFields,
    hasShape: fullFeaturedTypeHasShape,
    is: fullFeaturedTypeIs,
    partialCompare: fullFeaturedTypePartialCompare
} as const;

// Test 4: Multiple types to stress test deduplication across types

export interface TypeA {
    fieldA: string;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function typeADeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, TypeA> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = typeADeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                { field: '_root', message: 'TypeA.deserialize: root cannot be a forward reference' }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function typeADeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): TypeA | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'TypeA.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('fieldA' in obj)) {
        errors.push({ field: 'fieldA', message: 'missing required field' });
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
        const __raw_fieldA = obj['fieldA'] as string;
        instance.fieldA = __raw_fieldA;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as TypeA;
}
export function typeAValidateField<K extends keyof TypeA>(
    field: K,
    value: TypeA[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function typeAValidateFields(
    partial: Partial<TypeA>
): Array<{ field: string; message: string }> {
    return [];
}
export function typeAHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'fieldA' in o;
}
export function typeAIs(obj: unknown): obj is TypeA {
    if (!typeAHasShape(obj)) {
        return false;
    }
    const result = typeADeserialize(obj);
    return __mf_exitIsSuccess(result);
}

export const TypeA = {
    deserialize: typeADeserialize,
    deserializeWithContext: typeADeserializeWithContext,
    validateFields: typeAValidateFields,
    hasShape: typeAHasShape,
    is: typeAIs
} as const;

export interface TypeB {
    fieldB: number;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function typeBDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, TypeB> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = typeBDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                { field: '_root', message: 'TypeB.deserialize: root cannot be a forward reference' }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function typeBDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): TypeB | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'TypeB.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('fieldB' in obj)) {
        errors.push({ field: 'fieldB', message: 'missing required field' });
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
        const __raw_fieldB = obj['fieldB'] as number;
        instance.fieldB = __raw_fieldB;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as TypeB;
}
export function typeBValidateField<K extends keyof TypeB>(
    field: K,
    value: TypeB[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function typeBValidateFields(
    partial: Partial<TypeB>
): Array<{ field: string; message: string }> {
    return [];
}
export function typeBHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'fieldB' in o;
}
export function typeBIs(obj: unknown): obj is TypeB {
    if (!typeBHasShape(obj)) {
        return false;
    }
    const result = typeBDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

export const TypeB = {
    deserialize: typeBDeserialize,
    deserializeWithContext: typeBDeserializeWithContext,
    validateFields: typeBValidateFields,
    hasShape: typeBHasShape,
    is: typeBIs
} as const;

export interface TypeC {
    fieldC: number;
}

export function typeCPartialCompare(a: TypeC, b: TypeC): __mf_Option<number> {
    if (a === b) return __mf_optionSome(0);
    const cmp0 = a.fieldC < b.fieldC ? -1 : a.fieldC > b.fieldC ? 1 : 0;
    if (cmp0 === null) return __mf_optionNone();
    if (cmp0 !== 0) return __mf_optionSome(cmp0);
    return __mf_optionSome(0);
}

export const TypeC = {
    partialCompare: typeCPartialCompare
} as const;

// Export a helper to verify imports work at runtime
export function verifyImports(): void {
    // These should work because we imported from "effect"
    const success = Exit.succeed({ name: 'test', age: 25, email: 'test@example.com' });
    const failure = Exit.fail([{ field: 'name', message: 'required' }]);
    const some = Option.some(42);
    const none = Option.none();

    console.log('Exit.isSuccess:', Exit.isSuccess(success));
    console.log('Exit.isFailure:', Exit.isFailure(failure));
    console.log('Option.isSome:', Option.isSome(some));
    console.log('Option.isNone:', Option.isNone(none));
}
