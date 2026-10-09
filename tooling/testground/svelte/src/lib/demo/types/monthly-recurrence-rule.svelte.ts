/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface MonthlyRecurrenceRule {
    quantityOfMonths: number;
    day: number;
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
}
