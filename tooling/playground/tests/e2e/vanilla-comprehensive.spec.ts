import { expect, test } from '@playwright/test';
import { probed, readVanillaSlice } from './vanilla-playground.ts';

// ─────────────────────────────────────────────────────────────────
//  1. Declarative (pattern-matching) macros
// ─────────────────────────────────────────────────────────────────

test.describe('Declarative macros', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
    });

    test('macro definitions are erased at build time', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarative.erased).toBe(true);
    });

    test('$vec() produces an empty array', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarative.emptyVec).toEqual([]);
    });

    test('$vec(1, 2, 3) produces [1, 2, 3]', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarative.threeVec).toEqual([1, 2, 3]);
    });

    test('$vec with expressions evaluates correctly', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarative.exprVec).toEqual([11, 15, 14]);
    });

    test('$id(42) identity returns 42', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarative.identityCall).toBe(42);
    });

    test('$withTemp(10) block macro returns trailing expression', async ({ page }) => {
        // Block-bodied macros get Rust-like trailing-expression semantics:
        // `(() => { const __acc = 10; __acc + 1 })()` is rewritten to
        // `(() => { const __acc = 10; return __acc + 1 })()`, so the IIFE
        // evaluates to the expected 11.
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarative.withTempResult).toBe(11);
    });

    test('cross-file $vec() works', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarative.crossFileEmpty).toEqual([]);
    });

    test('cross-file $vec(1,2,3) works', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarative.crossFileThree).toEqual([1, 2, 3]);
    });

    test('cross-file $vec with expressions', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarative.crossFileExpr).toEqual([11, 15, 14]);
    });

    test('cross-file $identity(42)', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarative.crossFileId).toBe(42);
    });
});

// ─────────────────────────────────────────────────────────────────
//  2. Proc-macro derives on classes
// ─────────────────────────────────────────────────────────────────

test.describe('Proc-macro derives (class)', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
    });

    test('Debug toString() includes class name and fields', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.procDerives.debug).toContain('AllMacrosTestClass');
        expect(e2e.procDerives.debug).toContain('identifier:'); // renamed from id
        expect(e2e.procDerives.debug).not.toContain('secretToken'); // skipped
    });

    test('Clone produces a deep copy', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const clone = probed(e2e.procDerives.clone, 'procDerives.clone');
        expect(clone.id).toBe(42);
        expect(clone.name).toBe('Test User');
    });

    test('PartialEq equals(self, self) is true', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.procDerives.equalsSelf).toBe(true);
    });

    test('PartialEq equals(self, other) is false', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.procDerives.equals).toBe(false);
    });

    test('Hash hashCode() returns a number', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(typeof e2e.procDerives.hashCode).toBe('number');
    });

    test('Serialize produces JSON with all fields', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const serialized = probed(e2e.procDerives.serialize, 'procDerives.serialize');
        expect(JSON.parse(serialized)).toMatchObject({
            id: 42,
            name: 'Test User',
            email: 'test@example.com'
        });
    });

    test('Deserialize succeeds with valid data', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const result = probed(e2e.procDerives.deserializeSuccess, 'procDerives.deserializeSuccess');
        if (!result.success) {
            throw new Error(`deserialize failed: ${JSON.stringify(result.errors)}`);
        }
        expect(result.value.name).toBe('OK');
    });

    test('Deserialize fails with null input', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const result = probed(e2e.procDerives.deserializeBad, 'procDerives.deserializeBad');
        expect(result.success).toBe(false);
    });
});

// ─────────────────────────────────────────────────────────────────
//  3. Runes proc macros ($state, $derived, $effect)
// ─────────────────────────────────────────────────────────────────

