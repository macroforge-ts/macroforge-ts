/**
 * Generic types: parameters with constraints and defaults, parameters used
 * in every position a field can hold one, and fields that instantiate
 * another generic type, a derived one included.
 */

import type { EdgeItem } from './edge-types';

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface GenericBox<T> {
    held: T;
    list: T[];
    maybe: T | null;
    optional?: T;
    byName: Map<string, T>;
    set: Set<T>;
    record: Record<string, T>;
    pair: [T, T];
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface GenericPair<Left, Right = Left> {
    left: Left;
    right: Right;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface GenericConstrained<Item extends EdgeItem, Name extends string = string> {
    item: Item;
    name: Name;
    items: Item[];
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class GenericStack<T> {
    items: T[] = [];
    top: T | null = null;
    depth: number = 0;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type GenericResult<Value, Failure> =
    | { ok: true; value: Value }
    | { ok: false; failure: Failure };

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type GenericList<T> = T[];

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type GenericMaybe<T> = T | null;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface GenericUses {
    text: GenericBox<string>;
    item: GenericBox<EdgeItem>;
    nested: GenericBox<GenericBox<number>>;
    pair: GenericPair<string, EdgeItem>;
    same: GenericPair<number>;
    stack: GenericStack<EdgeItem>;
    stacks: GenericStack<string>[];
    list: GenericList<EdgeItem>;
    maybe: GenericMaybe<EdgeItem>;
    boxes: Map<string, GenericBox<Date>>;
}
