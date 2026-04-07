import pkg from 'macroforge';
const { expandSync: expand_node } = pkg;
import * as expand_wasm from '../../../crates/macroforge_ts/pkg/macroforge_ts.js';
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert';

// Utility to normalize code for parity comparison
// This handles differences in indentation/whitespace that can occur between native and WASM builds
function normalize(code) {
    return code
        .split('\n')
        .map((line) => line.trim())
        .filter((line) => line.length > 0)
        .join('\n');
}

// We use comment-style macros for parity tests as they don't depend on import resolution for decorators
const BASE_CODE = `
import { Derive } from "macroforge";
/** @derive(Debug, Clone) */
export class User {
  id: number;
  name: string;
}
`;
const BASE_FILE = 'user-simple.ts';

async function runTests() {
    console.log('🧪 Running Macroforge Parity Tests...');

    // Test 1: Identical Output (Core macros)
    {
        console.log(' - Core macros output parity check...');
        const resultNode = expand_node(BASE_CODE, BASE_FILE, {
            keep_decorators: false
        });
        const resultWasm = expand_wasm.expand_sync(BASE_CODE, BASE_FILE, {
            keep_decorators: false
        });

        assert.strictEqual(
            normalize(resultWasm.code),
            normalize(resultNode.code),
            'WASM and Node output code mismatch (normalized)'
        );
        assert.strictEqual(
            resultWasm.diagnostics.length,
            resultNode.diagnostics.length,
            'Diagnostics length mismatch'
        );

        assert.ok(resultWasm.code.includes('class User'), 'Missing class User');
        assert.ok(resultWasm.code.includes('toString('), 'Missing toString()');
        assert.ok(resultWasm.code.includes('static clone('), 'Missing clone()');
    }

    // Test 2: Complex built-ins
    {
        console.log(' - Complex built-ins check...');
        const complexCode = `
import { Derive } from "macroforge";
/** @derive(Serialize, Deserialize, Default) */
class Settings {
    /** @serde({ rename: "api_key" }) */
    apiKey: string;
    
    /** @serde({ default: 8080 }) */
    port: number;
}
`;
        const resultNode = expand_node(complexCode, 'settings.ts', {
            keep_decorators: false
        });
        const resultWasm = expand_wasm.expand_sync(complexCode, 'settings.ts', {
            keep_decorators: false
        });

        assert.strictEqual(
            normalize(resultWasm.code),
            normalize(resultNode.code),
            'Complex built-ins mismatch (normalized)'
        );
    }

    // Test 3: Diagnostics parity
    {
        console.log(' - Diagnostics parity check...');
        const invalidCode = `/** @derive(NonExistentMacro) */\nclass Test {}`;
        const fileName = 'invalid.ts';

        const resultNode = expand_node(invalidCode, fileName, {
            keep_decorators: false
        });
        const resultWasm = expand_wasm.expand_sync(invalidCode, fileName, {
            keep_decorators: false
        });

        assert.ok(
            resultWasm.diagnostics.length > 0,
            'WASM should have reported errors'
        );
        assert.strictEqual(
            resultWasm.diagnostics.length,
            resultNode.diagnostics.length,
            'Diagnostics length mismatch on error'
        );
        assert.strictEqual(
            resultWasm.diagnostics[0].message,
            resultNode.diagnostics[0].message,
            'Error message mismatch'
        );
    }

    console.log('✅ All parity tests passed!');
}

runTests().catch((err) => {
    console.error('❌ Parity test failed:');
    console.error(err);
    process.exit(1);
});
