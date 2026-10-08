/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Package {
    /** @hiddenController({}) */
    id: string;
    /** @dateTimeController({ label: "Date" }) */
    date: string;
}
