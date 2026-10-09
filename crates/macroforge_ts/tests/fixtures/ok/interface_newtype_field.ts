/** @derive(Encode, Decode) */
/** @endec(nonNegative) */
type Meters = $Newtype<number>;

/** @derive(Encode, Decode) */
interface Run {
  distance: Meters;
  splits: Meters[];
  best?: Meters;
}
