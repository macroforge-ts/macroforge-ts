/**
 * Ordering derives over array fields, tuple aliases, a class, and a class
 * whose array field holds another class deriving Ord.
 */

/** @derive(PartialEq, Eq, PartialOrd, Ord) */
export interface Scores {
    values: number[];
}

/** @derive(PartialEq, Eq, PartialOrd, Ord) */
export type Entry = [name: string, rank: number, ...tags: string[]];

/** @derive(PartialEq, Eq, PartialOrd, Ord) */
export type Span = [start: number, end?: number];

/** @derive(PartialEq, Eq, PartialOrd, Ord) */
export type Grid = [number[], Date];

/** @derive(PartialEq, Eq, PartialOrd, Ord) */
export class Version {
    major: number = 0;
    minor: number = 0;
}

/** @derive(PartialEq, Eq, PartialOrd, Ord) */
export class Release {
    versions: Version[] = [];
}
