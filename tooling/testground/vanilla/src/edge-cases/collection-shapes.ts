/**
 * Collections nested in one another and keyed or filled by every kind of
 * value a derive has a strategy for: primitives, dates, bigints, enums,
 * derived types, unions and `null`.
 */

import type { EdgeCard, EdgeItem } from './edge-types';

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export enum CollectionKind {
    /** @default */
    Plain = 'plain',
    Fancy = 'fancy'
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface CollectionNested {
    grid: number[][];
    dateGrid: Array<Array<Date>>;
    itemGrid: EdgeItem[][];
    datesByName: Map<string, Date[]>;
    itemsByName: Record<string, EdgeItem[]>;
    setsByName: Map<string, Set<string>>;
    mapOfMaps: Map<string, Map<string, number>>;
    cardsByName: Map<string, EdgeCard>;
    optionalItems?: EdgeItem[];
    maybeItems: EdgeItem[] | null;
    laterItems: EdgeItem[] | undefined;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface CollectionKeys {
    byNumber: Map<number, string>;
    byKind: Map<CollectionKind, number>;
    byDate: Map<Date, string>;
    kindLabels: Record<CollectionKind, string>;
    numbered: Record<number, EdgeItem>;
    kinds: Set<CollectionKind>;
    dates: Set<Date>;
    nullableDates: (Date | null)[];
    dateSet: Set<Date | null>;
    dateRecord: Record<string, Date>;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface CollectionUnions {
    mixed: (string | number)[];
    itemOrText: Array<EdgeItem | string>;
    itemOrNull: Set<EdgeItem | null>;
    valueOrNull: Map<string, EdgeItem | null>;
    recordOrNull: Record<string, EdgeItem | null>;
    literal: ('on' | 'off')[];
    kindOrNull: (CollectionKind | null)[];
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface CollectionTuples {
    pair: [string, number];
    named: [label: string, at: Date];
    itemPair: [EdgeItem, EdgeItem | null];
    readonlyPair: readonly [number, number];
    optionalTail: [string, number?];
    rest: [string, ...number[]];
    pairs: [string, EdgeItem][];
    nested: [[number, number], [string, Date]];
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface CollectionBig {
    total: bigint;
    totals: bigint[];
    maybeTotal: bigint | null;
    byName: Map<string, bigint>;
}
