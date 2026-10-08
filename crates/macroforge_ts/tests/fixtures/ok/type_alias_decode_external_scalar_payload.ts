// An externally-tagged variant whose payload is a link type: encodable in
// its own right, but carried as a bare id string as often as an object. The
// payload has to survive decoding either way: coercing the non-object case
// drops the link while still reporting success.

/** @derive(Encode, Decode) */
export type Ref = string | { id: string };

/** @derive(Encode, Decode) */
/** @endec({ externallyTagged: true }) */
export type Stage = 'Active' | { Invoice: Ref };
