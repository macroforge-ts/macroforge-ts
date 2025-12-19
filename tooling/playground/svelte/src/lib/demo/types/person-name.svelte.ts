/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface PersonName {
    /** @textController({ label: "First Name" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @textController({ label: "Last Name" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    lastName: string;
}
