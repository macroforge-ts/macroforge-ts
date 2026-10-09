/**
 * Consumes newtypes from another module, so the type-check covers exported
 * brands, their convenience consts and guard narrowing across files.
 */

import { Meters, type Run, Username } from './newtype-validator-tests.ts';

/** @derive(Encode, Decode) */
export interface Trip {
    leg: Meters;
    legs: Meters[];
}

export function totalDistance(run: Run): Meters {
    const total = run.splits.reduce((sum, split) => sum + split, 0);
    return Meters.is(total) ? total : Meters.defaultValue();
}

export function greet(name: string): string {
    const decoded = Username.decode(name);
    return decoded.success ? `hello ${decoded.value}` : 'hello stranger';
}
