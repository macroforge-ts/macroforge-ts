/**
 * Smoke test for `expand`. Uses a derive macro the engine ships with
 * built in (`Default`) so the test doesn't depend on any external
 * macro packages or a project config file.
 */

import { assertEquals, assertStringIncludes } from "@std/assert";
import { expand } from "../src/index.ts";

Deno.test("expand inlines a built-in derive macro", () => {
  const source = `/** @derive(Default) */\nexport interface Empty {}\n`;
  const result = expand(source, "/tmp/empty.ts");

  assertEquals(result.diagnostics.filter((d) => d.level === "error"), []);
  assertEquals(result.hasMacros, true);
  assertStringIncludes(result.code, "emptyDefault");
});

Deno.test("expand is a no-op for files without macros", () => {
  const source = `export const x = 1;\n`;
  const result = expand(source, "/tmp/plain.ts");

  assertEquals(result.diagnostics.filter((d) => d.level === "error"), []);
  assertEquals(result.hasMacros, false);
  assertEquals(result.code, source);
});
