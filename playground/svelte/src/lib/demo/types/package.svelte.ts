/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Package {
    /** @hiddenController({}) */
    id: string;
    /** @dateTimeController({ label: "Date" }) */
    date: string;
}
