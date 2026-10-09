/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Package {
    /** @hiddenController({}) */
    id: string;
    /** @dateTimeController({ label: "Date" }) */
    date: string;
}
