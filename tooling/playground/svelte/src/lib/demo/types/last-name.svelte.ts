/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface LastName {
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
}
