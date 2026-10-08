/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface PersonName {
    /** @textController({ label: "First Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @textController({ label: "Last Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    lastName: string;
}
