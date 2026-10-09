/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface ServiceDefaults {
    /** @numberController({ label: "Price", min: 0, step: 0.01 }) */
    price: number;
    /** @textAreaController({ label: "Description" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    description: string;
}
