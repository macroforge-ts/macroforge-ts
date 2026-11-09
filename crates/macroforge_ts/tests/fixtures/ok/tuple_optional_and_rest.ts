/** Each element of a tuple is coded as its own type, an optional one and the rest among them. */
/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export type Row = [label: string, count?: number, ...flags: boolean[]];

/** @derive(Encode, Decode) */
export interface Named {
    name: string;
}

/** @derive(Encode, Decode) */
export interface Dated {
    at: Date;
}

/** @derive(Encode, Decode) */
export interface Holder {
    mixed: [Named, ...Dated[]];
    later: [Named, Dated?, ...Date[]];
}
