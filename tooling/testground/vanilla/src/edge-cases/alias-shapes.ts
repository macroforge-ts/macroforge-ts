/**
 * Type aliases of every body a derive has to dispatch on: objects, tuples,
 * collections, intersections, unions of literals, objects and derived types
 * under each tagging mode, and types written through other type operators.
 */

import type { EdgeCard, EdgeItem } from './edge-types';

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export type AliasObject = {
    name: string;
    count: number;
    at?: Date;
};

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasTuple = [EdgeItem, Date, string | null];

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasArray = EdgeItem[];

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export type AliasReadonlyStrings = readonly string[];

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasDates = Date[];

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasRecord = Record<string, EdgeItem>;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasMap = Map<string, EdgeItem>;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasSet = Set<EdgeItem>;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasIntersection = EdgeItem & { extra: string };

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasIntersectionOfObjects = { left: string } & { right: number };

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export type AliasLiterals = /** @default */ 'small' | 'medium' | 'large';

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export type AliasMixedLiterals = /** @default */ 'auto' | 0 | 1 | true | null;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasNullable = EdgeItem | null;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasOptional = EdgeItem | undefined;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasDate = Date;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasDateOrNull = Date | null;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasMembers = EdgeItem | EdgeCard;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
/** @endec({ tag: "kind" }) */
export type AliasTagged =
    | { kind: 'circle'; radius: number }
    | { kind: 'square'; side: number; at: Date }
    | { kind: 'group'; members: AliasTagged[] };

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
/** @endec({ tag: "type", content: "value" }) */
export type AliasAdjacent = EdgeItem | EdgeCard;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
/** @endec({ externallyTagged: true }) */
export type AliasExternal = EdgeItem | EdgeCard;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
/** @endec({ untagged: true }) */
export type AliasUntagged = EdgeItem | EdgeCard | string;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasDeep = string | number | boolean | null | EdgeItem | AliasArray | Date;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface AliasOperators {
    indexed: EdgeItem['id'];
    keys: keyof EdgeItem;
    template: `item-${number}`;
    literal: 'fixed';
    truth: true;
    zero: 0;
    anything: unknown;
    structure: object;
    required: Required<AliasObject>;
    readonlyItem: Readonly<EdgeItem>;
    nonNull: NonNullable<EdgeItem | null>;
    inline: { x: number; nested: { at: Date; item: EdgeItem } };
    inlineList: { id: string }[];
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface AliasHolder {
    object: AliasObject;
    size: AliasLiterals;
    sizes: AliasLiterals[];
    shape: AliasTagged | null;
    shapes: AliasTagged[];
    tuple: AliasTuple | null;
    /** @default([]) */
    list: AliasArray;
    /** @default({}) */
    table: AliasRecord;
    members: AliasMembers[];
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type AliasOutcome =
    | { ok: true; at: Date; tags: Set<string> }
    | { ok: false; reason: string; retries?: number };

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
/** @endec({ untagged: true }) */
export type AliasLoose = { kind: 'a'; when: Date } | { kind: 'b'; counts: Map<string, number> };

/** @derive(Encode, Decode) */
export type AliasName = string;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface AliasCarried {
    tags: string | string[];
    size: number | 'auto' | null;
    when: Date | string;
    lookup: Record<string, number> | number[];
    anything: unknown;
}

/** @derive(Default, Encode, Decode) */
export type AliasPicked = Pick<EdgeItem, 'id'>;

/** @derive(Default, Encode, Decode) */
export type AliasRequired = Required<EdgeItem>;

/** @derive(Default, Encode, Decode) */
export type AliasInline = { x: number; label?: string; at: Date };
