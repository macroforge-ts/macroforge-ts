/** import macro {Gigaform} from "@playground/macro"; */

import { User } from './user.svelte';
import { Service } from './service.svelte';
import { Did } from './did.svelte';
import { Promotion } from './promotion.svelte';
import { Site } from './site.svelte';
import { Product } from './product.svelte';
import { Represents } from './represents.svelte';
import { Payment } from './payment.svelte';
import { Appointment } from './appointment.svelte';
import { Package } from './package.svelte';
import { Account } from './account.svelte';
import { Order } from './order.svelte';
import { TaxRate } from './tax-rate.svelte';
import { Lead } from './lead.svelte';
import { Company } from './company.svelte';
import { Employee } from './employee.svelte';
import { Route } from './route.svelte';
import { Ordered } from './ordered.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
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
