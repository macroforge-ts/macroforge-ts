/**
 * A type an ECMAScript module declares, derived on in place and held by a
 * TypeScript module that imports it by the `.mjs` name it compiles to.
 */

/** @derive(Debug, Clone, PartialEq, Hash, Default, Encode, Decode) */
export interface KindedPoint {
    x: number;
    y: number;
}
