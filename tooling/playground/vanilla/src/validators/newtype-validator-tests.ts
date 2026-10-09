/**
 * Newtype test types: `$Newtype` brands with alias-level validators, decoded
 * on their own and as fields of a derived interface.
 */

/** @derive(Encode, Decode, Default, Debug, Clone, PartialEq, Hash, Ord) */
/** @endec(nonNegative, finite) */
/** @default(0) */
export type Meters = $Newtype<number>;

/** @derive(Encode, Decode) */
/** @endec(nonEmpty, maxLength(16)) */
export type Username = $Newtype<string>;

/** @derive(Encode, Decode) */
export type Port = number;

/** @derive(Encode, Decode) */
/** @endec(nonNegativeBigInt) */
export type Cents = $Newtype<bigint>;

/** @derive(Encode, Decode) */
export interface Run {
    runner: Username;
    distance: Meters;
    splits: Meters[];
    best?: Meters;
}
