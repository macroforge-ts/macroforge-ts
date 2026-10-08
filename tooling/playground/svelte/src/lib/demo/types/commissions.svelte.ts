/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Commissions {
    /** @endec({ validate: ["nonEmpty"] }) */
    technician: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    salesRep: string;
}
