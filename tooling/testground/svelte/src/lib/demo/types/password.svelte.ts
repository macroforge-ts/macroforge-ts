/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Password {
    /** @endec({ validate: ["nonEmpty"] }) */
    password: string;
}
