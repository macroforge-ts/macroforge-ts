/**
 * # Macroforge Effect Reexports
 *
 * Re-exports Effect library functions for generated macro code.
 * @module macroforge/reexports/effect
 */
export { succeed as exitSucceed, fail as exitFail, isSuccess as exitIsSuccess } from "effect/Exit";
export type { Exit } from "effect/Exit";
export { some as optionSome, none as optionNone, isNone as optionIsNone } from "effect/Option";
export type { Option } from "effect/Option";
