/**
 * `macroforge expand --types-out` writes a declaration file: every
 * declaration of the expanded source, without a body or an initializer,
 * which `tsc` accepts as an ambient module.
 */

import { assertEquals, assertStringIncludes } from '@std/assert';
import fs from 'node:fs';
import path from 'node:path';
import { runCli, vanillaRoot, withProjectFiles } from './test-utils.mjs';

Deno.test('expand --types-out: the declaration file type-checks', () => {
    const probe = [
        '/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */',
        'export class Account {',
        '    name: string = "";',
        '    balance: number = 0;',
        '',
        '    deposit(amount: number): void {',
        '        this.balance += amount;',
        '    }',
        '}',
        '',
        '/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */',
        'export interface Point {',
        '    x: number;',
        '    y: number;',
        '}',
        ''
    ].join('\n');
    const declarations = 'types-out-probe.d.ts';
    withProjectFiles(
        vanillaRoot,
        {
            'src/types-out-probe.ts': probe,
            // Listed so it is removed afterwards; the expansion writes it.
            [declarations]: '',
            'tsconfig.types-out.json': JSON.stringify({
                extends: './tsconfig.json',
                files: [declarations],
                include: []
            })
        },
        () => {
            const expanded = runCli(
                ['expand', 'src/types-out-probe.ts', '--types-out', declarations],
                { cwd: vanillaRoot }
            );
            assertEquals(
                expanded.success,
                true,
                `expand failed:\n${expanded.stdout}\n${expanded.stderr}`
            );
            const surface = fs.readFileSync(path.join(vanillaRoot, declarations), 'utf8');
            assertStringIncludes(surface, 'deposit(amount: number): void;');
            assertStringIncludes(surface, 'pointDefaultValue');

            const checked = runCli(['tsc', '-p', './tsconfig.types-out.json'], {
                cwd: vanillaRoot
            });
            assertEquals(
                checked.success,
                true,
                `type check failed:\n${checked.stdout}\n${checked.stderr}`
            );
        }
    );
});
