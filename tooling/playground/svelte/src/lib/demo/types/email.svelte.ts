/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Email {
    /** @switchController({ label: "Can Email" }) */
    canEmail: boolean;
    /** @textController({ label: "Email" }) */
    /** @serde({ validate: ["nonEmpty", "email"] }) */
    emailString: string;
}
