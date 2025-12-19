import { test } from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { expandSync } = require("macroforge");

const repoRoot = path.resolve(
  path.dirname(new URL(import.meta.url).pathname),
  "..",
  "..",
);

function expandFile(relPath) {
  const filePath = path.join(repoRoot, relPath);
  const code = fs.readFileSync(filePath, "utf8");
  return expandSync(code, filePath, { keepDecorators: false });
}

function assertDecoratorsStripped(output, fileLabel) {
  assert.ok(
    !output.includes("@derive"),
    `${fileLabel}: @derive should be stripped from expanded output`,
  );
}

function assertMethodsGenerated(
  output,
  fileLabel,
  serializeMethod = "serialize",
) {
  // Debug macro now generates static toString method
  assert.ok(
    /static\s+toString\s*\(value:/.test(output),
    `${fileLabel}: expected generated static toString(value:) implementation`,
  );
  // Users can choose any api they want, like JSON uses instance methods. Built-in, in order to unify the API across classes, interfaces, and enum, like Serialize, use static
  // Check for either static or instance method depending on the macro
  if (serializeMethod === "toJSON") {
    // JSON macro generates instance method
    const methodPattern = new RegExp(`${serializeMethod}\\s*\\(\\).*?\\{`);
    assert.ok(
      methodPattern.test(output),
      `${fileLabel}: expected generated ${serializeMethod}() instance method`,
    );
  } else {
    // Serialize macro generates static method
    const methodPattern = new RegExp(
      `static\\s+${serializeMethod}\\s*\\(value:`,
    );
    assert.ok(
      methodPattern.test(output),
      `${fileLabel}: expected generated static ${serializeMethod}(value:) method`,
    );
  }
}

test("vanilla: decorators stripped and methods generated", () => {
  const { code } = expandFile("playground/vanilla/src/user.ts");
  assertDecoratorsStripped(code, "vanilla/user.ts");
  // vanilla uses @derive(JSON) which generates toJSON()
  assertMethodsGenerated(code, "vanilla/user.ts", "toJSON");
});

test("svelte: decorators stripped and methods generated", () => {
  const { code } = expandFile("playground/svelte/src/lib/demo/macro-user.ts");
  assertDecoratorsStripped(code, "svelte/macro-user.ts");
  // svelte uses @derive(Serialize) which generates serialize()
  assertMethodsGenerated(code, "svelte/macro-user.ts", "serialize");
});
