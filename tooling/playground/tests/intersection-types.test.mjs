/**
 * E2E tests for intersection type support.
 *
 * Tests that intersection types like `{ variant: 'savings' } & AccountBase`
 * correctly serialize, deserialize, and round-trip through the full
 * Vite macro pipeline at runtime.
 */

import { describe, test } from 'node:test';
import assert from 'node:assert/strict';
import { svelteRoot, withViteServer } from './test-utils.mjs';

describe('Intersection types E2E — Serialize & Deserialize via SvelteKit', () => {
    test('deserializes a SavingsAccount intersection type from raw JSON', async () => {
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
            const { savingsAccountDeserialize } = typesMod;
            assert.equal(
                typeof savingsAccountDeserialize,
                'function',
                'savingsAccountDeserialize should be exported'
            );

            // Deserialize
            const result = savingsAccountDeserialize(raw);
            assert.ok(
                result.success,
                `Deserialization failed: ${JSON.stringify(result.errors ?? [])}`
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

    test('deserializes a CheckingAccount intersection type from raw JSON', async () => {
        await withViteServer(svelteRoot, async (server) => {
            const fixtureMod = await server.ssrLoadModule('/src/lib/e2e/fixture.ts');
            const raw = fixtureMod.checkingAccountFixture;

            const typesMod = await server.ssrLoadModule(
                '/src/lib/e2e/types.svelte.ts'
            );
            const { checkingAccountDeserialize } = typesMod;
            assert.equal(
                typeof checkingAccountDeserialize,
                'function',
                'checkingAccountDeserialize should be exported'
            );

            const result = checkingAccountDeserialize(raw);
            assert.ok(
                result.success,
                `Deserialization failed: ${JSON.stringify(result.errors ?? [])}`
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

    test('serializes an intersection type and round-trips through deserialize', async () => {
        await withViteServer(svelteRoot, async (server) => {
            const fixtureMod = await server.ssrLoadModule('/src/lib/e2e/fixture.ts');
            const raw = fixtureMod.savingsAccountFixture;

            const typesMod = await server.ssrLoadModule(
                '/src/lib/e2e/types.svelte.ts'
            );
            const {
                savingsAccountSerialize,
                savingsAccountDeserialize
            } = typesMod;

            // Deserialize from raw
            const desResult = savingsAccountDeserialize(raw);
            assert.ok(
                desResult.success,
                `Initial deser failed: ${JSON.stringify(desResult.errors ?? [])}`
            );
            const account = desResult.value;

            // Serialize back to JSON string
            assert.equal(
                typeof savingsAccountSerialize,
                'function',
                'savingsAccountSerialize should be exported'
            );
            const jsonStr = savingsAccountSerialize(account);
            assert.equal(
                typeof jsonStr,
                'string',
                'serialize should return a string'
            );

            // Parse and deserialize again
            const parsed = JSON.parse(jsonStr);
            const roundTrip = savingsAccountDeserialize(parsed);
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
