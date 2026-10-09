/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface CompanyName {
    /** @textController({ label: "Company Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    companyName: string;
}
