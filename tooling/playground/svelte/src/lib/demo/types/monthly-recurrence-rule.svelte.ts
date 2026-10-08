/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface MonthlyRecurrenceRule {
    quantityOfMonths: number;
    day: number;
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
}
