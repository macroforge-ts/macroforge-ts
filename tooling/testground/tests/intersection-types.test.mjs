/**
 * E2E tests for intersection type support.
 *
 * Tests that intersection types like `{ variant: 'savings' } & AccountBase`
 * correctly encode, decode, and round-trip through the full
 * Vite macro pipeline at runtime.
 */

import { describe, test } from 'node:test';
import assert from 'node:assert/strict';
import { svelteRoot, withViteServer } from './test-utils.mjs';

describe('Intersection types E2E: Encode & Decode via SvelteKit', () => {
    test('decodes a SavingsAccount intersection type from raw JSON', async () => {
        await withViteServer(svelteRoot, async (server) => {
            const fixtureMod = await server.ssrLoadModule('/src/lib/e2e/fixture.ts');
            const raw = fixtureMod.savingsAccountFixture;

            // Sanity: raw data has ISO string, not Date
            assert.equal(typeof raw.createdAt, 'string');
            assert.equal(raw.variant, 'savings');
            assert.equal(typeof raw.interestRate, 'number');

            // Load macro-expanded module
            const typesMod = await server.ssrLoadModule(
                '/src/lib/e2e/types.svelte.ts'
            );
            const { savingsAccountDecode } = typesMod;
            assert.equal(
                typeof savingsAccountDecode,
                'function',
                'savingsAccountDecode should be exported'
            );

            // Decode
            const result = savingsAccountDecode(raw);
            assert.ok(
                result.success,
                `Decoding failed: ${JSON.stringify(result.errors ?? [])}`
            );
            const account = result.value;

            // Fields from inline object { variant, interestRate }
            assert.equal(account.variant, 'savings');
            assert.equal(account.interestRate, 0.045);

            // Fields from AccountBase
            assert.equal(account.id, 'acc_001');
            assert.equal(account.name, "Alice's Savings");
            assert.equal(account.balance, 15000.50);
            assert.ok(account.createdAt instanceof Date, 'createdAt should be Date');
            assert.equal(account.createdAt.toISOString(), '2023-06-15T10:00:00.000Z');
        });
    });

    test('decodes a CheckingAccount intersection type from raw JSON', async () => {
        await withViteServer(svelteRoot, async (server) => {
            const fixtureMod = await server.ssrLoadModule('/src/lib/e2e/fixture.ts');
            const raw = fixtureMod.checkingAccountFixture;

            const typesMod = await server.ssrLoadModule(
                '/src/lib/e2e/types.svelte.ts'
            );
            const { checkingAccountDecode } = typesMod;
            assert.equal(
                typeof checkingAccountDecode,
                'function',
                'checkingAccountDecode should be exported'
            );

            const result = checkingAccountDecode(raw);
            assert.ok(
                result.success,
                `Decoding failed: ${JSON.stringify(result.errors ?? [])}`
            );
            const account = result.value;

            // Fields from inline object { variant, overdraftLimit }
            assert.equal(account.variant, 'checking');
            assert.equal(account.overdraftLimit, 500);

            // Fields from AccountBase
            assert.equal(account.id, 'acc_002');
            assert.equal(account.name, "Bob's Checking");
            assert.equal(account.balance, 3200.75);
            assert.ok(account.createdAt instanceof Date, 'createdAt should be Date');
            assert.equal(account.createdAt.toISOString(), '2024-01-20T14:30:00.000Z');
        });
    });

    test('encodes an intersection type and round-trips through decode', async () => {
        await withViteServer(svelteRoot, async (server) => {
            const fixtureMod = await server.ssrLoadModule('/src/lib/e2e/fixture.ts');
            const raw = fixtureMod.savingsAccountFixture;

            const typesMod = await server.ssrLoadModule(
                '/src/lib/e2e/types.svelte.ts'
            );
            const {
                savingsAccountEncode,
                savingsAccountDecode
            } = typesMod;

            // Decode from raw
            const desResult = savingsAccountDecode(raw);
            assert.ok(
                desResult.success,
                `Initial deser failed: ${JSON.stringify(desResult.errors ?? [])}`
            );
            const account = desResult.value;

            // Encode back to JSON string
            assert.equal(
                typeof savingsAccountEncode,
                'function',
                'savingsAccountEncode should be exported'
            );
            const jsonStr = savingsAccountEncode(account);
            assert.equal(
                typeof jsonStr,
                'string',
                'encode should return a string'
            );

            // Parse and decode again
            const parsed = JSON.parse(jsonStr);
            const roundTrip = savingsAccountDecode(parsed);
            assert.ok(
                roundTrip.success,
                `Round-trip deser failed: ${JSON.stringify(roundTrip.errors ?? [])}`
            );
            const rt = roundTrip.value;

            // Verify all fields survived the round-trip
            assert.equal(rt.variant, 'savings');
            assert.equal(rt.interestRate, 0.045);
            assert.equal(rt.id, 'acc_001');
            assert.equal(rt.name, "Alice's Savings");
            assert.equal(rt.balance, 15000.50);
            assert.ok(
                rt.createdAt instanceof Date,
                'createdAt should be Date after round-trip'
            );
            assert.equal(rt.createdAt.toISOString(), '2023-06-15T10:00:00.000Z');
        });
    });
});
