# refactor(testground): name the stricter-library check by what it checks

- **Date:** 2026-10-12 15:17
- **Commit:** 8a81aa85
- **Area:** Testground, CLI docs

## Problem

The second type check of generated code was named after the library it happens to use, in its
tsconfig, its test suite and the CLI docs. What matters is what it proves: the code type-checks
against a standard library with no `any` in a value position.

## Fix

The tsconfig is `tsconfig.strict-lib.json` and the suite is `strict-lib`. The docs describe the
check without naming a library. The suite recognises a replacement lib among a project's files by
its file name, `es5.d.ts` where TypeScript's own is `lib.es5.d.ts`.

The branch and this directory are renamed to match, and two commit messages reworded, so the commits
recorded here from 060f8168 on have new hashes.

## Verified

The `strict-lib` suite passes, 5 of 5, after the rename. `pixi run docs:mcp` regenerated the two MCP
pages from the website's.
