/**
 * Records and collections by the kind of key they are over, an enum, a set
 * of literals or a pattern, whole and under `Partial`, `Pick` and `Omit`,
 * and a class whose fields are typed by their initializers under each
 * decorator option.
 */

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export enum KeyedLevel {
    /** @default */
    Low,
    High
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export enum KeyedKind {
    /** @default */
    A = 'a',
    B = 'b'
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface KeyedRecords {
    byLevel: Record<KeyedLevel, string>;
    byKind: Record<KeyedKind, number>;
    some: Partial<Record<'x' | 'y', number>>;
    levels: Map<KeyedLevel, string>;
    kinds: Set<KeyedKind>;
    codes: Set<`c-${number}`>;
    deep?: { inner?: { at?: Date; tags?: string[] } };
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
/** @endec({ renameAll: "snake_case" }) */
export class KeyedOptions {
    /** @endec({ rename: "n" }) */
    count = 0;
    /** @endec({ default: true }) */
    userLabel = 'none';
    /** @endec({ skipEncoding: true }) */
    hidden = false;
    /** @endec({ skipDecoding: true }) */
    derived = 1;
    /** @endec(positive) */
    weight = 1;
    /** @debug({ skip: true }) */
    secret = 's';
    /** @hash({ skip: true }) */
    noise = 0;
    /** @partialEq({ skip: true }) */
    ignored = 0;
    /** @default(5) */
    five = 0;
    /** @endec({ flatten: true }) */
    inner: KeyedInner = { deepValue: 1 };
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface KeyedInner {
    deepValue: number;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface KeyedSelections {
    partial: Partial<KeyedRecords>;
    picked: Pick<KeyedRecords, 'byKind' | 'codes'>;
    omitted: Omit<KeyedRecords, 'byLevel' | 'levels' | 'deep'>;
    required: Required<Pick<KeyedRecords, 'deep'>>;
}
