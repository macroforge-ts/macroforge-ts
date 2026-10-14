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
