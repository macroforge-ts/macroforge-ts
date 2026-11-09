/**
 * Generic types: parameters with constraints and defaults, parameters used
 * in every position a field can hold one, and fields that instantiate
 * another generic type, a derived one included.
 */

import type { EdgeCard, EdgeItem } from './edge-types';

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

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface GenericWrapper<T> {
    inner: GenericBox<T>;
    pairs: GenericPair<T, T[]>[];
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type GenericOutcome<T> = GenericPair<T, T[]> | 'none';

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface GenericArguments {
    dated: GenericBox<Date>;
    maybe: GenericBox<Date> | null;
    optional?: GenericBox<EdgeCard>;
    pair: GenericPair<Date, Map<string, Date>>;
    stack: GenericStack<Date>;
    result: GenericResult<Date, EdgeCard>;
    partial: Partial<GenericBox<Date>>;
    picked: Pick<GenericPair<Date, bigint>, 'right'>;
    wrapper: GenericWrapper<Date>;
    plain: GenericBox<string>;
    link: string | GenericBox<Date>;
    linkOrNull: string | GenericBox<Date> | null;
    outcome: GenericOutcome<Date>;
    outcomes: GenericOutcome<Date>[];
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class GenericDefaults<T extends string | number = string, U = T[]> {
    value: T | null = null;
    many: U | null = null;
    pair: [T, T] | null = null;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface GenericDefaulted {
    text: GenericDefaults;
    counted: GenericDefaults<number>;
    explicit: GenericDefaults<string, string[]>;
}

/** A member tagged in place over a type parameter, coded by the coders handed in for it. */
/** @derive(Encode, Decode) */
/** @endec({ tag: "kind" }) */
export type GenericTagged<T> = ({ kind: 'Held' } & T) | { kind: 'Empty'; size: number };
