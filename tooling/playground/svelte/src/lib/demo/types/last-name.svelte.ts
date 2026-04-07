/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface LastName {
    /** @serde({ validate: ["nonEmpty"] }) */
    name: string;
}
