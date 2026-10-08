/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type RowHeight =
    | 'ExtraSmall'
    | 'Small'
    | /** @default */ 'Medium'
    | 'Large';
