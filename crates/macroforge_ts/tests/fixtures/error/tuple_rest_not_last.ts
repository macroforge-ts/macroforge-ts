/** A tuple whose rest element is not its last is an error, not a shorter tuple. */
/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */
export interface Row {
    middle: [string, ...number[], boolean];
    held: Map<string, readonly [...string[], number]>;
}

/** @derive(Default, Decode) */
export type Leading = [...first: string[], last: number];

/** A rest that ends its tuple is read. */
/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */
export interface Fine {
    tail: [string, ...number[]];
    nested: [[string, ...number[]], boolean];
}
