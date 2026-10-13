/**
 * Types at the edge of what a type can say: template literal types, `object`,
 * a lone `null`, unions of literals of several kinds, types with nothing in
 * them, an interface that declares methods, records by the kind of key they
 * are over, and a type nested as deep as a derive reads.
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

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type ExoticLabels = `a-${number}` | `b-${string}` | 'plain' | 5;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface ExoticTexts {
    either: `a-${number}` | `b-${string}`;
    list: `k-${number}`[];
    mapped: Map<`m-${bigint}`, string>;
    optional?: `o-${boolean}`;
    nullable: `n-${string}` | null;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface ExoticKeyed {
    finite: Record<'a' | 'b', number>;
    counted: Record<number, string>;
    patterned: Record<`k-${number}`, boolean>;
    open: Record<string, number>;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface ExoticAccessors {
    id: string;
    get size(): number;
    set size(value: number);
    get stamp(): Date;
}

/** @derive(Debug, Clone, PartialEq, Hash) */
export type ExoticMethodAlias = {
    id: string;
    run(): string;
};

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface ExoticDeep {
    objects: {
        held: {
            held: {
                held: {
                    held: {
                        held: {
                            held: {
                                held: {
                                    held: {
                                        held: {
                                            held: {
                                                held: {
                                                    held: {
                                                        held: { held: { held: { held: number } } };
                                                    };
                                                };
                                            };
                                        };
                                    };
                                };
                            };
                        };
                    };
                };
            };
        };
    };
    lists: [[[[[[[[[[[[[[[[number]]]]]]]]]]]]]]]];
}
