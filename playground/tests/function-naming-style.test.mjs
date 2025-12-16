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
  ];
}

describe("function naming style (prefix only)", () => {
  test("prefix naming style is default", () => {
    const projectDir = mkTempProjectDir();
    try {
      writeProjectFiles(projectDir, {});

      const cases = buildMacroCases(projectDir);
      const results = runExpandCases({ projectDir, cases });
      expectAllNoErrors(results);

      const codeByName = Object.fromEntries(
        Object.entries(results).map(([k, v]) => [k, v.code]),
      );

      // Verify prefix style function names (typeName + FunctionName)
      expectContains(
        codeByName["default-interface"],
        "export function userDefaultValue",
        "prefix default-interface",
      );
      expectContains(
        codeByName["serialize-interface"],
        "export function userSerialize",
        "prefix serialize-interface",
      );
      expectContains(
        codeByName["deserialize-interface"],
        "export function userDeserialize",
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
    } finally {
      fs.rmSync(projectDir, { recursive: true, force: true });
    }
  });
});
