/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type LeadStage =
    | /** @default */ 'Open'
    | 'InitialContact'
    | 'Qualified'
    | 'Estimate'
    | 'Negotiation';
