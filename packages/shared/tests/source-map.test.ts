import { assertEquals } from 'jsr:@std/assert@1';
import { lineAndColumn, sourceMapOf } from '../src/source-map.ts';

/** The mappings decoded, each as [generated column, line, column] per generated line. */
function decoded(mappings: string): Array<Array<[number, number, number]>> {
    const digits = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/';
    let line = 0;
    let column = 0;
    return mappings.split(';').map((generated) => {
        let generatedColumn = 0;
        return generated.split(',').filter((entry) => entry.length > 0).map((entry) => {
            const values: number[] = [];
            let value = 0;
            let shift = 0;
            for (const letter of entry) {
                const digit = digits.indexOf(letter);
                value += (digit & 31) << shift;
                if (digit & 32) {
                    shift += 5;
                } else {
                    values.push(value & 1 ? -(value >>> 1) : value >>> 1);
                    value = 0;
                    shift = 0;
                }
            }
            generatedColumn += values[0] ?? 0;
            line += values[2] ?? 0;
            column += values[3] ?? 0;
            return [generatedColumn, line, column];
        });
    });
}

Deno.test('every line a segment spans maps to the original line it holds', () => {
    const original = 'class A {\n  a: string;\n  b: number;\n}\n';
    const expanded = 'import x;\nclass A {\n  a: string;\n  b: number;\n}\n';
    const map = sourceMapOf(
        {
            segments: [{
                originalStart: 0,
                originalEnd: original.length,
                expandedStart: 10,
                expandedEnd: expanded.length
            }]
        },
        'a.ts',
        original,
        expanded
    );
    assertEquals(decoded(map?.mappings ?? ''), [[], [[0, 0, 0]], [[0, 1, 0]], [[0, 2, 0]], [[
        0,
        3,
        0
    ]], []]);
    assertEquals(sourceMapOf({ segments: [] }, 'a.ts', original, expanded), undefined);
});

Deno.test('a place is named by its 1-based line and column', () => {
    assertEquals(lineAndColumn('ab\ncd', 4), { line: 2, column: 2 });
    assertEquals(lineAndColumn('ab', 0), { line: 1, column: 1 });
});
