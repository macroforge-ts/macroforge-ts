/** @derive(Clone, PartialEq, Eq, PartialOrd, Ord) */
export type Level = 1 | 2 | 3;

/** @derive(Clone, PartialEq, Eq, PartialOrd, Ord) */
export type Mode = "light" | "dark";

/** @derive(Clone, PartialEq, Eq, PartialOrd, Ord) */
export type Pair = [string, number];

/** @derive(Clone, PartialEq, Eq, PartialOrd, Ord) */
export type Meters = $Newtype<number>;
