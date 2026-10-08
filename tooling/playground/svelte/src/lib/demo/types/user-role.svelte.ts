/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type UserRole =
    | /** @default */ 'Administrator'
    | 'SalesRepresentative'
    | 'Technician'
    | 'HumanResources'
    | 'InformationTechnology';
