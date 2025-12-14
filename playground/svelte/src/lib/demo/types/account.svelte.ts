import { Did } from './did.svelte';
import { PersonName } from './person-name.svelte';
import { Site } from './site.svelte';
import { PhoneNumber } from './phone-number.svelte';
import { Represents } from './represents.svelte';
import { Payment } from './payment.svelte';
import { CompanyName } from './company-name.svelte';
import { Custom } from './custom.svelte';
import { Colors } from './colors.svelte';
import { TaxRate } from './tax-rate.svelte';
import { Lead } from './lead.svelte';
import { Company } from './company.svelte';
import { Ordered } from './ordered.svelte';
import { Email } from './email.svelte';
import { Sector } from './sector.svelte';
import { AccountName } from './account-name.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Account {
    /** @hiddenController({}) */
    id: string;
    /** @comboboxController({ label: "Tax Rate", allowCustom: false, fetchUrls: ["/api/tax-rates"] }) */
    /** @default("") */
    taxRate: string | TaxRate;
    /** @siteFieldsetController({ label: "Site" }) */
    /** @default("") */
    site: string | Site;
    /** @comboboxMultipleController({ label: "Sales Rep", fetchUrls: ["/api/employees"] }) */
    salesRep: Represents[] | null;
    /** @hiddenController({}) */
    orders: Ordered[];
    /** @hiddenController({}) */
    activity: Did[];
    /** @arrayFieldsetController({ legend: "Custom Fields" }) */
    customFields: [string, string][];
    /** @enumFieldsetController({ legend: "Name", variants: { CompanyName: { label: "Company" }, PersonName: { label: "Person" } } }) */
    accountName: AccountName;
    /** @radioGroupController({ label: "Sector", options: [{ label: "Residential", value: "Residential" }, { label: "Commercial", value: "Commercial" }], orientation: "horizontal" }) */
    /** @default("Residential") */
    sector: Sector;
    /** @textAreaController({ label: "Memo" }) */
    memo: string | null;
    /** @arrayFieldsetController({ legend: "Phones" }) */
    phones: PhoneNumber[];
    /** @emailFieldController({ label: "Email" }) */
    email: Email;
    /** @comboboxController({ label: "Lead Source", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    leadSource: string;
    /** @hiddenController({}) */
    colors: Colors;
    /** @toggleController({ label: "Needs Review" }) */
    needsReview: boolean;
    /** @toggleController({ label: "Has Alert" }) */
    hasAlert: boolean;
    /** @comboboxController({ label: "Account Type", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    accountType: string;
    /** @comboboxController({ label: "Subtype", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    subtype: string;
    /** @toggleController({ label: "Tax Exempt" }) */
    isTaxExempt: boolean;
    /** @comboboxController({ label: "Payment Terms", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    paymentTerms: string;
    /** @tagsController({ label: "Tags" }) */
    tags: string[];
    /** @hiddenController({}) */
    dateAdded: string;
}
