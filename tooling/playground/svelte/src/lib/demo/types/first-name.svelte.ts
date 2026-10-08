/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface FirstName {
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
}