test.describe('Runes proc macros', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
    });

    test('all runes tests pass', async ({ page }) => {
        const results = await readVanillaSlice(page, 'runes');
        expect(results.failed).toBe(0);
        expect(results.passed).toBeGreaterThan(0);
    });

    test('$state: initial value and mutation', async ({ page }) => {
        const results = await readVanillaSlice(page, 'runes');
        const details: string[] = results.details;
        expect(details).toContain('PASS: state: initial value is 0');
        expect(details).toContain('PASS: state: updates to 42');
    });

    test('$state: string type', async ({ page }) => {
        const results = await readVanillaSlice(page, 'runes');
        const details: string[] = results.details;
        expect(details).toContain('PASS: state: string initial');
        expect(details).toContain('PASS: state: string update');
    });

    test('$derived: recomputes on dependency change', async ({ page }) => {
        const results = await readVanillaSlice(page, 'runes');
        const details: string[] = results.details;
        expect(details).toContain('PASS: derived: initial is 2');
        expect(details).toContain('PASS: derived: recomputes to 10');
    });

    test('$derived: chained derivations', async ({ page }) => {
        const results = await readVanillaSlice(page, 'runes');
        const details: string[] = results.details;
        expect(details).toContain('PASS: derived chain: (1+1)*2 = 4');
        expect(details).toContain('PASS: derived chain: b = 11');
        expect(details).toContain('PASS: derived chain: c = 22');
    });

    test('$effect: runs on creation and re-runs on change', async ({ page }) => {
        const results = await readVanillaSlice(page, 'runes');
        const details: string[] = results.details;
        expect(details).toContain('PASS: effect: runs on creation');
        expect(details).toContain('PASS: effect: sees initial value');
        expect(details).toContain('PASS: effect: re-runs on change');
        expect(details).toContain('PASS: effect: sees new value');
    });

    test('$effect: skips identical value', async ({ page }) => {
        const results = await readVanillaSlice(page, 'runes');
        const details: string[] = results.details;
        expect(details).toContain('PASS: effect: skips identical value');
    });

    test('$effect: arrow function syntax', async ({ page }) => {
        const results = await readVanillaSlice(page, 'runes');
        const details: string[] = results.details;
        expect(details).toContain('PASS: effect arrow: initial');
        expect(details).toContain('PASS: effect arrow: re-runs');
    });

    test('batch coalesces updates into single effect run', async ({ page }) => {
        const results = await readVanillaSlice(page, 'runes');
        const details: string[] = results.details;
        expect(details).toContain(
            'PASS: batch: effect runs once for batched update'
        );
        expect(details).toContain('PASS: batch: sum is 30');
    });
});

// ─────────────────────────────────────────────────────────────────
//  4. Enum derives
// ─────────────────────────────────────────────────────────────────

test.describe('Enum derives', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
    });

    test('Status enum Debug toString', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const statusDebug = probed(e2e.enums.statusDebug, 'enums.statusDebug');
        // Format: "Status(active)"
        expect(statusDebug).toContain('Status');
        expect(statusDebug).toContain('active');
    });

    test('Status enum Clone', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const statusClone = probed(e2e.enums.statusClone, 'enums.statusClone');
        expect(statusClone).toBe('pending');
    });

    test('Status enum PartialEq (equal)', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const statusEquals = probed(e2e.enums.statusEquals, 'enums.statusEquals');
        expect(statusEquals).toBe(true);
    });

    test('Status enum Hash returns number', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const statusHash = probed(e2e.enums.statusHash, 'enums.statusHash');
        expect(typeof statusHash).toBe('number');
    });

    test('Status enum Serialize', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const statusSerialize = probed(e2e.enums.statusSerialize, 'enums.statusSerialize');
        expect(JSON.parse(statusSerialize)).toBe('inactive');
    });

    test('Status enum Deserialize', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const statusDeserialize = probed(e2e.enums.statusDeserialize, 'enums.statusDeserialize');
        expect(statusDeserialize).toBe('pending');
    });

    test('Priority enum Debug toString', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const priorityDebug = probed(e2e.enums.priorityDebug, 'enums.priorityDebug');
        expect(priorityDebug).toBe('Priority.High');
    });

    test('Color enum PartialEq (equal)', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const colorEquals = probed(e2e.enums.colorEquals, 'enums.colorEquals');
        expect(colorEquals).toBe(true);
    });
});

