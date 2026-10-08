/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Address {
    /** @endec({ validate: ["nonEmpty"] }) */
    street: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    city: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    state: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    zipcode: string;
}
