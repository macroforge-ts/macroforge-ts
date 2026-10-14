import { describe, test } from 'node:test';
import assert from 'node:assert';
import { originalPositionFor, TraceMap } from '@jridgewell/trace-mapping';
import { macroforgePreprocess } from '../src/index.ts';

describe('macroforgePreprocess', () => {
    test('returns a PreprocessorGroup with correct structure', () => {
        const preprocessor = macroforgePreprocess();

        assert.strictEqual(preprocessor.name, 'macroforge');
        assert.strictEqual(typeof preprocessor.script, 'function');
    });

    test('accepts keepDecorators option', () => {
        const preprocessor = macroforgePreprocess({ keepDecorators: true });
        assert.ok(preprocessor);
    });

    test('accepts processJavaScript option', () => {
        const preprocessor = macroforgePreprocess({ processJavaScript: true });
        assert.ok(preprocessor);
    });
});

describe('script preprocessor', () => {
    test('skips content without @derive', async () => {
        const preprocessor = macroforgePreprocess();
        const result = await preprocessor.script({
            content: 'const x = 1;',
            filename: 'test.svelte',
            attributes: { lang: 'ts' }
        });

        assert.strictEqual(
            result,
            undefined,
            'Should return undefined for content without @derive'
        );
    });

    test('skips non-TypeScript by default', async () => {
        const preprocessor = macroforgePreprocess();
        const result = await preprocessor.script({
            content: '/** @derive(Debug) */ class Foo {}',
            filename: 'test.svelte',
            attributes: { lang: 'js' }
        });

        assert.strictEqual(
            result,
            undefined,
            'Should skip JavaScript blocks by default'
        );
    });

    test('a built-in derive in a JavaScript block fails the block', async () => {
        // The block is JavaScript and a built-in derive writes TypeScript:
        // with `processJavaScript` it reaches the engine, which refuses it.
        await assert.rejects(
            macroforgePreprocess({ processJavaScript: true }).script({
                content: '/** @derive(Debug) */ class Foo {}',
                filename: 'test.svelte',
                attributes: {} // no lang = JavaScript
            }),
            /1 macro error\(s\) in test\.svelte:\n.*is a JavaScript file/
        );
    });

    test('processes TypeScript blocks with lang="ts"', async () => {
        const preprocessor = macroforgePreprocess();

        const result = await preprocessor.script({
            content: '/** @derive(Debug) */ class Foo {}',
            filename: 'test.svelte',
            attributes: { lang: 'ts' }
        });

        assert.ok(result, 'a block with a derive is expanded');
        assert.strictEqual(typeof result.code, 'string');
    });

    test('processes TypeScript blocks with lang="typescript"', async () => {
        const preprocessor = macroforgePreprocess();

        const result = await preprocessor.script({
            content: '/** @derive(Debug) */ class Foo {}',
            filename: 'test.svelte',
            attributes: { lang: 'typescript' }
        });

        assert.ok(result?.code, 'a block with a derive is expanded');
    });
});

describe('integration tests', () => {
    test('expands @derive(Debug) macro', async () => {
        const preprocessor = macroforgePreprocess();

        const result = await preprocessor.script({
            content: `/** @derive(Debug) */
class User {
    name: string;
    age: number;
}`,
            filename: 'User.svelte',
            attributes: { lang: 'ts' }
        });

        if (result !== undefined) {
            assert.ok(
                result.code.includes('toString') || result.code.includes('ToString'),
                'Should generate toString method'
            );
        } else {
            console.log('Skipping integration test: native bindings unavailable');
        }
    });

    test('expands @derive(Default) macro', async () => {
        const preprocessor = macroforgePreprocess();

        const result = await preprocessor.script({
            content: `/** @derive(Default) */
interface Config {
    host: string;
    port: number;
}`,
            filename: 'Config.svelte',
            attributes: { lang: 'ts' }
        });

        if (result !== undefined) {
            assert.ok(
                result.code.includes('defaultValue') || result.code.includes('Default'),
                'Should generate default value function'
            );
        } else {
            console.log('Skipping integration test: native bindings unavailable');
        }
    });

    test('handles multiple macros', async () => {
        const preprocessor = macroforgePreprocess();

        const result = await preprocessor.script({
            content: `/** @derive(Debug, Clone) */
class Point {
    x: number;
    y: number;
}`,
            filename: 'Point.svelte',
            attributes: { lang: 'ts' }
        });

        if (result !== undefined) {
            assert.ok(
                result.code.includes('debug') || result.code.includes('clone'),
                'Should generate methods for at least one macro'
            );
        }
    });
});

describe('error handling', () => {
    test('a macro error fails the block, with every message', async () => {
        await assert.rejects(
            macroforgePreprocess().script({
                content: `/** @derive(Default) */
interface Chain {
    next: Chain;
}

/** @derive(Clone) */
const unattached = 1;
`,
                filename: 'broken.svelte',
                attributes: { lang: 'ts' }
            }),
            (error) => {
                assert.match(error.message, /2 macro error\(s\) in broken\.svelte/);
                assert.match(error.message, /needs a default of 'Chain' itself/);
                assert.match(error.message, /attached to nothing a derive expands/);
                return true;
            }
        );
    });

    test('a macro error names its line and column in the component', async () => {
        const content = `
/** @derive(Default) */
interface Chain {
    next: Chain;
}
`;
        const markup = `<h1>Title</h1>\n<script lang="ts">${content}</script>\n`;
        await assert.rejects(
            macroforgePreprocess().script({
                content,
                markup,
                filename: 'chain.svelte',
                attributes: { lang: 'ts' }
            }),
            (error) => {
                assert.match(error.message, /chain\.svelte:5:5: @derive\(Default\)/);
                return true;
            }
        );
    });

    test('a script the engine cannot read fails the block', async () => {
        await assert.rejects(
            macroforgePreprocess().script({
                content: `/** @derive(Debug) */
class Broken {`,
                filename: 'broken.svelte',
                attributes: { lang: 'ts' }
            }),
            /broken\.svelte/
        );
    });
});

describe('source map', () => {
    test('every line the expansion keeps maps back to its own line', async () => {
        const content = `
/** @derive(Debug) */
interface Account {
    id: string;
    name: string;
    owner: string;
}
`;
        const result = await macroforgePreprocess().script({
            content,
            markup: `<script lang="ts">${content}</script>`,
            filename: 'account.svelte',
            attributes: { lang: 'ts' }
        });
        assert.ok(result?.map, 'the expanded script has a map');
        const tracer = new TraceMap(result.map);
        const generated = result.code.split('\n');
        const original = content.split('\n');
        for (const field of ['    name: string;', '    owner: string;']) {
            const line = generated.indexOf(field);
            assert.ok(line >= 0, `${field} is kept`);
            const position = originalPositionFor(tracer, { line: line + 1, column: 4 });
            assert.equal(
                position.line,
                original.indexOf(field) + 1,
                `${field} maps to its own line`
            );
        }
    });
});
