/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Commented {
    /** @endec({ validate: ["nonEmpty"] }) */
    comment: string;
    replyTo: string | null;
}
