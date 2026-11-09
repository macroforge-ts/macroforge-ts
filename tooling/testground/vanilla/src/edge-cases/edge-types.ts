/**
 * The types the edge-case modules build on. They live in a module of their
 * own so those modules reach every generated function across an import.
 */

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export interface EdgeItem {
    id: string;
    score: number;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class EdgeCard {
    title: string = '';
    items: EdgeItem[] = [];
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
/** @endec({ renameAll: "snake_case" }) */
export interface EdgeProfile {
    displayName: string;
    /** @endec({ rename: "mail", validate: ["email"] }) */
    email: string;
    joinedAt: Date;
    tags: string[];
    nickname?: string;
    /** @endec({ default: 1 }) */
    level: number;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
/** @endec({ denyUnknownFields: true }) */
export interface EdgeStrict {
    id: string;
    note: string;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface EdgeLabeled {
    label: string;
    /** @endec(flatten) */
    origin: EdgeItem;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface EdgeNode {
    name: string;
    next: EdgeNode | null;
}
