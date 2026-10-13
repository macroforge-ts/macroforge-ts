/** A type is coded through its functions whatever letter its name starts with. */
/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface lower {
    id: string;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface point2D {
    x: number;
    y: number;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface my_type {
    at: Date;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface Holder {
    held: lower;
    all: lower[];
    maybe: my_type | null;
    left?: point2D;
    byKey: Map<string, point2D>;
    byName: Record<string, my_type>;
    some: Partial<lower>;
    pair: [lower, point2D];
}
