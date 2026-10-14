/**
 * The project scan of the engine the integrations load, which reads the
 * project through Node's `fs`.
 */

import { assert, assertEquals } from '@std/assert';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { scanProjectSync } from '@macroforge/core';

const write = (root, path, text) => {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), text);
};

Deno.test('the scan reads the files of a project and leaves out what the native walk does', () => {
    const root = mkdtempSync(join(tmpdir(), 'macroforge-scan-'));
    try {
        mkdirSync(join(root, '.git'));
        write(root, '.gitignore', 'ignored.ts\n');
        write(root, 'src/alpha.ts', 'export interface Alpha { id: string }\n');
        write(root, 'src/nested/beta.ts', 'export interface Beta { id: string }\n');
        write(root, 'ignored.ts', 'export interface Gamma { id: string }\n');
        write(root, '.hidden/delta.ts', 'export interface Delta { id: string }\n');
        write(root, 'node_modules/pkg/epsilon.ts', 'export interface Epsilon { id: string }\n');

        const result = scanProjectSync(root, null);
        assertEquals(result.filesScanned, 2, JSON.stringify(result.diagnostics));
        for (const name of ['Alpha', 'Beta']) {
            assert(result.registryJson.includes(`"${name}"`), `${name} is registered`);
        }
        for (const name of ['Gamma', 'Delta', 'Epsilon']) {
            assert(!result.registryJson.includes(`"${name}"`), `${name} is left out`);
        }
    } finally {
        rmSync(root, { recursive: true, force: true });
    }
});
