/** @derive(Encode, Decode) */
/** @endec(nonNegative, finite) */
export type Meters = $Newtype<number>;

/** @derive(Encode, Decode, Default) */
/** @endec(nonEmpty, maxLength(64)) */
/** @default("guest") */
export type Username = $Newtype<string>;

/** @derive(Decode) */
export type Flag = $Newtype<boolean>;

export type Either = $Newtype<string | number>;

/** @derive(Encode, Decode, Hash, Debug) */
/** @endec(nonNegativeBigInt) */
export type Cents = $Newtype<bigint>;
