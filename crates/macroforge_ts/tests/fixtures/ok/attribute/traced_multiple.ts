/** import macro { traced } from "@macroforge/test-macros" */

/** @traced */
export function first(x: number): number {
    return x * 2;
}

/** @traced */
export function second(x: number): number {
    return x + 10;
}

function untouched(x: number): number {
    return x;
}
