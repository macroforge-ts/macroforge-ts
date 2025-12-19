/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Edited {
    /** @serde({ validate: ["nonEmpty"] }) */
    fieldName: string;
    oldValue: string | null;
    newValue: string | null;
}
