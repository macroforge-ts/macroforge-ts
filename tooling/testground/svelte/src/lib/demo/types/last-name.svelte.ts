/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface LastName {
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
}
