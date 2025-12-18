/**
 * # Macroforge Reexports
 *
 * Re-exports @rydshift/mirror functions for generated macro code.
 * @module macroforge/reexports
 */

// Result functions (Deserialize)
export { ok, error, isOk, Result } from "@rydshift/mirror/declarative";
export type { Result as ResultType } from "@rydshift/mirror/declarative";

// Option functions (PartialOrd)
export { some, none, isNone, Option } from "@rydshift/mirror/declarative";
export type { Option as OptionType } from "@rydshift/mirror/declarative";
