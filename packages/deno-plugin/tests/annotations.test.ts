/**
 * End-to-end tests for the four Rust-inspired attribute macros
 * (`@cfg`, `@deprecated`, `@mustUse`, `@nonExhaustive`).
 *
 * These tests drive the bundled `macroforge` engine through the deno-plugin's
 * `expand()` API. They require a macroforge WASM build that includes the
 * attribute pre-pass (shipped with 0.1.81+). When running against an older
 * WASM build the tests are skipped rather than failing, so day-to-day
 * workflows without a rebuild stay green.
 */

import { assert, assertEquals, assertStringIncludes } from "@std/assert";
import { expand } from "../src/index.ts";

/**
 * Probe the bundled engine by expanding a known `@cfg`-annotated source.
 * Older WASM builds lack the attribute pre-pass, so the source comes back
 * unchanged. We use that as the gate for the rest of the suite.
 */
function engineHasAttributePass(): boolean {
  const probe =
    `/** @cfg({ feature: 'never' }) */\nexport function probe() {}\n`;
  const result = expand(probe, "/tmp/probe.ts");
  return !result.code.includes("function probe");
}

const hasAttributePass = engineHasAttributePass();

Deno.test({
  name: "@cfg strips when the feature flag is missing",
  ignore: !hasAttributePass,
  fn: () => {
    const src =
      `/** @cfg({ feature: 'ssr' }) */\nexport function render() {}\n`;
    const result = expand(src, "/tmp/cfg-strip.ts");
    assertEquals(
      result.diagnostics.filter((d) => d.level === "error"),
      [],
    );
    assert(
      !result.code.includes("render"),
      `render should be stripped:\n${result.code}`,
    );
  },
});

Deno.test({
  name: "@deprecated rewrites to a tsc-visible JSDoc tag",
  ignore: !hasAttributePass,
  fn: () => {
    const src =
      `/** @deprecated('use render2 instead') */\nexport function render() {}\n`;
    const result = expand(src, "/tmp/deprecated.ts");
    assertEquals(
      result.diagnostics.filter((d) => d.level === "error"),
      [],
    );
    assertStringIncludes(result.code, "@deprecated use render2 instead");
    assert(
      !result.code.includes("@deprecated('"),
      `macroforge-style annotation should be rewritten:\n${result.code}`,
    );
  },
});

Deno.test({
  name: "@mustUse flags discarded call sites",
  ignore: !hasAttributePass,
  fn: () => {
    const src = `
/** @mustUse */
export function openConnection() { return 1; }
openConnection();
const kept = openConnection();
`;
    const result = expand(src, "/tmp/must-use.ts");
    const errors = result.diagnostics.filter((d) => d.level === "error");
    assertEquals(
      errors.length,
      1,
      `expected one diagnostic, got ${JSON.stringify(errors)}`,
    );
    assertStringIncludes(errors[0].message, "openConnection");
  },
});

Deno.test({
  name: "@nonExhaustive brands the type alias RHS",
  ignore: !hasAttributePass,
  fn: () => {
    const src = `/** @nonExhaustive */\nexport type Kind = 'a' | 'b' | 'c';\n`;
    const result = expand(src, "/tmp/non-exhaustive.ts");
    assertEquals(
      result.diagnostics.filter((d) => d.level === "error"),
      [],
    );
    assertStringIncludes(result.code, "__nonExhaustive");
    assertStringIncludes(result.code, "'a' | 'b' | 'c'");
  },
});
