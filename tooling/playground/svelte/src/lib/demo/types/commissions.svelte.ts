/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Commissions {
    /** @serde({ validate: ["nonEmpty"] }) */
    technician: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    salesRep: string;
}
