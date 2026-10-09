/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type JobTitle =
    | /** @default */ 'Technician'
    | 'SalesRepresentative'
    | 'HumanResources'
    | 'InformationTechnology';
