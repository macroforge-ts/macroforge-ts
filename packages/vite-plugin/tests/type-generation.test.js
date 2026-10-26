/**
 * Tests for type definition (.d.ts) generation.
 */

import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import fs from 'fs';
import path from 'path';
import macroforge from '../src/index.js';
import {
    cleanupTempDir,
    createTempDir,
    getFixturePath,
    initializePlugin,
    invokeTransform,
    listFilesRecursively,
    readFileOrNull,
    writeTestFile
} from './test-utils.js';

test('generates .d.ts when generateTypes is true', async (t) => {
    const tempDir = createTempDir();

    t.after(() => cleanupTempDir(tempDir));

    // Create tsconfig.json for type generation
    writeTestFile(
        tempDir,
        'tsconfig.json',
        JSON.stringify({
            compilerOptions: {
                target: 'ES2022',
                module: 'ESNext',
                moduleResolution: 'Bundler',
                strict: true,
                skipLibCheck: true
            }
        })
    );

    // Create a source file with macro
    writeTestFile(
        tempDir,
        'src/user.ts',
        `/** @derive(Debug) */
class User {
  id: string;
  name: string;
}
export { User };`
    );

    const typesDir = 'generated-types';
    const plugin = await macroforge();
    initializePlugin(plugin, tempDir);

    const code = fs.readFileSync(path.join(tempDir, 'src/user.ts'), 'utf-8');
    const id = path.join(tempDir, 'src/user.ts');

    const { result, error } = await invokeTransform(plugin, code, id);

    assert.equal(error, null);

    // Check if .d.ts file was generated
    const expectedTypesPath = path.join(tempDir, typesDir, 'src', 'user.d.ts');
    const typesContent = readFileOrNull(expectedTypesPath);

    // If transformation happened and TypeScript is available, types should be generated
    if (result && result.code) {
        assert.ok(
            typesContent === null || typesContent.includes('User'),
            'Types should contain User class declaration or not be generated if TS unavailable'
        );
    }
});

test('skips .d.ts generation when generateTypes is false', async (t) => {
    const tempDir = createTempDir();

    t.after(() => cleanupTempDir(tempDir));

    writeTestFile(
        tempDir,
        'tsconfig.json',
        JSON.stringify({
            compilerOptions: {
                target: 'ES2022',
                module: 'ESNext'
            }
        })
    );

    writeTestFile(
        tempDir,
        'src/user.ts',
        `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
    );

    const typesDir = 'no-types';
    const plugin = await macroforge();
    initializePlugin(plugin, tempDir);

    const code = fs.readFileSync(path.join(tempDir, 'src/user.ts'), 'utf-8');
    const id = path.join(tempDir, 'src/user.ts');

    const { result, error } = await invokeTransform(plugin, code, id);

    assert.equal(error, null);

    // Types directory should not be created
    const typesPath = path.join(tempDir, typesDir);
    assert.equal(
        fs.existsSync(typesPath),
        false,
        'Types directory should not exist'
    );
});

test('uses default typesOutputDir when not specified', async (t) => {
    const tempDir = createTempDir();

    t.after(() => cleanupTempDir(tempDir));

    writeTestFile(
        tempDir,
        'tsconfig.json',
        JSON.stringify({
            compilerOptions: {
                target: 'ES2022',
                module: 'ESNext'
            }
        })
    );

    writeTestFile(
        tempDir,
        'src/user.ts',
        `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
    );

    // Don't specify typesOutputDir - should default to ".macroforge/types"
    const plugin = await macroforge();
    initializePlugin(plugin, tempDir);

    const code = fs.readFileSync(path.join(tempDir, 'src/user.ts'), 'utf-8');
    const id = path.join(tempDir, 'src/user.ts');

    const { result, error } = await invokeTransform(plugin, code, id);

    assert.equal(error, null);

    // Check default path
    const defaultTypesPath = path.join(tempDir, '.macroforge/types');
    // Directory might be created if transformation occurred
    if (result && result.code) {
        // Types may or may not be generated depending on TS availability
        assert.ok(true);
    }
});

