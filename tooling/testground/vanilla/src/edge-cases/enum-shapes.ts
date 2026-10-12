/**
 * Enums by how their values come about: counted, negative, fractional,
 * computed, of both kinds at once, repeated, lone and with text that needs
 * escaping, and the fields and collections that hold them.
 */

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export enum EnumCounted {
    /** @default */
    First,
    Second,
    Third
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export enum EnumSigned {
    Below = -1,
    /** @default */
    Level = 0,
    Above = 1,
    Half = 0.5
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export enum EnumFlags {
    /** @default */
    None = 0,
    Read = 1 << 0,
    Write = 1 << 1,
    All = Read | Write
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export enum EnumMixed {
    /** @default */
    Zero = 0,
    Text = 'text',
    Five = 5,
    Other = 'other'
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export enum EnumRepeated {
    /** @default */
    One = 1,
    Uno = 1,
    Two = 2
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export enum EnumLone {
    /** @default */
    Only = 'only'
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export enum EnumText {
    /** @default */
    Empty = '',
    Quoted = 'it\'s "quoted"',
    Spaced = 'two words',
    Slashed = 'back\\slash'
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface EnumHolder {
    counted: EnumCounted;
    mixed: EnumMixed;
    flags: EnumFlags;
    maybe: EnumCounted | null;
    later: EnumText | undefined;
    optional?: EnumSigned;
    list: EnumMixed[];
    set: Set<EnumCounted>;
    byName: Map<string, EnumText>;
    byKind: Map<EnumText, number>;
    labels: Record<string, EnumSigned>;
    pair: [EnumCounted, EnumText];
}

/** @derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord) */
export interface EnumOrdered {
    counted: EnumCounted;
    mixed: EnumMixed;
    text: EnumText;
}
