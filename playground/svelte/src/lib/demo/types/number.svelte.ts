/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Number {
    /** @serde({ validate: ["nonEmpty"] }) */
    countryCode: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    areaCode: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    localNumber: string;
}
