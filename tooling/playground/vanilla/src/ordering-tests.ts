/**
 * Ordering derives over array fields and tuple aliases.
 */

/** @derive(Ord, PartialOrd) */
export interface Scores {
    values: number[];
}

/** @derive(Ord, PartialOrd) */
export type Entry = [name: string, rank: number, ...tags: string[]];

/** @derive(Ord, PartialOrd) */
export type Span = [start: number, end?: number];

/** @derive(Ord, PartialOrd) */
export type Grid = [number[], Date];
