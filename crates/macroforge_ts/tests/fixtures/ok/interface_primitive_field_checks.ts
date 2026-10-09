/** @derive(Decode) */
interface Profile {
  name: string;
  age: number;
  active: boolean;
  kind: "user";
  mode: "light" | "dark";
  nickname: string | null;
  bio?: string;
  rank: number | undefined;
  height: $Newtype<number>;
  balance: bigint;
  site: URL;
}

/** @derive(Encode, Decode, Hash, Debug) */
type Tagged = number & { readonly __brand: "Tagged" };

/** @derive(Decode) */
type Loose = string & {};
