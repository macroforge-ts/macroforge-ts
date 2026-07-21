/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Password {
    /** @serde({ validate: ["nonEmpty"] }) */
    password: string;
}
