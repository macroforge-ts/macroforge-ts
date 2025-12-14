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

function assertMethodsGenerated(output, fileLabel, serializeMethod = "toObject") {
  assert.ok(
    /toString\s*\(\).*?\{/.test(output),
    `${fileLabel}: expected generated toString() implementation`,
  );
  const methodPattern = new RegExp(`${serializeMethod}\\s*\\(\\).*?\\{`);
  assert.ok(
    methodPattern.test(output),
    `${fileLabel}: expected generated ${serializeMethod}() implementation`,
  );
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
  // svelte uses @derive(Serialize) which generates toObject()
  assertMethodsGenerated(code, "svelte/macro-user.ts", "toObject");
});
