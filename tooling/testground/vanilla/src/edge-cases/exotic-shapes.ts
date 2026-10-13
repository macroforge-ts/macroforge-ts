/**
 * Types at the edge of what a type can say: template literal types, `object`,
 * a lone `null`, unions of literals of several kinds, types with nothing in
 * them, and an interface that declares methods.
 */

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface ExoticScalars {
    id: `id-${number}`;
    tag: `${string}-${boolean}`;
    held: object;
    nothing: null;
    choice: 'a' | 'b' | 3 | true;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type ExoticCode = `code-${bigint}`;

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export type ExoticNone = [];

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface ExoticEmpty {}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ExoticBlank {}

/** @derive(Debug, Clone, PartialEq, Hash) */
export interface ExoticMethods {
    id: string;
    describe(): string;
    maybe?(): void;
}
