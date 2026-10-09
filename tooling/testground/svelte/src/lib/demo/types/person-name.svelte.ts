/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface PersonName {
    /** @textController({ label: "First Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @textController({ label: "Last Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    lastName: string;
}
