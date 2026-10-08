/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Commented {
    /** @endec({ validate: ["nonEmpty"] }) */
    comment: string;
    replyTo: string | null;
}
