/** @derive(Clone, PartialEq, Encode, Decode) */
export interface TooDeep {
    value: number[][][][][][][][][][][][][][][][][];
}

/** @derive(Clone, PartialEq, Encode, Decode) */
export type TooNested = Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<number>>>>>>>>>>>>>>>>>;

/** @derive(Clone, PartialEq, Encode, Decode) */
export interface AtTheLimit {
    value: { held: { held: { held: { held: { held: { held: { held: { held: { held: { held: { held: { held: { held: { held: { held: { held: number } } } } } } } } } } } } } } } };
}
