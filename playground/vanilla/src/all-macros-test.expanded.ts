import { Result } from "macroforge/result";
/**
 * Comprehensive test class demonstrating all available macros.
 * Used for Playwright e2e tests to verify macro expansion works at runtime.
 */

/**  */
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

  toJSON(): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    result["id"] = this.id;
    result["name"] = this.name;
    result["email"] = this.email;
    result["secretToken"] = this.secretToken;
    result["isActive"] = this.isActive;
    result["score"] = this.score;
    return result;
}

  constructor(init: {
    id: number;
    name: string;
    email: string;
    secretToken: string;
    isActive: boolean;
    score: number;
}){
    this.id = init.id;
    this.name = init.name;
    this.email = init.email;
    this.secretToken = init.secretToken;
    this.isActive = init.isActive;
    this.score = init.score;
}

  static fromJSON(data: unknown): Result<AllMacrosTestClass, string[]> {
    if (typeof data !== "object" || data === null || Array.isArray(data)) {
        return Result.err([
            "AllMacrosTestClass.fromJSON: expected an object, got " + (Array.isArray(data) ? "array" : typeof data)
        ]);
    }
    const obj = data as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
        errors.push("AllMacrosTestClass.fromJSON: missing required field \"id\"");
    }
    if (!("name" in obj)) {
        errors.push("AllMacrosTestClass.fromJSON: missing required field \"name\"");
    }
    if (!("email" in obj)) {
        errors.push("AllMacrosTestClass.fromJSON: missing required field \"email\"");
    }
    if (!("secretToken" in obj)) {
        errors.push("AllMacrosTestClass.fromJSON: missing required field \"secretToken\"");
    }
    if (!("isActive" in obj)) {
        errors.push("AllMacrosTestClass.fromJSON: missing required field \"isActive\"");
    }
    if (!("score" in obj)) {
        errors.push("AllMacrosTestClass.fromJSON: missing required field \"score\"");
    }
    const __raw_id = obj["id"];
    const __raw_name = obj["name"];
    const __raw_email = obj["email"];
    const __raw_secretToken = obj["secretToken"];
    const __raw_isActive = obj["isActive"];
    const __raw_score = obj["score"];
    if (errors.length > 0) {
        return Result.err(errors);
    }
    const init: Record<string, unknown> = {};
    init.id = __raw_id as number;
    init.name = __raw_name as string;
    init.email = __raw_email as string;
    init.secretToken = __raw_secretToken as string;
    init.isActive = __raw_isActive as boolean;
    init.score = __raw_score as number;
    return Result.ok(new AllMacrosTestClass(init as any));
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