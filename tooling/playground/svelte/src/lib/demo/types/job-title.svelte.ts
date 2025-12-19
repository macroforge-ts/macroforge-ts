/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type JobTitle =
    | /** @default */ 'Technician'
    | 'SalesRepresentative'
    | 'HumanResources'
    | 'InformationTechnology';
