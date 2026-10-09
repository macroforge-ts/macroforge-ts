/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface FirstName {
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
}
