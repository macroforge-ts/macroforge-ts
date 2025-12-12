import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * Edge case test classes for comprehensive deserializer validation testing.
 */

// Multiple validators on single field
/**  */
export class MultipleValidatorsTest {
  
  text: string;

  constructor(props: {
    text: string;
}){
    this.text = props.text;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<MultipleValidatorsTest, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = MultipleValidatorsTest.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "MultipleValidatorsTest.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): MultipleValidatorsTest | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("MultipleValidatorsTest.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("text" in obj)) {
        errors.push("MultipleValidatorsTest.__deserialize: missing required field \"text\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(MultipleValidatorsTest.prototype) as MultipleValidatorsTest;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_text = obj["text"];
        if (__raw_text.length === 0) {
            errors.push("MultipleValidatorsTest.fromStringifiedJSON: field 'text' must not be empty");
        }
        if (__raw_text.length > 100) {
            errors.push("MultipleValidatorsTest.fromStringifiedJSON: field 'text' must have at most 100 characters");
        }
        if (__raw_text !== __raw_text.trim()) {
            errors.push("MultipleValidatorsTest.fromStringifiedJSON: field 'text' must be trimmed (no leading/trailing whitespace)");
        }
        (instance as any).text = __raw_text;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Custom error message
/**  */
export class CustomMessageTest {
  
  email: string;

  constructor(props: {
    email: string;
}){
    this.email = props.email;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<CustomMessageTest, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = CustomMessageTest.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "CustomMessageTest.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): CustomMessageTest | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("CustomMessageTest.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("email" in obj)) {
        errors.push("CustomMessageTest.__deserialize: missing required field \"email\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(CustomMessageTest.prototype) as CustomMessageTest;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_email = obj["email"];
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_email)) {
            errors.push("CustomMessageTest.fromStringifiedJSON: field 'email' Please enter a valid email address");
        }
        (instance as any).email = __raw_email;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Mixed validators with custom message
/**  */
export class MixedValidatorsTest {
  
  email: string;

  constructor(props: {
    email: string;
}){
    this.email = props.email;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<MixedValidatorsTest, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = MixedValidatorsTest.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "MixedValidatorsTest.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): MixedValidatorsTest | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("MixedValidatorsTest.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("email" in obj)) {
        errors.push("MixedValidatorsTest.__deserialize: missing required field \"email\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(MixedValidatorsTest.prototype) as MixedValidatorsTest;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_email = obj["email"];
        if (__raw_email.length === 0) {
            errors.push("MixedValidatorsTest.fromStringifiedJSON: field 'email' must not be empty");
        }
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_email)) {
            errors.push("MixedValidatorsTest.fromStringifiedJSON: field 'email' Invalid email format");
        }
        (instance as any).email = __raw_email;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Combined string validators
/**  */
export class CombinedStringValidatorsTest {
  
  username: string;

  constructor(props: {
    username: string;
}){
    this.username = props.username;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<CombinedStringValidatorsTest, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = CombinedStringValidatorsTest.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "CombinedStringValidatorsTest.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): CombinedStringValidatorsTest | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("CombinedStringValidatorsTest.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("username" in obj)) {
        errors.push("CombinedStringValidatorsTest.__deserialize: missing required field \"username\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(CombinedStringValidatorsTest.prototype) as CombinedStringValidatorsTest;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_username = obj["username"];
        if (__raw_username.length < 3) {
            errors.push("CombinedStringValidatorsTest.fromStringifiedJSON: field 'username' must have at least 3 characters");
        }
        if (__raw_username.length > 20) {
            errors.push("CombinedStringValidatorsTest.fromStringifiedJSON: field 'username' must have at most 20 characters");
        }
        if (__raw_username !== __raw_username.toLowerCase()) {
            errors.push("CombinedStringValidatorsTest.fromStringifiedJSON: field 'username' must be lowercase");
        }
        (instance as any).username = __raw_username;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Combined number validators
/**  */
export class CombinedNumberValidatorsTest {
  
  score: number;

  constructor(props: {
    score: number;
}){
    this.score = props.score;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<CombinedNumberValidatorsTest, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = CombinedNumberValidatorsTest.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "CombinedNumberValidatorsTest.fromStringifiedJSON: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): CombinedNumberValidatorsTest | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("CombinedNumberValidatorsTest.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("score" in obj)) {
        errors.push("CombinedNumberValidatorsTest.__deserialize: missing required field \"score\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(CombinedNumberValidatorsTest.prototype) as CombinedNumberValidatorsTest;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_score = obj["score"];
        if (!Number.isInteger(__raw_score)) {
            errors.push("CombinedNumberValidatorsTest.fromStringifiedJSON: field 'score' must be an integer");
        }
        if (__raw_score <= 0) {
            errors.push("CombinedNumberValidatorsTest.fromStringifiedJSON: field 'score' must be positive");
        }
        if (__raw_score >= 1000) {
            errors.push("CombinedNumberValidatorsTest.fromStringifiedJSON: field 'score' must be less than 1000");
        }
        (instance as any).score = __raw_score;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}