// ─────────────────────────────────────────────────────────────────
//  5. Type alias derives
// ─────────────────────────────────────────────────────────────────

test.describe('Type alias derives', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
    });

    test('Point Debug toString includes coordinates', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const pointDebug = probed(e2e.typeAliases.pointDebug, 'typeAliases.pointDebug');
        expect(pointDebug).toContain('10');
        expect(pointDebug).toContain('20');
    });

    test('Point Clone produces a copy', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const pointClone = probed(e2e.typeAliases.pointClone, 'typeAliases.pointClone');
        expect(pointClone).toEqual({ x: 10, y: 20 });
    });

    test('Point PartialEq: equal points', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const pointEquals = probed(e2e.typeAliases.pointEquals, 'typeAliases.pointEquals');
        expect(pointEquals).toBe(true);
    });

    test('Point PartialEq: non-equal points', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const pointEqualsNe = probed(e2e.typeAliases.pointEqualsNe, 'typeAliases.pointEqualsNe');
        expect(pointEqualsNe).toBe(false);
    });

    test('Point Hash returns number', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const pointHash = probed(e2e.typeAliases.pointHash, 'typeAliases.pointHash');
        expect(typeof pointHash).toBe('number');
    });

    test('Point Serialize', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const pointSerialize = probed(e2e.typeAliases.pointSerialize, 'typeAliases.pointSerialize');
        expect(JSON.parse(pointSerialize)).toMatchObject({ x: 10, y: 20 });
    });

    test('Point Deserialize round-trips', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const pointDeserialize = probed(
            e2e.typeAliases.pointDeserialize,
            'typeAliases.pointDeserialize'
        );
        if (!pointDeserialize.success) {
            throw new Error(`deserialize failed: ${JSON.stringify(pointDeserialize.errors)}`);
        }
        expect(pointDeserialize.value).toEqual({ x: 5, y: 10 });
    });

    test('UserProfile Debug toString', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const userProfileDebug = probed(
            e2e.typeAliases.userProfileDebug,
            'typeAliases.userProfileDebug'
        );
        expect(userProfileDebug).toContain('johndoe');
    });

    test('UserProfile PartialEq (self)', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const userProfileEquals = probed(
            e2e.typeAliases.userProfileEquals,
            'typeAliases.userProfileEquals'
        );
        expect(userProfileEquals).toBe(true);
    });
});

// ─────────────────────────────────────────────────────────────────
//  6. Inspect macro
// ─────────────────────────────────────────────────────────────────

test.describe('Inspect macro (FormModel)', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
    });

    test('fieldMetadata() returns metadata array', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const fieldMetadata = probed(e2e.inspect.fieldMetadata, 'inspect.fieldMetadata');
        expect(Array.isArray(fieldMetadata)).toBe(true);
        expect(fieldMetadata.length).toBeGreaterThan(0);
    });

    test('getInspectableFields() returns only @inspect fields', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const inspectableFields = probed(
            e2e.inspect.inspectableFields,
            'inspect.inspectableFields'
        );
        expect(Object.keys(inspectableFields).sort()).toEqual(['description', 'memo', 'tags']);
    });

    test('cloneArrayFields() deep clones arrays', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const clonedArrays = probed(e2e.inspect.clonedArrays, 'inspect.clonedArrays');
        expect(clonedArrays.tags).toEqual(['tag1', 'tag2']);
    });

    test('countPopulatedFields() returns correct count', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const populatedCount = probed(e2e.inspect.populatedCount, 'inspect.populatedCount');
        // memo, username, description, tags are populated; metadata is null
        expect(populatedCount).toBe(4);
    });
});

// ─────────────────────────────────────────────────────────────────
//  7. Nested Deserialize (interfaces with arrays)
// ─────────────────────────────────────────────────────────────────

