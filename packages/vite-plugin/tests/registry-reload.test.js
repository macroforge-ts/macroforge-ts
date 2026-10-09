/**
 * A registry the CLI rewrites while the dev server runs reaches the macros
 * without a restart, and expansions made against the old one are dropped.
 */

import test from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { EventEmitter } from 'node:events';
import fs from 'fs';
import path from 'path';
import macroforge from '../src/index.js';
import { cleanupTempDir, cliBinary, createTempDir, createTransformContext } from './test-utils.js';

/** A dev server whose watcher the test drives, recording what the plugin asks of it. */
function createDevServer() {
    const watcher = new EventEmitter();
    const watched = [];
    watcher.add = (paths) => watched.push(...[paths].flat());
    const invalidations = [];
    const sent = [];
    return {
        watcher,
        watched,
        invalidations,
        sent,
        moduleGraph: { invalidateAll: () => invalidations.push('all') },
        ws: { send: (payload) => sent.push(payload) }
    };
}

/** The default the expansion of `file` gives the `items` field. */
async function itemsDefault(plugin, file) {
    const result = await plugin.transform.call(
        createTransformContext(),
        fs.readFileSync(file, 'utf-8'),
        file
    );
    const code = result?.code ?? '';
    const match = /items: ([^,\n]+),/.exec(code);
    assert.ok(match, `expected a default for items in:\n${code}`);
    return match[1];
}

test('a type registered mid-session reaches the macros after the CLI rescans', async () => {
    const root = createTempDir('vite-plugin-reload-');
    try {
        fs.mkdirSync(path.join(root, 'src'));
        fs.writeFileSync(
            path.join(root, 'package.json'),
            JSON.stringify({ name: 'reload', type: 'module' })
        );
        const order = path.join(root, 'src', 'order.ts');
        fs.writeFileSync(
            order,
            [
                "import type { List } from './list';",
                '',
                '/** @derive(Default) */',
                'export interface Order {',
                '    items: List<string>;',
                '}',
                ''
            ].join('\n')
        );
        const sync = () => execFileSync(cliBinary(), ['sync'], { cwd: root, stdio: 'pipe' });
        sync();

        const plugin = await macroforge();
        plugin.configResolved({ root, command: 'serve', mode: 'development' });
        plugin.buildStart();
        const server = createDevServer();
        plugin.configureServer(server);
        const typeRegistry = path.join(root, '.macroforge', 'type-registry.json');
        assert.ok(server.watched.includes(typeRegistry), `expected ${typeRegistry} to be watched`);

        const before = await itemsDefault(plugin, order);
        assert.notEqual(before, '[]', 'an unregistered alias cannot resolve to its array');

        fs.writeFileSync(path.join(root, 'src', 'list.ts'), 'export type List<T> = Array<T>;\n');
        sync();
        server.watcher.emit('change', typeRegistry);

        assert.deepEqual(server.invalidations, ['all']);
        assert.deepEqual(server.sent, [{ type: 'full-reload' }]);
        assert.equal(await itemsDefault(plugin, order), '[]');

        // A rewrite that changes nothing reloads nothing.
        sync();
        server.watcher.emit('change', typeRegistry);
        assert.deepEqual(server.invalidations, ['all']);
    } finally {
        cleanupTempDir(root);
    }
});
