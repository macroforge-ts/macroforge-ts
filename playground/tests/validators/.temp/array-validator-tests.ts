import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
/**
 * Array validator test classes for comprehensive deserializer validation testing.
 */

// MaxItems validator

export class MaxItemsValidator {
    
    items: string[];

    constructor(props: {
    items: string[];
}){
    this.items = props.items;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: __mf_DeserializeOptions): {
    success: true;
    value: MaxItemsValidator;
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
        const resultOrRef = MaxItemsValidator.deserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "MaxItemsValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: __mf_DeserializeContext): MaxItemsValidator | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: "MaxItemsValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("items" in obj)) {
        errors.push({
            field: "items",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance = Object.create(MaxItemsValidator.prototype) as MaxItemsValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_items = obj["items"] as string[];
        if (Array.isArray(__raw_items)) {
            if (__raw_items.length > 5) {
                errors.push({
                    field: "items",
                    message: "must have at most 5 items"
                });
            }
            instance.items = __raw_items as string[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof MaxItemsValidator>(_field: K, _value: MaxItemsValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(_field){
        case "items":
            {
                const __val = _value as string[];
                if (__val.length > 5) {
                    errors.push({
                        field: "items",
                        message: "must have at most 5 items"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(_partial: Partial<MaxItemsValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("items" in _partial && _partial.items !== undefined) {
        const __val = _partial.items as string[];
        if (__val.length > 5) {
            errors.push({
                field: "items",
                message: "must have at most 5 items"
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
    return "items" in o;
}

    static is(obj: unknown): obj is MaxItemsValidator {
    if (obj instanceof MaxItemsValidator) {
        return true;
    }
    if (!MaxItemsValidator.hasShape(obj)) {
        return false;
    }
    const result = MaxItemsValidator.deserialize(obj);
    return result.success;
}
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */export function maxItemsValidatorDeserialize(input: unknown, opts?: __mf_DeserializeOptions): { success: true; value: MaxItemsValidator } | { success: false; errors: Array<{ field: string; message: string }> } {return MaxItemsValidator.deserialize(input, opts);}/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */export function maxItemsValidatorDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): MaxItemsValidator | __mf_PendingRef {return MaxItemsValidator.deserializeWithContext(value, ctx);}/** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */export function maxItemsValidatorIs(value: unknown): value is MaxItemsValidator {return MaxItemsValidator.is(value);}

// MinItems validator

export class MinItemsValidator {
    
    items: string[];

    constructor(props: {
    items: string[];
}){
    this.items = props.items;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: __mf_DeserializeOptions): {
    success: true;
    value: MinItemsValidator;
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
        const resultOrRef = MinItemsValidator.deserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "MinItemsValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: __mf_DeserializeContext): MinItemsValidator | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: "MinItemsValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("items" in obj)) {
        errors.push({
            field: "items",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance = Object.create(MinItemsValidator.prototype) as MinItemsValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_items = obj["items"] as string[];
        if (Array.isArray(__raw_items)) {
            if (__raw_items.length < 2) {
                errors.push({
                    field: "items",
                    message: "must have at least 2 items"
                });
            }
            instance.items = __raw_items as string[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof MinItemsValidator>(_field: K, _value: MinItemsValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(_field){
        case "items":
            {
                const __val = _value as string[];
                if (__val.length < 2) {
                    errors.push({
                        field: "items",
                        message: "must have at least 2 items"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(_partial: Partial<MinItemsValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("items" in _partial && _partial.items !== undefined) {
        const __val = _partial.items as string[];
        if (__val.length < 2) {
            errors.push({
                field: "items",
                message: "must have at least 2 items"
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
    return "items" in o;
}

    static is(obj: unknown): obj is MinItemsValidator {
    if (obj instanceof MinItemsValidator) {
        return true;
    }
    if (!MinItemsValidator.hasShape(obj)) {
        return false;
    }
    const result = MinItemsValidator.deserialize(obj);
    return result.success;
}
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */export function minItemsValidatorDeserialize(input: unknown, opts?: __mf_DeserializeOptions): { success: true; value: MinItemsValidator } | { success: false; errors: Array<{ field: string; message: string }> } {return MinItemsValidator.deserialize(input, opts);}/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */export function minItemsValidatorDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): MinItemsValidator | __mf_PendingRef {return MinItemsValidator.deserializeWithContext(value, ctx);}/** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */export function minItemsValidatorIs(value: unknown): value is MinItemsValidator {return MinItemsValidator.is(value);}

// ItemsCount validator

export class ItemsCountValidator {
    
    items: string[];

    constructor(props: {
    items: string[];
}){
    this.items = props.items;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: __mf_DeserializeOptions): {
    success: true;
    value: ItemsCountValidator;
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
        const resultOrRef = ItemsCountValidator.deserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ItemsCountValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: __mf_DeserializeContext): ItemsCountValidator | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: "ItemsCountValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("items" in obj)) {
        errors.push({
            field: "items",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance = Object.create(ItemsCountValidator.prototype) as ItemsCountValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_items = obj["items"] as string[];
        if (Array.isArray(__raw_items)) {
            if (__raw_items.length !== 3) {
                errors.push({
                    field: "items",
                    message: "must have exactly 3 items"
                });
            }
            instance.items = __raw_items as string[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof ItemsCountValidator>(_field: K, _value: ItemsCountValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(_field){
        case "items":
            {
                const __val = _value as string[];
                if (__val.length !== 3) {
                    errors.push({
                        field: "items",
                        message: "must have exactly 3 items"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(_partial: Partial<ItemsCountValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("items" in _partial && _partial.items !== undefined) {
        const __val = _partial.items as string[];
        if (__val.length !== 3) {
            errors.push({
                field: "items",
                message: "must have exactly 3 items"
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
    return "items" in o;
}

    static is(obj: unknown): obj is ItemsCountValidator {
    if (obj instanceof ItemsCountValidator) {
        return true;
    }
    if (!ItemsCountValidator.hasShape(obj)) {
        return false;
    }
    const result = ItemsCountValidator.deserialize(obj);
    return result.success;
}
}

/** Deserializes input to an instance.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors */export function itemsCountValidatorDeserialize(input: unknown, opts?: __mf_DeserializeOptions): { success: true; value: ItemsCountValidator } | { success: false; errors: Array<{ field: string; message: string }> } {return ItemsCountValidator.deserialize(input, opts);}/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */export function itemsCountValidatorDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ItemsCountValidator | __mf_PendingRef {return ItemsCountValidator.deserializeWithContext(value, ctx);}/** Type guard: checks if a value can be successfully deserialized.
@param value - The value to check
@returns True if the value can be deserialized to this type */export function itemsCountValidatorIs(value: unknown): value is ItemsCountValidator {return ItemsCountValidator.is(value);}