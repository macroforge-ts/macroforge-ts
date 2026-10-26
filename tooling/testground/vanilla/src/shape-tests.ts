/**
 * Field and alias shapes every derive has to generate type-correct code for:
 * nullable elements, readonly collections and fields, tuples, enums of both
 * kinds, unions that mix primitives, `null`, interfaces and classes, and
 * values that hold only some of a type's fields.
 */

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export interface ShapeItem {
    id: string;
    score: number;
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export enum ShapeLevel {
    /** @default */
    Low = 'low',
    High = 'high'
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export enum ShapeRank {
    /** @default */
    One = 1,
    Two = 2
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface ShapeFields {
    note: string | undefined;
    maybe: string | null;
    pair: [string, number];
    nullableTexts: (string | null)[];
    nullableItems: Array<ShapeItem | null>;
    parenthesizedItems: (ShapeItem | null)[];
    readonlyItems: ReadonlyArray<ShapeItem>;
    readonlyNumbers: readonly number[];
    readonlyTags: ReadonlySet<string>;
    readonlyCounts: ReadonlyMap<string, number>;
    level: ShapeLevel;
    rank: ShapeRank;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class ShapeCard {
    readonly title: string = '';
    items: ShapeItem[] = [];
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type ShapeMember = ShapeItem | ShapeCard;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export type ShapeMixed = string | number | ShapeItem | null;

/** @derive(Encode, Decode) */
export interface ShapeEvent {
    id: string;
    at: Date;
    tags: string[];
    owner?: ShapeItem;
}

/** @derive(Encode, Decode) */
export interface ShapeDraft {
    patch: Partial<ShapeEvent>;
    header: Pick<ShapeEvent, 'id' | 'at'>;
    rest: Omit<ShapeEvent, 'at'>;
    card: Partial<ShapeCard>;
    frozen: Readonly<ShapeItem>;
}
