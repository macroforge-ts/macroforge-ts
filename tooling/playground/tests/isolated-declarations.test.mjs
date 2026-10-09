/**
 * `macroforge tsc` with `--isolatedDeclarations`: macro output must declare
 * every exported type explicitly, as JSR publishing requires.
 */

import { assertEquals } from '@std/assert';
import { runCli, vanillaRoot, withProjectFiles } from './test-utils.mjs';

Deno.test('tsc: generated code satisfies isolatedDeclarations', () => {
    const probe = [
        '/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */',
        'export class Account {',
        '    name: string = "";',
        '    balance: number = 0;',
        '}',
        '',
        '/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */',
        'export interface Point {',
        '    x: number;',
        '    y: number;',
        '}',
        '',
        '/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */',
        "export type Mode = 'light' | 'dark';",
        '',
        '/** @derive(Debug, Clone, PartialEq, Hash, Ord, Default, Encode, Decode) */',
        '/** @endec(nonNegative) */',
        '/** @default(0) */',
        'export type Meters = $Newtype<number>;',
        '',
        '/** @derive(Debug, Default, Encode, Decode) */',
        'export enum Level {',
        '    /** @default */',
        '    Low = "low",',
        '    High = "high"',
        '}',
        ''
    ].join('\n');
    withProjectFiles(
        vanillaRoot,
        {
            'src/isolated-declarations-probe.ts': probe,
            'tsconfig.isolated-declarations.json': JSON.stringify({
                extends: './tsconfig.json',
                compilerOptions: {
                    declaration: true,
                    emitDeclarationOnly: true,
                    noEmit: false,
                    isolatedDeclarations: true
                },
                include: ['src/isolated-declarations-probe.ts']
            })
        },
        () => {
            const result = runCli(['tsc', '-p', './tsconfig.isolated-declarations.json'], {
                cwd: vanillaRoot
            });
            assertEquals(
                result.success,
                true,
                `isolatedDeclarations check failed:\n${result.stdout}\n${result.stderr}`
            );
        }
    );
});
