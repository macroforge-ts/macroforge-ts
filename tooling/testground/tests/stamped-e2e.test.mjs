/**
 * A generic alias over a foreign type, used from a module that imports
 * neither: `Stamped<T> = DateTime.DateTime | T` declared in `stamp.ts` and
 * used by `stamped-entry.svelte.ts`. Each field tells the members apart with
 * the foreign type's `hasShape`, so a timestamp goes through the foreign
 * handlers and a note through its own encoder and decoder.
 */

import { describe, test } from 'node:test';
import assert from 'node:assert/strict';
import { svelteRoot, withViteServer } from './test-utils.mjs';

const ISO = '2024-05-01T12:00:00.000Z';
const NOTE = { text: 'drafted' };

describe('A cross-module generic alias over a foreign type', () => {
    test('decodes, encodes and defaults each member through its own handlers', async () => {
        await withViteServer(svelteRoot, async (server) => {
            const { DateTime } = await server.ssrLoadModule('effect');
            const { stampedEntryDecode, stampedEntryEncode, stampedEntryDefaultValue } =
                await server.ssrLoadModule('/src/lib/e2e/stamped-entry.svelte.ts');

            const decoded = stampedEntryDecode({
                at: ISO,
                previous: NOTE,
                history: [NOTE, ISO],
                slug: 'about',
                label: { slug: 'starred' },
                since: ISO
            });
            assert.ok(decoded.success, JSON.stringify(decoded.errors ?? []));
            const entry = decoded.value;
            assert.ok(DateTime.isDateTime(entry.at), 'a timestamp decodes to a DateTime');
            assert.equal(DateTime.formatIso(entry.at), ISO);
            assert.deepEqual({ ...entry.previous }, NOTE, 'a note decodes as a note');
            assert.ok(!DateTime.isDateTime(entry.previous));
            assert.deepEqual({ ...entry.history[0] }, NOTE);
            assert.ok(DateTime.isDateTime(entry.history[1]));

            const encoded = JSON.parse(stampedEntryEncode(entry));
            assert.equal(encoded.at, ISO, 'a DateTime encodes through its handler');
            assert.deepEqual(encoded.previous, NOTE, 'a note encodes through its encoder');
            assert.deepEqual(encoded.history, [NOTE, ISO]);

            const nullPrevious = stampedEntryDecode({
                ...JSON.parse(stampedEntryEncode(entry)),
                previous: null
            });
            assert.ok(nullPrevious.success, JSON.stringify(nullPrevious.errors ?? []));
            assert.equal(nullPrevious.value.previous, null);

            const fallback = stampedEntryDefaultValue();
            assert.ok(
                DateTime.isDateTime(fallback.at),
                'the alias defaults to its marked member'
            );
            assert.equal(fallback.slug, 'home', 'a literal default decodes into the newtype');
            assert.equal(fallback.label.slug, 'pinned', 'an object literal decodes into its type');
            assert.ok(
                DateTime.isDateTime(fallback.since),
                'a literal default decodes through the generic alias'
            );
        });
    });

    test('a literal default its newtype rejects fails loudly', async () => {
        await withViteServer(svelteRoot, async (server) => {
            const { slugDecode } = await server.ssrLoadModule('/src/lib/e2e/stamp.ts');
            const { decodedLiteral } = await server.ssrLoadModule('@macroforge/core/endec');
            assert.throws(() => decodedLiteral(slugDecode('')), /nonEmpty|empty/i);
        });
    });
});
