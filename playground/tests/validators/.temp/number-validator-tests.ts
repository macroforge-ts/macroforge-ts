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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<GreaterThanValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return GreaterThanValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<GreaterThanValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = GreaterThanValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "GreaterThanValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): GreaterThanValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "GreaterThanValidator.__deserialize: expected an object"
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
        const __raw_positive = obj["positive"];
        if (__raw_positive <= 0) {
            errors.push({
                field: "positive",
                message: "must be greater than 0"
            });
        }
        (instance as any).positive = __raw_positive;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<GreaterThanOrEqualToValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return GreaterThanOrEqualToValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<GreaterThanOrEqualToValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = GreaterThanOrEqualToValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "GreaterThanOrEqualToValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): GreaterThanOrEqualToValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "GreaterThanOrEqualToValidator.__deserialize: expected an object"
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
        const __raw_nonNegative = obj["nonNegative"];
        if (__raw_nonNegative < 0) {
            errors.push({
                field: "nonNegative",
                message: "must be greater than or equal to 0"
            });
        }
        (instance as any).nonNegative = __raw_nonNegative;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LessThanValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return LessThanValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<LessThanValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = LessThanValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "LessThanValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): LessThanValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "LessThanValidator.__deserialize: expected an object"
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
        const __raw_capped = obj["capped"];
        if (__raw_capped >= 100) {
            errors.push({
                field: "capped",
                message: "must be less than 100"
            });
        }
        (instance as any).capped = __raw_capped;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LessThanOrEqualToValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return LessThanOrEqualToValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<LessThanOrEqualToValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = LessThanOrEqualToValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "LessThanOrEqualToValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): LessThanOrEqualToValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "LessThanOrEqualToValidator.__deserialize: expected an object"
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
        const __raw_maxed = obj["maxed"];
        if (__raw_maxed > 100) {
            errors.push({
                field: "maxed",
                message: "must be less than or equal to 100"
            });
        }
        (instance as any).maxed = __raw_maxed;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<BetweenValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return BetweenValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<BetweenValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = BetweenValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "BetweenValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): BetweenValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "BetweenValidator.__deserialize: expected an object"
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
        const __raw_ranged = obj["ranged"];
        if (__raw_ranged < 1 || __raw_ranged > 100) {
            errors.push({
                field: "ranged",
                message: "must be between 1 and 100"
            });
        }
        (instance as any).ranged = __raw_ranged;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<IntValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return IntValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<IntValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = IntValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "IntValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): IntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "IntValidator.__deserialize: expected an object"
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
        const __raw_integer = obj["integer"];
        if (!Number.isInteger(__raw_integer)) {
            errors.push({
                field: "integer",
                message: "must be an integer"
            });
        }
        (instance as any).integer = __raw_integer;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NonNaNValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return NonNaNValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<NonNaNValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = NonNaNValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "NonNaNValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): NonNaNValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "NonNaNValidator.__deserialize: expected an object"
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
        const __raw_valid = obj["valid"];
        if (Number.isNaN(__raw_valid)) {
            errors.push({
                field: "valid",
                message: "must not be NaN"
            });
        }
        (instance as any).valid = __raw_valid;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<FiniteValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return FiniteValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<FiniteValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = FiniteValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "FiniteValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): FiniteValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "FiniteValidator.__deserialize: expected an object"
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
        const __raw_finite = obj["finite"];
        if (!Number.isFinite(__raw_finite)) {
            errors.push({
                field: "finite",
                message: "must be finite"
            });
        }
        (instance as any).finite = __raw_finite;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<PositiveValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return PositiveValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<PositiveValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = PositiveValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "PositiveValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): PositiveValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "PositiveValidator.__deserialize: expected an object"
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
        const __raw_positive = obj["positive"];
        if (__raw_positive <= 0) {
            errors.push({
                field: "positive",
                message: "must be positive"
            });
        }
        (instance as any).positive = __raw_positive;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NonNegativeValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return NonNegativeValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<NonNegativeValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = NonNegativeValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "NonNegativeValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): NonNegativeValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "NonNegativeValidator.__deserialize: expected an object"
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
        const __raw_nonNegative = obj["nonNegative"];
        if (__raw_nonNegative < 0) {
            errors.push({
                field: "nonNegative",
                message: "must be non-negative"
            });
        }
        (instance as any).nonNegative = __raw_nonNegative;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NegativeValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return NegativeValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<NegativeValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = NegativeValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "NegativeValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): NegativeValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "NegativeValidator.__deserialize: expected an object"
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
        const __raw_negative = obj["negative"];
        if (__raw_negative >= 0) {
            errors.push({
                field: "negative",
                message: "must be negative"
            });
        }
        (instance as any).negative = __raw_negative;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NonPositiveValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return NonPositiveValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<NonPositiveValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = NonPositiveValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "NonPositiveValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): NonPositiveValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "NonPositiveValidator.__deserialize: expected an object"
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
        const __raw_nonPositive = obj["nonPositive"];
        if (__raw_nonPositive > 0) {
            errors.push({
                field: "nonPositive",
                message: "must be non-positive"
            });
        }
        (instance as any).nonPositive = __raw_nonPositive;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<MultipleOfValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return MultipleOfValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<MultipleOfValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = MultipleOfValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "MultipleOfValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): MultipleOfValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "MultipleOfValidator.__deserialize: expected an object"
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
        const __raw_multiple = obj["multiple"];
        if (__raw_multiple % 5 !== 0) {
            errors.push({
                field: "multiple",
                message: "must be a multiple of 5"
            });
        }
        (instance as any).multiple = __raw_multiple;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
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

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<Uint8Validator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return Uint8Validator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<Uint8Validator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = Uint8Validator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "Uint8Validator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): Uint8Validator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "Uint8Validator.__deserialize: expected an object"
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
        const __raw_byte = obj["byte"];
        if (!Number.isInteger(__raw_byte) || __raw_byte < 0 || __raw_byte > 255) {
            errors.push({
                field: "byte",
                message: "must be a uint8 (0-255)"
            });
        }
        (instance as any).byte = __raw_byte;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}
}