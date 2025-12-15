import { describe, test } from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { repoRoot } from "./test-utils.mjs";

const macroforgeEntry = path.join(repoRoot, "crates/macroforge_ts/index.js");

const RUNNER_SCRIPT = `
  const fs = require("node:fs");
  const { expandSync } = require(process.argv[1]);
  const cases = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
  const out = {};
  for (const c of cases) {
    const code = fs.readFileSync(c.filePath, "utf8");
    const res = expandSync(code, c.filePath, { keepDecorators: false });
    out[c.name] = { code: res.code, diagnostics: res.diagnostics };
  }
  process.stdout.write(JSON.stringify(out));
`;

function mkTempProjectDir() {
  const base = path.join(repoRoot, "playground/tests/.tmp-projects");
  fs.mkdirSync(base, { recursive: true });
  return fs.mkdtempSync(path.join(base, "proj-"));
}

function writeProjectFiles(projectDir, macroforgeConfig) {
  fs.writeFileSync(
    path.join(projectDir, "package.json"),
    JSON.stringify({ name: "tmp", private: true, type: "module" }, null, 2),
  );
  fs.writeFileSync(
    path.join(projectDir, "macroforge.json"),
    JSON.stringify(macroforgeConfig ?? {}, null, 2),
  );
}

function runExpandCases({ projectDir, cases }) {
  const casesPath = path.join(projectDir, "cases.json");
  fs.writeFileSync(casesPath, JSON.stringify(cases, null, 2));

  const stdout = execFileSync(
    process.execPath,
    ["-e", RUNNER_SCRIPT, macroforgeEntry, casesPath],
    { cwd: projectDir, encoding: "utf8" },
  );
  return JSON.parse(stdout);
}

function expectAllNoErrors(results) {
  for (const [name, result] of Object.entries(results)) {
    const errors = (result.diagnostics ?? []).filter((d) => d.level === "error");
    assert.equal(errors.length, 0, `${name}: expected no error diagnostics`);
  }
}

function expectContains(haystack, needle, label) {
  assert.ok(haystack.includes(needle), `${label}: expected to include ${needle}`);
}

function expectMatches(haystack, re, label) {
  assert.ok(re.test(haystack), `${label}: expected to match ${re}`);
}

function buildMacroCases(projectDir) {
  const mk = (name, code) => {
    const filePath = path.join(projectDir, `${name}.ts`);
    fs.writeFileSync(filePath, code);
    return { name, filePath };
  };

  return [
    mk(
      "default-interface",
      `
      /** @derive(Default) */
      export interface User { id: string }
    `,
    ),
    mk(
      "serialize-interface",
      `
      /** @derive(Serialize) */
      export interface User { id: string }
    `,
    ),
    mk(
      "deserialize-interface",
      `
      /** @derive(Deserialize) */
      export interface User { id: string }
    `,
    ),
    mk(
      "clone-interface",
      `
      /** @derive(Clone) */
      export interface User { id: string }
    `,
    ),
    mk(
      "debug-interface",
      `
      /** @derive(Debug) */
      export interface User { id: string }
    `,
    ),
    mk(
      "partial-eq-interface",
      `
      /** @derive(PartialEq) */
      export interface User { id: string }
    `,
    ),
    mk(
      "partial-ord-interface",
      `
      /** @derive(PartialOrd) */
      export interface User { id: string }
    `,
    ),
    mk(
      "ord-interface",
      `
      /** @derive(Ord) */
      export interface User { id: string }
    `,
    ),
    mk(
      "hash-interface",
      `
      /** @derive(Hash) */
      export interface User { id: string }
    `,
    ),
    mk(
      "external-imports",
      `
      import type { Metadata } from "./metadata.svelte";

      /** @derive(Default, Serialize, Deserialize) */
      export interface User { metadata: Metadata }
    `,
    ),
  ];
}

function writeExternalTypeStub(projectDir) {
  fs.writeFileSync(
    path.join(projectDir, "metadata.svelte.ts"),
    `
      /** @derive(Default, Serialize, Deserialize) */
      export interface Metadata { createdAt: string }
    `,
  );
}

