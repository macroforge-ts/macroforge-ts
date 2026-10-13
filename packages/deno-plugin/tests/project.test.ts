/**
 * `expandProject` over a project with a file whose macros fail: the file is
 * reported as failed and nothing is written for it, where the files beside
 * it are expanded.
 */

import { assert, assertEquals } from '@std/assert';
import { join } from '@std/path';
import { expandProject } from '../src/project.ts';

Deno.test('a file whose expansion reports an error is not written', async () => {
    const root = await Deno.makeTempDir({ prefix: 'macroforge-deno-plugin-' });
    try {
        await Deno.writeTextFile(join(root, 'deno.json'), '{}\n');
        await Deno.writeTextFile(
            join(root, 'good.ts'),
            '/** @derive(Default) */\nexport interface Good {\n    count: number;\n}\n'
        );
        await Deno.writeTextFile(
            join(root, 'bad.ts'),
            '/** @derive(Default) */\nexport interface Chain {\n    next: Chain;\n}\n'
        );

        const events = await expandProject({ root, outDir: 'out' });
        const event = (name: string) => {
            const found = events.find((candidate) => candidate.source.endsWith(name));
            assert(found, `no event for ${name}`);
            return found;
        };

        assertEquals(event('good.ts').failed, false);
        assertEquals(event('good.ts').expanded, true);
        assert(
            (await Deno.readTextFile(join(root, 'out', 'good.ts'))).includes('goodDefaultValue')
        );

        assertEquals(event('bad.ts').failed, true);
        assertEquals(event('bad.ts').expanded, false);
        assert(
            event('bad.ts').diagnostics.some((diag) =>
                diag.level === 'error' && diag.message.includes("needs a default of 'Chain' itself")
            )
        );
        let written = true;
        try {
            await Deno.stat(join(root, 'out', 'bad.ts'));
        } catch (error) {
            if (!(error instanceof Deno.errors.NotFound)) throw error;
            written = false;
        }
        assertEquals(written, false, 'a failed file leaves nothing in the mirror');
    } finally {
        await Deno.remove(root, { recursive: true });
    }
});
