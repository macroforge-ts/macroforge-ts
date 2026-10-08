/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface CompanyName {
    /** @textController({ label: "Company Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    companyName: string;
}