function assertNamingStyle(results, namingStyleLabel) {
  const codeByName = Object.fromEntries(
    Object.entries(results).map(([k, v]) => [k, v.code]),
  );

  switch (namingStyleLabel) {
    case "suffix":
      expectContains(
        codeByName["default-interface"],
        "export function defaultValueUser",
        "suffix default-interface",
      );
      expectContains(
        codeByName["serialize-interface"],
        "export function toObjectUser",
        "suffix serialize-interface",
      );
      expectContains(
        codeByName["deserialize-interface"],
        "export function fromObjectUser",
        "suffix deserialize-interface",
      );
      expectContains(
        codeByName["clone-interface"],
        "export function cloneUser",
        "suffix clone-interface",
      );
      expectContains(
        codeByName["debug-interface"],
        "export function toStringUser",
        "suffix debug-interface",
      );
      expectContains(
        codeByName["partial-eq-interface"],
        "export function equalsUser",
        "suffix partial-eq-interface",
      );
      expectContains(
        codeByName["partial-ord-interface"],
        "export function partialCompareUser",
        "suffix partial-ord-interface",
      );
      expectContains(
        codeByName["ord-interface"],
        "export function compareUser",
        "suffix ord-interface",
      );
      expectContains(
        codeByName["hash-interface"],
        "export function hashCodeUser",
        "suffix hash-interface",
      );

      expectMatches(
        codeByName["external-imports"],
        /import\s+\{\s*__serializeMetadata\s*\}\s+from\s+['"]\.\/metadata\.svelte['"];/,
        "suffix external-imports",
      );
      expectMatches(
        codeByName["external-imports"],
        /import\s+\{\s*__deserializeMetadata\s*\}\s+from\s+['"]\.\/metadata\.svelte['"];/,
        "suffix external-imports",
      );
      // `metadata: Metadata` (non-null) should require a default value helper.
      expectMatches(
        codeByName["external-imports"],
        /import\s+\{\s*defaultValueMetadata\s*\}\s+from\s+['"]\.\/metadata\.svelte['"];/,
        "suffix external-imports",
      );
      expectContains(
        codeByName["external-imports"],
        "__serializeMetadata(",
        "suffix external-imports",
      );
      expectContains(
        codeByName["external-imports"],
        "__deserializeMetadata(",
        "suffix external-imports",
      );
      break;

    case "prefix":
      expectContains(
        codeByName["default-interface"],
        "export function userDefaultValue",
        "prefix default-interface",
      );
      expectContains(
        codeByName["serialize-interface"],
        "export function userToObject",
        "prefix serialize-interface",
      );
      expectContains(
        codeByName["deserialize-interface"],
        "export function userFromObject",
        "prefix deserialize-interface",
      );
      expectContains(
        codeByName["clone-interface"],
        "export function userClone",
        "prefix clone-interface",
      );
      expectContains(
        codeByName["debug-interface"],
        "export function userToString",
        "prefix debug-interface",
      );
      expectContains(
        codeByName["partial-eq-interface"],
        "export function userEquals",
        "prefix partial-eq-interface",
      );
      expectContains(
        codeByName["partial-ord-interface"],
        "export function userPartialCompare",
        "prefix partial-ord-interface",
      );
      expectContains(
        codeByName["ord-interface"],
        "export function userCompare",
        "prefix ord-interface",
      );
      expectContains(
        codeByName["hash-interface"],
        "export function userHashCode",
        "prefix hash-interface",
      );
      break;

    case "generic":
      expectMatches(
        codeByName["default-interface"],
        /export function defaultValue\s*<\s*T\s+extends\s+User\s*>/m,
        "generic default-interface",
      );
      // Serde macros use the generic style as "unsuffixed" (not `<T extends User>`).
      expectMatches(
        codeByName["serialize-interface"],
        /export function toObject\s*\(\s*value:\s*User\s*\)/m,
        "generic serialize-interface",
      );
      expectMatches(
        codeByName["deserialize-interface"],
        /export function fromObject\s*\(\s*obj:\s*unknown/m,
        "generic deserialize-interface",
      );
      expectMatches(
        codeByName["clone-interface"],
        /export function clone\s*<\s*T\s+extends\s+User\s*>/m,
        "generic clone-interface",
      );
      expectMatches(
        codeByName["debug-interface"],
        /export function toString\s*<\s*T\s+extends\s+User\s*>/m,
        "generic debug-interface",
      );
      expectMatches(
        codeByName["partial-eq-interface"],
        /export function equals\s*<\s*T\s+extends\s+User\s*>/m,
        "generic partial-eq-interface",
      );
      expectMatches(
        codeByName["partial-ord-interface"],
        /export function partialCompare\s*<\s*T\s+extends\s+User\s*>/m,
        "generic partial-ord-interface",
      );
      expectMatches(
        codeByName["ord-interface"],
        /export function compare\s*<\s*T\s+extends\s+User\s*>/m,
        "generic ord-interface",
      );
      expectMatches(
        codeByName["hash-interface"],
        /export function hashCode\s*<\s*T\s+extends\s+User\s*>/m,
        "generic hash-interface",
      );
      break;

    case "namespace":
      for (const [name, code] of Object.entries(codeByName)) {
        if (name === "external-imports") continue;
        expectContains(code, "export namespace User", `namespace ${name}`);
      }
      break;

    default:
      throw new Error(`Unknown naming style: ${namingStyleLabel}`);
  }
}

describe("functionNamingStyle (built-ins)", () => {
  const styles = [
    { label: "suffix (default)", config: {} },
    { label: "suffix", config: { functionNamingStyle: "suffix" } },
    { label: "prefix", config: { functionNamingStyle: "prefix" } },
    { label: "generic", config: { functionNamingStyle: "generic" } },
    { label: "namespace", config: { functionNamingStyle: "namespace" } },
  ];

  for (const style of styles) {
    test(style.label, () => {
      const projectDir = mkTempProjectDir();
      try {
        writeProjectFiles(projectDir, style.config);
        writeExternalTypeStub(projectDir);

        const cases = buildMacroCases(projectDir);
        const results = runExpandCases({ projectDir, cases });
        expectAllNoErrors(results);

        const namingStyleLabel = style.label === "suffix (default)" ? "suffix" : style.label;
        assertNamingStyle(results, namingStyleLabel);
      } finally {
        fs.rmSync(projectDir, { recursive: true, force: true });
      }
    });
  }
});
