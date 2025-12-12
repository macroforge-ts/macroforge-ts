import { SerializeContext } from "macroforge/serde";
import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * Comprehensive test class demonstrating all available macros.
 * Used for Playwright e2e tests to verify macro expansion works at runtime.
 */


export class AllMacrosTestClass {
  
  id: number;

  name: string;

  email: string;

  
  secretToken: string;

  isActive: boolean;

  score: number;

  toString(): string {
    const parts: string[] = [];
    parts.push("identifier: " + this.id);
    parts.push("name: " + this.name);
    parts.push("email: " + this.email);
    parts.push("isActive: " + this.isActive);
    parts.push("score: " + this.score);
    return "AllMacrosTestClass { " + parts.join(", ") + " }";
}

  clone(): AllMacrosTestClass {
    const cloned = Object.create(Object.getPrototypeOf(this));
    cloned.id = this.id;
    cloned.name = this.name;
    cloned.email = this.email;
    cloned.secretToken = this.secretToken;
    cloned.isActive = this.isActive;
    cloned.score = this.score;
    return cloned;
}

  equals(other: unknown): boolean {
    if (this === other) return true;
    if (!(other instanceof AllMacrosTestClass)) return false;
    const typedOther = other as AllMacrosTestClass;
    return this.id === typedOther.id && this.name === typedOther.name && this.email === typedOther.email && this.secretToken === typedOther.secretToken && this.isActive === typedOther.isActive && this.score === typedOther.score;
}

  toStringifiedJSON(): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(this.__serialize(ctx));
}

  toObject(): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return this.__serialize(ctx);
}

  __serialize(ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(this);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(this);
    const result: Record<string, unknown> = {
        __type: "AllMacrosTestClass",
        __id
    };
    result["id"] = this.id;
    result["name"] = this.name;
    result["email"] = this.email;
    result["secretToken"] = this.secretToken;
    result["isActive"] = this.isActive;
    result["score"] = this.score;
    return result;
}

  constructor(props: {
    id: number;
    name: string;
    email: string;
    secretToken: string;
    isActive: boolean;
    score: number;
}){
    this.id = props.id;
    this.name = props.name;
    this.email = props.email;
    this.secretToken = props.secretToken;
    this.isActive = props.isActive;
    this.score = props.score;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<AllMacrosTestClass, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return AllMacrosTestClass.fromObject(raw, opts);
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

  static fromObject(obj: unknown, opts?: DeserializeOptions): Result<AllMacrosTestClass, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = AllMacrosTestClass.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "AllMacrosTestClass.fromObject: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): AllMacrosTestClass | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "AllMacrosTestClass.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("id" in obj)) {
        errors.push({
            field: "id",
            message: "missing required field"
        });
    }
    if (!("name" in obj)) {
        errors.push({
            field: "name",
            message: "missing required field"
        });
    }
    if (!("email" in obj)) {
        errors.push({
            field: "email",
            message: "missing required field"
        });
    }
    if (!("secretToken" in obj)) {
        errors.push({
            field: "secretToken",
            message: "missing required field"
        });
    }
    if (!("isActive" in obj)) {
        errors.push({
            field: "isActive",
            message: "missing required field"
        });
    }
    if (!("score" in obj)) {
        errors.push({
            field: "score",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(AllMacrosTestClass.prototype) as AllMacrosTestClass;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_id = obj["id"];
        (instance as any).id = __raw_id;
    }
    {
        const __raw_name = obj["name"];
        (instance as any).name = __raw_name;
    }
    {
        const __raw_email = obj["email"];
        (instance as any).email = __raw_email;
    }
    {
        const __raw_secretToken = obj["secretToken"];
        (instance as any).secretToken = __raw_secretToken;
    }
    {
        const __raw_isActive = obj["isActive"];
        (instance as any).isActive = __raw_isActive;
    }
    {
        const __raw_score = obj["score"];
        (instance as any).score = __raw_score;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}
}

// Pre-instantiated test instance for e2e tests
export const testInstance = new AllMacrosTestClass({
  id: 42,
  name: "Test User",
  email: "test@example.com",
  secretToken: "secret-token-123",
  isActive: true,
  score: 100,
});