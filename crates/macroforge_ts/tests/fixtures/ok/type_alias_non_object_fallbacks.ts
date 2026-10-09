/** @derive(Clone, PartialOrd, Ord) */
export type Level = 1 | 2 | 3;

/** @derive(Clone, PartialOrd, Ord) */
export type Mode = "light" | "dark";

/** @derive(Clone, PartialOrd, Ord) */
export type Pair = [string, number];

/** @derive(Clone, PartialOrd, Ord) */
export type Meters = $Newtype<number>;
