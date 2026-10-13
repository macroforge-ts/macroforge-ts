/** @derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Default) */
export interface Options {
    /** @debug({ skp: true }) */
    misspelled: string;
    /** @partialEq({ ignore: true }) */
    unknown: string;
    /** @ord({ skipp: true }) */
    ordered: string;
    /** @ord(skip, nonsense) */
    several: string;
    /** @default(1, 2) */
    twice: number;
    /** @debug({ skip: true, rename: "kept, as it is" }) */
    known: string;
    /** @hash(skip) */
    flagged: string;
    /** @default({ a: 1, b: [2, 3] }) */
    whole: { a: number; b: number[] };
}

/** @derive(Debug, Encode) */
/** @endec({ tagg: "kind", renameAll: "snake_case" }) */
export interface Container {
    id: string;
}
