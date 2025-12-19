/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Address {
    /** @serde({ validate: ["nonEmpty"] }) */
    street: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    city: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    state: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    zipcode: string;
}
