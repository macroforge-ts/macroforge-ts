import { expect, test } from '@playwright/test';
import { expandSync as expand_node } from 'macroforge';
import * as expand_wasm from '../../../crates/macroforge_ts/pkg-wasm/macroforge_ts.js';
import fs from 'node:fs';
import path from 'node:path';

const TEST_FILE = path.resolve(process.cwd(), '../vanilla/src/user.ts');
const code = fs.readFileSync(TEST_FILE, 'utf-8');

test.describe('Macroforge WASM parity', () => {
    test('WASM and Node should produce identical output', async () => {
        const resultNode = expand_node(code, TEST_FILE, { keep_decorators: false });
        const resultWasm = expand_wasm.expand_sync(code, TEST_FILE, {
            keep_decorators: false
        });

        // Basic structural check
        expect(resultWasm.code).toBeDefined();
        expect(resultWasm.diagnostics).toBeDefined();

        // Parity check
        expect(resultWasm.code).toBe(resultNode.code);
        expect(resultWasm.diagnostics.length).toBe(resultNode.diagnostics.length);

        // Check content if successful
        if (resultNode.diagnostics.length === 0) {
            expect(resultWasm.code).toContain('class User');
            expect(resultWasm.code).toContain('toString()'); // Debug
            expect(resultWasm.code).toContain('clone()'); // Clone
        }
    });

    test('WASM should handle complex files', async () => {
        const complexFile = path.resolve(
            process.cwd(),
            '../vanilla/src/validator-form.ts'
        );
        const complexCode = fs.readFileSync(complexFile, 'utf-8');

        const resultNode = expand_node(complexCode, complexFile, {
            keep_decorators: false
        });
        const resultWasm = expand_wasm.expand_sync(complexCode, complexFile, {
            keep_decorators: false
        });

        expect(resultWasm.code).toBe(resultNode.code);
        expect(resultWasm.diagnostics.length).toBe(resultNode.diagnostics.length);
    });

    test('WASM should support diagnostics parity', async () => {
        const invalidCode = `@Derive(NonExistentMacro)\nclass Test {}`;
        const fileName = 'invalid.ts';

        const resultNode = expand_node(invalidCode, fileName, {
            keep_decorators: false
        });
        const resultWasm = expand_wasm.expand_sync(invalidCode, fileName, {
            keep_decorators: false
        });

        expect(resultWasm.diagnostics.length).toBeGreaterThan(0);
        expect(resultWasm.diagnostics.length).toBe(resultNode.diagnostics.length);

        // Compare messages (ignoring potential platform-specific path formatting in errors if any)
        expect(resultWasm.diagnostics[0].message).toBe(
            resultNode.diagnostics[0].message
        );
    });
});
