/**
 * Object graphs rather than trees: a value held twice, a child pointing back
 * at its parent through a collection, a value that holds itself, and a
 * `Pick` or `Omit` standing in a cycle.
 */

import type { EdgeItem } from './edge-types';

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface GraphTree {
    name: string;
    parent: GraphTree | null;
    children: GraphTree[];
}

/** @derive(Encode, Decode) */
export class GraphPerson {
    name: string = '';
    best: GraphPerson | null = null;
    friends: GraphPerson[] = [];
    circle: Set<GraphPerson> = new Set();
    byName: Map<string, GraphPerson> = new Map();
}

/** @derive(Encode, Decode) */
export interface GraphShared {
    first: EdgeItem;
    again: EdgeItem;
    maybe: EdgeItem | null;
    list: EdgeItem[];
    set: Set<EdgeItem>;
    byName: Map<string, EdgeItem>;
    record: Record<string, EdgeItem>;
    pair: [EdgeItem, EdgeItem];
}

/** @derive(Encode, Decode) */
export interface GraphLoop {
    name: string;
    self: GraphLoop | null;
    view: Pick<GraphLoop, 'name' | 'view'> | null;
    rest: Omit<GraphLoop, 'name'> | null;
    patch: Partial<GraphLoop> | null;
}

/** @derive(Encode, Decode) */
export interface GraphPatched {
    whole: EdgeItem;
    patch: Partial<EdgeItem>;
    patches: Partial<EdgeItem>[];
}

/** @derive(Encode, Decode) */
export interface GraphPatchedFirst {
    patch: Partial<EdgeItem>;
    whole: EdgeItem;
}
