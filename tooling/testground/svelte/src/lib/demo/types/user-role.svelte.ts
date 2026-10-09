/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type UserRole =
    | /** @default */ 'Administrator'
    | 'SalesRepresentative'
    | 'Technician'
    | 'HumanResources'
    | 'InformationTechnology';
