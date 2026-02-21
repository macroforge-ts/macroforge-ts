/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Applications =
    | /** @default */ 'Sales'
    | 'Accounting'
    | 'Errand'
    | 'HumanResources'
    | 'Logistics'
    | 'Marketing'
    | 'Website';
