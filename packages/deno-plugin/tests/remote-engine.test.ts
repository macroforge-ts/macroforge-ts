/**
 * The engine's JSR entry runs from `https://jsr.io/`, where the wasm sidecar
 * can only be fetched. In the workspace every import resolves to a local path,
 * so nothing else here exercises that: this test serves the Deno build over
 * HTTP and imports it the way a JSR consumer does.
 *
 * The server takes a free port, so the suite runs with `--no-lock`: every run
 * would otherwise add its own `http://localhost:<port>/` entries to the lock.
 */

import { assert, assertStringIncludes } from '@std/assert';
import { contentType } from '@std/media-types';
import { extname, fromFileUrl, join } from '@std/path';

const engineDir = fromFileUrl(import.meta.resolve('../../../crates/macroforge_ts/pkg-deno'));

interface RemoteEngine {
    expandSync(code: string, filepath: string, options: unknown): { code: string };
}

Deno.test('the Deno build loads its wasm over HTTP', async () => {
    const server = Deno.serve({ port: 0, onListen: () => {} }, async (request) => {
        const path = join(engineDir, new URL(request.url).pathname);
        const file = await Deno.readFile(path).catch(() => null);
        if (!file) return new Response('not found', { status: 404 });
        return new Response(file, {
            headers: { 'content-type': contentType(extname(path)) ?? 'application/octet-stream' }
        });
    });

    try {
        const origin = `http://localhost:${server.addr.port}`;
        const engine: RemoteEngine = await import(`${origin}/macroforge_ts.js`);
        const source = '/** @derive(Debug) */\nexport class User { name: string = ""; }\n';
        const expanded = engine.expandSync(source, 'user.ts', {});

        assert(expanded.code.length > 0, 'the remote engine returned no code');
        assertStringIncludes(expanded.code, 'toString');
    } finally {
        await server.shutdown();
    }
});
