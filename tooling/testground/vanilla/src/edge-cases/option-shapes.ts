/**
 * Every option a derive's decorator takes, each on a field of its own and in
 * the combinations that interact: a skipped field beside a default, a renamed
 * field under `renameAll`, a flattened type with renames of its own.
 */

import type { EdgeItem, EdgeProfile } from './edge-types';

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord) */
export interface OptionTraits {
    id: string;
    /** @debug({ rename: "shown" }) */
    label: string;
    /** @debug({ skip: true }) */
    secret: string;
    /** @partialEq({ skip: true }) @hash({ skip: true }) @ord({ skip: true }) */
    cached: number;
    /** @ord({ skip: true }) */
    unordered: string;
}

/** @derive(Default) */
export interface OptionDefaults {
    /** @default("light") */
    theme: string;
    /** @default(10) */
    pageSize: number;
    /** @default(true) */
    enabled: boolean;
    /** @default(["a", "b"]) */
    tags: string[];
    /** @default({ id: "seed", score: 1 }) */
    item: EdgeItem;
    /** @default(null) */
    maybe: EdgeItem | null;
    /** @default(new Date(0)) */
    at: Date;
    /** @default(10n) */
    total: bigint;
    plain: number;
    optional?: string;
}

/** @derive(Default) */
export class OptionDefaultsClass {
    /** @default("light") */
    theme: string = 'dark';
    pageSize: number = 25;
    /** @default(new Map([["a", 1]])) */
    counts: Map<string, number> = new Map();
    label?: string;
}

export function upperCase(text: string): string {
    return text.toUpperCase();
}

export function lowerCase(raw: unknown): string {
    return String(raw).toLowerCase();
}

/** @derive(Encode, Decode) */
export interface OptionFields {
    /** @endec({ rename: "user_name" }) */
    name: string;
    /** @endec({ skip: true }) */
    skipped?: string;
    /** @endec({ skipEncoding: true }) */
    writeOnly: string;
    /** @endec({ skipDecoding: true, default: "\"computed\"" }) */
    readOnly: string;
    /** @endec({ default: true }) */
    count: number;
    /** @endec({ default: "\"pending\"" }) */
    status: string;
    /** @endec({ default: "[]" }) */
    tags: string[];
    /** @endec({ default: true }) */
    item: EdgeItem;
    /** @endec({ format: "decimal" }) */
    balance: number;
    /** @endec({ format: "decimal" }) */
    optionalBalance?: number;
    /** @endec({ encodeWith: "upperCase", decodeWith: "lowerCase" }) */
    shout: string;
    /** @endec({ encodeWith: "upperCase", decodeWith: "lowerCase" }) */
    optionalShout?: string;
}

/** @derive(Encode, Decode) */
/** @endec({ renameAll: "SCREAMING_SNAKE_CASE" }) */
export interface OptionScreaming {
    firstName: string;
    /** @endec({ rename: "kept" }) */
    lastName: string;
    homeAddress2: string;
}

/** @derive(Encode, Decode) */
/** @endec({ renameAll: "kebab-case" }) */
export interface OptionKebab {
    firstName: string;
    isActive: boolean;
    items: EdgeItem[];
}

/** @derive(Encode, Decode) */
/** @endec({ renameAll: "PascalCase" }) */
export class OptionPascal {
    firstName: string = '';
    homeTown: string | null = null;
}

/** @derive(Encode, Decode) */
/** @endec({ renameAll: "camelCase" }) */
export interface OptionCamel {
    first_name: string;
    home_town: string;
}

/** @derive(Encode, Decode) */
export interface OptionFlattened {
    title: string;
    /** @endec(flatten) */
    item: EdgeItem;
    /** @endec(flatten) */
    profile: EdgeProfile;
}

/** @derive(Encode, Decode) */
export interface OptionFlattenedOptional {
    title: string;
    /** @endec(flatten) */
    item?: EdgeItem;
    /** @endec(flatten) */
    extra: Record<string, number>;
}

/** @derive(Encode, Decode) */
/** @endec({ denyUnknownFields: true, renameAll: "snake_case" }) */
export interface OptionStrict {
    firstName: string;
    item: EdgeItem;
    nested?: EdgeItem;
}

/** @derive(Encode, Decode) */
/** @endec({ tag: "kind" }) */
export interface OptionTagged {
    name: string;
    child: OptionTagged | null;
}

/** @derive(Encode, Decode) */
export interface OptionValidated {
    /** @endec({ validate: ["email"] }) */
    email: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    optionalName?: string;
    /** @endec({ validate: ["positive"] }) */
    nullableCount: number | null;
    /** @endec({ validate: ["int"] }) */
    laterCount: number | undefined;
    /** @endec({ validate: ["minItems(1)"] }) */
    tags: string[];
    /** @endec({ validate: ["validDate"] }) */
    at: Date;
    /** @endec({ validate: ["validDate"] }) */
    maybeAt: Date | null;
    /** @endec({ default: 3, validate: ["positive"] }) */
    retries: number;
}
