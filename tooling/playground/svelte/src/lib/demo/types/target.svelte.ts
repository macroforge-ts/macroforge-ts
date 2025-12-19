/** import macro {Gigaform} from "@playground/macro"; */

import type { User } from './user.svelte';
import type { Service } from './service.svelte';
import type { Promotion } from './promotion.svelte';
import type { Site } from './site.svelte';
import type { Product } from './product.svelte';
import type { Represents } from './represents.svelte';
import type { Payment } from './payment.svelte';
import type { Appointment } from './appointment.svelte';
import type { Package } from './package.svelte';
import type { Account } from './account.svelte';
import type { Order } from './order.svelte';
import type { TaxRate } from './tax-rate.svelte';
import type { Lead } from './lead.svelte';
import type { Company } from './company.svelte';
import type { Employee } from './employee.svelte';
import type { Route } from './route.svelte';
import type { Ordered } from './ordered.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Target =
    | /** @default */ Account
    | User
    | Employee
    | Appointment
    | Lead
    | TaxRate
    | Site
    | Route
    | Company
    | Product
    | Service
    | Order
    | Payment
    | Package
    | Promotion
    | Represents
    | Ordered;
