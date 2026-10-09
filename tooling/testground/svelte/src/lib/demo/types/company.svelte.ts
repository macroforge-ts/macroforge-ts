/** import macro {Gigaform} from "@testground/macro"; */

import type { ColorsConfig } from './colors-config.svelte';
import type { PhoneNumber } from './phone-number.svelte';
import type { Site } from './site.svelte';
import type { TaxRate } from './tax-rate.svelte';

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Company {
    id: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    legalName: string;
    /** @default("") */
    headquarters: string | Site;
    phones: Array<PhoneNumber>;
    /** @endec({ validate: ["nonEmpty"] }) */
    fax: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    email: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    website: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    taxId: string;
    referenceNumber: number;
    /** @endec({ validate: ["nonEmpty"] }) */
    postalCodeLookup: string;
    timeZone: string;
    /** @default("") */
    defaultTax: string | TaxRate;
    /** @endec({ validate: ["nonEmpty"] }) */
    defaultTaxLocation: string;
    defaultAreaCode: number;
    /** @endec({ validate: ["nonEmpty"] }) */
    defaultAccountType: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    lookupFormatting: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    accountNameFormat: string;
    merchantServiceProvider: string | null;
    /** @endec({ validate: ["nonEmpty"] }) */
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
