/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Email {
    /** @switchController({ label: "Can Email" }) */
    canEmail: boolean;
    /** @textController({ label: "Email" }) */
    /** @endec({ validate: ["nonEmpty", "email"] }) */
    emailString: string;
}
