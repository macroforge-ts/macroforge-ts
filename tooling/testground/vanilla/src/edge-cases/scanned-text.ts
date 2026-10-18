/**
 * Type text a derive reads by scanning it: a literal ending in an escaped
 * backslash, a function whose return type is a union, a template literal
 * alias whose parameter is substituted where it is used, an empty pattern, and
 * types in parentheses or holding brackets in their strings.
 */

export type ScannedKey<T extends string> = `key-${T}`;

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface ScannedText {
    separator: '\\' | '/';
    key: ScannedKey<'a' | 'b'>;
    /** @endec({ validate: ['pattern("")'] }) */
    note: string;
}

/** @derive(Debug, Clone, PartialEq) */
export interface ScannedHandlers {
    pick: () => string | number;
    when: (value: string) => value is 'yes' | 'no';
}

/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */
export enum ScannedShade {
    /** @default */
    Light = 'light',
    Dark = 'dark'
}

/** @derive(Debug, PartialEq, Default, Encode, Decode) */
export interface ScannedGroups {
    shade: ScannedShade;
    either: (string) | Record<string, ''>;
    pair: [']?)', 4?];
}

/** @derive(Debug, PartialEq, Encode, Decode) */
export interface ScannedUnions {
    pair: string | [number, string?];
    rest: string | ['a', ...number[]];
}

/** @derive(Debug, PartialEq, Encode, Decode) */
export interface ScannedBigints {
    one: 1n;
    some: -1n | 1_000n;
    maybe: 2n | null;
}

/** @derive(Debug, PartialEq, Encode, Decode) */
export interface ScannedNothing {
    none?: undefined;
    other: string;
}

/** @derive(Debug, Clone, PartialEq, Default) */
export interface ScannedPicks {
    pick: ((value: string) => string)[] | null;
    picked: { 'key with space': undefined; b?: string } | ReadonlyArray<string>;
}
