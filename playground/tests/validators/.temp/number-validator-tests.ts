import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * Number validator test classes for comprehensive deserializer validation testing.
 */

// GreaterThan validator
/**  */
export class GreaterThanValidator {
  
  positive: number;

  constructor(props: {
    positive: number;
}){
    this.positive = props.positive;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<GreaterThanValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = GreaterThanValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "GreaterThanValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): GreaterThanValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("GreaterThanValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("positive" in obj)) {
        errors.push("GreaterThanValidator.__deserialize: missing required field \"positive\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(GreaterThanValidator.prototype) as GreaterThanValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_positive = obj["positive"];
        if (__raw_positive <= 0) {
            errors.push("GreaterThanValidator.fromStringifiedJSON: field 'positive' must be greater than 0");
        }
        (instance as any).positive = __raw_positive;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// GreaterThanOrEqualTo validator
/**  */
export class GreaterThanOrEqualToValidator {
  
  nonNegative: number;

  constructor(props: {
    nonNegative: number;
}){
    this.nonNegative = props.nonNegative;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<GreaterThanOrEqualToValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = GreaterThanOrEqualToValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "GreaterThanOrEqualToValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): GreaterThanOrEqualToValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("GreaterThanOrEqualToValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("nonNegative" in obj)) {
        errors.push("GreaterThanOrEqualToValidator.__deserialize: missing required field \"nonNegative\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(GreaterThanOrEqualToValidator.prototype) as GreaterThanOrEqualToValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_nonNegative = obj["nonNegative"];
        if (__raw_nonNegative < 0) {
            errors.push("GreaterThanOrEqualToValidator.fromStringifiedJSON: field 'nonNegative' must be greater than or equal to 0");
        }
        (instance as any).nonNegative = __raw_nonNegative;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// LessThan validator
/**  */
export class LessThanValidator {
  
  capped: number;

  constructor(props: {
    capped: number;
}){
    this.capped = props.capped;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LessThanValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = LessThanValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "LessThanValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): LessThanValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("LessThanValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("capped" in obj)) {
        errors.push("LessThanValidator.__deserialize: missing required field \"capped\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(LessThanValidator.prototype) as LessThanValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_capped = obj["capped"];
        if (__raw_capped >= 100) {
            errors.push("LessThanValidator.fromStringifiedJSON: field 'capped' must be less than 100");
        }
        (instance as any).capped = __raw_capped;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// LessThanOrEqualTo validator
/**  */
export class LessThanOrEqualToValidator {
  
  maxed: number;

  constructor(props: {
    maxed: number;
}){
    this.maxed = props.maxed;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LessThanOrEqualToValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = LessThanOrEqualToValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "LessThanOrEqualToValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): LessThanOrEqualToValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("LessThanOrEqualToValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("maxed" in obj)) {
        errors.push("LessThanOrEqualToValidator.__deserialize: missing required field \"maxed\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(LessThanOrEqualToValidator.prototype) as LessThanOrEqualToValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_maxed = obj["maxed"];
        if (__raw_maxed > 100) {
            errors.push("LessThanOrEqualToValidator.fromStringifiedJSON: field 'maxed' must be less than or equal to 100");
        }
        (instance as any).maxed = __raw_maxed;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Between validator
/**  */
export class BetweenValidator {
  
  ranged: number;

  constructor(props: {
    ranged: number;
}){
    this.ranged = props.ranged;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<BetweenValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = BetweenValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "BetweenValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): BetweenValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("BetweenValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("ranged" in obj)) {
        errors.push("BetweenValidator.__deserialize: missing required field \"ranged\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(BetweenValidator.prototype) as BetweenValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_ranged = obj["ranged"];
        if (__raw_ranged < 1 || __raw_ranged > 100) {
            errors.push("BetweenValidator.fromStringifiedJSON: field 'ranged' must be between 1 and 100");
        }
        (instance as any).ranged = __raw_ranged;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Int validator
/**  */
export class IntValidator {
  
  integer: number;

  constructor(props: {
    integer: number;
}){
    this.integer = props.integer;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<IntValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = IntValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "IntValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): IntValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("IntValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("integer" in obj)) {
        errors.push("IntValidator.__deserialize: missing required field \"integer\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(IntValidator.prototype) as IntValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_integer = obj["integer"];
        if (!Number.isInteger(__raw_integer)) {
            errors.push("IntValidator.fromStringifiedJSON: field 'integer' must be an integer");
        }
        (instance as any).integer = __raw_integer;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// NonNaN validator
/**  */
export class NonNaNValidator {
  
  valid: number;

  constructor(props: {
    valid: number;
}){
    this.valid = props.valid;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NonNaNValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = NonNaNValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "NonNaNValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): NonNaNValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("NonNaNValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("valid" in obj)) {
        errors.push("NonNaNValidator.__deserialize: missing required field \"valid\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(NonNaNValidator.prototype) as NonNaNValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_valid = obj["valid"];
        if (Number.isNaN(__raw_valid)) {
            errors.push("NonNaNValidator.fromStringifiedJSON: field 'valid' must not be NaN");
        }
        (instance as any).valid = __raw_valid;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Finite validator
/**  */
export class FiniteValidator {
  
  finite: number;

  constructor(props: {
    finite: number;
}){
    this.finite = props.finite;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<FiniteValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = FiniteValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "FiniteValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): FiniteValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("FiniteValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("finite" in obj)) {
        errors.push("FiniteValidator.__deserialize: missing required field \"finite\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(FiniteValidator.prototype) as FiniteValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_finite = obj["finite"];
        if (!Number.isFinite(__raw_finite)) {
            errors.push("FiniteValidator.fromStringifiedJSON: field 'finite' must be finite");
        }
        (instance as any).finite = __raw_finite;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Positive validator
/**  */
export class PositiveValidator {
  
  positive: number;

  constructor(props: {
    positive: number;
}){
    this.positive = props.positive;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<PositiveValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = PositiveValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "PositiveValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): PositiveValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("PositiveValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("positive" in obj)) {
        errors.push("PositiveValidator.__deserialize: missing required field \"positive\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(PositiveValidator.prototype) as PositiveValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_positive = obj["positive"];
        if (__raw_positive <= 0) {
            errors.push("PositiveValidator.fromStringifiedJSON: field 'positive' must be positive");
        }
        (instance as any).positive = __raw_positive;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// NonNegative validator
/**  */
export class NonNegativeValidator {
  
  nonNegative: number;

  constructor(props: {
    nonNegative: number;
}){
    this.nonNegative = props.nonNegative;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NonNegativeValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = NonNegativeValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "NonNegativeValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): NonNegativeValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("NonNegativeValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("nonNegative" in obj)) {
        errors.push("NonNegativeValidator.__deserialize: missing required field \"nonNegative\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(NonNegativeValidator.prototype) as NonNegativeValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_nonNegative = obj["nonNegative"];
        if (__raw_nonNegative < 0) {
            errors.push("NonNegativeValidator.fromStringifiedJSON: field 'nonNegative' must be non-negative");
        }
        (instance as any).nonNegative = __raw_nonNegative;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Negative validator
/**  */
export class NegativeValidator {
  
  negative: number;

  constructor(props: {
    negative: number;
}){
    this.negative = props.negative;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NegativeValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = NegativeValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "NegativeValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): NegativeValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("NegativeValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("negative" in obj)) {
        errors.push("NegativeValidator.__deserialize: missing required field \"negative\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(NegativeValidator.prototype) as NegativeValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_negative = obj["negative"];
        if (__raw_negative >= 0) {
            errors.push("NegativeValidator.fromStringifiedJSON: field 'negative' must be negative");
        }
        (instance as any).negative = __raw_negative;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// NonPositive validator
/**  */
export class NonPositiveValidator {
  
  nonPositive: number;

  constructor(props: {
    nonPositive: number;
}){
    this.nonPositive = props.nonPositive;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NonPositiveValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = NonPositiveValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "NonPositiveValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): NonPositiveValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("NonPositiveValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("nonPositive" in obj)) {
        errors.push("NonPositiveValidator.__deserialize: missing required field \"nonPositive\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(NonPositiveValidator.prototype) as NonPositiveValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_nonPositive = obj["nonPositive"];
        if (__raw_nonPositive > 0) {
            errors.push("NonPositiveValidator.fromStringifiedJSON: field 'nonPositive' must be non-positive");
        }
        (instance as any).nonPositive = __raw_nonPositive;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// MultipleOf validator
/**  */
export class MultipleOfValidator {
  
  multiple: number;

  constructor(props: {
    multiple: number;
}){
    this.multiple = props.multiple;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<MultipleOfValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = MultipleOfValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "MultipleOfValidator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): MultipleOfValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("MultipleOfValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("multiple" in obj)) {
        errors.push("MultipleOfValidator.__deserialize: missing required field \"multiple\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(MultipleOfValidator.prototype) as MultipleOfValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_multiple = obj["multiple"];
        if (__raw_multiple % 5 !== 0) {
            errors.push("MultipleOfValidator.fromStringifiedJSON: field 'multiple' must be a multiple of 5");
        }
        (instance as any).multiple = __raw_multiple;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Uint8 validator
/**  */
export class Uint8Validator {
  
  byte: number;

  constructor(props: {
    byte: number;
}){
    this.byte = props.byte;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<Uint8Validator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = Uint8Validator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "Uint8Validator.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): Uint8Validator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("Uint8Validator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("byte" in obj)) {
        errors.push("Uint8Validator.__deserialize: missing required field \"byte\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(Uint8Validator.prototype) as Uint8Validator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_byte = obj["byte"];
        if (!Number.isInteger(__raw_byte) || __raw_byte < 0 || __raw_byte > 255) {
            errors.push("Uint8Validator.fromStringifiedJSON: field 'byte' must be a uint8 (0-255)");
        }
        (instance as any).byte = __raw_byte;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}