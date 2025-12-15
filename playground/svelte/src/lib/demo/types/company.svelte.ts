/** import macro {Gigaform} from "@playground/macro"; */

import type { Site } from './site.svelte';
import type { PhoneNumber } from './phone-number.svelte';
import type { TaxRate } from './tax-rate.svelte';
import type { ColorsConfig } from './colors-config.svelte';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Company {
    id: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    legalName: string;
    /** @default("") */
    headquarters: string | Site;
    phones: PhoneNumber[];
    /** @serde({ validate: ["nonEmpty"] }) */
    fax: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    email: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    website: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    taxId: string;
    referenceNumber: number;
    /** @serde({ validate: ["nonEmpty"] }) */
    postalCodeLookup: string;
    timeZone: string;
    /** @default("") */
    defaultTax: string | TaxRate;
    /** @serde({ validate: ["nonEmpty"] }) */
    defaultTaxLocation: string;
    defaultAreaCode: number;
    /** @serde({ validate: ["nonEmpty"] }) */
    defaultAccountType: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    lookupFormatting: string;
    /** @serde({ validate: ["nonEmpty"] }) */
    accountNameFormat: string;
    merchantServiceProvider: string | null;
    /** @serde({ validate: ["nonEmpty"] }) */
    dateDisplayStyle: string;
    hasAutoCommission: boolean;
    hasAutoDaylightSavings: boolean;
    hasAutoFmsTracking: boolean;
    hasNotifications: boolean;
    hasRequiredLeadSource: boolean;
    hasRequiredEmail: boolean;
    hasSortServiceItemsAlphabetically: boolean;
    hasAttachOrderToAppointmentEmails: boolean;
    scheduleInterval: number;
    colorsConfig: ColorsConfig;
}
