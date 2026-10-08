/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type WeekOfMonth =
    | /** @default */ 'First'
    | 'Second'
    | 'Third'
    | 'Fourth'
    | 'Last';
