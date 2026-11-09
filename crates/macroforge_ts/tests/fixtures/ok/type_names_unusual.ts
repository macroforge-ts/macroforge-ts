/** A type's functions are named after it whatever its name opens with. */
/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */
export interface _Hidden {
    id: string;
}

/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */
export interface _1 {
    id: string;
}

/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */
export type HTTPStatus = 'ok' | 'gone';

/** @derive(Debug, Clone, PartialEq, Default, Encode, Decode) */
export interface Holder {
    hidden: _Hidden;
    first: _1;
    status: HTTPStatus | null;
}
