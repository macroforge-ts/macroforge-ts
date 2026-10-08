/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type LeadStage =
    | /** @default */ 'Open'
    | 'InitialContact'
    | 'Qualified'
    | 'Estimate'
    | 'Negotiation';
