/** A tuple is held to its length, when it has no element and when none of them is checked. */
/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface Holder {
    none: [];
    nested: [[]];
    pair: [unknown, unknown];
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export type Nothing = [];
