/**
 * A derive whose generated code does not type-check in several places for one
 * reason: `Plain` derives no coder, and `Holder` calls for one at each field.
 * The type checker's errors all map to the one `@derive`, and are read as one.
 */

export interface Plain {
    id: string;
}

/** @derive(Decode) */
export interface Holder {
    first: Plain;
    second: Plain;
    third: Plain;
}
