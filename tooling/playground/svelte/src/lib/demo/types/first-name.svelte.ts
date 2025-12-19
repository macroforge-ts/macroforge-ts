/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface FirstName {
    /** @serde({ validate: ["nonEmpty"] }) */
    name: string;
}
