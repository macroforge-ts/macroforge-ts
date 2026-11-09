/** @derive(Encode, Decode) */
/** @endec({ tag: "kind" }) */
type Shape =
    | { kind: 'circle'; radius: number }
    | { kind: 'event'; at: Date; note?: string };
