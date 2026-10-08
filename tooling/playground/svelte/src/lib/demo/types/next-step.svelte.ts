/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type NextStep =
    | /** @default */ 'InitialContact'
    | 'Qualified'
    | 'Estimate'
    | 'Negotiation';
