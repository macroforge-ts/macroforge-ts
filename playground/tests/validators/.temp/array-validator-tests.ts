import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<MaxItemsValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return MaxItemsValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<MaxItemsValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = MaxItemsValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "MaxItemsValidator.fromObject: root cannot be a forward reference"
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
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): MaxItemsValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "MaxItemsValidator.__deserialize: expected an object"
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
        throw new DeserializeError(errors);
    }
    const instance = Object.create(MaxItemsValidator.prototype) as MaxItemsValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_items = obj["items"];
        if (Array.isArray(__raw_items)) {
            if (__raw_items.length > 5) {
                errors.push({
                    field: "items",
                    message: "must have at most 5 items"
                });
            }
            const __arr = (__raw_items as any[]).map((item, idx)=>{
                if (item?.__ref !== undefined) {
                    const result = ctx.getOrDefer(item.__ref);
                    if (PendingRef.is(result)) {
                        return {
                            __pendingIdx: idx,
                            __refId: result.id
                        };
                    }
                    return result;
                }
                return item as string;
            });
            (instance as any).items = __arr;
            __arr.forEach((item, idx)=>{
                if (item && typeof item === "object" && "__pendingIdx" in item) {
                    ctx.addPatch((instance as any).items, idx, (item as any).__refId);
                }
            });
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}
}

// MinItems validator

export class MinItemsValidator {
    
    items: string[];

    constructor(props: {
    items: string[];
}){
    this.items = props.items;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<MinItemsValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return MinItemsValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<MinItemsValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = MinItemsValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "MinItemsValidator.fromObject: root cannot be a forward reference"
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
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): MinItemsValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "MinItemsValidator.__deserialize: expected an object"
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
        throw new DeserializeError(errors);
    }
    const instance = Object.create(MinItemsValidator.prototype) as MinItemsValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_items = obj["items"];
        if (Array.isArray(__raw_items)) {
            if (__raw_items.length < 2) {
                errors.push({
                    field: "items",
                    message: "must have at least 2 items"
                });
            }
            const __arr = (__raw_items as any[]).map((item, idx)=>{
                if (item?.__ref !== undefined) {
                    const result = ctx.getOrDefer(item.__ref);
                    if (PendingRef.is(result)) {
                        return {
                            __pendingIdx: idx,
                            __refId: result.id
                        };
                    }
                    return result;
                }
                return item as string;
            });
            (instance as any).items = __arr;
            __arr.forEach((item, idx)=>{
                if (item && typeof item === "object" && "__pendingIdx" in item) {
                    ctx.addPatch((instance as any).items, idx, (item as any).__refId);
                }
            });
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}
}

// ItemsCount validator

export class ItemsCountValidator {
    
    items: string[];

    constructor(props: {
    items: string[];
}){
    this.items = props.items;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<ItemsCountValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return ItemsCountValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<ItemsCountValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = ItemsCountValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "ItemsCountValidator.fromObject: root cannot be a forward reference"
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
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): ItemsCountValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "ItemsCountValidator.__deserialize: expected an object"
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
        throw new DeserializeError(errors);
    }
    const instance = Object.create(ItemsCountValidator.prototype) as ItemsCountValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_items = obj["items"];
        if (Array.isArray(__raw_items)) {
            if (__raw_items.length !== 3) {
                errors.push({
                    field: "items",
                    message: "must have exactly 3 items"
                });
            }
            const __arr = (__raw_items as any[]).map((item, idx)=>{
                if (item?.__ref !== undefined) {
                    const result = ctx.getOrDefer(item.__ref);
                    if (PendingRef.is(result)) {
                        return {
                            __pendingIdx: idx,
                            __refId: result.id
                        };
                    }
                    return result;
                }
                return item as string;
            });
            (instance as any).items = __arr;
            __arr.forEach((item, idx)=>{
                if (item && typeof item === "object" && "__pendingIdx" in item) {
                    ctx.addPatch((instance as any).items, idx, (item as any).__refId);
                }
            });
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}
}