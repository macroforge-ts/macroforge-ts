import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * Number validator test classes for comprehensive deserializer validation testing.
 */

// GreaterThan validator

export class GreaterThanValidator {
    
    positive: number;

    constructor(props: {
    positive: number;
}){
    this.positive = props.positive;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<GreaterThanValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = GreaterThanValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "GreaterThanValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): GreaterThanValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "GreaterThanValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("positive" in obj)) {
        errors.push({
            field: "positive",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(GreaterThanValidator.prototype) as GreaterThanValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_positive = obj["positive"] as number;
        if (__raw_positive <= 0) {
            errors.push({
                field: "positive",
                message: "must be greater than 0"
            });
        }
        instance.positive = __raw_positive;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof GreaterThanValidator>(field: K, value: GreaterThanValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "positive":
            {
                const __val = value as number;
                if (__val <= 0) {
                    errors.push({
                        field: "positive",
                        message: "must be greater than 0"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<GreaterThanValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("positive" in partial && partial.positive !== undefined) {
        const __val = partial.positive as number;
        if (__val <= 0) {
            errors.push({
                field: "positive",
                message: "must be greater than 0"
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
    return "positive" in o;
}

    static is(obj: unknown): obj is GreaterThanValidator {
    if (obj instanceof GreaterThanValidator) {
        return true;
    }
    if (!GreaterThanValidator.hasShape(obj)) {
        return false;
    }
    const result = GreaterThanValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// GreaterThanOrEqualTo validator

export class GreaterThanOrEqualToValidator {
    
    nonNegative: number;

    constructor(props: {
    nonNegative: number;
}){
    this.nonNegative = props.nonNegative;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<GreaterThanOrEqualToValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = GreaterThanOrEqualToValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "GreaterThanOrEqualToValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): GreaterThanOrEqualToValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "GreaterThanOrEqualToValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("nonNegative" in obj)) {
        errors.push({
            field: "nonNegative",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(GreaterThanOrEqualToValidator.prototype) as GreaterThanOrEqualToValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_nonNegative = obj["nonNegative"] as number;
        if (__raw_nonNegative < 0) {
            errors.push({
                field: "nonNegative",
                message: "must be greater than or equal to 0"
            });
        }
        instance.nonNegative = __raw_nonNegative;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof GreaterThanOrEqualToValidator>(field: K, value: GreaterThanOrEqualToValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "nonNegative":
            {
                const __val = value as number;
                if (__val < 0) {
                    errors.push({
                        field: "nonNegative",
                        message: "must be greater than or equal to 0"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<GreaterThanOrEqualToValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("nonNegative" in partial && partial.nonNegative !== undefined) {
        const __val = partial.nonNegative as number;
        if (__val < 0) {
            errors.push({
                field: "nonNegative",
                message: "must be greater than or equal to 0"
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
    return "nonNegative" in o;
}

    static is(obj: unknown): obj is GreaterThanOrEqualToValidator {
    if (obj instanceof GreaterThanOrEqualToValidator) {
        return true;
    }
    if (!GreaterThanOrEqualToValidator.hasShape(obj)) {
        return false;
    }
    const result = GreaterThanOrEqualToValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// LessThan validator

export class LessThanValidator {
    
    capped: number;

    constructor(props: {
    capped: number;
}){
    this.capped = props.capped;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<LessThanValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = LessThanValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "LessThanValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): LessThanValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "LessThanValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("capped" in obj)) {
        errors.push({
            field: "capped",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(LessThanValidator.prototype) as LessThanValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_capped = obj["capped"] as number;
        if (__raw_capped >= 100) {
            errors.push({
                field: "capped",
                message: "must be less than 100"
            });
        }
        instance.capped = __raw_capped;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof LessThanValidator>(field: K, value: LessThanValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "capped":
            {
                const __val = value as number;
                if (__val >= 100) {
                    errors.push({
                        field: "capped",
                        message: "must be less than 100"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<LessThanValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("capped" in partial && partial.capped !== undefined) {
        const __val = partial.capped as number;
        if (__val >= 100) {
            errors.push({
                field: "capped",
                message: "must be less than 100"
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
    return "capped" in o;
}

    static is(obj: unknown): obj is LessThanValidator {
    if (obj instanceof LessThanValidator) {
        return true;
    }
    if (!LessThanValidator.hasShape(obj)) {
        return false;
    }
    const result = LessThanValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// LessThanOrEqualTo validator

export class LessThanOrEqualToValidator {
    
    maxed: number;

    constructor(props: {
    maxed: number;
}){
    this.maxed = props.maxed;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<LessThanOrEqualToValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = LessThanOrEqualToValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "LessThanOrEqualToValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): LessThanOrEqualToValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "LessThanOrEqualToValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("maxed" in obj)) {
        errors.push({
            field: "maxed",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(LessThanOrEqualToValidator.prototype) as LessThanOrEqualToValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_maxed = obj["maxed"] as number;
        if (__raw_maxed > 100) {
            errors.push({
                field: "maxed",
                message: "must be less than or equal to 100"
            });
        }
        instance.maxed = __raw_maxed;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof LessThanOrEqualToValidator>(field: K, value: LessThanOrEqualToValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "maxed":
            {
                const __val = value as number;
                if (__val > 100) {
                    errors.push({
                        field: "maxed",
                        message: "must be less than or equal to 100"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<LessThanOrEqualToValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("maxed" in partial && partial.maxed !== undefined) {
        const __val = partial.maxed as number;
        if (__val > 100) {
            errors.push({
                field: "maxed",
                message: "must be less than or equal to 100"
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
    return "maxed" in o;
}

    static is(obj: unknown): obj is LessThanOrEqualToValidator {
    if (obj instanceof LessThanOrEqualToValidator) {
        return true;
    }
    if (!LessThanOrEqualToValidator.hasShape(obj)) {
        return false;
    }
    const result = LessThanOrEqualToValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// Between validator

export class BetweenValidator {
    
    ranged: number;

    constructor(props: {
    ranged: number;
}){
    this.ranged = props.ranged;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<BetweenValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = BetweenValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "BetweenValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): BetweenValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "BetweenValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("ranged" in obj)) {
        errors.push({
            field: "ranged",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(BetweenValidator.prototype) as BetweenValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_ranged = obj["ranged"] as number;
        if (__raw_ranged < 1 || __raw_ranged > 100) {
            errors.push({
                field: "ranged",
                message: "must be between 1 and 100"
            });
        }
        instance.ranged = __raw_ranged;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof BetweenValidator>(field: K, value: BetweenValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "ranged":
            {
                const __val = value as number;
                if (__val < 1 || __val > 100) {
                    errors.push({
                        field: "ranged",
                        message: "must be between 1 and 100"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<BetweenValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("ranged" in partial && partial.ranged !== undefined) {
        const __val = partial.ranged as number;
        if (__val < 1 || __val > 100) {
            errors.push({
                field: "ranged",
                message: "must be between 1 and 100"
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
    return "ranged" in o;
}

    static is(obj: unknown): obj is BetweenValidator {
    if (obj instanceof BetweenValidator) {
        return true;
    }
    if (!BetweenValidator.hasShape(obj)) {
        return false;
    }
    const result = BetweenValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// Int validator

export class IntValidator {
    
    integer: number;

    constructor(props: {
    integer: number;
}){
    this.integer = props.integer;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<IntValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = IntValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "IntValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): IntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "IntValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("integer" in obj)) {
        errors.push({
            field: "integer",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(IntValidator.prototype) as IntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_integer = obj["integer"] as number;
        if (!Number.isInteger(__raw_integer)) {
            errors.push({
                field: "integer",
                message: "must be an integer"
            });
        }
        instance.integer = __raw_integer;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof IntValidator>(field: K, value: IntValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "integer":
            {
                const __val = value as number;
                if (!Number.isInteger(__val)) {
                    errors.push({
                        field: "integer",
                        message: "must be an integer"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<IntValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("integer" in partial && partial.integer !== undefined) {
        const __val = partial.integer as number;
        if (!Number.isInteger(__val)) {
            errors.push({
                field: "integer",
                message: "must be an integer"
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
    return "integer" in o;
}

    static is(obj: unknown): obj is IntValidator {
    if (obj instanceof IntValidator) {
        return true;
    }
    if (!IntValidator.hasShape(obj)) {
        return false;
    }
    const result = IntValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// NonNaN validator

export class NonNaNValidator {
    
    valid: number;

    constructor(props: {
    valid: number;
}){
    this.valid = props.valid;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<NonNaNValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = NonNaNValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "NonNaNValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): NonNaNValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "NonNaNValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("valid" in obj)) {
        errors.push({
            field: "valid",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(NonNaNValidator.prototype) as NonNaNValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_valid = obj["valid"] as number;
        if (Number.isNaN(__raw_valid)) {
            errors.push({
                field: "valid",
                message: "must not be NaN"
            });
        }
        instance.valid = __raw_valid;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof NonNaNValidator>(field: K, value: NonNaNValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "valid":
            {
                const __val = value as number;
                if (Number.isNaN(__val)) {
                    errors.push({
                        field: "valid",
                        message: "must not be NaN"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<NonNaNValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("valid" in partial && partial.valid !== undefined) {
        const __val = partial.valid as number;
        if (Number.isNaN(__val)) {
            errors.push({
                field: "valid",
                message: "must not be NaN"
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
    return "valid" in o;
}

    static is(obj: unknown): obj is NonNaNValidator {
    if (obj instanceof NonNaNValidator) {
        return true;
    }
    if (!NonNaNValidator.hasShape(obj)) {
        return false;
    }
    const result = NonNaNValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// Finite validator

export class FiniteValidator {
    
    finite: number;

    constructor(props: {
    finite: number;
}){
    this.finite = props.finite;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<FiniteValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = FiniteValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "FiniteValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): FiniteValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "FiniteValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("finite" in obj)) {
        errors.push({
            field: "finite",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(FiniteValidator.prototype) as FiniteValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_finite = obj["finite"] as number;
        if (!Number.isFinite(__raw_finite)) {
            errors.push({
                field: "finite",
                message: "must be finite"
            });
        }
        instance.finite = __raw_finite;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof FiniteValidator>(field: K, value: FiniteValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "finite":
            {
                const __val = value as number;
                if (!Number.isFinite(__val)) {
                    errors.push({
                        field: "finite",
                        message: "must be finite"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<FiniteValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("finite" in partial && partial.finite !== undefined) {
        const __val = partial.finite as number;
        if (!Number.isFinite(__val)) {
            errors.push({
                field: "finite",
                message: "must be finite"
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
    return "finite" in o;
}

    static is(obj: unknown): obj is FiniteValidator {
    if (obj instanceof FiniteValidator) {
        return true;
    }
    if (!FiniteValidator.hasShape(obj)) {
        return false;
    }
    const result = FiniteValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// Positive validator

export class PositiveValidator {
    
    positive: number;

    constructor(props: {
    positive: number;
}){
    this.positive = props.positive;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<PositiveValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = PositiveValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "PositiveValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): PositiveValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "PositiveValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("positive" in obj)) {
        errors.push({
            field: "positive",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(PositiveValidator.prototype) as PositiveValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_positive = obj["positive"] as number;
        if (__raw_positive <= 0) {
            errors.push({
                field: "positive",
                message: "must be positive"
            });
        }
        instance.positive = __raw_positive;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof PositiveValidator>(field: K, value: PositiveValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "positive":
            {
                const __val = value as number;
                if (__val <= 0) {
                    errors.push({
                        field: "positive",
                        message: "must be positive"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<PositiveValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("positive" in partial && partial.positive !== undefined) {
        const __val = partial.positive as number;
        if (__val <= 0) {
            errors.push({
                field: "positive",
                message: "must be positive"
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
    return "positive" in o;
}

    static is(obj: unknown): obj is PositiveValidator {
    if (obj instanceof PositiveValidator) {
        return true;
    }
    if (!PositiveValidator.hasShape(obj)) {
        return false;
    }
    const result = PositiveValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// NonNegative validator

export class NonNegativeValidator {
    
    nonNegative: number;

    constructor(props: {
    nonNegative: number;
}){
    this.nonNegative = props.nonNegative;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<NonNegativeValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = NonNegativeValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "NonNegativeValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): NonNegativeValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "NonNegativeValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("nonNegative" in obj)) {
        errors.push({
            field: "nonNegative",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(NonNegativeValidator.prototype) as NonNegativeValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_nonNegative = obj["nonNegative"] as number;
        if (__raw_nonNegative < 0) {
            errors.push({
                field: "nonNegative",
                message: "must be non-negative"
            });
        }
        instance.nonNegative = __raw_nonNegative;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof NonNegativeValidator>(field: K, value: NonNegativeValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "nonNegative":
            {
                const __val = value as number;
                if (__val < 0) {
                    errors.push({
                        field: "nonNegative",
                        message: "must be non-negative"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<NonNegativeValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("nonNegative" in partial && partial.nonNegative !== undefined) {
        const __val = partial.nonNegative as number;
        if (__val < 0) {
            errors.push({
                field: "nonNegative",
                message: "must be non-negative"
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
    return "nonNegative" in o;
}

    static is(obj: unknown): obj is NonNegativeValidator {
    if (obj instanceof NonNegativeValidator) {
        return true;
    }
    if (!NonNegativeValidator.hasShape(obj)) {
        return false;
    }
    const result = NonNegativeValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// Negative validator

export class NegativeValidator {
    
    negative: number;

    constructor(props: {
    negative: number;
}){
    this.negative = props.negative;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<NegativeValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = NegativeValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "NegativeValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): NegativeValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "NegativeValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("negative" in obj)) {
        errors.push({
            field: "negative",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(NegativeValidator.prototype) as NegativeValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_negative = obj["negative"] as number;
        if (__raw_negative >= 0) {
            errors.push({
                field: "negative",
                message: "must be negative"
            });
        }
        instance.negative = __raw_negative;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof NegativeValidator>(field: K, value: NegativeValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "negative":
            {
                const __val = value as number;
                if (__val >= 0) {
                    errors.push({
                        field: "negative",
                        message: "must be negative"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<NegativeValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("negative" in partial && partial.negative !== undefined) {
        const __val = partial.negative as number;
        if (__val >= 0) {
            errors.push({
                field: "negative",
                message: "must be negative"
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
    return "negative" in o;
}

    static is(obj: unknown): obj is NegativeValidator {
    if (obj instanceof NegativeValidator) {
        return true;
    }
    if (!NegativeValidator.hasShape(obj)) {
        return false;
    }
    const result = NegativeValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// NonPositive validator

export class NonPositiveValidator {
    
    nonPositive: number;

    constructor(props: {
    nonPositive: number;
}){
    this.nonPositive = props.nonPositive;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<NonPositiveValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = NonPositiveValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "NonPositiveValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): NonPositiveValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "NonPositiveValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("nonPositive" in obj)) {
        errors.push({
            field: "nonPositive",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(NonPositiveValidator.prototype) as NonPositiveValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_nonPositive = obj["nonPositive"] as number;
        if (__raw_nonPositive > 0) {
            errors.push({
                field: "nonPositive",
                message: "must be non-positive"
            });
        }
        instance.nonPositive = __raw_nonPositive;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof NonPositiveValidator>(field: K, value: NonPositiveValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "nonPositive":
            {
                const __val = value as number;
                if (__val > 0) {
                    errors.push({
                        field: "nonPositive",
                        message: "must be non-positive"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<NonPositiveValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("nonPositive" in partial && partial.nonPositive !== undefined) {
        const __val = partial.nonPositive as number;
        if (__val > 0) {
            errors.push({
                field: "nonPositive",
                message: "must be non-positive"
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
    return "nonPositive" in o;
}

    static is(obj: unknown): obj is NonPositiveValidator {
    if (obj instanceof NonPositiveValidator) {
        return true;
    }
    if (!NonPositiveValidator.hasShape(obj)) {
        return false;
    }
    const result = NonPositiveValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// MultipleOf validator

export class MultipleOfValidator {
    
    multiple: number;

    constructor(props: {
    multiple: number;
}){
    this.multiple = props.multiple;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<MultipleOfValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = MultipleOfValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "MultipleOfValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): MultipleOfValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "MultipleOfValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("multiple" in obj)) {
        errors.push({
            field: "multiple",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(MultipleOfValidator.prototype) as MultipleOfValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_multiple = obj["multiple"] as number;
        if (__raw_multiple % 5 !== 0) {
            errors.push({
                field: "multiple",
                message: "must be a multiple of 5"
            });
        }
        instance.multiple = __raw_multiple;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof MultipleOfValidator>(field: K, value: MultipleOfValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "multiple":
            {
                const __val = value as number;
                if (__val % 5 !== 0) {
                    errors.push({
                        field: "multiple",
                        message: "must be a multiple of 5"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<MultipleOfValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("multiple" in partial && partial.multiple !== undefined) {
        const __val = partial.multiple as number;
        if (__val % 5 !== 0) {
            errors.push({
                field: "multiple",
                message: "must be a multiple of 5"
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
    return "multiple" in o;
}

    static is(obj: unknown): obj is MultipleOfValidator {
    if (obj instanceof MultipleOfValidator) {
        return true;
    }
    if (!MultipleOfValidator.hasShape(obj)) {
        return false;
    }
    const result = MultipleOfValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// Uint8 validator

export class Uint8Validator {
    
    byte: number;

    constructor(props: {
    byte: number;
}){
    this.byte = props.byte;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<Uint8Validator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = Uint8Validator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "Uint8Validator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): Uint8Validator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "Uint8Validator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("byte" in obj)) {
        errors.push({
            field: "byte",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(Uint8Validator.prototype) as Uint8Validator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_byte = obj["byte"] as number;
        if (!Number.isInteger(__raw_byte) || __raw_byte < 0 || __raw_byte > 255) {
            errors.push({
                field: "byte",
                message: "must be a uint8 (0-255)"
            });
        }
        instance.byte = __raw_byte;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof Uint8Validator>(field: K, value: Uint8Validator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "byte":
            {
                const __val = value as number;
                if (!Number.isInteger(__val) || __val < 0 || __val > 255) {
                    errors.push({
                        field: "byte",
                        message: "must be a uint8 (0-255)"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<Uint8Validator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("byte" in partial && partial.byte !== undefined) {
        const __val = partial.byte as number;
        if (!Number.isInteger(__val) || __val < 0 || __val > 255) {
            errors.push({
                field: "byte",
                message: "must be a uint8 (0-255)"
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
    return "byte" in o;
}

    static is(obj: unknown): obj is Uint8Validator {
    if (obj instanceof Uint8Validator) {
        return true;
    }
    if (!Uint8Validator.hasShape(obj)) {
        return false;
    }
    const result = Uint8Validator.deserialize(obj);
    return Result.isOk(result);
}
}