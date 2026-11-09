/** @derive(Encode, Decode) */
interface Circle {
    radius: number;
}

/** @derive(Encode, Decode) */
interface Square {
    side: number;
}

/** @derive(Encode, Decode) */
/** @endec({ tag: "kind", content: "data" }) */
type Shape = Circle | Square;