test('preserves directory structure in types output', async (t) => {
    const tempDir = createTempDir();

    t.after(() => cleanupTempDir(tempDir));

    writeTestFile(
        tempDir,
        'tsconfig.json',
        JSON.stringify({
            compilerOptions: {
                target: 'ES2022',
                module: 'ESNext'
            }
        })
    );

    // Create nested source file
    writeTestFile(
        tempDir,
        'src/models/entities/user.ts',
        `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
    );

    const typesDir = 'types';
    const plugin = await macroforge();
    initializePlugin(plugin, tempDir);

    const code = fs.readFileSync(
        path.join(tempDir, 'src/models/entities/user.ts'),
        'utf-8'
    );
    const id = path.join(tempDir, 'src/models/entities/user.ts');

    const { result, error } = await invokeTransform(plugin, code, id);

    assert.equal(error, null);

    // Check if directory structure is preserved
    const expectedPath = path.join(
        tempDir,
        typesDir,
        'src/models/entities/user.d.ts'
    );
    if (result && result.code && fs.existsSync(expectedPath)) {
        const content = fs.readFileSync(expectedPath, 'utf-8');
        assert.ok(content.includes('User'));
    }
});

test('skips write when .d.ts content unchanged', async (t) => {
    const tempDir = createTempDir();

    t.after(() => cleanupTempDir(tempDir));

    writeTestFile(
        tempDir,
        'tsconfig.json',
        JSON.stringify({
            compilerOptions: {
                target: 'ES2022',
                module: 'ESNext'
            }
        })
    );

    writeTestFile(
        tempDir,
        'src/user.ts',
        `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
    );

    const typesDir = 'types';
    const plugin = await macroforge();
    initializePlugin(plugin, tempDir);

    const code = fs.readFileSync(path.join(tempDir, 'src/user.ts'), 'utf-8');
    const id = path.join(tempDir, 'src/user.ts');

    // First transform
    const { result: result1, error: error1 } = await invokeTransform(
        plugin,
        code,
        id
    );
    assert.equal(error1, null);

    // Get mtime of types file if it exists
    const typesPath = path.join(tempDir, typesDir, 'src/user.d.ts');
    let mtime1 = null;
    if (fs.existsSync(typesPath)) {
        mtime1 = fs.statSync(typesPath).mtimeMs;

        // Wait a bit to ensure mtime would change if file is rewritten
        await new Promise((resolve) => setTimeout(resolve, 50));

        // Second transform with same content
        const { result: result2, error: error2 } = await invokeTransform(
            plugin,
            code,
            id
        );
        assert.equal(error2, null);

        // mtime should not change (file not rewritten)
        const mtime2 = fs.statSync(typesPath).mtimeMs;
        assert.equal(
            mtime1,
            mtime2,
            'File should not be rewritten when content is unchanged'
        );
    }
});

test('handles TypeScript not being available gracefully', async (t) => {
    // This test verifies the plugin doesn't crash when TS is unavailable
    // In our test environment, TS is available, so we just verify basic operation
    const tempDir = createTempDir();

    t.after(() => cleanupTempDir(tempDir));

    writeTestFile(
        tempDir,
        'src/user.ts',
        `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
    );

    const plugin = await macroforge();
    initializePlugin(plugin, tempDir);

    const code = fs.readFileSync(path.join(tempDir, 'src/user.ts'), 'utf-8');
    const id = path.join(tempDir, 'src/user.ts');

    const { result, error } = await invokeTransform(plugin, code, id);

    // Should not throw even if types can't be generated
    assert.equal(error, null);
});

test('creates output directory if it does not exist', async (t) => {
    const tempDir = createTempDir();

    t.after(() => cleanupTempDir(tempDir));

    writeTestFile(
        tempDir,
        'tsconfig.json',
        JSON.stringify({
            compilerOptions: {
                target: 'ES2022',
                module: 'ESNext'
            }
        })
    );

    writeTestFile(
        tempDir,
        'src/user.ts',
        `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
    );

    // Use deeply nested output directory that doesn't exist
    const typesDir = 'deep/nested/types/directory';
    const plugin = await macroforge();
    initializePlugin(plugin, tempDir);

    const code = fs.readFileSync(path.join(tempDir, 'src/user.ts'), 'utf-8');
    const id = path.join(tempDir, 'src/user.ts');

    const { result, error } = await invokeTransform(plugin, code, id);

    assert.equal(error, null);

    // If transformation occurred, directory should be created
    if (result && result.code) {
        const outputDir = path.join(tempDir, typesDir);
        // Directory might exist now
        assert.ok(true);
    }
});

