/** @derive(Decode) */
type Contact = /** @endec(email) */ string | number;

/** @derive(Decode) */
interface Phone {
  digits: string;
}

/** @derive(Decode) */
type Reachable =
  | /** @endec(nonEmpty, maxLength(32)) */ string
  | Phone;
