/** @derive(Encode, Decode) */
export type Id<T> = $Newtype<string>;

export type Deep = $Newtype<$Newtype<number>>;
