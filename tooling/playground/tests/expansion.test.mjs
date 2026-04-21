import assert from 'node:assert/strict';
import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { test } from 'node:test';
import { initExternalMacros } from './test-utils.mjs';

const require = createRequire(import.meta.url);
const { expandSync } = require('macroforge');

// Register external macro callbacks for WASM builds.
// Required because the WASM build cannot spawn Node subprocesses
// to resolve external macro packages (like @playground/macro).
initExternalMacros(require('macroforge'));

const repoRoot = path.resolve(
    path.dirname(new URL(import.meta.url).pathname),
    '..',
    '..'
);

function expandFile(relPath) {
    const filePath = path.join(repoRoot, relPath);
    const code = fs.readFileSync(filePath, 'utf8');
    return expandSync(code, filePath, { keepDecorators: false });
}

function assertDecoratorsStripped(output, fileLabel) {
    assert.ok(
        !output.includes('@derive'),
        `${fileLabel}: @derive should be stripped from expanded output`
    );
}

function assertMethodsGenerated(
    output,
    fileLabel,
    serializeMethod = 'serialize'
) {
    // Debug macro now generates static toString method
    assert.ok(
        /static\s+toString\s*\(value:/.test(output),
        `${fileLabel}: expected generated static toString(value:) implementation`
    );
    // Users can choose any api they want, like JSON uses instance methods. Built-in, in order to unify the API across classes, interfaces, and enum, like Serialize, use static
    // Check for either static or instance method depending on the macro
    if (serializeMethod === 'toJSON') {
        // JSON macro generates instance method
        const methodPattern = new RegExp(`${serializeMethod}\\s*\\(\\).*?\\{`);
        assert.ok(
            methodPattern.test(output),
            `${fileLabel}: expected generated ${serializeMethod}() instance method`
        );
    } else {
        // Serialize macro generates static method
        const methodPattern = new RegExp(
            `static\\s+${serializeMethod}\\s*\\(value:`
        );
        assert.ok(
            methodPattern.test(output),
            `${fileLabel}: expected generated static ${serializeMethod}(value:) method`
        );
    }
}

test('vanilla: decorators stripped and methods generated', () => {
    const { code } = expandFile('playground/vanilla/src/user.ts');
    assertDecoratorsStripped(code, 'vanilla/user.ts');
    // vanilla uses @derive(JSON) which generates toJSON()
    assertMethodsGenerated(code, 'vanilla/user.ts', 'toJSON');
});

// ---------------------------------------------------------------------------
// Attribute-macro pre-pass: @cfg / @deprecated / @mustUse / @nonExhaustive
//
// These tests assert against the playground fixtures directly. The Vite
// plugin runs the same pre-pass during a build, so passing here means the
// browser will see the same output that the e2e Playwright specs check.
// ---------------------------------------------------------------------------

// Walk up from `start` until a macroforge.config.* lands in the directory,
// stopping at the repo root. Returns null when no config is found — that's
// fine, the engine then uses defaults.
function findMacroforgeConfigPath(start) {
    let dir = path.dirname(start);
    while (dir.startsWith(repoRoot)) {
        for (const ext of ['.ts', '.mts', '.js', '.mjs', '.cjs']) {
            const candidate = path.join(dir, `macroforge.config${ext}`);
            if (fs.existsSync(candidate)) return candidate;
        }
        const parent = path.dirname(dir);
        if (parent === dir) break;
        dir = parent;
    }
    return null;
}

function expandPlaygroundAttributeFile(relPath) {
    const filePath = path.join(repoRoot, relPath);
    const code = fs.readFileSync(filePath, 'utf8');
    // Pull the playground's macroforge.config so `cfg.features` /
    // `cfg.target` flow through. expandSync caches the parsed config keyed
    // by configPath.
    const configPath = findMacroforgeConfigPath(filePath);
    if (configPath) {
        const { loadConfig } = require('macroforge');
        loadConfig(fs.readFileSync(configPath, 'utf8'), configPath);
    }
    return expandSync(code, filePath, {
        keepDecorators: false,
        configPath: configPath ?? undefined
    });
}

function assertAttributeFixtureExpanded(output, fileLabel) {
    // @cfg keeps `keptByPlayground` (feature 'playground' is in the config)
    // and `keptByWebTarget` (target 'web' matches).
    assert.ok(
        /export function keptByPlayground\b/.test(output),
        `${fileLabel}: keptByPlayground should survive @cfg`
    );
    assert.ok(
        /export function keptByWebTarget\b/.test(output),
        `${fileLabel}: keptByWebTarget should survive @cfg`
    );

    // @cfg strips `strippedByMissingFeature` and `strippedByNodeTarget`
    // entirely — the function declarations must be gone from the output.
    assert.ok(
        !/export function strippedByMissingFeature\b/.test(output),
        `${fileLabel}: strippedByMissingFeature should be stripped by @cfg`
    );
    assert.ok(
        !/export function strippedByNodeTarget\b/.test(output),
        `${fileLabel}: strippedByNodeTarget should be stripped by @cfg`
    );

    // @deprecated is rewritten from `@deprecated('msg', {...})` to a
    // tsc-readable `@deprecated msg` JSDoc tag.
    assert.ok(
        output.includes('@deprecated use renderV2 instead'),
        `${fileLabel}: @deprecated JSDoc rewrite missing`
    );
    assert.ok(
        !output.includes("@deprecated('"),
        `${fileLabel}: macroforge-style @deprecated annotation should be gone`
    );

    // @mustUse strips its JSDoc — the diagnostic is the action, not the markup.
    // Only match the annotation form `@mustUse(` or a JSDoc-style `/** @mustUse`
    // so narrative comments mentioning the feature survive.
    assert.ok(
        !output.includes('@mustUse('),
        `${fileLabel}: @mustUse(...) annotation should be stripped`
    );
    assert.ok(
        !/\/\*\*\s*@mustUse\b/.test(output),
        `${fileLabel}: bare @mustUse JSDoc annotation should be stripped`
    );

    // @nonExhaustive intersects the type alias's RHS with a brand sentinel.
    assert.ok(
        /readonly __nonExhaustive: unique symbol/.test(output),
        `${fileLabel}: @nonExhaustive brand intersection missing`
    );
    // Variants must survive — check each independently since the formatter
    // may change quote style (single vs double) between fixtures.
    for (const variant of ['green', 'yellow', 'red']) {
        assert.ok(
            output.includes(`'${variant}'`) || output.includes(`"${variant}"`),
            `${fileLabel}: @nonExhaustive should preserve variant '${variant}'`
        );
    }

    // No annotation tags should leak into the output. Match the JSDoc tag
    // form (`/** @cfg(`, `/** @nonExhaustive`) — narrative `//` comments
    // that happen to mention `@cfg(...)` are prose and must survive.
    assert.ok(
        !/\/\*\*\s*@cfg\s*\(/.test(output),
        `${fileLabel}: @cfg JSDoc tags should be stripped from kept declarations`
    );
    assert.ok(
        !/\/\*\*\s*@nonExhaustive\b/.test(output),
        `${fileLabel}: @nonExhaustive JSDoc tag should be stripped`
    );
}

test('vanilla: attribute pre-pass strips, brands, and rewrites annotations', () => {
    const { code, diagnostics } = expandPlaygroundAttributeFile(
        'playground/vanilla/src/attributes-test.ts'
    );
    assert.deepEqual(
        diagnostics.filter((d) => d.level === 'error'),
        [],
        `vanilla attributes-test.ts: unexpected error diagnostics: ${
            JSON.stringify(
                diagnostics
            )
        }`
    );
    assertAttributeFixtureExpanded(code, 'vanilla/attributes-test.ts');
});

test('svelte: attribute pre-pass strips, brands, and rewrites annotations', () => {
    const { code, diagnostics } = expandPlaygroundAttributeFile(
        'playground/svelte/src/lib/demo/attributes-demo.ts'
    );
    assert.deepEqual(
        diagnostics.filter((d) => d.level === 'error'),
        [],
        `svelte attributes-demo.ts: unexpected error diagnostics: ${
            JSON.stringify(
                diagnostics
            )
        }`
    );
    assertAttributeFixtureExpanded(code, 'svelte/attributes-demo.ts');
});

test('svelte: decorators stripped and methods generated', () => {
    const { code } = expandFile('playground/svelte/src/lib/demo/macro-user.ts');
    assertDecoratorsStripped(code, 'svelte/macro-user.ts');
    // svelte uses @derive(Serialize) which generates serialize()
    assertMethodsGenerated(code, 'svelte/macro-user.ts', 'serialize');
});

test('declarative macros: $vec expands inline at call sites', () => {
    const source = `import { macroRules } from "macroforge/rules";

const $vec = macroRules\`
  () => []
  ($($x:Expr),+) => [$($x),+]
\`;

const empty = $vec();
const xs = $vec(1, 2, 3);
`;
    const { code } = expandSync(source, '/tmp/decl.ts', {
        keepDecorators: false
    });
    assert.ok(
        !code.includes('const $vec = macroRules'),
        'macro definition should be stripped'
    );
    assert.ok(
        code.includes('const empty = [];'),
        `$vec() should expand to [], got: ${code}`
    );
    assert.ok(
        code.includes('const xs = [1,2,3];'),
        `$vec(1,2,3) should expand to [1,2,3], got: ${code}`
    );
});

test('declarative macros: hygiene renames __ identifiers', () => {
    const source = `import { macroRules } from "macroforge/rules";

const $withTemp = macroRules\`
  ($x:Expr) => {
    const __v = $x;
    __v + 1
  }
\`;

const result = $withTemp(10);
`;
    const { code } = expandSync(source, '/tmp/decl_hygiene.ts', {
        keepDecorators: false
    });
    assert.ok(
        code.includes('__v$1'),
        `expected hygienic rename __v$1 in output: ${code}`
    );
    assert.ok(
        code.includes('(() =>'),
        `expected IIFE wrap for block in expression position: ${code}`
    );
});

// Both declarative cross-file tests read the pre-built registry file
// written by `macroforge watch` (or the CLI `tsc`/`svelte-check` wrappers
// that run `ensure_type_registry_cache`). The WASM `scanProjectSync` can't
// walk the filesystem from inside the WASM sandbox — that path only works
// from the native CLI — so the tests pick up the on-disk registry
// directly, matching how the Vite plugin reads it in `buildStart`.
function loadDeclarativeRegistry(vanillaRoot) {
    const registryPath = path.join(
        vanillaRoot,
        '.macroforge',
        'declarative-registry.json'
    );
    if (!fs.existsSync(registryPath)) {
        return null;
    }
    return fs.readFileSync(registryPath, 'utf8');
}

test('declarative macros: project-wide registry file lists macro-defining files', () => {
    const vanillaRoot = path.join(repoRoot, 'playground', 'vanilla');
    const declarativeRegistryJson = loadDeclarativeRegistry(vanillaRoot);
    assert.ok(
        declarativeRegistryJson,
        `expected .macroforge/declarative-registry.json in ${vanillaRoot}. ` +
            'Run `macroforge tsc` in the vanilla playground once to populate it.'
    );

    const parsed = JSON.parse(declarativeRegistryJson);
    const files = Object.keys(parsed.by_file ?? {});
    assert.ok(
        files.some((f) => f.includes('decl_macros_lib')),
        `expected decl_macros_lib.ts in registry files, got: ${
            files.join(
                ', '
            )
        }`
    );
    // decl_macros_lib.ts defines $vec and $identity; declarative-macros.ts
    // defines $vec, $id, $withTemp. Total = 5 macros across 2 files.
    let macroCount = 0;
    for (const macros of Object.values(parsed.by_file ?? {})) {
        macroCount += Object.keys(macros).length;
    }
    assert.ok(
        macroCount >= 5,
        `expected at least 5 macros across 2 files, got ${macroCount}`
    );
});

test('declarative macros: cross-file imports resolve at expand time', () => {
    const vanillaRoot = path.join(repoRoot, 'playground', 'vanilla');
    const declarativeRegistryJson = loadDeclarativeRegistry(vanillaRoot);
    assert.ok(
        declarativeRegistryJson,
        'precondition: .macroforge/declarative-registry.json must exist'
    );

    // Expand the consumer file with the declarative registry installed.
    // The registry maps `decl_macros_lib.ts`'s absolute path to its
    // `$vec` / `$identity` definitions; the pre-pass uses it to resolve
    // the `/** import macro { $vec, $identity } from "./decl_macros_lib" */`
    // JSDoc at the top of `cross-file-decl.ts`.
    const consumerPath = path.join(vanillaRoot, 'src', 'cross-file-decl.ts');
    const source = fs.readFileSync(consumerPath, 'utf8');

    const { code, diagnostics } = expandSync(source, consumerPath, {
        keepDecorators: false,
        declarativeRegistryJson
    });

    // No error diagnostics — imports resolved cleanly.
    const errors = (diagnostics ?? []).filter((d) => d.level === 'error');
    assert.equal(
        errors.length,
        0,
        `expected no error diagnostics, got: ${JSON.stringify(errors)}`
    );

    // $vec() → []
    assert.ok(
        /crossFileEmpty\s*=\s*\[\s*\]/.test(code),
        `expected $vec() to expand to []; got:\n${code}`
    );
    // $vec(1, 2, 3) → [1, 2, 3]
    assert.ok(
        /crossFileThree\s*=\s*\[\s*1\s*,\s*2\s*,\s*3\s*\]/.test(code),
        `expected $vec(1,2,3) to expand to [1,2,3]; got:\n${code}`
    );
    // $identity(42) → 42
    assert.ok(
        /crossFileId\s*=\s*42/.test(code),
        `expected $identity(42) to expand to 42; got:\n${code}`
    );
    // The import comment should still be present in the raw expand
    // output; the Vite plugin strips it separately at line ~1007.
    assert.ok(
        !code.includes('$vec('),
        `expected all $vec call sites to be rewritten; got:\n${code}`
    );
    assert.ok(
        !code.includes('$identity('),
        `expected all $identity call sites to be rewritten; got:\n${code}`
    );
});
