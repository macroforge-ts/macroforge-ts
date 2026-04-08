interface UnknownRecord { source: string }
interface PersonRecord { name: string }

/** @derive(Default) */
export type EntityRecord =
  | /** @default */ ({ variant: 'Unknown' } & UnknownRecord)
  | ({ variant: 'Person' } & PersonRecord);
