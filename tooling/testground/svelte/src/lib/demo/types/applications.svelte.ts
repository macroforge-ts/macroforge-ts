/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type Applications =
    | /** @default */ 'Sales'
    | 'Accounting'
    | 'Errand'
    | 'HumanResources'
    | 'Logistics'
    | 'Marketing'
    | 'Website';
