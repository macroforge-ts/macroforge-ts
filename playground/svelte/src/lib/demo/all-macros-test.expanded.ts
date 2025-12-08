import { Result } from "macroforge/result";
/**
 * Comprehensive test class for Svelte playground.
 * Uses all available macros for Playwright e2e testing.
 */

/**  */
export interface SvelteAllMacrosTest {
  id: string;
  title: string;
  content: string;

  apiKey: string;
  count: number;
  enabled: boolean;
}

export namespace SvelteAllMacrosTest {
  export function toString(self: SvelteAllMacrosTest): string {
    const parts: string[] = [];
    parts.push("testId: " + self.id);
    parts.push("title: " + self.title);
    parts.push("content: " + self.content);
    parts.push("count: " + self.count);
    parts.push("enabled: " + self.enabled);
    return "SvelteAllMacrosTest { " + parts.join(", ") + " }";
  }
}

export namespace SvelteAllMacrosTest {
  export function clone(self: SvelteAllMacrosTest): SvelteAllMacrosTest {
    return {
      id: self.id,
      title: self.title,
      content: self.content,
      apiKey: self.apiKey,
      count: self.count,
      enabled: self.enabled,
    };
  }
}

export namespace SvelteAllMacrosTest {
  export function equals(
    self: SvelteAllMacrosTest,
    other: SvelteAllMacrosTest,
  ): boolean {
    if (self === other) return true;
    return (
      self.id === other.id &&
      self.title === other.title &&
      self.content === other.content &&
      self.apiKey === other.apiKey &&
      self.count === other.count &&
      self.enabled === other.enabled
    );
  }
}

export namespace SvelteAllMacrosTest {
  export function toJSON(self: SvelteAllMacrosTest): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    result["id"] = self.id;
    result["title"] = self.title;
    result["content"] = self.content;
    result["apiKey"] = self.apiKey;
    result["count"] = self.count;
    result["enabled"] = self.enabled;
    return result;
  }
}

export namespace SvelteAllMacrosTest {
  export function is(data: unknown): data is SvelteAllMacrosTest {
    if (typeof data !== "object" || data === null || Array.isArray(data)) {
      return false;
    }
    const obj = data as Record<string, unknown>;
    if (!("id" in obj) || typeof obj["id"] !== "string") {
      return false;
    }
    if (!("title" in obj) || typeof obj["title"] !== "string") {
      return false;
    }
    if (!("content" in obj) || typeof obj["content"] !== "string") {
      return false;
    }
    if (!("apiKey" in obj) || typeof obj["apiKey"] !== "string") {
      return false;
    }
    if (!("count" in obj) || typeof obj["count"] !== "number") {
      return false;
    }
    if (!("enabled" in obj) || typeof obj["enabled"] !== "boolean") {
      return false;
    }
    return true;
  }
  export function fromJSON(
    data: unknown,
  ): Result<SvelteAllMacrosTest, string[]> {
    if (!is(data)) {
      return Result.err([
        "SvelteAllMacrosTest.fromJSON: type validation failed",
      ]);
    }
    const obj = data as Record<string, unknown>;
    const errors: string[] = [];
    if (errors.length > 0) {
      return Result.err(errors);
    }
    return Result.ok(data);
  }
}

export namespace SvelteAllMacrosTest {
  export function make(
    id: string,
    title: string,
    content: string,
    apiKey: string,
    count: number,
    enabled: boolean,
  ): SvelteAllMacrosTest {
    return { id, title, content, apiKey, count, enabled };
  }
}

// Pre-instantiated test instance for e2e tests
export const svelteTestInstance = SvelteAllMacrosTest.make(
  "svelte-001",
  "Svelte Test",
  "Testing all macros in SvelteKit",
  "sk-secret-key",
  42,
  true,
);
