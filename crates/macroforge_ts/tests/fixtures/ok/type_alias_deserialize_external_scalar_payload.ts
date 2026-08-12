// An externally-tagged variant whose payload is a link type: serializable in
// its own right, but carried as a bare id string as often as an object. The
// payload has to survive decoding either way — coercing the non-object case
// drops the link while still reporting success.

/** @derive(Serialize, Deserialize) */
export type Ref = string | { id: string };

/** @derive(Serialize, Deserialize) */
/** @serde({ externallyTagged: true }) */
export type Stage = 'Active' | { Invoice: Ref };
