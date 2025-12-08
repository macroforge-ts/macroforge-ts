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

  constructor(
    id: number,
    name: string,
    email: string,
    secretToken: string,
    isActive: boolean,
    score: number
  ) {
    this.id = id;
    this.name = name;
    this.email = email;
    this.secretToken = secretToken;
    this.isActive = isActive;
    this.score = score;
  }

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

    hashCode(): number {
    let hash = 0;
    hash = (hash * 31 + (this.id ? this.id.toString().charCodeAt(0) : 0)) | 0;
    hash = (hash * 31 + (this.name ? this.name.toString().charCodeAt(0) : 0)) | 0;
    hash = (hash * 31 + (this.email ? this.email.toString().charCodeAt(0) : 0)) | 0;
    hash = (hash * 31 + (this.secretToken ? this.secretToken.toString().charCodeAt(0) : 0)) | 0;
    hash = (hash * 31 + (this.isActive ? this.isActive.toString().charCodeAt(0) : 0)) | 0;
    hash = (hash * 31 + (this.score ? this.score.toString().charCodeAt(0) : 0)) | 0;
    return hash;
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

    static fromJSON(data: unknown): AllMacrosTestClass {
    if (typeof data !== "object" || data === null || Array.isArray(data)) {
        throw new Error("AllMacrosTestClass.fromJSON: expected an object, got " + (Array.isArray(data) ? "array" : typeof data));
    }
    const obj = data as Record<string, unknown>;
    if (!("id" in obj)) {
        throw new Error("AllMacrosTestClass.fromJSON: missing required field \"id\"");
    }
    if (!("name" in obj)) {
        throw new Error("AllMacrosTestClass.fromJSON: missing required field \"name\"");
    }
    if (!("email" in obj)) {
        throw new Error("AllMacrosTestClass.fromJSON: missing required field \"email\"");
    }
    if (!("secretToken" in obj)) {
        throw new Error("AllMacrosTestClass.fromJSON: missing required field \"secretToken\"");
    }
    if (!("isActive" in obj)) {
        throw new Error("AllMacrosTestClass.fromJSON: missing required field \"isActive\"");
    }
    if (!("score" in obj)) {
        throw new Error("AllMacrosTestClass.fromJSON: missing required field \"score\"");
    }
    const instance = new AllMacrosTestClass();
    const __raw_id = obj["id"];
    instance.id = __raw_id as number;
    const __raw_name = obj["name"];
    instance.name = __raw_name as string;
    const __raw_email = obj["email"];
    instance.email = __raw_email as string;
    const __raw_secretToken = obj["secretToken"];
    instance.secretToken = __raw_secretToken as string;
    const __raw_isActive = obj["isActive"];
    instance.isActive = __raw_isActive as boolean;
    const __raw_score = obj["score"];
    instance.score = __raw_score as number;
    return instance;
}
}

// Pre-instantiated test instance for e2e tests
export const testInstance = new AllMacrosTestClass(
  42,
  "Test User",
  "test@example.com",
  "secret-token-123",
  true,
  100
);