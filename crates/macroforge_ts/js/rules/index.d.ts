/**
 * # Macroforge Rules Module
 *
 * Declarative pattern-matching macros for TypeScript. Define macros with
 * `` const $name = macroRules`...` ``, invoke them as `$name(args)`, and the
 * macroforge build pass rewrites call sites at compile time.
 *
 * Example:
 *
 * ```typescript
 * import { macroRules } from "macroforge/rules";
 *
 * const $vec = macroRules`
 *   () => []
 *
 *   ($($x:Expr),+ $(,)?) => {
 *     const __v = [];
 *     $( __v.push($x); )+
 *     __v
 *   }
 * `;
 *
 * const empty = $vec();
 * const xs = $vec(1, 2, 3);
 * ```
 *
 * ## Runtime behavior
 *
 * `macroRules` is a sentinel tag function. It throws at runtime if the
 * macroforge build pass is not installed — if you see the runtime error,
 * your build toolchain is not running macroforge on this file.
 *
 * @module macroforge/rules
 */
/**
 * A macro invocation — a function that takes any arguments and may produce
 * any value. The exact return type depends on the macro body; for accurate
 * types, the macroforge build pass is responsible for erasing the macro
 * definition and inlining the expansion at each call site. Only this
 * placeholder shape is visible to the TypeScript type checker.
 */
export type MacroInvocation = (...args: any[]) => any;
/**
 * The macro definition tag function.
 *
 * At build time, `` const $name = macroRules`...` `` is recognized by
 * macroforge, the template body is parsed as a macro definition, and the
 * declaration is erased from the output. Call sites of `$name(...)` are
 * rewritten in place with the matching arm's body.
 *
 * The return type is a generic callable, so TypeScript lets users invoke
 * `$name(...)` without complaint. At runtime (if the build pass did not
 * run) the tag itself throws — any caller would already have seen the
 * compile-time rewrite.
 */
export declare function macroRules(_strings: TemplateStringsArray, ..._values: unknown[]): MacroInvocation;
/**
 * Configuration for a macro's reverse-monomorphization behavior.
 *
 * Currently a type-only declaration — the object form
 * `macroRules({ expand, runtime, call, mode })` is part of the reverse-mono
 * follow-up and is not yet wired through the build pass. The type is
 * exported now so consumer code can start using the shape.
 */
export interface MacroConfig {
    /**
     * Controls how the macro emits in dev vs. prod builds.
     *
     * - `"auto"` (default): dev expands inline, prod shares runtime when safe.
     * - `"expand-only"`: always expand inline at every call site.
     * - `"share-only"`: always emit calls to a shared runtime helper.
     * - `"share-anyway"`: share even past the megamorphism threshold.
     */
    readonly mode?: "auto" | "expand-only" | "share-only" | "share-anyway";
    /**
     * The expand-form macro template (used for dev + `expand-only` + type-check).
     */
    readonly expand?: unknown;
    /**
     * The shared runtime body, emitted once per module when the macro
     * is in a sharing mode.
     */
    readonly runtime?: string;
    /**
     * The call-form template that replaces call sites when the macro
     * is in a sharing mode.
     */
    readonly call?: unknown;
    /**
     * Above this count of distinct types calling the shared runtime,
     * the megamorphism analyzer emits a warning. Default: 4.
     */
    readonly megamorphismThreshold?: number;
}
