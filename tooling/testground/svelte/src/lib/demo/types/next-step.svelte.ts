/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type NextStep =
    | /** @default */ 'InitialContact'
    | 'Qualified'
    | 'Estimate'
    | 'Negotiation';
