/** @derive(Encode, Decode) */
export interface Fields {
    /** @endec({ rename: 5 }) */
    numbered: string;
    /** @endec({ skip: "yes" }) */
    worded?: string;
    /** @endec({ rename: "" }) */
    unnamed: string;
    /** @endec({ format: "decimal" }) */
    spelled: string;
    /** @endec({ validate: [5, email] }) */
    listed: string;
    /** @endec({ validate: ["between(5, 1)"] }) */
    ranged: number;
    /** @endec({ validate: ["length(4, 2)"] }) */
    sized: string;
    /** @endec({ tag: "kind" }) */
    tagged: string;
    /** @endec({ rename: "a" }) @endec({ rename: "b" }) */
    twice: string;
    /** @endec({ rename: "skip", default: "\"flatten, (skip)\"" }) */
    kept: string;
    /** @endec({ encodeWith: "open(", decodeWith: "String" }) */
    unclosed: string;
}

/** @derive(Encode, Decode) */
/** @endec({ renameAll: "weird" }) */
export interface Cased {
    firstName: string;
}

/** @derive(Encode, Decode) */
/** @endec({ tag: 5, denyUnknownFields: "yes" }) */
export interface Numbered {
    firstName: string;
}

/** @derive(Encode, Decode) */
/** @endec({ skip: true }) */
export interface Skipped {
    firstName: string;
}

/** @derive(Encode, Decode) */
/** @endec({ untagged: true }) */
export interface Untagged {
    firstName: string;
}

/** @derive(Encode, Decode) */
/** @endec({ tag: "kind", untagged: true }) */
export type Both = { kind: 'a' } | { kind: 'b' };

/** @derive(Encode, Decode) */
/** @endec({ content: "value" }) */
export type Content = { kind: 'a' } | { kind: 'b' };

/** @derive(Encode, Decode) */
/** @endec({ tag: "t", content: "t" }) */
export type Same = Cased | Numbered;

/** @derive(Encode, Decode) */
/** @endec({ renameAll: "snake_case" }) */
/** @endec({ renameAll: "kebab-case" }) */
export interface Twice {
    firstName: string;
}

/** @derive(Encode, Decode) */
/** @endec({ tag: "kind" }) */
export enum Kind {
    /** @endec({ rename: "first" }) */
    A = 'a'
}
