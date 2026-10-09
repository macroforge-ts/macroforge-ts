/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export type Table =
    | /** @default */ 'Account'
    | 'Did'
    | 'Appointment'
    | 'Lead'
    | 'TaxRate'
    | 'Site'
    | 'Employee'
    | 'Route'
    | 'Company'
    | 'Product'
    | 'Service'
    | 'User'
    | 'Order'
    | 'Payment'
    | 'Package'
    | 'Promotion'
    | 'Represents'
    | 'Ordered';
