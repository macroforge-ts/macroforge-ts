/**
 * Each built-in derive on the kinds of declaration no other edge case derives
 * it on: Default on an enum, the orderings on an object alias, and the value
 * traits on an intersection alias.
 */

/** @derive(Default) */
export enum KindShade {
    Dark = 'dark',
    /** @default */
    Light = 'light'
}

/** @derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash) */
export type KindPoint = { x: number; y: number };

/** @derive(Debug, Clone, PartialEq, Hash) */
export interface KindNamed {
    name: string;
}

/** @derive(Debug, Clone, PartialEq, Hash) */
export interface KindAged {
    age: number;
}

/** @derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash) */
export type KindPerson = KindNamed & KindAged;
