import { SerializeContext } from 'macroforge/serde';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
/**
 * Comprehensive test class for Svelte playground.
 */

export interface SvelteAllMacrosTest {
    id: string;
    title: string;
    content: string;

    apiKey: string;
    count: number;
    enabled: boolean;
}

export function svelteAllMacrosTestToString(value: SvelteAllMacrosTest): string {
    const parts: string[] = [];
    parts.push('testId: ' + value.id);
    parts.push('title: ' + value.title);
    parts.push('content: ' + value.content);
    parts.push('count: ' + value.count);
    parts.push('enabled: ' + value.enabled);
    return 'SvelteAllMacrosTest { ' + parts.join(', ') + ' }';
}

export function svelteAllMacrosTestClone(value: SvelteAllMacrosTest): SvelteAllMacrosTest {
    return {
        id: value.id,
        title: value.title,
        content: value.content,
        apiKey: value.apiKey,
        count: value.count,
        enabled: value.enabled
    };
}

export function svelteAllMacrosTestEquals(a: SvelteAllMacrosTest, b: SvelteAllMacrosTest): boolean {
    if (a === b) return true;
    return (
        a.id === b.id &&
        a.title === b.title &&
        a.content === b.content &&
        a.apiKey === b.apiKey &&
        a.count === b.count &&
        a.enabled === b.enabled
    );
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function svelteAllMacrosTestSerialize(
    value: SvelteAllMacrosTest
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(svelteAllMacrosTestSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function svelteAllMacrosTestSerializeWithContext(
    value: SvelteAllMacrosTest,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'SvelteAllMacrosTest', __id };
    result['id'] = value.id;
    result['title'] = value.title;
    result['content'] = value.content;
    result['apiKey'] = value.apiKey;
    result['count'] = value.count;
    result['enabled'] = value.enabled;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function svelteAllMacrosTestDeserialize(
    input: unknown,
    opts?: DeserializeOptions
):
    | { success: true; value: SvelteAllMacrosTest }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = svelteAllMacrosTestDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message:
                            'SvelteAllMacrosTest.deserialize: root cannot be a forward reference'
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return { success: true, value: resultOrRef };
    } catch (e) {
        if (e instanceof DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function svelteAllMacrosTestDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): SvelteAllMacrosTest | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'SvelteAllMacrosTest.deserializeWithContext: expected an object'
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('title' in obj)) {
        errors.push({ field: 'title', message: 'missing required field' });
    }
    if (!('content' in obj)) {
        errors.push({ field: 'content', message: 'missing required field' });
    }
    if (!('apiKey' in obj)) {
        errors.push({ field: 'apiKey', message: 'missing required field' });
    }
    if (!('count' in obj)) {
        errors.push({ field: 'count', message: 'missing required field' });
    }
    if (!('enabled' in obj)) {
        errors.push({ field: 'enabled', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_title = obj['title'] as string;
        instance.title = __raw_title;
    }
    {
        const __raw_content = obj['content'] as string;
        instance.content = __raw_content;
    }
    {
        const __raw_apiKey = obj['apiKey'] as string;
        instance.apiKey = __raw_apiKey;
    }
    {
        const __raw_count = obj['count'] as number;
        instance.count = __raw_count;
    }
    {
        const __raw_enabled = obj['enabled'] as boolean;
        instance.enabled = __raw_enabled;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as SvelteAllMacrosTest;
}
export function svelteAllMacrosTestValidateField<K extends keyof SvelteAllMacrosTest>(
    field: K,
    value: SvelteAllMacrosTest[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function svelteAllMacrosTestValidateFields(
    partial: Partial<SvelteAllMacrosTest>
): Array<{ field: string; message: string }> {
    return [];
}
export function svelteAllMacrosTestHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return (
        'id' in o &&
        'title' in o &&
        'content' in o &&
        'apiKey' in o &&
        'count' in o &&
        'enabled' in o
    );
}
export function svelteAllMacrosTestIs(obj: unknown): obj is SvelteAllMacrosTest {
    if (!svelteAllMacrosTestHasShape(obj)) {
        return false;
    }
    const result = svelteAllMacrosTestDeserialize(obj);
    return result.success;
}

export function svelteAllMacrosTestHashCode(value: SvelteAllMacrosTest): number {
    let hash = 17;
    hash =
        (hash * 31 +
            (value.id ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    hash =
        (hash * 31 +
            (value.title ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    hash =
        (hash * 31 +
            (value.content ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    hash =
        (hash * 31 +
            (value.apiKey ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    hash =
        (hash * 31 +
            (Number.isInteger(value.count)
                ? value.count | 0
                : value.count
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash = (hash * 31 + (value.enabled ? 1231 : 1237)) | 0;
    return hash;
}

export namespace SvelteAllMacrosTest {
    export function make(
        id: string,
        title: string,
        content: string,
        apiKey: string,
        count: number,
        enabled: boolean
    ): SvelteAllMacrosTest {
        return { id, title, content, apiKey, count, enabled };
    }
}

// Pre-instantiated test instance for e2e tests
export const svelteTestInstance = SvelteAllMacrosTest.make(
    'svelte-001',
    'Svelte Test',
    'Testing all macros in SvelteKit',
    'sk-secret-key',
    42,
    true
);
