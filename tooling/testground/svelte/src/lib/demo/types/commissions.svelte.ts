/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Commissions {
    /** @endec({ validate: ["nonEmpty"] }) */
    technician: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    salesRep: string;
}
