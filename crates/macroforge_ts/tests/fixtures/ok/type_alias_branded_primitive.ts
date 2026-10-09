declare const MetersBrand: unique symbol;

/** @derive(Encode, Decode, Default) */
/** @endec(nonNegative) */
/** @default(0) */
type Meters = number & {
  readonly [MetersBrand]: true;
};
