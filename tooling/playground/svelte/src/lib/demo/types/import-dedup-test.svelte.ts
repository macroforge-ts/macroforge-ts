/**
 * Stress test for aliased imports and tree-shaking.
 *
 * Macroforge now imports specific functions with `__mf_` prefixed aliases
 * to avoid any risk of collision with user imports:
 *
 * - `import { succeed as __mf_exitSucceed } from "effect/Exit";`
 * - `import { some as __mf_optionSome } from "effect/Option";`
 *
 * This approach:
 * - Eliminates namespace collisions with user imports
 * - Enables better tree-shaking (only specific functions imported)
 * - Is explicit about what macroforge uses
 *
 * The user's `import { Exit, Option } from "effect"` is preserved and works
 * alongside the aliased imports without any conflict.
 */

import { Exit, Option } from 'effect';

// Test 1: Type with Deserialize (uses Exit for return type)
/** @derive(Deserialize) */
export interface UserWithDeserialize {
    name: string;
    age: number;
    email: string;
}

// Test 2: Type with PartialOrd (uses Option for return type)
/** @derive(PartialOrd) */
export interface ComparableItem {
    priority: number;
    name: string;
}

// Test 3: Type with both Deserialize and PartialOrd
/** @derive(Deserialize, PartialOrd) */
export interface FullFeaturedType {
    id: number;
    value: string;
    score: number;
}

// Test 4: Multiple types to stress test deduplication across types
/** @derive(Deserialize) */
export interface TypeA {
    fieldA: string;
}

/** @derive(Deserialize) */
export interface TypeB {
    fieldB: number;
}

/** @derive(PartialOrd) */
export interface TypeC {
    fieldC: number;
}

// Export a helper to verify imports work at runtime
export function verifyImports(): void {
    // These should work because we imported from "effect"
    const success = Exit.succeed({ name: 'test', age: 25, email: 'test@example.com' });
    const failure = Exit.fail([{ field: 'name', message: 'required' }]);
    const some = Option.some(42);
    const none = Option.none();

    console.log('Exit.isSuccess:', Exit.isSuccess(success));
    console.log('Exit.isFailure:', Exit.isFailure(failure));
    console.log('Option.isSome:', Option.isSome(some));
    console.log('Option.isNone:', Option.isNone(none));
}
