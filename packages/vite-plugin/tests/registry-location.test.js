/**
 * Registries scanned where a project used to be are ignored, not loaded, and
 * registries scanned where it is load however its root is reached.
 */

import test from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import fs from 'fs';
import path from 'path';
import macroforge from '../src/index.js';
import { cleanupTempDir, cliBinary, createTempDir, initializePlugin } from './test-utils.js';

test('registries listing files outside the project are ignored with a warning', async () => {
    const root = createTempDir('vite-plugin-moved-');
    const elsewhere = path.join(path.dirname(root), 'old-location', 'src', 'lib.ts');
    try {
        fs.mkdirSync(path.join(root, '.macroforge'));
        fs.writeFileSync(
            path.join(root, '.macroforge', 'declarative-registry.json'),
            JSON.stringify({ by_file: { [elsewhere]: {} } })
        );
        fs.writeFileSync(
            path.join(root, '.macroforge', 'type-registry.json'),
            JSON.stringify({ version: 2, entries: { 'src/lib.ts::Lib': { file_path: elsewhere } } })
        );

        const plugin = await macroforge();
        initializePlugin(plugin, root);
        const warnings = [];
        const warn = console.warn;
        console.warn = (message) => warnings.push(String(message));
        try {
            plugin.buildStart();
        } finally {
            console.warn = warn;
        }

        for (const name of ['type-registry.json', 'declarative-registry.json']) {
            assert.ok(
                warnings.some(
                    (message) =>
                        message.includes(`.macroforge/${name}`) &&
                        message.includes('scanned at another location') &&
                        message.includes(elsewhere)
                ),
                `expected a warning for ${name}, got ${JSON.stringify(warnings)}`
            );
        }
    } finally {
        cleanupTempDir(root);
    }
});

test('registries the CLI wrote load for a root reached through a symlink', async () => {
    const parent = createTempDir('vite-plugin-linked-');
    const real = path.join(parent, 'real');
    const link = path.join(parent, 'link');
    try {
        fs.mkdirSync(path.join(real, 'src'), { recursive: true });
        fs.symlinkSync(real, link, 'dir');
        fs.writeFileSync(
            path.join(real, 'package.json'),
            JSON.stringify({ name: 'linked', type: 'module' })
        );
        fs.writeFileSync(
            path.join(real, 'src', 'point.ts'),
            ['/** @derive(Debug) */', 'export interface Point {', '    x: number;', '}', ''].join(
                '\n'
            )
        );
        execFileSync(cliBinary(), ['sync'], { cwd: real, stdio: 'pipe' });

        const plugin = await macroforge();
        initializePlugin(plugin, link);
        const messages = [];
        const { log, warn } = console;
        console.log = (message) => messages.push(String(message));
        console.warn = (message) => messages.push(String(message));
        try {
            plugin.buildStart();
        } finally {
            console.log = log;
            console.warn = warn;
        }

        assert.deepEqual(
            messages.filter((message) => message.includes('scanned at another location')),
            []
        );
        assert.ok(
            messages.some((message) => message.includes('Type registry loaded')),
            `expected the type registry to load, got ${JSON.stringify(messages)}`
        );
    } finally {
        cleanupTempDir(parent);
    }
});
