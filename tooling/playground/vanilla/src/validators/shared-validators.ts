/**
 * Validator functions shared across modules, imported by the expansion of
 * the classes that name them with `custom({ function, source })`.
 */

export function isMultipleOfThree(value: number): boolean {
    return value % 3 === 0;
}
