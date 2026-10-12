/** @derive(Encode, Decode) */
interface Box<T> {
    held: T;
    list: T[];
    maybe: T | null;
    later?: T;
}

/** @derive(Encode, Decode) */
class Stack<T> {
    items: T[] = [];
}

/** @derive(Encode, Decode) */
type Outcome<Value, Failure> = { ok: true; value: Value } | { ok: false; failure: Failure };

/** @derive(Encode, Decode) */
type Maybe<T> = T | null;

/** @derive(Encode, Decode) */
type Pair<Left, Right> = [Left, Right];

/** @derive(Encode, Decode) */
interface Wrapper<T> {
    inner: Box<T>;
}

/** @derive(Encode, Decode) */
interface Shelf {
    dated: Box<Date>;
    stack: Stack<Box<Date>>;
    outcome: Outcome<Date, string>;
    maybe: Maybe<Date>;
    pair: Pair<Date, bigint>;
    partial: Partial<Box<Date>>;
    wrapper: Wrapper<Date> | null;
    plain: Box<string>;
}
