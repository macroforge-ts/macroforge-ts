/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Number {
    /** @endec({ validate: ["nonEmpty"] }) */
    countryCode: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    areaCode: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    localNumber: string;
}
