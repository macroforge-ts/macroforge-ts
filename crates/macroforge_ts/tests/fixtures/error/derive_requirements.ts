/** @derive(Eq) */
export interface OnlyEq {
    id: number;
}

/** @derive(PartialOrd) */
export interface OnlyPartialOrd {
    id: number;
}

/** @derive(PartialEq, PartialOrd, Ord) */
export interface OrdWithoutEq {
    id: number;
}
