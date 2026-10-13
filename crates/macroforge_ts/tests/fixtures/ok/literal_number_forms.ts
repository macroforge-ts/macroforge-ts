/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface Literals {
    sign: -1 | 0 | 1;
    grouped: 1_000 | 2_000;
    hex: 0x10 | 0b11;
    big: 1n | -2n;
    exp: 1e3 | 2.5e-1;
    byNumber: Record<1 | 2, string>;
}

/** A type named as a float parser's words for a number is a type. */
/** @derive(Debug, Clone, Default, Encode, Decode) */
export interface Inf {
    value: number;
}

/** @derive(Debug, Clone, Default, Encode, Decode) */
export interface Holder {
    inf: Inf;
    list: Inf[];
}
