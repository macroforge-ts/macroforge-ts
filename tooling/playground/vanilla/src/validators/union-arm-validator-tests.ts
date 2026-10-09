/**
 * Untagged unions whose primitive arms carry their own validators.
 */

/** @derive(Decode) */
export type Contact = /** @endec(email) */ string | number;

/** @derive(Decode) */
export interface Phone {
    digits: string;
}

/** @derive(Decode) */
export type Reachable =
    | /** @endec(nonEmpty, maxLength(32)) */ string
    | Phone;

/** @derive(Decode) */
export type Fallback = 'none' | /** @endec(email) */ string;
