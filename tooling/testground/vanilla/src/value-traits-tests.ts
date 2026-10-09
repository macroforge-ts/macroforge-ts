/**
 * PartialEq, Hash and Clone over built-in, collection, optional, union and
 * tuple types. The three must agree: a clone equals its source, and equal
 * values hash the same.
 */

/** @derive(PartialEq, Hash, Clone) */
export interface Timeline {
    events: Date[];
    labels: Map<string, Date>;
    started?: Date;
    note: string | null;
}

/** @derive(PartialEq, Hash, Clone) */
export class Badge {
    label: string = '';
}

/** @derive(PartialEq, Hash, Clone) */
export interface Wall {
    badges: Set<Badge>;
    pinned: unknown;
}

/** @derive(PartialEq, Hash, Clone) */
export type Shape =
    | { kind: 'circle'; radius: number }
    | { kind: 'square'; side: number; at: Date };

/** @derive(PartialEq, Hash, Clone) */
export type Moment = [label: string, at: Date];

/** @derive(PartialEq, Hash, Clone) */
export type Reading = string | number[];

/** @derive(Debug) */
export interface Inventory {
    counts: Map<string, number>;
    owner: { name: string };
    total: bigint;
}

/** @derive(Debug) */
export type Amount = bigint | { value: bigint };

/** @derive(Encode, Decode) */
export class Ledger {
    entries: Map<string, number> = new Map();
}

/** @derive(Encode, Decode) */
export type LedgerPair = [Ledger, Ledger];

/** @derive(Encode, Decode) */
export type MainLedger = Ledger;

/** @derive(Encode, Decode) */
export type Totals = Map<string, number>;

/** @derive(Encode) */
export type LedgerOrNote = Ledger | string;

/**
 * @derive(Encode)
 * @endec({ tag: "kind" })
 */
export type Tally =
    | { kind: 'counts'; counts: Map<string, number> }
    | { kind: 'note'; text: string };

/** @derive(Default) */
export class Shelf<T> {
    items: T[] = [];
    label: string = '';
}

/** @derive(PartialEq, Eq, PartialOrd, Ord) */
export interface Rank {
    label: string;
    at?: Date;
    meta: { tier: number };
}

/** @derive(PartialEq, Eq, PartialOrd, Ord) */
export type Choice = { kind: 'a'; count: number } | { kind: 'b'; name: string } | null;

/** @derive(PartialEq, Hash, Clone, Debug, Encode, Decode) */
export interface Labeled<T extends string = string> {
    label: T;
    weight: number;
}

/** @derive(Default) */
export class Crate<T extends object> {
    items: T[] = [];
}

/** @derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Encode, Decode) */
export interface Keyed<K extends string> {
    key: K;
    rank: number;
}

/** @derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Encode, Decode) */
export class Slot<T extends object> {
    held: T | null = null;
    index: number = 0;
}

/** @derive(Encode, Decode) */
export type Counted<T extends object> = [T, number];

/** @derive(Encode, Decode) */
export type Code<T extends string> = T | number;

/** @derive(Encode, Decode) */
export type Boxed<T extends object> = Array<T>;
