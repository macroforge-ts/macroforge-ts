/**
 * Newtypes over each primitive under every derive, and the fields and
 * collections that hold one: as an element, a map's key or value, a record's
 * value, nullable, optional and nested in another newtype.
 */

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export type NewText = $Newtype<string>;

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export type NewCount = $Newtype<number>;

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export type NewTotal = $Newtype<bigint>;

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export type NewFlag = $Newtype<boolean>;

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
/** @endec(nonEmpty, lowercase) */
/** @default("slug") */
export type NewSlug = $Newtype<string>;

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export type NewNested = $Newtype<$Newtype<number>>;

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export type NewPlain = string;

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface NewHolder {
    text: NewText;
    count: NewCount;
    flag: NewFlag;
    slug: NewSlug;
    nested: NewNested;
    plain: NewPlain;
    maybe: NewSlug | null;
    later: NewCount | undefined;
    optional?: NewText;
    list: NewSlug[];
    set: Set<NewText>;
    bySlug: Map<NewSlug, NewCount>;
    record: Record<string, NewCount>;
    pair: [NewText, NewCount];
}

/** @derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord) */
export interface NewOrdered {
    slug: NewSlug;
    count: NewCount;
    total: NewTotal;
    flag: NewFlag;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface NewLedger {
    total: NewTotal;
    totals: NewTotal[];
}
