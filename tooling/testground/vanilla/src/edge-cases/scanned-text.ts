/**
 * Type text a derive reads by scanning it: a literal ending in an escaped
 * backslash, a function whose return type is a union, a template literal
 * alias whose parameter is substituted where it is used, and an empty pattern.
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
