/**
 * Values at the edges of their types: numbers that are not ordinary numbers,
 * dates that are no date, collections whose order says nothing, and
 * collections that hold `undefined`, which JSON has no word for.
 */

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export interface SpecialScalars {
    value: number;
    at: Date;
    big: bigint;
    text: string;
    flag: boolean;
    list: number[];
}

/** @derive(Encode, Decode) */
export type SpecialNumber = number;

/** @derive(Encode, Decode) */
export type SpecialEither = number | SpecialCarried;

/** @derive(Encode, Decode) */
export interface SpecialCarried {
    data: unknown;
    amounts: Record<string, number>;
    pair: [number, string];
    mixed: string | number;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface SpecialCollections {
    set: Set<number>;
    map: Map<string, number[]>;
    record: Record<string, Date>;
    nested: Map<string, Set<string>>;
    optional?: number;
    nullable: number | null;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface SpecialHoles {
    list: (number | undefined)[];
    pair: [number | undefined, string?];
    record: Record<string, number | undefined>;
    finite: Record<'a' | 'b', Date | undefined>;
    set: Set<number | undefined>;
    held: Map<string, number | undefined>;
    either: (number | null | undefined)[];
    plain: string | undefined;
    grouped: string | undefined;
}
