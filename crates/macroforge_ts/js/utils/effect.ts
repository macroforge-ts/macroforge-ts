/**
 * # Macroforge Effect Utils Module
 *
 * This module re-exports Effect library types for use in generated macro code
 * when `returnTypes: 'effect'` is configured.
 *
 * ## Exit<E, A>
 *
 * Represents the result of a computation that may fail.
 * Used by the `Deserialize` macro for validation errors.
 *
 * ```typescript
 * import { Exit } from "macroforge/utils/effect";
 *
 * const result = User.fromStringifiedJSON(json);
 * if (Exit.isSuccess(result)) {
 *   const user = result.value;
 * } else {
 *   console.error(result.cause); // Array of field errors
 * }
 * ```
 *
 * ## Option<A>
 *
 * Represents an optional value.
 * Used by `PartialOrd` macro for incomparable values.
 *
 * ```typescript
 * import { Option } from "macroforge/utils/effect";
 *
 * const cmp = user1.compareTo(user2);
 * if (Option.isSome(cmp)) {
 *   console.log(cmp.value); // -1, 0, or 1
 * } else {
 *   console.log("Values are incomparable");
 * }
 * ```
 *
 * @module macroforge/utils/effect
 */

// Re-export Exit and Option from effect for use in generated code
export { Exit, Option } from "effect";
