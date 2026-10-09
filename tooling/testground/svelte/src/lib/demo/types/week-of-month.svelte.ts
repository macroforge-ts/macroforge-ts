/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type WeekOfMonth =
    | /** @default */ 'First'
    | 'Second'
    | 'Third'
    | 'Fourth'
    | 'Last';
