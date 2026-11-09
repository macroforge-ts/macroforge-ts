/**
 * Names that can collide with what a derive generates or that need quoting:
 * fields named like the generated functions' own locals, keys that are not
 * identifiers, reserved words, type names that are acronyms or lowercase, and
 * type parameters named like the ones generated signatures declare.
 */

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export interface NameLocals {
    value: string;
    ctx: string;
    result: string;
    obj: string;
    errors: string[];
    instance: string;
    fields: string;
    hash: number;
    parts: string[];
    a: number;
    b: number;
    input: string;
    opts: string;
    data: string;
    item: string;
    key: string;
    idx: number;
    other: string;
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export interface NameQuoted {
    'kebab-key': string;
    'with space': number;
    'dotted.key': boolean;
    0: string;
    class: string;
    default: number;
    new: boolean;
    function: string;
    typeof: string;
    $dollar: string;
    _underscore: string;
    ünïcode: string;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export class NameQuotedClass {
    'kebab-key': string = '';
    class: string = '';
    default: number = 0;
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export interface HTTPResponse {
    statusCode: number;
}

/** @derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Encode, Decode) */
export interface point2D {
    x: number;
    y: number;
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface NameHolder {
    response: HTTPResponse;
    responses: HTTPResponse[];
    point: point2D;
    points: Map<string, point2D>;
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export interface NameParams<K, Key, T, Value> {
    key: K;
    other: Key;
    held: T;
    values: Value[];
}

/** @derive(Debug, Clone, PartialEq, Hash, Encode, Decode) */
export class NameParamsClass<K, Key, T> {
    key: K | null = null;
    other: Key | null = null;
    held: T[] = [];
}

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface NamePrototype {
    constructor: string;
    toString: string;
    valueOf: number;
    hasOwnProperty: boolean;
    length: number;
    then: string;
}
