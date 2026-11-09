/** @derive(Encode, Decode) */
interface Index {
    byPoint: Map<{ x: number; y: number }, string>;
}
