import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * BigInt validator test classes for comprehensive deserializer validation testing.
 */

// GreaterThanBigInt validator
/**  */
export class GreaterThanBigIntValidator {
  
  value: bigint;

  constructor(props: {
    value: bigint;
}){
    this.value = props.value;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<GreaterThanBigIntValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = GreaterThanBigIntValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "GreaterThanBigIntValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): GreaterThanBigIntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("GreaterThanBigIntValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("value" in obj)) {
        errors.push("GreaterThanBigIntValidator.__deserialize: missing required field \"value\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(GreaterThanBigIntValidator.prototype) as GreaterThanBigIntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_value = obj["value"];
        if (__raw_value <= BigInt(0)) {
            errors.push("GreaterThanBigIntValidator.fromStringifiedJSON: field 'value' must be greater than 0");
        }
        (instance as any).value = __raw_value;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// GreaterThanOrEqualToBigInt validator
/**  */
export class GreaterThanOrEqualToBigIntValidator {
  
  value: bigint;

  constructor(props: {
    value: bigint;
}){
    this.value = props.value;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<GreaterThanOrEqualToBigIntValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = GreaterThanOrEqualToBigIntValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "GreaterThanOrEqualToBigIntValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): GreaterThanOrEqualToBigIntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("GreaterThanOrEqualToBigIntValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("value" in obj)) {
        errors.push("GreaterThanOrEqualToBigIntValidator.__deserialize: missing required field \"value\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(GreaterThanOrEqualToBigIntValidator.prototype) as GreaterThanOrEqualToBigIntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_value = obj["value"];
        if (__raw_value < BigInt(0)) {
            errors.push("GreaterThanOrEqualToBigIntValidator.fromStringifiedJSON: field 'value' must be greater than or equal to 0");
        }
        (instance as any).value = __raw_value;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// LessThanBigInt validator
/**  */
export class LessThanBigIntValidator {
  
  value: bigint;

  constructor(props: {
    value: bigint;
}){
    this.value = props.value;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LessThanBigIntValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = LessThanBigIntValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "LessThanBigIntValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): LessThanBigIntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("LessThanBigIntValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("value" in obj)) {
        errors.push("LessThanBigIntValidator.__deserialize: missing required field \"value\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(LessThanBigIntValidator.prototype) as LessThanBigIntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_value = obj["value"];
        if (__raw_value >= BigInt(1000)) {
            errors.push("LessThanBigIntValidator.fromStringifiedJSON: field 'value' must be less than 1000");
        }
        (instance as any).value = __raw_value;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// LessThanOrEqualToBigInt validator
/**  */
export class LessThanOrEqualToBigIntValidator {
  
  value: bigint;

  constructor(props: {
    value: bigint;
}){
    this.value = props.value;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LessThanOrEqualToBigIntValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = LessThanOrEqualToBigIntValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "LessThanOrEqualToBigIntValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): LessThanOrEqualToBigIntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("LessThanOrEqualToBigIntValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("value" in obj)) {
        errors.push("LessThanOrEqualToBigIntValidator.__deserialize: missing required field \"value\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(LessThanOrEqualToBigIntValidator.prototype) as LessThanOrEqualToBigIntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_value = obj["value"];
        if (__raw_value > BigInt(1000)) {
            errors.push("LessThanOrEqualToBigIntValidator.fromStringifiedJSON: field 'value' must be less than or equal to 1000");
        }
        (instance as any).value = __raw_value;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// BetweenBigInt validator
/**  */
export class BetweenBigIntValidator {
  
  value: bigint;

  constructor(props: {
    value: bigint;
}){
    this.value = props.value;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<BetweenBigIntValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = BetweenBigIntValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "BetweenBigIntValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): BetweenBigIntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("BetweenBigIntValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("value" in obj)) {
        errors.push("BetweenBigIntValidator.__deserialize: missing required field \"value\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(BetweenBigIntValidator.prototype) as BetweenBigIntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_value = obj["value"];
        if (__raw_value < BigInt(0) || __raw_value > BigInt(1000)) {
            errors.push("BetweenBigIntValidator.fromStringifiedJSON: field 'value' must be between 0 and 1000");
        }
        (instance as any).value = __raw_value;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// PositiveBigInt validator
/**  */
export class PositiveBigIntValidator {
  
  value: bigint;

  constructor(props: {
    value: bigint;
}){
    this.value = props.value;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<PositiveBigIntValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = PositiveBigIntValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "PositiveBigIntValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): PositiveBigIntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("PositiveBigIntValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("value" in obj)) {
        errors.push("PositiveBigIntValidator.__deserialize: missing required field \"value\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(PositiveBigIntValidator.prototype) as PositiveBigIntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_value = obj["value"];
        if (__raw_value <= 0n) {
            errors.push("PositiveBigIntValidator.fromStringifiedJSON: field 'value' must be positive");
        }
        (instance as any).value = __raw_value;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// NonNegativeBigInt validator
/**  */
export class NonNegativeBigIntValidator {
  
  value: bigint;

  constructor(props: {
    value: bigint;
}){
    this.value = props.value;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NonNegativeBigIntValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = NonNegativeBigIntValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "NonNegativeBigIntValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): NonNegativeBigIntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("NonNegativeBigIntValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("value" in obj)) {
        errors.push("NonNegativeBigIntValidator.__deserialize: missing required field \"value\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(NonNegativeBigIntValidator.prototype) as NonNegativeBigIntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_value = obj["value"];
        if (__raw_value < 0n) {
            errors.push("NonNegativeBigIntValidator.fromStringifiedJSON: field 'value' must be non-negative");
        }
        (instance as any).value = __raw_value;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// NegativeBigInt validator
/**  */
export class NegativeBigIntValidator {
  
  value: bigint;

  constructor(props: {
    value: bigint;
}){
    this.value = props.value;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NegativeBigIntValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = NegativeBigIntValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "NegativeBigIntValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): NegativeBigIntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("NegativeBigIntValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("value" in obj)) {
        errors.push("NegativeBigIntValidator.__deserialize: missing required field \"value\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(NegativeBigIntValidator.prototype) as NegativeBigIntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_value = obj["value"];
        if (__raw_value >= 0n) {
            errors.push("NegativeBigIntValidator.fromStringifiedJSON: field 'value' must be negative");
        }
        (instance as any).value = __raw_value;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// NonPositiveBigInt validator
/**  */
export class NonPositiveBigIntValidator {
  
  value: bigint;

  constructor(props: {
    value: bigint;
}){
    this.value = props.value;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NonPositiveBigIntValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = NonPositiveBigIntValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "NonPositiveBigIntValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): NonPositiveBigIntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("NonPositiveBigIntValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("value" in obj)) {
        errors.push("NonPositiveBigIntValidator.__deserialize: missing required field \"value\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(NonPositiveBigIntValidator.prototype) as NonPositiveBigIntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_value = obj["value"];
        if (__raw_value > 0n) {
            errors.push("NonPositiveBigIntValidator.fromStringifiedJSON: field 'value' must be non-positive");
        }
        (instance as any).value = __raw_value;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}