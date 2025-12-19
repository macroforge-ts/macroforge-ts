#!/usr/bin/env node
/**
 * Sets up test fixtures that can't be committed to git (node_modules directories).
 * Run this before tests with `npm run pretest`.
 */

const fs = require('fs');
const path = require('path');

const fixturesDir = path.join(__dirname, 'plugins/typescript/features/diagnostics/fixtures');

// exports-map-svelte fixture
const exportsMapSvelte = path.join(fixturesDir, 'exports-map-svelte/node_modules/package');
fs.mkdirSync(exportsMapSvelte, { recursive: true });

fs.writeFileSync(path.join(exportsMapSvelte, 'package.json'), JSON.stringify({
    name: "package",
    version: "1.0.0",
    exports: {
        ".": { svelte: "./foo.svelte" },
        "./x": { types: "./x-types.d.ts", svelte: "./x.svelte" },
        "./y": { svelte: "./y.svelte" }
    }
}, null, 4) + '\n');

fs.writeFileSync(path.join(exportsMapSvelte, 'foo.svelte'),
    '<script lang="ts">export let foo: string;</script>\n');

fs.writeFileSync(path.join(exportsMapSvelte, 'x.svelte'),
    '<script lang="ts">export let x: number;</script>\n');

fs.writeFileSync(path.join(exportsMapSvelte, 'x-types.d.ts'),
    'declare const X: any;\nexport default X;\n');

fs.writeFileSync(path.join(exportsMapSvelte, 'y.svelte'),
    '<script lang="ts">export let y: boolean;</script>\n');

console.log('Test fixtures created successfully');