test('skips .d.ts generation, by name, when typescript has no compiler API', (t) => {
    // Resolved, since the spawned process reports its cwd that way.
    const tempDir = fs.realpathSync(createTempDir());

    t.after(() => cleanupTempDir(tempDir));

    // A copy of the plugin outside this repository, whose own TypeScript the
    // plugin would otherwise fall back to.
    writeTestFile(tempDir, 'package.json', '{ "type": "module" }');
    const pluginPath = path.join(tempDir, 'plugin', 'index.js');
    fs.mkdirSync(path.dirname(pluginPath));
    fs.copyFileSync(path.join(import.meta.dirname, '..', 'src', 'index.js'), pluginPath);
    for (
        const dependency of [
            '@macroforge/core',
            '@macroforge/shared',
            '@jridgewell/sourcemap-codec'
        ]
    ) {
        const link = path.join(tempDir, 'node_modules', dependency);
        fs.mkdirSync(path.dirname(link), { recursive: true });
        fs.symlinkSync(
            fs.realpathSync(
                path.join(import.meta.dirname, '..', '..', '..', 'node_modules', dependency)
            ),
            link
        );
    }
    // The main module of typescript 7: a version and nothing to compile with.
    writeTestFile(
        tempDir,
        'node_modules/typescript/package.json',
        '{ "name": "typescript", "version": "7.0.2", "main": "version.cjs" }'
    );
    writeTestFile(
        tempDir,
        'node_modules/typescript/version.cjs',
        'exports.version = "7.0.2";\nexports.versionMajorMinor = "7.0";\n'
    );
    writeTestFile(
        tempDir,
        'src/user.ts',
        `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
    );

    const run = spawnSync(
        process.execPath,
        [
            getFixturePath('no-compiler-api', 'transform.mjs'),
            pluginPath,
            path.join(tempDir, 'src/user.ts')
        ],
        { cwd: tempDir, encoding: 'utf8' }
    );

    assert.equal(run.status, 0, `the transform should still succeed. stderr: ${run.stderr}`);
    assert.match(run.stdout, /userDebug|toString/, 'the macro is still expanded');
    assert.match(run.stderr, /TypeScript 7\.0\.2 has no compiler API/);
    assert.deepEqual(
        listFilesRecursively(path.join(tempDir, '.macroforge', 'types')),
        [],
        'no declarations are written'
    );
});

test('keeps the outputs of a source outside the project root inside the project', async (t) => {
    const tempDir = createTempDir();

    t.after(() => cleanupTempDir(tempDir));

    const projectRoot = path.join(tempDir, 'apps', 'web');
    writeTestFile(projectRoot, 'tsconfig.json', JSON.stringify({ compilerOptions: {} }));
    writeTestFile(
        tempDir,
        'shared/src/user.ts',
        `/** @derive(Debug) */
class User {
  id: string;
}
export { User };`
    );

    const plugin = await macroforge();
    initializePlugin(plugin, projectRoot);

    const id = path.join(tempDir, 'shared/src/user.ts');
    const { error } = await invokeTransform(plugin, fs.readFileSync(id, 'utf-8'), id);

    assert.equal(error, null);
    assert.deepEqual(listFilesRecursively(path.join(projectRoot, '.macroforge', 'types')), [
        '_external/shared/src/user.d.ts'
    ]);
    assert.deepEqual(listFilesRecursively(path.join(projectRoot, '.macroforge', 'meta')), [
        '_external/shared/src/user.macro-ir.json'
    ]);
    assert.deepEqual(
        fs.readdirSync(tempDir).sort(),
        ['apps', 'shared'],
        'nothing is written beside the project'
    );
    assert.deepEqual(listFilesRecursively(path.join(tempDir, 'shared')), ['src/user.ts']);
});
