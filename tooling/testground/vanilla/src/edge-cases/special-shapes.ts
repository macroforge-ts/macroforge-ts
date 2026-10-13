/**
 * Values at the edges of their types: numbers that are not ordinary numbers,
 * dates that are no date, and collections whose order says nothing.
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

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface SpecialCollections {
    set: Set<number>;
    map: Map<string, number[]>;
    record: Record<string, Date>;
    nested: Map<string, Set<string>>;
    optional?: number;
    nullable: number | null;
}
