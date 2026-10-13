/**
 * `expandProject` and `watchProject` over a project with a file whose macros
 * fail: the file is reported as failed and the mirror holds nothing for it,
 * where the files beside it are expanded.
 */

import { assert, assertEquals } from '@std/assert';
import { join } from '@std/path';
import { type ExpandFileEvent, expandProject, watchProject } from '../src/project.ts';

const GOOD = '/** @derive(Default) */\nexport interface Good {\n    count: number;\n}\n';
const ENDLESS = '/** @derive(Default) */\nexport interface Good {\n    next: Good;\n}\n';

/** Whether a file is at `file`. */
async function isWritten(file: string): Promise<boolean> {
    try {
        await Deno.stat(file);
        return true;
    } catch (error) {
        if (!(error instanceof Deno.errors.NotFound)) throw error;
        return false;
    }
}

/** Waits for `settled` to hold, and says whether it came to. */
async function eventually(settled: () => boolean | Promise<boolean>): Promise<boolean> {
    for (let attempt = 0; attempt < 200; attempt++) {
        if (await settled()) return true;
        await new Promise((resolve) => setTimeout(resolve, 50));
    }
    return false;
}

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

Deno.test('a file that stops expanding loses its copy in the mirror', async () => {
    const root = await Deno.makeTempDir({ prefix: 'macroforge-deno-plugin-' });
    try {
        await Deno.writeTextFile(join(root, 'deno.json'), '{}\n');
        await Deno.writeTextFile(join(root, 'good.ts'), GOOD);
        await expandProject({ root, outDir: 'out' });
        assert(await isWritten(join(root, 'out', 'good.ts')));

        await Deno.writeTextFile(join(root, 'good.ts'), ENDLESS);
        const failing = await expandProject({ root, outDir: 'out' });
        assertEquals(failing.map((event) => event.failed), [true]);
        assertEquals(await isWritten(join(root, 'out', 'good.ts')), false);

        // A file with no macros left has no expansion to keep either.
        await Deno.writeTextFile(join(root, 'good.ts'), GOOD);
        await expandProject({ root, outDir: 'out' });
        await Deno.writeTextFile(join(root, 'good.ts'), 'export const plain = 1;\n');
        const plain = await expandProject({ root, outDir: 'out' });
        assertEquals(plain.map((event) => event.skipped), [true]);
        assertEquals(await isWritten(join(root, 'out', 'good.ts')), false);
    } finally {
        await Deno.remove(root, { recursive: true });
    }
});

Deno.test('the watcher reports a change that fails and follows a file that is removed', async () => {
    const root = await Deno.realPath(await Deno.makeTempDir({ prefix: 'macroforge-deno-plugin-' }));
    const events: ExpandFileEvent[] = [];
    const stop = await (async () => {
        await Deno.writeTextFile(join(root, 'deno.json'), '{}\n');
        await Deno.writeTextFile(join(root, 'good.ts'), GOOD);
        return watchProject({ root, outDir: 'out', onFile: (event) => events.push(event) });
    })();
    try {
        const mirrored = join(root, 'out', 'good.ts');
        assert(await isWritten(mirrored), 'the first run fills the mirror');

        await Deno.writeTextFile(join(root, 'good.ts'), ENDLESS);
        assert(
            await eventually(() => events.some((event) => event.failed)),
            'a change that fails is reported as a failed event'
        );
        assert(await eventually(async () => !(await isWritten(mirrored))));

        await Deno.writeTextFile(join(root, 'good.ts'), GOOD);
        assert(
            await eventually(() => isWritten(mirrored)),
            'a change that expands is written again'
        );

        await Deno.remove(join(root, 'good.ts'));
        assert(
            await eventually(async () => !(await isWritten(mirrored))),
            'a removed file takes its copy in the mirror with it'
        );
    } finally {
        stop();
        await Deno.remove(root, { recursive: true });
    }
});