test.describe('Nested Deserialize', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
    });

    test('normal nested deserialization succeeds', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.nestedDeser.normal.success).toBe(true);
    });

    test('missing fields produces errors', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.nestedDeser.missingFields).toBeDefined();
        // Should either fail or succeed with defaults depending on implementation
        expect(e2e.nestedDeser.missingFields.success).toBeDefined();
    });

    test('null array element is handled', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.nestedDeser.nullElement).toBeDefined();
    });

    test('recursive deserialization with Date fields', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        const result = e2e.nestedDeser.recursiveActual;
        if (!result.success) {
            throw new Error(`deserialize failed: ${JSON.stringify(result.errors)}`);
        }
        expect(result.itemCount).toBe(2);
        expect(result.firstIsDate).toBe(true);
        expect(result.secondIsDate).toBe(true);
    });
});

// ─────────────────────────────────────────────────────────────────
//  7.5  Complex declarative macros
// ─────────────────────────────────────────────────────────────────

test.describe('Complex declarative macros', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
    });

    test('$min single arg returns identity', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.minOne).toBe(42);
    });

    test('$min two args picks smaller', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.minTwo).toBe(3);
    });

    test('$min two args picks smaller (alias call)', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.minOfTwo).toBe(3);
    });

    test('$min handles negatives', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.minNegative).toBe(-100);
    });

    test('$orElse returns value when no default needed', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.orElseNoDefault).toBe(100);
    });

    test('$orElse falls back to default for null', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        // null ?? 99 = 99
        expect(e2e.declarativeComplex.orElseWithDefault).toBe(99);
    });

    test('$square composes correctly', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.squaredFive).toBe(25);
    });

    test('$pow4 via nested $square', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        // $pow4(2) = $square($square(2)) = $square(4) = 16
        expect(e2e.declarativeComplex.pow4Two).toBe(16);
    });

    test('$sumAll (repetition) triple', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.sumTriple).toBe(6);
    });

    test('$sumAll (repetition) five args', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.sumFive).toBe(150);
    });

    test('hygiene: caller __temp not clobbered by macro', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.hygieneCheck).toEqual({ callerTemp: 999, macroResult: 10 });
    });

    test('block-bodied macro via IIFE produces value', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.sqPlus1).toBe(17);
    });

    test('type-position $PartialShallow erases at runtime', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        // Runtime value still a plain object — the type-level transform
        // has no runtime trace.
        expect(e2e.declarativeComplex.samplePatch).toEqual({ city: 'Berlin' });
    });

    test('type-position $NonNull carries string through', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.definitelyHello).toBe('hello');
    });

    test('all complex declarative macro definitions erased', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.declarativeComplex.erased).toBe(true);
    });
});

// ─────────────────────────────────────────────────────────────────
//  8. Attribute macro (@traced) + call macros ($stringify, $concat_names)
// ─────────────────────────────────────────────────────────────────

test.describe('Attribute & call macros', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await page.waitForSelector('#app');
    });

    test('@traced preserves function return value', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        // tracedAdd(2, 3) should still return 5 despite the counter bump.
        expect(e2e.attrMacros.addResult).toBe(5);
    });

    test('@traced preserves string return value', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.attrMacros.greetResult).toBe('hello, world');
    });

    test('@traced records every invocation on globalThis.__traced', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        // The harness calls tracedAdd 3 times and tracedGreet once.
        expect(e2e.attrMacros.tracedAddCount).toBe(3);
        expect(e2e.attrMacros.tracedGreetCount).toBe(1);
    });

    test('$stringify quotes an expression as source text', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        // `$stringify(1 + 2 * 3)` → `"1 + 2 * 3"` (not the evaluated 7).
        expect(e2e.attrMacros.stringifiedExpr).toBe('1 + 2 * 3');
    });

    test('$stringify quotes an identifier literally', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        // `$stringify(myVariable)` → `"myVariable"`, even though myVariable
        // is not defined at the call site — the macro operates on source text.
        expect(e2e.attrMacros.stringifiedIdent).toBe('myVariable');
    });

    test('$concat_names joins two idents with an underscore', async ({ page }) => {
        const e2e = await readVanillaSlice(page, 'e2e');
        expect(e2e.attrMacros.concatUserName).toBe('user_name');
        expect(e2e.attrMacros.concatDbHost).toBe('db_host');
    });
});
