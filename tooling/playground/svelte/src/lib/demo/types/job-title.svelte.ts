/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type JobTitle =
    | /** @default */ 'Technician'
    | 'SalesRepresentative'
    | 'HumanResources'
    | 'InformationTechnology';
