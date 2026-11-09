/** @derive(Encode, Decode) */
interface Box<T> {
    held: T;
}

/** @derive(Encode, Decode) */
interface Circle {
    radius: number;
}

/** @derive(Encode, Decode) */
type Shape = Box<Date> | Circle;

/** @derive(Encode, Decode) */
type Outcome<T> = Box<T> | 'none';

/** @derive(Encode, Decode) */
/** @endec({ tag: "kind" }) */
type Tagged<T> = ({ kind: 'box' } & Box<T>) | ({ kind: 'circle' } & Circle);
