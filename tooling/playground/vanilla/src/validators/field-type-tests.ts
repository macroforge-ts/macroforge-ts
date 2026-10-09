/**
 * Field types decode checks: primitive-like fields, foreign-type fields,
 * and primitive aliases branded by string keys.
 */

/** @derive(Decode) */
export interface Profile {
    name: string;
    age: number;
    active: boolean;
    kind: 'user';
    mode: 'light' | 'dark';
    nickname: string | null;
    bio?: string;
    rank: number | undefined;
    height: $Newtype<number>;
    balance: bigint;
    site: URL;
}

/** @derive(Encode, Decode, Hash, Debug) */
export type Tagged = number & { readonly __brand: 'Tagged' };

/** @derive(Encode, Decode, Hash, Debug) */
export type Cents = $Newtype<bigint>;
