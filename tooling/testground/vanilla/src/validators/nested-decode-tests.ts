/**
 * Nested values encoded and decoded through an interface and an object type
 * alias, which share one field encoder and decoder with classes.
 */

/** @derive(Encode, Decode) */
export interface Point {
    /** @endec(nonNegative) */
    x: number;
}

/** @derive(Encode, Decode) */
export type Route = {
    stops: Point[];
    named: Record<string, Point>;
    visits?: Map<string, number>;
};

/** @derive(Encode, Decode) */
export interface Labeled {
    label: string;
    /** @endec(flatten) */
    origin: Point;
}

/** @derive(Encode, Decode) */
export interface Box<T> {
    label: string;
    value: T;
}

/** @derive(Encode, Decode) */
export type Pair<A, B> = {
    first: A;
    second: B;
};

/** @derive(Encode, Decode) */
export type Outcome<T> = Box<T> | 'none';
