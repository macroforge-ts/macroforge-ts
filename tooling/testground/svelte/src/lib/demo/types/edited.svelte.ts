/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Edited {
    /** @endec({ validate: ["nonEmpty"] }) */
    fieldName: string;
    oldValue: string | null;
    newValue: string | null;
}
