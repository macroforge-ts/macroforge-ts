/**
 * Types a module exports under a name other than the one they are declared
 * by: a default export, and a declaration exported under an alias.
 */

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export default class NameModel {
    title: string = '';
    at: Date = new Date(0);
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
interface NameLate {
    note: string;
    at: Date;
}

export type { NameLate as NameShown };
