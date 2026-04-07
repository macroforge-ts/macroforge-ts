/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface MonthlyRecurrenceRule {
    quantityOfMonths: number;
    day: number;
    /** @serde({ validate: ["nonEmpty"] }) */
    name: string;
}
