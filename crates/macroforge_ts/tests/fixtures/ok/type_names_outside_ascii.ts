/** A type named outside ASCII gets its functions and the object that groups them. */
/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */
export interface Ünïcode {
    id: string;
}

/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */
export interface ΑΒΓ {
    id: string;
}

/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */
export interface Holder {
    named: Ünïcode[];
    greek: ΑΒΓ | null;
}
